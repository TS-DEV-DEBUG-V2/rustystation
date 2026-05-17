use std::io::Write;
use std::sync::Once;

#[cfg(windows)]
fn enable_windows_ansi() {
    use std::ptr;
    extern "system" {
        fn GetStdHandle(n: u32) -> *mut core::ffi::c_void;
        fn GetConsoleMode(h: *mut core::ffi::c_void, m: *mut u32) -> i32;
        fn SetConsoleMode(h: *mut core::ffi::c_void, m: u32) -> i32;
    }
    const STD_OUTPUT_HANDLE: u32 = (-11i32) as u32;
    const STD_ERROR_HANDLE: u32 = (-12i32) as u32;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
    unsafe {
        for h_id in [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
            let h = GetStdHandle(h_id);
            if h.is_null() || h == ptr::null_mut() { continue; }
            let mut mode: u32 = 0;
            if GetConsoleMode(h, &mut mode) != 0 {
                let _ = SetConsoleMode(h, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}

#[cfg(not(windows))]
fn enable_windows_ansi() {}

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        enable_windows_ansi();
    });
}

pub mod color {
    pub const RESET:   &str = "\x1b[0m";
    pub const BOLD:    &str = "\x1b[1m";
    pub const DIM:     &str = "\x1b[2m";

    pub const GRAY:    &str = "\x1b[90m";
    pub const RED:     &str = "\x1b[31m";
    pub const GREEN:   &str = "\x1b[32m";
    pub const YELLOW:  &str = "\x1b[33m";
    pub const BLUE:    &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN:    &str = "\x1b[36m";
    pub const WHITE:   &str = "\x1b[37m";

    pub const BRED:    &str = "\x1b[91m";
    pub const BGREEN:  &str = "\x1b[92m";
    pub const BYELLOW: &str = "\x1b[93m";
    pub const BCYAN:   &str = "\x1b[96m";
}

pub fn write_line(level_color: &str, level: &str, tag_color: &str, tag: &str, msg: &str) {
    init();
    let stderr = std::io::stderr();
    let mut h = stderr.lock();
    let _ = writeln!(
        h,
        "{bold}{lvc}{lv:>5}{r} {tc}[{tag}]{r} {msg}",
        bold = color::BOLD,
        lvc  = level_color,
        lv   = level,
        r    = color::RESET,
        tc   = tag_color,
        tag  = tag,
        msg  = msg,
    );
}

#[macro_export]
macro_rules! log_info {
    ($tag:expr, $($arg:tt)*) => {{
        $crate::log::write_line(
            $crate::log::color::BGREEN,  "INFO",
            $crate::log::color::CYAN,    $tag,
            &format!($($arg)*),
        );
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($tag:expr, $($arg:tt)*) => {{
        $crate::log::write_line(
            $crate::log::color::BYELLOW, "WARN",
            $crate::log::color::CYAN,    $tag,
            &format!($($arg)*),
        );
    }};
}

#[macro_export]
macro_rules! log_error {
    ($tag:expr, $($arg:tt)*) => {{
        $crate::log::write_line(
            $crate::log::color::BRED,    "ERROR",
            $crate::log::color::CYAN,    $tag,
            &format!($($arg)*),
        );
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($tag:expr, $($arg:tt)*) => {{
        $crate::log::write_line(
            $crate::log::color::MAGENTA, "DEBUG",
            $crate::log::color::CYAN,    $tag,
            &format!($($arg)*),
        );
    }};
}
