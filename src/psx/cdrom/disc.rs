use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

pub const SECTOR_RAW_SIZE: u64 = 2352;

pub trait Disc: Send {
    fn read_sector(&mut self, lba: u64, out: &mut [u8; SECTOR_RAW_SIZE as usize]) -> Result<(), String>;
    fn sector_count(&self) -> u64;
}

pub fn open(filepath: &str) -> Result<Box<dyn Disc>, String> {
    let path = Path::new(filepath);

    if !path.exists() {
        return Err(format!("file does not exist: {}", path.display()));
    }

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "cue" => {
            let disc = CueDisc::open(path)?;
            Ok(Box::new(disc))
        }
        "ccd" => {
            let disc = CcdDisc::open(path)?;
            Ok(Box::new(disc))
        }
        // .img / .iso / .bin and anything else: treat as a single raw 2352-byte
        // sector image. If there is a sibling .ccd next to the .img, prefer
        // the .ccd because it carries the track/mode information.
        _ => {
            if ext == "img" {
                if let Some(ccd_path) = sibling_ccd(path) {
                    return Ok(Box::new(CcdDisc::open(&ccd_path)?));
                }
            }
            let disc = BinDisc::open(path)?;
            Ok(Box::new(disc))
        }
    }
}

fn sibling_ccd(img_path: &Path) -> Option<PathBuf> {
    let stem = img_path.file_stem()?.to_string_lossy().to_string();
    let parent = img_path.parent()?;
    // Try lower-, upper-, and as-is case for the .ccd extension.
    for ext in &["ccd", "CCD", "Ccd"] {
        let cand = parent.join(format!("{}.{}", stem, ext));
        if cand.exists() {
            return Some(cand);
        }
    }
    None
}

pub struct BinDisc {
    file: fs::File,
    sectors: u64,
}

impl BinDisc {
    pub fn open(path: &Path) -> Result<Self, String> {
        let file = fs::File::open(path).map_err(|e| e.to_string())?;
        let len = file.metadata().map_err(|e| e.to_string())?.len();
        Ok(Self {
            file,
            sectors: len / SECTOR_RAW_SIZE,
        })
    }
}

