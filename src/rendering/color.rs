//! Color representation and utilities

use serde::{Deserialize, Serialize};

/// Represents an RGBA color
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0.0-1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color with full opacity
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a new color with specified alpha
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create a color from RGB tuple
    pub fn from_rgb_tuple(rgb: (u8, u8, u8)) -> Self {
        Self::rgb(rgb.0, rgb.1, rgb.2)
    }

    /// Create a color from RGBA tuple
    pub fn from_rgba_tuple(rgba: (u8, u8, u8, f32)) -> Self {
        Self::rgba(rgba.0, rgba.1, rgba.2, rgba.3)
    }

    /// Get color as normalized values (0.0-1.0)
    pub fn as_normalized(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a,
        )
    }

    /// Darken the color by a factor (0.0 = black, 1.0 = no change)
    pub fn darken(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self::rgba(
            (self.r as f32 * factor) as u8,
            (self.g as f32 * factor) as u8,
            (self.b as f32 * factor) as u8,
            self.a,
        )
    }

    /// Lighten the color by a factor (0.0 = no change, 1.0 = white)
    pub fn lighten(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self::rgba(
            (self.r as f32 + (255.0 - self.r as f32) * factor) as u8,
            (self.g as f32 + (255.0 - self.g as f32) * factor) as u8,
            (self.b as f32 + (255.0 - self.b as f32) * factor) as u8,
            self.a,
        )
    }

    /// Set the alpha value
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self::rgba(self.r, self.g, self.b, alpha)
    }

    /// Common colors
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 1.0,
    };
    pub const MATRIX_GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 70,
        a: 1.0,
    };
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::rgb(100, 150, 200);
        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_with_alpha() {
        let color = Color::rgba(100, 150, 200, 0.5);
        assert_eq!(color.a, 0.5);
    }

    #[test]
    fn test_alpha_clamping() {
        let color1 = Color::rgba(100, 150, 200, 1.5);
        assert_eq!(color1.a, 1.0);

        let color2 = Color::rgba(100, 150, 200, -0.5);
        assert_eq!(color2.a, 0.0);
    }

    #[test]
    fn test_normalized_values() {
        let color = Color::rgb(255, 128, 0);
        let (r, g, b, a) = color.as_normalized();
        assert!((r - 1.0).abs() < 0.01);
        assert!((g - 0.502).abs() < 0.01);
        assert!((b - 0.0).abs() < 0.01);
        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_darken() {
        let color = Color::rgb(100, 100, 100);
        let darkened = color.darken(0.5);
        assert_eq!(darkened.r, 50);
        assert_eq!(darkened.g, 50);
        assert_eq!(darkened.b, 50);
    }

    #[test]
    fn test_lighten() {
        let color = Color::rgb(100, 100, 100);
        let lightened = color.lighten(0.5);
        assert!(lightened.r > 100);
        assert!(lightened.g > 100);
        assert!(lightened.b > 100);
    }

    #[test]
    fn test_with_alpha() {
        let color = Color::rgb(100, 150, 200);
        let with_alpha = color.with_alpha(0.7);
        assert_eq!(with_alpha.r, 100);
        assert_eq!(with_alpha.g, 150);
        assert_eq!(with_alpha.b, 200);
        assert_eq!(with_alpha.a, 0.7);
    }

    #[test]
    fn test_from_tuples() {
        let color1 = Color::from_rgb_tuple((100, 150, 200));
        assert_eq!(color1.r, 100);
        assert_eq!(color1.g, 150);
        assert_eq!(color1.b, 200);

        let color2 = Color::from_rgba_tuple((100, 150, 200, 0.5));
        assert_eq!(color2.a, 0.5);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Color::BLACK.r, 0);
        assert_eq!(Color::WHITE.r, 255);
        assert_eq!(Color::MATRIX_GREEN.g, 255);
    }
}
