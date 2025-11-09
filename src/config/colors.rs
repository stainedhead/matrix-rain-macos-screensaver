//! Color schemes for the Matrix rain effect

use serde::{Deserialize, Serialize};

/// Available color schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ColorScheme {
    /// Classic Matrix green
    #[default]
    MatrixGreen,
    /// Dark blue
    DarkBlue,
    /// Purple/Violet
    Purple,
    /// Orange/Amber
    Orange,
    /// Red
    Red,
    /// Cyan/Aqua
    Cyan,
    /// Yellow/Gold
    Yellow,
    /// Pink/Magenta
    Pink,
    /// White/Silver
    White,
    /// Lime Green
    LimeGreen,
    /// Teal
    Teal,
}

impl ColorScheme {
    /// Get the primary color RGB values (0-255)
    /// Returns (r, g, b) for the brightest/newest character
    pub fn get_primary_color(&self) -> (u8, u8, u8) {
        match self {
            ColorScheme::MatrixGreen => (0, 255, 70),
            ColorScheme::DarkBlue => (0, 150, 255),
            ColorScheme::Purple => (200, 100, 255),
            ColorScheme::Orange => (255, 165, 0),
            ColorScheme::Red => (255, 50, 50),
            ColorScheme::Cyan => (0, 255, 255),
            ColorScheme::Yellow => (255, 255, 0),
            ColorScheme::Pink => (255, 105, 180),
            ColorScheme::White => (255, 255, 255),
            ColorScheme::LimeGreen => (50, 255, 50),
            ColorScheme::Teal => (0, 200, 200),
        }
    }

    /// Get the secondary color RGB values (0-255)
    /// Returns (r, g, b) for mid-trail characters
    pub fn get_secondary_color(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.get_primary_color();
        // Darken by about 40%
        (
            (r as f32 * 0.6) as u8,
            (g as f32 * 0.6) as u8,
            (b as f32 * 0.6) as u8,
        )
    }

    /// Get the tertiary color RGB values (0-255)
    /// Returns (r, g, b) for oldest/fading characters
    pub fn get_tertiary_color(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.get_primary_color();
        // Darken by about 70%
        (
            (r as f32 * 0.3) as u8,
            (g as f32 * 0.3) as u8,
            (b as f32 * 0.3) as u8,
        )
    }

    /// Get color with alpha transparency (0.0 = transparent, 1.0 = opaque)
    /// Returns (r, g, b, a) with RGB in 0-255 range and alpha in 0.0-1.0 range
    ///
    /// Classic Matrix effect:
    /// - Leading character (position 0.0): Bright white
    /// - Next few characters (0.0-0.15): Bright primary color
    /// - Mid trail (0.15-0.5): Medium brightness
    /// - Tail (0.5-1.0): Fading to black
    pub fn get_color_with_alpha(&self, position_in_trail: f32) -> (u8, u8, u8, f32) {
        let (r, g, b) = if position_in_trail < 0.05 {
            // Leading character is bright white for that classic Matrix look
            (255, 255, 255)
        } else if position_in_trail < 0.15 {
            // Very bright primary color right behind the leader
            self.get_primary_color()
        } else if position_in_trail < 0.5 {
            // Medium brightness in mid-trail
            self.get_secondary_color()
        } else {
            // Fading tail
            self.get_tertiary_color()
        };

        // Alpha decreases more gradually for a longer visible trail
        let alpha = if position_in_trail < 0.1 {
            1.0 // Leading characters fully opaque
        } else {
            // Smooth fade from 1.0 to 0.0
            (1.0 - (position_in_trail - 0.1) / 0.9).clamp(0.0, 1.0)
        };

        (r, g, b, alpha)
    }

    /// Get all available color schemes
    pub fn all_schemes() -> Vec<ColorScheme> {
        vec![
            ColorScheme::MatrixGreen,
            ColorScheme::DarkBlue,
            ColorScheme::Purple,
            ColorScheme::Orange,
            ColorScheme::Red,
            ColorScheme::Cyan,
            ColorScheme::Yellow,
            ColorScheme::Pink,
            ColorScheme::White,
            ColorScheme::LimeGreen,
            ColorScheme::Teal,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_green_colors() {
        let scheme = ColorScheme::MatrixGreen;
        let primary = scheme.get_primary_color();
        let secondary = scheme.get_secondary_color();
        let tertiary = scheme.get_tertiary_color();

        assert_eq!(primary, (0, 255, 70));
        assert!(secondary.1 < primary.1); // Green channel should be darker
        assert!(tertiary.1 < secondary.1); // Even darker
    }

    #[test]
    fn test_all_color_schemes() {
        let schemes = ColorScheme::all_schemes();
        assert_eq!(schemes.len(), 11);

        for scheme in schemes {
            let (r, g, b) = scheme.get_primary_color();
            // At least one channel should be fairly bright
            assert!(r > 0 || g > 0 || b > 0);
        }
    }

    #[test]
    fn test_color_with_alpha() {
        let scheme = ColorScheme::MatrixGreen;

        // Test head of trail (bright)
        let (r, g, b, a) = scheme.get_color_with_alpha(0.1);
        assert!(a > 0.8);
        assert_eq!((r, g, b), scheme.get_primary_color());

        // Test middle of trail
        let (_, _, _, a) = scheme.get_color_with_alpha(0.5);
        assert!(a > 0.3 && a < 0.7);

        // Test end of trail (faded)
        let (_, _, _, a) = scheme.get_color_with_alpha(0.9);
        assert!(a < 0.2);
    }

    #[test]
    fn test_default_color_scheme() {
        assert_eq!(ColorScheme::default(), ColorScheme::MatrixGreen);
    }

    #[test]
    fn test_color_progression() {
        let scheme = ColorScheme::DarkBlue;
        let primary = scheme.get_primary_color();
        let secondary = scheme.get_secondary_color();
        let tertiary = scheme.get_tertiary_color();

        // Each level should be darker than the previous
        assert!(primary.0 >= secondary.0);
        assert!(primary.1 >= secondary.1);
        assert!(primary.2 >= secondary.2);
        assert!(secondary.0 >= tertiary.0);
        assert!(secondary.1 >= tertiary.1);
        assert!(secondary.2 >= tertiary.2);
    }
}
