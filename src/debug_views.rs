//! Optional debug windows:
//!   F8  - VRAM viewer (live 1024x512 VRAM)
//!   F9  - CPU debugger (registers + disasm of current instruction)
//!   F10 - RAM viewer (hex / ASCII, scroll with wheel)
//!
//! Each owns a separate SDL window with its own software-rendered Canvas.
//! Events for these windows (close, scroll) are routed by `window_id` from
//! the shared event pump in frontend.rs.

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use crate::font::{glyph, GLYPH_H, GLYPH_W};
use crate::psx::System;

const PIXEL_SCALE: i32 = 2;
const CHAR_W: i32 = (GLYPH_W as i32 + 1) * PIXEL_SCALE;
const CHAR_H: i32 = (GLYPH_H as i32 + 2) * PIXEL_SCALE;

fn draw_text(canvas: &mut Canvas<Window>, x: i32, y: i32, s: &str, color: Color) {
    canvas.set_draw_color(color);
    let mut cx = x;
    for c in s.chars() {
        let g = glyph(c);
        for (row, byte) in g.iter().enumerate() {
            for col in 0..GLYPH_W {
                if (byte >> (7 - col)) & 1 != 0 {
                    let _ = canvas.fill_rect(Rect::new(
                        cx + (col as i32) * PIXEL_SCALE,
                        y + (row as i32) * PIXEL_SCALE,
                        PIXEL_SCALE as u32,
                        PIXEL_SCALE as u32,
                    ));
                }
            }
        }
        cx += CHAR_W;
    }
}

// =========================================================================
// VRAM viewer (F8)
// =========================================================================

const VRAM_WIDTH: u32 = 1024;
const VRAM_HEIGHT: u32 = 512;
const VRAM_BYTES: usize = (VRAM_WIDTH * VRAM_HEIGHT * 3) as usize;

pub struct VramWindow {
    canvas: Canvas<Window>,
    framebuffer: Box<[u8]>,
    window_id: u32,
}

impl VramWindow {
    pub fn open(video: &VideoSubsystem) -> Result<Self, String> {
        let window = video
            .window("RustyStation VRAM Viewer", VRAM_WIDTH, VRAM_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let window_id = window.id();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            canvas,
            framebuffer: vec![0; VRAM_BYTES].into_boxed_slice(),
            window_id,
        })
    }

    pub fn window_id(&self) -> u32 {
        self.window_id
    }

    pub fn render(&mut self, system: &System) {
        system.get_framebuffer(&mut self.framebuffer, true);

        // Recreate the streaming texture each frame; SDL pools allocations.
        let creator = self.canvas.texture_creator();
        let mut texture = match creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            VRAM_WIDTH,
            VRAM_HEIGHT,
        ) {
            Ok(t) => t,
            Err(_) => return,
        };
        let _ = texture.update(None, &self.framebuffer, (VRAM_WIDTH * 3) as usize);

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        let _ = self.canvas.copy(&texture, None, None);
        self.canvas.present();
    }
}

// =========================================================================
// CPU debugger window (F9)
// =========================================================================

const DEBUG_WIDTH: u32 = 880;
const DEBUG_HEIGHT: u32 = 520;

pub struct DebugWindow {
    canvas: Canvas<Window>,
    window_id: u32,
    last_regs: [u32; 32],
}