impl Disc for BinDisc {
    fn read_sector(&mut self, lba: u64, out: &mut [u8; SECTOR_RAW_SIZE as usize]) -> Result<(), String> {
        let offset = lba * SECTOR_RAW_SIZE;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|e| e.to_string())?;
        self.file.read_exact(out).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn sector_count(&self) -> u64 {
        self.sectors
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TrackKind {
    Data,
    Audio,
}

#[derive(Debug)]
struct CueTrack {
    #[allow(dead_code)]
    number: u32,
    #[allow(dead_code)]
    kind: TrackKind,
    file_index: usize,
    sector_size: u64,
    // Disc-wide LBA where track INDEX 01 begins.
    disc_lba: u64,
    // First LBA within the source file for INDEX 01.
    file_lba: u64,
    // Number of sectors in this track within the source file.
    length: u64,
}

pub struct CueDisc {
    files: Vec<fs::File>,
    tracks: Vec<CueTrack>,
    total_sectors: u64,
}

impl CueDisc {
    pub fn open(cue_path: &Path) -> Result<Self, String> {
        let cue_text = fs::read_to_string(cue_path).map_err(|e| e.to_string())?;
        let parent = cue_path.parent().unwrap_or_else(|| Path::new("."));

        let mut files: Vec<fs::File> = Vec::new();
        let mut file_sizes: Vec<u64> = Vec::new();
        let mut tracks: Vec<CueTrack> = Vec::new();

        let mut current_file: Option<usize> = None;
        let mut pending_track: Option<(u32, TrackKind, u64)> = None;
        let mut pending_index0: Option<u64> = None;
        let mut pending_index1: Option<u64> = None;

        for raw_line in cue_text.lines() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }

            let mut it = TokenIter::new(line);
            let keyword = match it.next() {
                Some(k) => k.to_ascii_uppercase(),
                None => continue,
            };

            match keyword.as_str() {
                "FILE" => {
                    // Flush any pending track before changing file.
                    flush_track(
                        &mut tracks,
                        &mut pending_track,
                        &mut pending_index0,
                        &mut pending_index1,
                        current_file,
                    );

                    let filename = it.next().ok_or("FILE entry missing filename")?;
                    let _format = it.next();

                    let mut candidate = PathBuf::from(parent);
                    candidate.push(&filename);

                    let resolved = resolve_path(&candidate).ok_or_else(|| {
                        format!("referenced file not found: {}", candidate.display())
                    })?;

                    let f = fs::File::open(&resolved).map_err(|e| {
                        format!("failed to open {}: {}", resolved.display(), e)
                    })?;
                    let size = f.metadata().map_err(|e| e.to_string())?.len();
                    files.push(f);
                    file_sizes.push(size);
                    current_file = Some(files.len() - 1);
                }
                "TRACK" => {
                    flush_track(
                        &mut tracks,
                        &mut pending_track,
                        &mut pending_index0,
                        &mut pending_index1,
                        current_file,
                    );

                    let num_str = it.next().ok_or("TRACK missing number")?;
                    let number: u32 = num_str.parse().map_err(|_| "bad TRACK number")?;
                    let mode = it.next().ok_or("TRACK missing mode")?;
                    let mode_upper = mode.to_ascii_uppercase();
                    let (kind, sector_size) = match mode_upper.as_str() {
                        "AUDIO" => (TrackKind::Audio, 2352u64),
                        "MODE1/2048" => (TrackKind::Data, 2048u64),
                        "MODE1/2352" => (TrackKind::Data, 2352u64),
                        "MODE2/2336" => (TrackKind::Data, 2336u64),
                        "MODE2/2352" => (TrackKind::Data, 2352u64),
                        other => return Err(format!("unsupported TRACK mode: {}", other)),
                    };
                    pending_track = Some((number, kind, sector_size));
                    pending_index0 = None;
                    pending_index1 = None;
                }
                "INDEX" => {
                    let idx_str = it.next().ok_or("INDEX missing number")?;
                    let idx: u32 = idx_str.parse().map_err(|_| "bad INDEX number")?;
                    let msf = it.next().ok_or("INDEX missing MSF")?;
                    let lba = parse_msf(&msf)?;
                    match idx {
                        0 => pending_index0 = Some(lba),
                        1 => pending_index1 = Some(lba),
                        _ => {} // ignore INDEX >= 2
                    }
                }
                _ => {} // ignore PREGAP, POSTGAP, REM, CATALOG, PERFORMER, TITLE...
            }
        }

        // flush trailing track
        flush_track(
            &mut tracks,
            &mut pending_track,
            &mut pending_index0,
            &mut pending_index1,
            current_file,
        );

        if files.is_empty() || tracks.is_empty() {
            return Err("cue sheet contains no usable tracks".to_string());
        }

        // Compute per-track length using next track's file_lba within the same
        // file, or the file's end. Then assign disc_lba sequentially starting
        // at 0 (this matches how the cdrom emulator addresses sectors).
        let n = tracks.len();
        for i in 0..n {
            let file_index = tracks[i].file_index;
            let file_total_sectors = file_sizes[file_index] / tracks[i].sector_size;
            let end_in_file = if i + 1 < n && tracks[i + 1].file_index == file_index {
                tracks[i + 1].file_lba
            } else {
                file_total_sectors
            };
            tracks[i].length = end_in_file.saturating_sub(tracks[i].file_lba);
        }

        let mut running_lba: u64 = 0;
        for t in tracks.iter_mut() {
            t.disc_lba = running_lba;
            running_lba += t.length;
        }
        let total_sectors = running_lba;

        Ok(Self {
            files,
            tracks,
            total_sectors,
        })
    }

    fn find_track(&self, lba: u64) -> Option<&CueTrack> {
        // tracks are in ascending order of disc_lba
        let mut last: Option<&CueTrack> = None;
        for t in &self.tracks {
            if t.disc_lba <= lba {
                last = Some(t);
            } else {
                break;
            }
        }
        last
    }
}

