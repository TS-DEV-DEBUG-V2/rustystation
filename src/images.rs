//! Embedded raster art bundled at compile time via `include_bytes!`.
//! Both files are stored as 24-bit Windows .BMP next to this module so the
//! Rust compiler doesn't have to parse them and the build stays fast.

pub static GITHUB_BMP: &[u8] = include_bytes!("github.bmp");
pub static LOGO_BMP: &[u8] = include_bytes!("logo.bmp");
