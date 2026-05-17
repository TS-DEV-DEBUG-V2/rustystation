#[macro_use]
extern crate clap;

extern crate imgui;

mod audio_interface;
pub mod bios;
mod debug_views;
pub mod font;
mod frontend;
//mod gui;
pub mod images;
mod launcher;

#[macro_use]
pub mod log;

mod psx;
pub mod queue;
pub mod util;

use clap::App;

use audio_interface::AudioInterface;
use frontend::Frontend;
//use gui::Gui;

use psx::System;

#[derive(Clone, Copy)]
pub enum Scaling {
    None,
    Aspect,
    Fullscreen,
}

impl Scaling {
    pub fn from(value: i32) -> Scaling {
        use Scaling::*;

        match value {
            0 => None,
            1 => Aspect,
            2 => Fullscreen,
            _ => panic!(),
        }
    }
}

pub struct Options {
    draw_full_vram: bool,
    scaling: Scaling,
    crop_overscan: bool,

    pause: bool,
    step: bool,

    frame_limit: bool,

    state_index: usize,
}

fn main() {
    log::init();

    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // --custom-bios: read the file now so we can report errors before the
    // SDL window opens. Falls back to embedded on any failure.
    let custom_bios: Option<Vec<u8>> = match matches.value_of("custom-bios") {
        Some(path) => match std::fs::read(path) {
            Ok(bytes) => {
                log_info!("BIOS", "custom: {} ({} bytes)", path, bytes.len());
                Some(bytes)
            }
            Err(e) => {
                log_error!("BIOS", "failed to read custom BIOS '{}': {}", path, e);
                None
            }
        },
        None => None,
    };

    if custom_bios.is_none() {
        log_info!("BOOT", "BIOS: embedded ({} bytes)", bios::BIOS_SIZE);
    }

    let mut sdl_ctx_temp = sdl2::init().unwrap();
    // SDL only hands out the event pump once per context; take it here and
    // pass it through the launcher (if any) and then into Frontend.
    let mut event_pump = sdl_ctx_temp.event_pump().unwrap();

    let game_filepath: String = match matches.value_of("GAME") {
        Some(p) => {
            log_info!("BOOT", "GAME: {}", p);
            p.to_string()
        }
        None => {
            log_info!("BOOT", "no GAME argument — opening launcher");
            match launcher::run(&sdl_ctx_temp, &mut event_pump) {
                Some(path) => {
                    log_info!("BOOT", "GAME (dropped): {}", path);
                    path
                }
                None => {
                    log_info!("BOOT", "launcher closed without a ROM, exiting");
                    return;
                }
            }
        }
    };

    let mut options = Options {
        draw_full_vram: false,
        scaling: Scaling::Aspect,
        crop_overscan: true,

        pause: false,
        step: false,

        frame_limit: true,

        state_index: 0,
    };

    let mut audio = AudioInterface::new(&mut sdl_ctx_temp, 44100, 2, 512);
    let mut frontend = Frontend::create(&mut sdl_ctx_temp, event_pump, 640, 480);

    // Disabled due to Dear ImGui version bump
    //let mut gui = Gui::new(&video.display);

    let mut system = System::new_with_bios(game_filepath, custom_bios);
    system.reset();

    audio.play();

    while system.running {
        if options.step {
            system.run_frame();

            options.step = false;
            options.pause = true;
        }

        if !options.pause {
            system.run_frame();
        }

        audio.push_samples(system.get_audio_samples());
        frontend.update(&mut options, &mut system);
        frontend.render(&options, &system);
    }
}