impl DebugWindow {
    pub fn open(video: &VideoSubsystem) -> Result<Self, String> {
        let window = video
            .window("RustyStation CPU Debugger", DEBUG_WIDTH, DEBUG_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let window_id = window.id();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            canvas,
            window_id,
            last_regs: [0; 32],
        })
    }

    pub fn window_id(&self) -> u32 {
        self.window_id
    }

    pub fn render(&mut self, system: &System) {
        let snap = system.debug_snapshot();

        self.canvas.set_draw_color(Color::RGB(10, 10, 14));
        self.canvas.clear();

        let fg = Color::RGB(220, 220, 230);
        let hi = Color::RGB(255, 220, 120);
        let chg = Color::RGB(255, 140, 140);
        let dim = Color::RGB(110, 110, 130);

        let disasm = disassemble(snap.current_instruction, snap.current_pc);
        let header = format!(
            "PC={:08X}  INS={:08X}  >>  {}",
            snap.current_pc, snap.current_instruction, disasm
        );
        draw_text(&mut self.canvas, 8, 8, &header, hi);

        let line2 = format!(
            "NEXT={:08X}    HI={:08X}    LO={:08X}",
            snap.pc, snap.hi, snap.lo
        );
        draw_text(&mut self.canvas, 8, 8 + CHAR_H, &line2, fg);

        let grid_top = 8 + CHAR_H * 3;
        let col_w = (DEBUG_WIDTH as i32) / 4;
        for i in 0..32usize {
            let col = (i as i32) % 4;
            let row = (i as i32) / 4;
            let x = 8 + col * col_w;
            let y = grid_top + row * CHAR_H;

            let val = snap.regs[i];
            let changed = self.last_regs[i] != val;
            let line = format!("{:>4}={:08X}", REG_NAMES[i], val);

            let color = if changed { chg } else if val == 0 { dim } else { fg };
            draw_text(&mut self.canvas, x, y, &line, color);
        }

        let footer_y = grid_top + 9 * CHAR_H;
        self.canvas.set_draw_color(pc_color(snap.pc));
        let _ = self.canvas.fill_rect(Rect::new(8, footer_y, 12, 12));
        draw_text(
            &mut self.canvas,
            28,
            footer_y + 2,
            "PC pulse (colour cycles when game runs)",
            dim,
        );

        let strobe_y = footer_y + 24;
        draw_text(&mut self.canvas, 8, strobe_y, "ACT", dim);
        for i in 0..32usize {
            let changed = self.last_regs[i] != snap.regs[i];
            let c = if changed {
                Color::RGB(255, 200, 80)
            } else {
                Color::RGB(40, 40, 50)
            };
            self.canvas.set_draw_color(c);
            let _ = self.canvas.fill_rect(Rect::new(
                40 + (i as i32) * 16,
                strobe_y,
                12,
                12,
            ));
        }

        self.canvas.present();
        self.last_regs = snap.regs;
    }
}

const REG_NAMES: [&str; 32] = [
    "zr", "at", "v0", "v1", "a0", "a1", "a2", "a3",
    "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
    "t8", "t9", "k0", "k1", "gp", "sp", "fp", "ra",
];

fn pc_color(pc: u32) -> Color {
    Color::RGB(
        (pc & 0xff) as u8,
        ((pc >> 8) & 0xff) as u8,
        ((pc >> 16) & 0xff) as u8,
    )
}

// =========================================================================
// RAM viewer window (F10)
// =========================================================================

const RAM_WIN_WIDTH: u32 = 980;
const RAM_WIN_HEIGHT: u32 = 600;
const BYTES_PER_ROW: usize = 16;
const VISIBLE_ROWS: usize = 32;
const PAGE_BYTES: usize = BYTES_PER_ROW * VISIBLE_ROWS;
const DEFAULT_RAM_ADDR: usize = 0x10000; // start of PSX game-loadable code

pub struct RamWindow {
    canvas: Canvas<Window>,
    window_id: u32,
    view_addr: usize,
    scratch: Vec<u8>,
}