impl Disc for CueDisc {
    fn read_sector(&mut self, lba: u64, out: &mut [u8; SECTOR_RAW_SIZE as usize]) -> Result<(), String> {
        let track = match self.find_track(lba) {
            Some(t) => t,
            None => return Err(format!("LBA {} before first track", lba)),
        };

        let track_offset = lba - track.disc_lba;
        if track_offset >= track.length {
            return Err(format!(
                "LBA {} past end of track (track len {})",
                lba, track.length
            ));
        }

        let file_lba = track.file_lba + track_offset;
        let sector_size = track.sector_size;
        let file_index = track.file_index;

        for b in out.iter_mut() {
            *b = 0;
        }

        let file = &mut self.files[file_index];
        let byte_offset = file_lba * sector_size;
        file.seek(SeekFrom::Start(byte_offset))
            .map_err(|e| e.to_string())?;

        match sector_size {
            2352 => {
                file.read_exact(out).map_err(|e| e.to_string())?;
            }
            2048 => {
                // Mode 1 cooked: data only. Synthesise raw layout so callers
                // that index at offset 24 still see the right bytes.
                let mut data = [0u8; 2048];
                file.read_exact(&mut data).map_err(|e| e.to_string())?;
                out[12] = 0; out[13] = 0; out[14] = 0; out[15] = 1; // mode 1
                out[24..24 + 2048].copy_from_slice(&data);
            }
            2336 => {
                // Mode 2 form-mixed cooked.
                let mut data = [0u8; 2336];
                file.read_exact(&mut data).map_err(|e| e.to_string())?;
                out[12] = 0; out[13] = 0; out[14] = 0; out[15] = 2;
                out[16..16 + 2336].copy_from_slice(&data);
            }
            other => {
                return Err(format!("unsupported sector size: {}", other));
            }
        }

        Ok(())
    }

    fn sector_count(&self) -> u64 {
        self.total_sectors
    }
}

fn flush_track(
    tracks: &mut Vec<CueTrack>,
    pending_track: &mut Option<(u32, TrackKind, u64)>,
    pending_index0: &mut Option<u64>,
    pending_index1: &mut Option<u64>,
    current_file: Option<usize>,
) {
    if let (Some((number, kind, sector_size)), Some(file_index)) = (pending_track.take(), current_file) {
        // Prefer INDEX 01 file-LBA. Fall back to INDEX 00 if missing.
        let file_lba = pending_index1.take().or(*pending_index0).unwrap_or(0);
        *pending_index0 = None;
        tracks.push(CueTrack {
            number,
            kind,
            file_index,
            sector_size,
            disc_lba: 0, // filled in later
            file_lba,
            length: 0,   // filled in later
        });
    }
    *pending_index0 = None;
    *pending_index1 = None;
}

fn resolve_path(candidate: &Path) -> Option<PathBuf> {
    if candidate.exists() {
        return Some(candidate.to_path_buf());
    }
    // case-insensitive scan of parent directory (Windows .cue files often
    // reference upper-case filenames even when the .bin is lower-case).
    let parent = candidate.parent()?;
    let target = candidate.file_name()?.to_string_lossy().to_ascii_lowercase();
    for entry in fs::read_dir(parent).ok()? {
        let entry = entry.ok()?;
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if name == target {
            return Some(entry.path());
        }
    }
    None
}

fn parse_msf(s: &str) -> Result<u64, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(format!("bad MSF: {}", s));
    }
    let m: u64 = parts[0].parse().map_err(|_| "bad MSF minutes")?;
    let sec: u64 = parts[1].parse().map_err(|_| "bad MSF seconds")?;
    let f: u64 = parts[2].parse().map_err(|_| "bad MSF frames")?;
    Ok(m * 60 * 75 + sec * 75 + f)
}

struct TokenIter<'a> {
    rest: &'a str,
}

