//! Startup launcher shown when the user invokes the emulator without a GAME
//! argument. Provides branding, a clickable GitHub button, and ROM
//! drag-and-drop with extension validation.
//!
//! Returns either `Some(path)` with the dropped ROM, or `None` if the user
//! closed the window without dropping anything (in which case the host should
//! exit).

use std::path::Path;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

use crate::font::{glyph, GLYPH_H, GLYPH_W};

const WIN_W: u32 = 1024;
const WIN_H: u32 = 600;

const ACCEPTED_EXTS: &[&str] = &["bin", "img", "cue", "ccd"];
const GITHUB_URL: &str = "https://github.com/ts-dev-debug-v2/rustystation";

pub fn run(sdl: &Sdl, event_pump: &mut EventPump) -> Option<String> {
    let video = sdl.video().ok()?;

    let window = video
        .window("RustyStation", WIN_W, WIN_H)
        .position_centered()
        .build()
        .ok()?;

    let mut canvas: Canvas<Window> = window.into_canvas().accelerated().build().ok()?;

    // GitHub button: a centered pill in the upper-middle band.
    let button_w: i32 = 220;
    let button_h: i32 = 56;
    let button_x: i32 = (WIN_W as i32 - button_w) / 2;
    let button_y: i32 = 230;
    let github_rect = Rect::new(button_x, button_y, button_w as u32, button_h as u32);

    let mut error: Option<(String, Instant)> = None;
    let error_duration = Duration::from_secs(4);
    let mut hovering_github = false;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return None,
                Event::DropFile { filename, .. } => {
                    if is_valid_rom(&filename) {
                        return Some(filename);
                    } else {
                        error = Some((
                            "Not a valid or supported PSX Rom".to_string(),
                            Instant::now(),
                        ));
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    hovering_github = github_rect.contains_point((x, y));
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if github_rect.contains_point((x, y)) {
                        open_url(GITHUB_URL);
                    }
                }
                _ => {}
            }
        }

        // Background.
        canvas.set_draw_color(Color::RGB(14, 14, 22));
        canvas.clear();

        // Big title.
        let title = "Made By TS-DEV-DEBUG-V2";
        draw_text_centered(&mut canvas, title, 60, 5, Color::RGB(245, 245, 255));

        // GitHub button.
        let btn_fill = if hovering_github {
            Color::RGB(60, 80, 130)
        } else {
            Color::RGB(40, 55, 90)
        };
        canvas.set_draw_color(btn_fill);
        let _ = canvas.fill_rect(github_rect);
        canvas.set_draw_color(Color::RGB(200, 200, 220));
        let _ = canvas.draw_rect(github_rect);
        // "GITHUB" label, centered in the button.
        let label = "GITHUB";
        let label_scale = 3;
        let label_w = (label.len() as i32) * (GLYPH_W as i32 + 1) * label_scale;
        let label_h = (GLYPH_H as i32 + 2) * label_scale;
        let lx = button_x + (button_w - label_w) / 2;
        let ly = button_y + (button_h - label_h) / 2;
        draw_text(&mut canvas, label, lx, ly, label_scale, Color::RGB(245, 245, 255));

        // Drop prompt.
        let prompt = "Drag your ROM into this window";
        draw_text_centered(&mut canvas, prompt, 340, 3, Color::RGB(230, 230, 240));

        // Accepted formats list.
        let formats = "Accepted formats: BIN  IMG  CUE  CCD";
        draw_text_centered(&mut canvas, formats, 400, 2, Color::RGB(150, 160, 180));

        // Error toast.
        if let Some((msg, when)) = &error {
            if when.elapsed() < error_duration {
                let banner_y = WIN_H as i32 - 80;
                canvas.set_draw_color(Color::RGB(140, 30, 30));
                let _ = canvas.fill_rect(Rect::new(0, banner_y, WIN_W, 50));
                draw_text_centered(&mut canvas, msg, banner_y + 12, 3, Color::RGB(255, 230, 230));
            } else {
                error = None;
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}

fn is_valid_rom(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    ACCEPTED_EXTS.iter().any(|e| *e == ext)
}

fn open_url(url: &str) {
    // Cross-platform default-browser launcher. Best-effort: swallow any error.
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd")
            .args(&["/c", "start", "", url])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

fn text_pixel_width(s: &str, scale: i32) -> i32 {
    (s.chars().count() as i32) * (GLYPH_W as i32 + 1) * scale
}

fn draw_text_centered(
    canvas: &mut Canvas<Window>,
    s: &str,
    y: i32,
    scale: i32,
    color: Color,
) {
    let w = text_pixel_width(s, scale);
    let x = (WIN_W as i32 - w) / 2;
    draw_text(canvas, s, x, y, scale, color);
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    s: &str,
    x: i32,
    y: i32,
    scale: i32,
    color: Color,
) {
    canvas.set_draw_color(color);
    let mut cx = x;
    let char_w = (GLYPH_W as i32 + 1) * scale;
    for c in s.chars() {
        let g = glyph(c);
        for (row, byte) in g.iter().enumerate() {
            for col in 0..GLYPH_W {
                if (byte >> (7 - col)) & 1 != 0 {
                    let _ = canvas.fill_rect(Rect::new(
                        cx + (col as i32) * scale,
                        y + (row as i32) * scale,
                        scale as u32,
                        scale as u32,
                    ));
                }
            }
        }
        cx += char_w;
    }
}