impl RamWindow {
    pub fn open(video: &VideoSubsystem) -> Result<Self, String> {
        let window = video
            .window("RustyStation RAM Viewer", RAM_WIN_WIDTH, RAM_WIN_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let window_id = window.id();

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            canvas,
            window_id,
            view_addr: DEFAULT_RAM_ADDR,
            scratch: vec![0; PAGE_BYTES],
        })
    }

    pub fn window_id(&self) -> u32 {
        self.window_id
    }

    /// Scroll by `lines` rows (positive = down). Wraps to RAM size.
    pub fn scroll_lines(&mut self, lines: i32, ram_size: usize) {
        let step = lines as isize * BYTES_PER_ROW as isize;
        let ram = ram_size.max(1) as isize;
        let new = self.view_addr as isize + step;
        let wrapped = ((new % ram) + ram) % ram;
        self.view_addr = wrapped as usize;
    }

    pub fn page(&mut self, pages: i32, ram_size: usize) {
        self.scroll_lines(pages * VISIBLE_ROWS as i32, ram_size);
    }

    pub fn reset_view(&mut self) {
        self.view_addr = DEFAULT_RAM_ADDR;
    }

    pub fn render(&mut self, system: &System) {
        let ram_size = system.ram_size();
        system.peek_ram(self.view_addr, PAGE_BYTES, &mut self.scratch);

        self.canvas.set_draw_color(Color::RGB(10, 10, 14));
        self.canvas.clear();

        let fg = Color::RGB(220, 220, 230);
        let hi = Color::RGB(255, 220, 120);
        let dim = Color::RGB(110, 110, 130);

        let header = format!(
            "RAM 0x{:08X}  (size 0x{:X})  -  wheel: scroll  PgUp/PgDn: page  Home: reset",
            self.view_addr, ram_size
        );
        draw_text(&mut self.canvas, 8, 8, &header, hi);

        let column_header =
            "ADDR      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F  ASCII";
        draw_text(&mut self.canvas, 8, 8 + CHAR_H + 4, column_header, dim);

        let mut y = 8 + (CHAR_H * 2) + 8;
        for row in 0..VISIBLE_ROWS {
            let row_addr =
                (self.view_addr + row * BYTES_PER_ROW) & (ram_size.saturating_sub(1).max(0xF));
            let mut line = String::with_capacity(80);
            line.push_str(&format!("{:08X}  ", row_addr));
            for i in 0..BYTES_PER_ROW {
                let b = self.scratch[row * BYTES_PER_ROW + i];
                line.push_str(&format!("{:02X} ", b));
            }
            line.push(' ');
            for i in 0..BYTES_PER_ROW {
                let b = self.scratch[row * BYTES_PER_ROW + i];
                let c = if (0x20..=0x7e).contains(&b) { b as char } else { '.' };
                line.push(c);
            }
            draw_text(&mut self.canvas, 8, y, &line, fg);
            y += CHAR_H;
        }

        self.canvas.present();
    }
}

// =========================================================================
// Minimal MIPS R3000A disassembler (used by CPU debugger window)
// =========================================================================