impl<'a> TokenIter<'a> {
    fn new(s: &'a str) -> Self {
        Self { rest: s }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let bytes = self.rest.as_bytes();
        let mut i = 0;
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }
        if i >= bytes.len() {
            self.rest = "";
            return None;
        }
        if bytes[i] == b'"' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            let token = self.rest[start..i].to_string();
            if i < bytes.len() {
                i += 1; // skip closing quote
            }
            self.rest = &self.rest[i..];
            Some(token)
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b' ' && bytes[i] != b'\t' {
                i += 1;
            }
            let token = self.rest[start..i].to_string();
            self.rest = &self.rest[i..];
            Some(token)
        }
    }
}

// ---------------------------------------------------------------------------
// CloneCD (.ccd) parser
// ---------------------------------------------------------------------------
//
// A .ccd is an INI-style descriptor that names a sibling .img (raw 2352-byte
// sector dump) plus a [TRACK N] section per track giving the mode, and
// [Entry N] TOC sections whose Point=0x01..0x63 hold each track's start LBA.
// Special Points 0xa0/0xa1/0xa2 give the first/last track numbers and the
// lead-out start (= total sector count). We ignore subchannel-only fields.

pub struct CcdDisc {
    file: fs::File,
    tracks: Vec<CueTrack>,
    total_sectors: u64,
}

impl CcdDisc {
    pub fn open(ccd_path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(ccd_path).map_err(|e| e.to_string())?;
        let parent = ccd_path.parent().unwrap_or_else(|| Path::new("."));
        let stem = ccd_path
            .file_stem()
            .ok_or("ccd has no file stem")?
            .to_string_lossy()
            .to_string();

        // Find the sibling .img (case-insensitive).
        let img_path = find_companion_image(parent, &stem)
            .ok_or_else(|| format!("no .img next to {}", ccd_path.display()))?;

        let file = fs::File::open(&img_path).map_err(|e| {
            format!("failed to open {}: {}", img_path.display(), e)
        })?;
        let file_len = file.metadata().map_err(|e| e.to_string())?.len();
        let file_sectors = file_len / SECTOR_RAW_SIZE;

        // First pass: walk every section, collect Entry/Track info.
        let mut current: Option<String> = None;
        let mut entries: Vec<CcdEntry> = Vec::new();
        let mut cur_entry: Option<CcdEntry> = None;
        let mut track_modes: std::collections::HashMap<u32, u32> = Default::default();
        let mut cur_track: Option<u32> = None;
        let mut lead_out_lba: u64 = file_sectors;

        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                // section change
                if let Some(e) = cur_entry.take() {
                    entries.push(e);
                }
                let header = line[1..line.len() - 1].trim().to_string();
                let lower = header.to_ascii_lowercase();
                if lower.starts_with("entry ") {
                    let idx: u32 = lower[6..].trim().parse().unwrap_or(0);
                    cur_entry = Some(CcdEntry::new(idx));
                    cur_track = None;
                } else if lower.starts_with("track ") {
                    let n: u32 = lower[6..].trim().parse().unwrap_or(0);
                    cur_track = Some(n);
                    track_modes.entry(n).or_insert(2);
                } else {
                    cur_track = None;
                }
                current = Some(lower);
                continue;
            }

            let (key, value) = match line.split_once('=') {
                Some(kv) => (kv.0.trim().to_ascii_lowercase(), kv.1.trim().to_string()),
                None => continue,
            };

