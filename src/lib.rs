//! Matrix Rain Core Library
//!
//! A library for rendering Matrix-style digital rain effects with customizable
//! character sets, colors, and animation speeds.

pub mod config;
pub mod engine;
pub mod rendering;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use config::{CharacterSet, ColorScheme, RainSpeed, ScreenSaverConfig};
pub use engine::{MatrixRain, RainColumn};
pub use rendering::{Color, Renderer};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = ScreenSaverConfig::default();
        assert_eq!(config.character_set, CharacterSet::Japanese);
        assert_eq!(config.speed, RainSpeed::Medium);
    }
}
