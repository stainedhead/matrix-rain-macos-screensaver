//! Terminal-based renderer using crossterm

#[cfg(feature = "cli")]
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color as TermColor, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{Color, RenderChar, Renderer};

#[cfg(feature = "cli")]
use std::io::{self, Write};

/// Terminal renderer using crossterm
#[cfg(feature = "cli")]
pub struct TerminalRenderer {
    width: u32,
    height: u32,
}

#[cfg(feature = "cli")]
impl TerminalRenderer {
    /// Create a new terminal renderer
    pub fn new() -> io::Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            width: width as u32,
            height: height as u32,
        })
    }

    /// Initialize the terminal for rendering
    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            Hide,
            SetBackgroundColor(TermColor::Black),
            Clear(ClearType::All)
        )?;
        Ok(())
    }

    /// Restore the terminal to normal mode
    pub fn cleanup(&mut self) -> io::Result<()> {
        execute!(io::stdout(), ResetColor, Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Convert Color to terminal Color
    fn to_term_color(&self, color: &Color) -> TermColor {
        TermColor::Rgb {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

#[cfg(feature = "cli")]
impl Renderer for TerminalRenderer {
    fn clear(&mut self, _color: Color) {
        let _ = execute!(
            io::stdout(),
            SetBackgroundColor(TermColor::Black),
            Clear(ClearType::All)
        );
    }

    fn draw_char(&mut self, render_char: &RenderChar) {
        // Convert pixel coordinates to character coordinates
        // Assuming average terminal char width/height ratio
        let col = (render_char.x / 8.0) as u16;
        let row = (render_char.y / 16.0) as u16;

        // Only render if within terminal bounds
        if col < self.width as u16 && row < self.height as u16 {
            let term_color = self.to_term_color(&render_char.color);

            // Apply alpha by adjusting brightness (approximate)
            let adjusted_color = if render_char.color.a < 0.3 {
                // Very transparent - use dark version
                TermColor::Rgb {
                    r: (render_char.color.r as f32 * 0.3) as u8,
                    g: (render_char.color.g as f32 * 0.3) as u8,
                    b: (render_char.color.b as f32 * 0.3) as u8,
                }
            } else if render_char.color.a < 0.7 {
                // Semi-transparent - use medium brightness
                TermColor::Rgb {
                    r: (render_char.color.r as f32 * 0.6) as u8,
                    g: (render_char.color.g as f32 * 0.6) as u8,
                    b: (render_char.color.b as f32 * 0.6) as u8,
                }
            } else {
                // Mostly opaque - use full color
                term_color
            };

            let _ = execute!(
                io::stdout(),
                MoveTo(col, row),
                SetForegroundColor(adjusted_color),
                Print(render_char.character)
            );
        }
    }

    fn present(&mut self) {
        let _ = io::stdout().flush();
    }

    fn width(&self) -> u32 {
        // Return pixel width (approximate)
        self.width * 8
    }

    fn height(&self) -> u32 {
        // Return pixel height (approximate)
        self.height * 16
    }
}

#[cfg(feature = "cli")]
impl Default for TerminalRenderer {
    fn default() -> Self {
        Self::new().unwrap_or(Self {
            width: 120,
            height: 30,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "cli")]
    fn test_color_conversion() {
        let renderer = TerminalRenderer::default();
        let color = Color::rgb(255, 128, 64);
        let term_color = renderer.to_term_color(&color);

        match term_color {
            TermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 128);
                assert_eq!(b, 64);
            }
            _ => panic!("Expected RGB color"),
        }
    }
}