            if let Some(section) = &current {
                if section.starts_with("entry ") {
                    if let Some(entry) = cur_entry.as_mut() {
                        entry.set(&key, &value);
                    }
                } else if section.starts_with("track ") {
                    if let Some(n) = cur_track {
                        if key == "mode" {
                            let m: u32 = value.parse().unwrap_or(2);
                            track_modes.insert(n, m);
                        }
                    }
                }
            }
        }
        if let Some(e) = cur_entry.take() {
            entries.push(e);
        }

        // Lead-out start = end of disc, total sector count.
        for e in &entries {
            if e.point == Some(0xa2) {
                if let Some(plba) = e.plba {
                    if plba >= 0 {
                        lead_out_lba = plba as u64;
                    }
                }
            }
        }

        // Track entries (Point 0x01..0x63), sorted by point.
        let mut tps: Vec<&CcdEntry> = entries
            .iter()
            .filter(|e| matches!(e.point, Some(p) if (0x01..=0x63).contains(&p)))
            .collect();
        tps.sort_by_key(|e| e.point.unwrap());

        if tps.is_empty() {
            return Err("ccd has no tracks".to_string());
        }

        let mut tracks: Vec<CueTrack> = Vec::with_capacity(tps.len());
        for (i, t) in tps.iter().enumerate() {
            let number = t.point.unwrap();
            let start_lba = t.plba.unwrap_or(0).max(0) as u64;
            let next_lba = if i + 1 < tps.len() {
                tps[i + 1].plba.unwrap_or(0).max(0) as u64
            } else {
                lead_out_lba
            };
            let length = next_lba.saturating_sub(start_lba);

            // Control bit 2 (0x04) marks a data track. CCD writes Control as
            // a hex byte in the entry. Default to data if absent.
            let is_data = t.control.map(|c| (c & 0x04) != 0).unwrap_or(true);
            let kind = if is_data { TrackKind::Data } else { TrackKind::Audio };

            let _mode = track_modes.get(&number).copied().unwrap_or(2);

            tracks.push(CueTrack {
                number,
                kind,
                file_index: 0,
                sector_size: SECTOR_RAW_SIZE,
                disc_lba: start_lba,
                file_lba: start_lba,
                length,
            });
        }

        Ok(Self {
            file,
            tracks,
            total_sectors: lead_out_lba,
        })
    }

    fn find_track(&self, lba: u64) -> Option<&CueTrack> {
        let mut last: Option<&CueTrack> = None;
        for t in &self.tracks {
            if t.disc_lba <= lba {
                last = Some(t);
            } else {
                break;
            }
        }
        last
    }
}

impl Disc for CcdDisc {
    fn read_sector(&mut self, lba: u64, out: &mut [u8; SECTOR_RAW_SIZE as usize]) -> Result<(), String> {
        let track = match self.find_track(lba) {
            Some(t) => t,
            None => return Err(format!("LBA {} before first track", lba)),
        };

        if lba >= self.total_sectors {
            return Err(format!("LBA {} past lead-out", lba));
        }

        let offset = lba - track.disc_lba;
        if offset >= track.length && track.length != 0 {
            return Err(format!(
                "LBA {} past end of track {} (len {})",
                lba, track.number, track.length
            ));
        }

        let file_lba = track.file_lba + offset;
        let byte_offset = file_lba * SECTOR_RAW_SIZE;

        self.file
            .seek(SeekFrom::Start(byte_offset))
            .map_err(|e| e.to_string())?;
        self.file.read_exact(out).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn sector_count(&self) -> u64 {
        self.total_sectors
    }
}

#[derive(Default)]
struct CcdEntry {
    #[allow(dead_code)]
    index: u32,
    point: Option<u32>,
    plba: Option<i64>,
    control: Option<u32>,
}

impl CcdEntry {
    fn new(index: u32) -> Self {
        Self { index, ..Default::default() }
    }

    fn set(&mut self, key: &str, value: &str) {
        match key {
            "point" => self.point = parse_ccd_uint(value),
            "plba" => self.plba = parse_ccd_int(value),
            "control" => self.control = parse_ccd_uint(value),
            _ => {}
        }
    }
}

fn parse_ccd_uint(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

fn parse_ccd_int(s: &str) -> Option<i64> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<i64>().ok()
    }
}

fn find_companion_image(parent: &Path, stem: &str) -> Option<PathBuf> {
    for ext in &["img", "IMG", "Img", "bin", "BIN"] {
        let cand = parent.join(format!("{}.{}", stem, ext));
        if cand.exists() {
            return Some(cand);
        }
    }
    // Fall back to a case-insensitive directory scan.
    let target = format!("{}.", stem).to_ascii_lowercase();
    for entry in fs::read_dir(parent).ok()? {
        let entry = entry.ok()?;
        let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
        if name.starts_with(&target) {
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_ascii_lowercase())
                .unwrap_or_default();
            if ext == "img" || ext == "bin" {
                return Some(path);
            }
        }
    }
    None
}
