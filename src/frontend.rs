use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

#[cfg(windows)]
#[link(name = "winmm")]
extern "system" {
    fn timeBeginPeriod(uPeriod: u32) -> u32;
}

#[cfg(windows)]
fn raise_timer_resolution() {
    unsafe { timeBeginPeriod(1); }
}

#[cfg(not(windows))]
fn raise_timer_resolution() {}

use sdl2::controller::{Axis, Button};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

use xz2::read::XzDecoder;
use xz2::write::XzEncoder;

use crate::debug_views::{DebugWindow, VramWindow};
use crate::{Options, Scaling};
use crate::{log_error, log_info, log_warn};
use crate::psx::System;
use crate::util;

fn shader_from_source(source: &std::ffi::CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, ()> {
    let shader;

    unsafe {
        shader = gl::CreateShader(kind);
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    Ok(shader)
}

pub struct Frontend {
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    video_subsystem: sdl2::VideoSubsystem,

    event_pump: sdl2::EventPump,

    controller_subsystem: sdl2::GameControllerSubsystem,
    controllers: Vec<sdl2::controller::GameController>,

    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    program: gl::types::GLuint,
    texture: gl::types::GLuint,

    imgui: imgui::Context,
    imgui_sdl2: imgui_sdl2::ImguiSdl2,
    imgui_renderer: imgui_opengl_renderer::Renderer,

    last_frame: Instant,
    frame_deadline: Instant,

    framebuffer: Box<[u8]>,

    vram_window: Option<VramWindow>,
    debug_window: Option<DebugWindow>,
}

impl Frontend {
    pub fn create(ctx_temp: &mut sdl2::Sdl, width: u32, height: u32) -> Self {
        raise_timer_resolution();

        let video = ctx_temp.video().unwrap();
        let ctr = ctx_temp.game_controller().unwrap();

        let window = video.window("RustyStation", width, height)
            .resizable()
            .opengl()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let mut vao = 0;
        let mut vbo = 0;
        let program;
        let mut texture = 0;

        let fragment_shader = shader_from_source(&std::ffi::CString::new(include_str!("../shaders/shader.frag")).unwrap(), gl::FRAGMENT_SHADER).unwrap();
        let vertex_shader = shader_from_source(&std::ffi::CString::new(include_str!("../shaders/shader.vert")).unwrap(), gl::VERTEX_SHADER).unwrap();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            program = gl::CreateProgram();

            gl::AttachShader(program, fragment_shader);
            gl::AttachShader(program, vertex_shader);
            gl::LinkProgram(program);
            gl::DetachShader(program, fragment_shader);
            gl::DetachShader(program, vertex_shader);

            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader);

            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BindVertexArray(vao);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 16, std::ptr::null());
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 16, 8 as _);
        }

        let mut imgui = imgui::Context::create();
        let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
        let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

        let mut controllers = Vec::new();
        let num_joysticks = ctr.num_joysticks().unwrap_or(0);
        for i in 0..num_joysticks {
            if ctr.is_game_controller(i) {
                match ctr.open(i) {
                    Ok(c) => {
                        log_info!("FRONTEND", "opened controller #{} {}", i, c.name());
                        controllers.push(c);
                    }
                    Err(e) => log_error!("FRONTEND", "failed to open controller #{}: {}", i, e),
                }
            } else {
                log_warn!("FRONTEND", "joystick #{} not recognized as game controller (no SDL mapping)", i);
            }
        }

        Self {
            window: window,
            _gl_context: gl_context,
            video_subsystem: video,

            event_pump: ctx_temp.event_pump().unwrap(),

            controller_subsystem: ctr,
            controllers: controllers,

            vao: vao,
            vbo: vbo,
            program: program,
            texture: texture,

            imgui: imgui,
            imgui_sdl2: imgui_sdl2,
            imgui_renderer: imgui_renderer,

            last_frame: Instant::now(),
            frame_deadline: Instant::now(),

            framebuffer: vec![0; 1024 * 512 * 3].into_boxed_slice(),

            vram_window: None,
            debug_window: None,
        }
    }

    fn toggle_vram_window(&mut self) {
        if self.vram_window.is_some() {
            self.vram_window = None;
            log_info!("DEBUG", "VRAM window closed");
        } else {
            match VramWindow::open(&self.video_subsystem) {
                Ok(w) => {
                    log_info!("DEBUG", "VRAM window opened");
                    self.vram_window = Some(w);
                }
                Err(e) => log_error!("DEBUG", "failed to open VRAM window: {}", e),
            }
        }
    }

    fn toggle_debug_window(&mut self) {
        if self.debug_window.is_some() {
            self.debug_window = None;
            log_info!("DEBUG", "CPU debugger closed");
        } else {
            match DebugWindow::open(&self.video_subsystem) {
                Ok(w) => {
                    log_info!("DEBUG", "CPU debugger opened");
                    self.debug_window = Some(w);
                }
                Err(e) => log_error!("DEBUG", "failed to open debugger: {}", e),
            }
        }
    }

    pub fn update(&mut self, options: &mut Options, system: &mut System) {
        // Collect events first so we can call &mut self methods (e.g. open a
        // debug window) inside the loop without re-borrowing the pump.
        let events: Vec<_> = self.event_pump.poll_iter().collect();
        let main_window_id = self.window.id();
        for event in events {
            // Auxiliary-window events (close, resize) must be handled BEFORE
            // imgui sees them — imgui_sdl2 was bound to the main window and
            // will tell us to ignore events for any other window, which
            // would swallow our close handler.
            if let Event::Window { window_id, win_event, .. } = &event {
                if *window_id != main_window_id {
                    let vram_id = self.vram_window.as_ref().map(|w| w.window_id());
                    let debug_id = self.debug_window.as_ref().map(|w| w.window_id());
                    match win_event {
                        WindowEvent::Close => {
                            if Some(*window_id) == vram_id {
                                self.vram_window = None;
                                log_info!("DEBUG", "VRAM window closed");
                            } else if Some(*window_id) == debug_id {
                                self.debug_window = None;
                                log_info!("DEBUG", "CPU debugger closed");
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
            }

            self.imgui_sdl2.handle_event(&mut self.imgui, &event);

            if self.imgui_sdl2.ignore_event(&event) {
                continue;
            }

            match event {
                Event::KeyDown { keycode: Some(k), .. } => Frontend::handle_keydown(k, options, system),
                Event::KeyUp { keycode: Some(Keycode::F8), .. } => self.toggle_vram_window(),
                Event::KeyUp { keycode: Some(Keycode::F9), .. } => self.toggle_debug_window(),
                Event::KeyUp { keycode: Some(k), .. } => Frontend::handle_keyup(k, options, system),

                Event::ControllerButtonDown { button, .. } => Frontend::handle_controller_button(button, true, system),
                Event::ControllerButtonUp { button, .. } => Frontend::handle_controller_button(button, false, system),
                Event::ControllerAxisMotion { axis, value, .. } => Frontend::handle_controller_axis(axis, value, system),

                Event::ControllerDeviceAdded { which, .. } => {
                    if self.controller_subsystem.is_game_controller(which) {
                        match self.controller_subsystem.open(which) {
                            Ok(c) => {
                                log_info!("FRONTEND", "hotplug opened controller #{} {}", which, c.name());
                                self.controllers.push(c);
                            }
                            Err(e) => log_error!("FRONTEND", "hotplug failed for #{}: {}", which, e),
                        }
                    } else {
                        log_warn!("FRONTEND", "hotplug joystick #{} not recognized as game controller", which);
                    }
                },
                Event::ControllerDeviceRemoved { which, .. } => {
                    self.controllers.retain(|c| c.instance_id() != which);
                    log_info!("FRONTEND", "controller instance {} removed", which);
                },

                Event::Window { win_event, window_id, .. } => {
                    let main_id = self.window.id();
                    let vram_id = self.vram_window.as_ref().map(|w| w.window_id());
                    let debug_id = self.debug_window.as_ref().map(|w| w.window_id());
                    match win_event {
                        WindowEvent::Resized(width, height) => {
                            if window_id == main_id {
                                unsafe { gl::Viewport(0, 0, width, height); }
                            }
                        },
                        WindowEvent::Close => {
                            if Some(window_id) == vram_id {
                                self.vram_window = None;
                                log_info!("DEBUG", "VRAM window closed");
                            } else if Some(window_id) == debug_id {
                                self.debug_window = None;
                                log_info!("DEBUG", "CPU debugger closed");
                            } else if window_id == main_id {
                                system.running = false;
                            }
                        },
                        _ => {},
                    };
                },
                Event::Quit { .. } => system.running = false,
                _ => {},
            };
        }

        let id = system.get_disc_id();
        let title = format!("RustyStation - {} - slot {}", id, options.state_index);
        self.window.set_title(&title).expect("unable to set window title");
    }

    fn handle_controller_button(button: Button, down: bool, system: &mut System) {
        let controller = system.get_controller();

        match button {
            Button::A => controller.button_cross = down,
            Button::B => controller.button_circle = down,
            Button::X => controller.button_square = down,
            Button::Y => controller.button_triangle = down,
            Button::LeftShoulder => controller.button_l1 = down,
            Button::RightShoulder => controller.button_r1 = down,
            Button::Back => controller.button_select = down,
            Button::Start => controller.button_start = down,
            Button::LeftStick => controller.button_l3 = down,
            Button::RightStick => controller.button_r3 = down,
            Button::DPadUp => controller.button_dpad_up = down,
            Button::DPadDown => controller.button_dpad_down = down,
            Button::DPadLeft => controller.button_dpad_left = down,
            Button::DPadRight => controller.button_dpad_right = down,
            Button::Guide => {
                if !down {
                    controller.digital_mode ^= true;
                    log_info!("FRONTEND", "digital mode {}", if controller.digital_mode { "enabled" } else { "disabled" });
                }
            },
            _ => log_warn!("FRONTEND", "unhandled button {:#?}", button),
        }
    }

    fn handle_controller_axis(axis: Axis, value: i16, system: &mut System) {
        let controller = system.get_controller();
        let normalised = ((value >> 8) + 128) as u8;

        match axis {
            Axis::LeftX => controller.axis_lx = normalised,
            Axis::LeftY => controller.axis_ly = normalised,
            Axis::RightX => controller.axis_rx = normalised,
            Axis::RightY => controller.axis_ry = normalised,
            Axis::TriggerLeft => controller.button_l2 = normalised >= 192,
            Axis::TriggerRight => controller.button_r2 = normalised >= 192,
        }
    }

    fn handle_keydown(keycode: Keycode, _options: &mut Options, system: &mut System) {
        let controller = system.get_controller();

        match keycode {
            Keycode::W => controller.button_dpad_up = true,
            Keycode::A => controller.button_dpad_left = true,
            Keycode::S => controller.button_dpad_down = true,
            Keycode::D => controller.button_dpad_right = true,
            Keycode::Q => controller.button_select = true,
            Keycode::E => controller.button_start = true,
            Keycode::Kp2 => controller.button_cross = true,
            Keycode::Kp4 => controller.button_square = true,
            Keycode::Kp6 => controller.button_circle = true,
            Keycode::Kp8 => controller.button_triangle = true,
            Keycode::Num1 => controller.button_l1 = true,
            Keycode::Num2 => controller.button_l2 = true,
            Keycode::Num3 => controller.button_r1 = true,
            Keycode::Num4 => controller.button_r2 = true,
            _ => {},
        };
    }

    fn handle_keyup(keycode: Keycode, options: &mut Options, system: &mut System) {
        let controller = system.get_controller();

        match keycode {
            Keycode::Tab => options.frame_limit ^= true,
            Keycode::F2 => system.reset(),
            Keycode::F3 => options.step = true,
            Keycode::F4 => {
                options.scaling = match options.scaling {
                    Scaling::None => Scaling::Aspect,
                    Scaling::Aspect => Scaling::Fullscreen,
                    Scaling::Fullscreen => Scaling::None
                };
            }
            Keycode::F6 => Frontend::load_state(system, options.state_index),
            Keycode::F7 => Frontend::save_state(system, options.state_index),
            Keycode::Comma => {
                options.state_index += 1;
                options.state_index %= 10;
                log_info!("STATE", "selected save slot {}", options.state_index);
            },
            // F8/F9 are handled in update() because they need &mut self.
            Keycode::F10 => system.toggle_debug_dump(),
            // F11 keeps the old overscan toggle accessible (was F9).
            Keycode::F11 => options.crop_overscan ^= true,
            Keycode::P => options.pause ^= true,

            Keycode::W => controller.button_dpad_up = false,
            Keycode::A => controller.button_dpad_left = false,
            Keycode::S => controller.button_dpad_down = false,
            Keycode::D => controller.button_dpad_right = false,
            Keycode::Q => controller.button_select = false,
            Keycode::E => controller.button_start = false,
            Keycode::Kp2 => controller.button_cross = false,
            Keycode::Kp4 => controller.button_square = false,
            Keycode::Kp6 => controller.button_circle = false,
            Keycode::Kp8 => controller.button_triangle = false,
            Keycode::Num1 => controller.button_l1 = false,
            Keycode::Num2 => controller.button_l2 = false,
            Keycode::Num3 => controller.button_r1 = false,
            Keycode::Num4 => controller.button_r2 = false,
            _ => {},
        };
    }

    fn load_state(system: &mut System, index: usize) {
        log_info!("STATE", "loading slot {}", index);

        let id = system.get_disc_id_raw();
        let name = format!("./states/{id}_slot{index}.state");
        let path = Path::new(&name);

        if !path.exists() {
            log_warn!("STATE", "no file for save state {}", index);
            return;
        }

        if let Ok(file) = File::open(path) {
            let mut bytes = Vec::new();
            let mut decompressor = XzDecoder::new(file);
            decompressor.read_to_end(&mut bytes).unwrap();
            *system = rmp_serde::from_slice(&bytes).unwrap();
            system.reload_host_files();
            system.get_controller().reset_switch_state();
            log_info!("STATE", "loaded slot {}", index);
        } else {
            log_error!("STATE", "unable to open save state file");
        }
    }

    fn save_state(system: &mut System, index: usize) {
        log_info!("STATE", "saving slot {}", index);

        let id = system.get_disc_id_raw();
        let name = format!("./states/{id}_slot{index}.state");
        let path = Path::new(&name);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("unable to create path to save state file");
        }

        if let Ok(file) = File::create(path) {
            let bytes = rmp_serde::to_vec(system).expect("unable to serialize state");
            let mut compressor = XzEncoder::new(file, 6);
            compressor.write_all(&bytes).unwrap();
            compressor.finish().unwrap();
            log_info!("STATE", "saved slot {}", index);
        } else {
            log_error!("STATE", "unable to create save state file");
        }
    }

    pub fn render(&mut self, options: &Options, system: &System) {
        let (width, height) = match options.draw_full_vram {
            true => (1024, 512),
            false => system.get_display_size(),
        };

        let mut vertices: [[f32; 4]; 4] = [
            [-1.0,  1.0, 0.0, 0.0],
            [ 1.0,  1.0, 1.0, 0.0],
            [-1.0, -1.0, 0.0, 1.0],
            [ 1.0, -1.0, 1.0, 1.0],
        ];

        if !options.draw_full_vram {
            let (scale_x, scale_y) = match options.scaling {
                Scaling::None => self.calculate_scale_none(),
                Scaling::Aspect => self.calculate_scale_aspect(options.crop_overscan, system.pal_mode()),
                Scaling::Fullscreen => {
                    let mut scale = (1.0, 1.0);

                    if options.crop_overscan {
                        scale.1 *= if system.pal_mode() { 288.0/252.0 } else { 240.0/216.0 };
                    }

                    scale
                },
            };

            vertices[0][0] *= scale_x;
            vertices[0][1] *= scale_y;

            vertices[1][0] *= scale_x;
            vertices[1][1] *= scale_y;

            vertices[2][0] *= scale_x;
            vertices[2][1] *= scale_y;

            vertices[3][0] *= scale_x;
            vertices[3][1] *= scale_y;
        };

        system.get_framebuffer(&mut self.framebuffer, options.draw_full_vram);

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&vertices) as isize, vertices.as_ptr() as _, gl::DYNAMIC_DRAW);

            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB8 as i32, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, self.framebuffer.as_ptr() as _);
        }

        self.imgui_sdl2.prepare_frame(self.imgui.io_mut(), &self.window, &self.event_pump.mouse_state());

        let now = Instant::now();
        let delta = now - self.last_frame;
        self.last_frame = now;

        self.imgui.io_mut().delta_time = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;

        let ui = self.imgui.frame();
        //ui.show_demo_window(&mut true);

        unsafe {
            gl::UseProgram(self.program);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        self.imgui_sdl2.prepare_render(&ui, &self.window);
        self.imgui_renderer.render(ui);

        self.window.gl_swap_window();

        // Render debug auxiliary windows after the main GL swap. These use a
        // software Canvas so they don't interfere with the main GL context.
        if let Some(w) = self.vram_window.as_mut() {
            w.render(system);
        }
        if let Some(w) = self.debug_window.as_mut() {
            w.render(system);
        }

        if options.frame_limit {
            let frame_time = if system.pal_mode() {
                Duration::from_nanos(20_000_000)
            } else {
                Duration::from_nanos(16_666_667)
            };

            self.frame_deadline += frame_time;
            let now = Instant::now();

            if self.frame_deadline > now + frame_time {
                self.frame_deadline = now + frame_time;
            }

            if self.frame_deadline > now {
                thread::sleep(self.frame_deadline - now);
            } else if now - self.frame_deadline > frame_time {
                self.frame_deadline = now;
            }
        } else {
            self.frame_deadline = Instant::now();
        }
    }

    fn get_screen_ratio(&self) -> (f32, f32) {
        let (window_w, window_h) = self.window.size();

        let rx = 640.0 / window_w as f32;
        let ry = 480.0 / window_h as f32;

        (rx, ry)
    }

    fn calculate_scale_none(&self) -> (f32, f32) {
        let (x, y) = self.get_screen_ratio();
        (util::clip(x, 0.0, 1.0), util::clip(y, 0.0, 1.0))
    }

    fn calculate_scale_aspect(&self, crop_overscan: bool, pal: bool) -> (f32, f32) {
        let (x, y) = self.get_screen_ratio();

        let scale = if crop_overscan {
            if pal { 288.0/252.0 } else { 240.0/216.0 }
        } else {
            1.0
        };

        (x / x.max(y / scale), y * scale / x.max(y))
    }
}

impl Drop for Frontend {
    fn drop(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::UseProgram(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteProgram(self.program);
            gl::DeleteTextures(1, &self.texture);
        }
    }
}
