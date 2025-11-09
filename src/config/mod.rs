//! Configuration module for Matrix Rain screensaver
//!
//! Provides character sets, color schemes, speed settings, and overall configuration.

mod character_sets;
mod colors;
mod speed;

pub use character_sets::CharacterSet;
pub use colors::ColorScheme;
pub use speed::RainSpeed;

use serde::{Deserialize, Serialize};

/// Main configuration for the Matrix Rain screensaver
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScreenSaverConfig {
    /// The character set to use for the rain effect
    pub character_set: CharacterSet,
    /// The color scheme for the characters
    pub color_scheme: ColorScheme,
    /// The speed of the rain animation
    pub speed: RainSpeed,
    /// Screen width in pixels
    pub screen_width: u32,
    /// Screen height in pixels
    pub screen_height: u32,
}

impl Default for ScreenSaverConfig {
    fn default() -> Self {
        Self {
            character_set: CharacterSet::Japanese,
            color_scheme: ColorScheme::MatrixGreen,
            speed: RainSpeed::Medium,
            screen_width: 1920,
            screen_height: 1080,
        }
    }
}

impl ScreenSaverConfig {
    /// Create a new configuration with specified parameters
    pub fn new(
        character_set: CharacterSet,
        color_scheme: ColorScheme,
        speed: RainSpeed,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        Self {
            character_set,
            color_scheme,
            speed,
            screen_width,
            screen_height,
        }
    }

    /// Load configuration from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Save configuration to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ScreenSaverConfig::default();
        assert_eq!(config.character_set, CharacterSet::Japanese);
        assert_eq!(config.color_scheme, ColorScheme::MatrixGreen);
        assert_eq!(config.speed, RainSpeed::Medium);
    }

    #[test]
    fn test_config_serialization() {
        let config = ScreenSaverConfig::default();
        let json = config.to_json().unwrap();
        let deserialized = ScreenSaverConfig::from_json(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_creation() {
        let config = ScreenSaverConfig::new(
            CharacterSet::Korean,
            ColorScheme::DarkBlue,
            RainSpeed::Fast,
            2560,
            1440,
        );
        assert_eq!(config.character_set, CharacterSet::Korean);
        assert_eq!(config.color_scheme, ColorScheme::DarkBlue);
        assert_eq!(config.speed, RainSpeed::Fast);
    }
}
