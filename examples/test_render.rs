//! Test the rendering output to verify colors and background layer

use matrix_rain_core::rendering::{Color, RenderChar, Renderer};
use matrix_rain_core::*;

struct DebugRenderer {
    width: u32,
    height: u32,
    chars_rendered: Vec<RenderChar>,
}

impl DebugRenderer {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            chars_rendered: Vec::new(),
        }
    }
}

impl Renderer for DebugRenderer {
    fn clear(&mut self, _color: Color) {
        self.chars_rendered.clear();
    }

    fn draw_char(&mut self, render_char: &RenderChar) {
        self.chars_rendered.push(render_char.clone());
    }

    fn present(&mut self) {
        // No-op
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    // Create config with background layer enabled
    let config = ScreenSaverConfig::new(
        CharacterSet::Japanese,
        ColorScheme::MatrixGreen,
        RainSpeed::Medium,
        800,
        600,
    );

    println!("Config created:");
    println!("  Character set: {:?}", config.character_set);
    println!("  Color scheme: {:?}", config.color_scheme);
    println!("  Speed: {:?}", config.speed);
    println!("  Screen: {}x{}", config.screen_width, config.screen_height);
    println!(
        "  Background layer enabled: {}",
        config.enable_background_layer
    );

    // Create engine
    let mut matrix = MatrixRain::new(config);
    let mut renderer = DebugRenderer::new(800, 600);

    // Update and render several times
    for i in 0..10 {
        matrix.update();
        matrix.render(&mut renderer);

        println!(
            "\nFrame {}: Rendered {} characters",
            i + 1,
            renderer.chars_rendered.len()
        );

        // Check for color variations
        let mut colors_seen = std::collections::HashSet::new();
        for ch in &renderer.chars_rendered {
            colors_seen.insert((ch.color.r, ch.color.g, ch.color.b));
        }

        println!("  Unique RGB colors: {}", colors_seen.len());

        // Show first few colors
        for (i, (r, g, b)) in colors_seen.iter().take(5).enumerate() {
            println!("    Color {}: RGB({}, {}, {})", i + 1, r, g, b);
        }
    }
}