fn disassemble(ins: u32, pc: u32) -> String {
    let op = (ins >> 26) & 0x3f;
    let rs = ((ins >> 21) & 0x1f) as usize;
    let rt = ((ins >> 16) & 0x1f) as usize;
    let rd = ((ins >> 11) & 0x1f) as usize;
    let sh = (ins >> 6) & 0x1f;
    let funct = ins & 0x3f;
    let imm = ins & 0xffff;
    let imm_se = (imm as i16) as i32;
    let target = ins & 0x3ff_ffff;
    let r = REG_NAMES;

    match op {
        0x00 => match funct {
            0x00 => format!("SLL    ${},${},{}", r[rd], r[rt], sh),
            0x02 => format!("SRL    ${},${},{}", r[rd], r[rt], sh),
            0x03 => format!("SRA    ${},${},{}", r[rd], r[rt], sh),
            0x04 => format!("SLLV   ${},${},${}", r[rd], r[rt], r[rs]),
            0x06 => format!("SRLV   ${},${},${}", r[rd], r[rt], r[rs]),
            0x07 => format!("SRAV   ${},${},${}", r[rd], r[rt], r[rs]),
            0x08 => format!("JR     ${}", r[rs]),
            0x09 => format!("JALR   ${},${}", r[rd], r[rs]),
            0x0c => "SYSCALL".to_string(),
            0x0d => "BREAK".to_string(),
            0x10 => format!("MFHI   ${}", r[rd]),
            0x11 => format!("MTHI   ${}", r[rs]),
            0x12 => format!("MFLO   ${}", r[rd]),
            0x13 => format!("MTLO   ${}", r[rs]),
            0x18 => format!("MULT   ${},${}", r[rs], r[rt]),
            0x19 => format!("MULTU  ${},${}", r[rs], r[rt]),
            0x1a => format!("DIV    ${},${}", r[rs], r[rt]),
            0x1b => format!("DIVU   ${},${}", r[rs], r[rt]),
            0x20 => format!("ADD    ${},${},${}", r[rd], r[rs], r[rt]),
            0x21 => format!("ADDU   ${},${},${}", r[rd], r[rs], r[rt]),
            0x22 => format!("SUB    ${},${},${}", r[rd], r[rs], r[rt]),
            0x23 => format!("SUBU   ${},${},${}", r[rd], r[rs], r[rt]),
            0x24 => format!("AND    ${},${},${}", r[rd], r[rs], r[rt]),
            0x25 => format!("OR     ${},${},${}", r[rd], r[rs], r[rt]),
            0x26 => format!("XOR    ${},${},${}", r[rd], r[rs], r[rt]),
            0x27 => format!("NOR    ${},${},${}", r[rd], r[rs], r[rt]),
            0x2a => format!("SLT    ${},${},${}", r[rd], r[rs], r[rt]),
            0x2b => format!("SLTU   ${},${},${}", r[rd], r[rs], r[rt]),
            _ => format!("SPECIAL.0x{:02X}", funct),
        },
        0x01 => {
            let branch = pc.wrapping_add(4).wrapping_add((imm_se << 2) as u32);
            let mnem = match rt {
                0x00 => "BLTZ",
                0x01 => "BGEZ",
                0x10 => "BLTZAL",
                0x11 => "BGEZAL",
                _ => "BCOND?",
            };
            format!("{}   ${},{:08X}", mnem, r[rs], branch)
        }
        0x02 => format!("J      {:08X}", (pc & 0xf000_0000) | (target << 2)),
        0x03 => format!("JAL    {:08X}", (pc & 0xf000_0000) | (target << 2)),
        0x04 => {
            let b = pc.wrapping_add(4).wrapping_add((imm_se << 2) as u32);
            format!("BEQ    ${},${},{:08X}", r[rs], r[rt], b)
        }
        0x05 => {
            let b = pc.wrapping_add(4).wrapping_add((imm_se << 2) as u32);
            format!("BNE    ${},${},{:08X}", r[rs], r[rt], b)
        }
        0x06 => {
            let b = pc.wrapping_add(4).wrapping_add((imm_se << 2) as u32);
            format!("BLEZ   ${},{:08X}", r[rs], b)
        }
        0x07 => {
            let b = pc.wrapping_add(4).wrapping_add((imm_se << 2) as u32);
            format!("BGTZ   ${},{:08X}", r[rs], b)
        }
        0x08 => format!("ADDI   ${},${},{}", r[rt], r[rs], imm_se),
        0x09 => format!("ADDIU  ${},${},{}", r[rt], r[rs], imm_se),
        0x0a => format!("SLTI   ${},${},{}", r[rt], r[rs], imm_se),
        0x0b => format!("SLTIU  ${},${},{}", r[rt], r[rs], imm_se),
        0x0c => format!("ANDI   ${},${},0x{:X}", r[rt], r[rs], imm),
        0x0d => format!("ORI    ${},${},0x{:X}", r[rt], r[rs], imm),
        0x0e => format!("XORI   ${},${},0x{:X}", r[rt], r[rs], imm),
        0x0f => format!("LUI    ${},0x{:X}", r[rt], imm),
        0x10 => format!("COP0   0x{:08X}", ins),
        0x12 => format!("COP2   0x{:08X}", ins),
        0x20 => format!("LB     ${},{}(${})", r[rt], imm_se, r[rs]),
        0x21 => format!("LH     ${},{}(${})", r[rt], imm_se, r[rs]),
        0x22 => format!("LWL    ${},{}(${})", r[rt], imm_se, r[rs]),
        0x23 => format!("LW     ${},{}(${})", r[rt], imm_se, r[rs]),
        0x24 => format!("LBU    ${},{}(${})", r[rt], imm_se, r[rs]),
        0x25 => format!("LHU    ${},{}(${})", r[rt], imm_se, r[rs]),
        0x26 => format!("LWR    ${},{}(${})", r[rt], imm_se, r[rs]),
        0x28 => format!("SB     ${},{}(${})", r[rt], imm_se, r[rs]),
        0x29 => format!("SH     ${},{}(${})", r[rt], imm_se, r[rs]),
        0x2a => format!("SWL    ${},{}(${})", r[rt], imm_se, r[rs]),
        0x2b => format!("SW     ${},{}(${})", r[rt], imm_se, r[rs]),
        0x2e => format!("SWR    ${},{}(${})", r[rt], imm_se, r[rs]),
        0x32 => format!("LWC2   ${},{}(${})", r[rt], imm_se, r[rs]),
        0x3a => format!("SWC2   ${},{}(${})", r[rt], imm_se, r[rs]),
        _ => format!("OP.0x{:02X}", op),
    }
}
