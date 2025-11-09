//! Abstract renderer interface

use super::Color;

/// A character to be rendered at a specific position
#[derive(Debug, Clone)]
pub struct RenderChar {
    /// The character to render
    pub character: char,
    /// X position in pixels
    pub x: f32,
    /// Y position in pixels
    pub y: f32,
    /// Color of the character
    pub color: Color,
    /// Font size
    pub font_size: f32,
}

/// Trait for rendering the matrix rain effect
/// Implementations will handle platform-specific drawing
pub trait Renderer {
    /// Clear the screen with the specified background color
    fn clear(&mut self, color: Color);

    /// Draw a single character at the specified position
    fn draw_char(&mut self, render_char: &RenderChar);

    /// Draw multiple characters
    fn draw_chars(&mut self, chars: &[RenderChar]) {
        for ch in chars {
            self.draw_char(ch);
        }
    }

    /// Present/flush the rendered frame to screen
    fn present(&mut self);

    /// Get the screen width in pixels
    fn width(&self) -> u32;

    /// Get the screen height in pixels
    fn height(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock renderer for testing
    struct MockRenderer {
        width: u32,
        height: u32,
        chars_drawn: Vec<RenderChar>,
    }

    impl MockRenderer {
        fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height,
                chars_drawn: Vec::new(),
            }
        }
    }

    impl Renderer for MockRenderer {
        fn clear(&mut self, _color: Color) {
            self.chars_drawn.clear();
        }

        fn draw_char(&mut self, render_char: &RenderChar) {
            self.chars_drawn.push(render_char.clone());
        }

        fn present(&mut self) {
            // No-op for mock
        }

        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }
    }

    #[test]
    fn test_mock_renderer() {
        let mut renderer = MockRenderer::new(1920, 1080);
        assert_eq!(renderer.width(), 1920);
        assert_eq!(renderer.height(), 1080);

        let render_char = RenderChar {
            character: 'A',
            x: 100.0,
            y: 200.0,
            color: Color::MATRIX_GREEN,
            font_size: 16.0,
        };

        renderer.draw_char(&render_char);
        assert_eq!(renderer.chars_drawn.len(), 1);
        assert_eq!(renderer.chars_drawn[0].character, 'A');
    }

    #[test]
    fn test_draw_multiple_chars() {
        let mut renderer = MockRenderer::new(1920, 1080);

        let chars = vec![
            RenderChar {
                character: 'A',
                x: 0.0,
                y: 0.0,
                color: Color::MATRIX_GREEN,
                font_size: 16.0,
            },
            RenderChar {
                character: 'B',
                x: 20.0,
                y: 0.0,
                color: Color::MATRIX_GREEN,
                font_size: 16.0,
            },
        ];

        renderer.draw_chars(&chars);
        assert_eq!(renderer.chars_drawn.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut renderer = MockRenderer::new(1920, 1080);

        let render_char = RenderChar {
            character: 'A',
            x: 100.0,
            y: 200.0,
            color: Color::MATRIX_GREEN,
            font_size: 16.0,
        };

        renderer.draw_char(&render_char);
        assert_eq!(renderer.chars_drawn.len(), 1);

        renderer.clear(Color::BLACK);
        assert_eq!(renderer.chars_drawn.len(), 0);
    }
}
