//! Main Matrix Rain engine

use crate::config::ScreenSaverConfig;
use crate::rendering::{Color, RenderChar, Renderer};
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::RainColumn;

/// The main Matrix Rain engine
pub struct MatrixRain {
    /// Configuration
    config: ScreenSaverConfig,
    /// Foreground rain columns (main bright rain)
    columns: Vec<RainColumn>,
    /// Background rain columns (subtle depth layer)
    background_columns: Vec<RainColumn>,
    /// Random number generator
    rng: StdRng,
    /// Character width in pixels
    char_width: f32,
    /// Character height in pixels
    char_height: f32,
    /// Font size
    font_size: f32,
}

impl MatrixRain {
    /// Create a new Matrix Rain engine
    pub fn new(config: ScreenSaverConfig) -> Self {
        let mut rng = StdRng::from_entropy();

        // Calculate character dimensions
        let font_size = 16.0;
        let char_width = font_size * 0.6; // Monospace font ratio
        let char_height = font_size * 1.2; // Include line spacing

        // Calculate number of columns
        let num_columns = (config.screen_width as f32 / char_width).ceil() as usize;

        // Create foreground columns with staggered start times
        let max_length = config.speed.max_trail_length();
        let base_speed = config.speed.speed_multiplier();

        let mut columns = Vec::with_capacity(num_columns);
        for x in 0..num_columns {
            let column = RainColumn::new(x, max_length, base_speed, &mut rng);
            columns.push(column);
        }

        // Create background columns (more sparse, slower, dimmer)
        let mut background_columns = Vec::new();
        if config.enable_background_layer {
            // Use every 3rd column for background (sparser)
            for x in (0..num_columns).step_by(3) {
                // Background rain is slower (60% of normal speed)
                let bg_speed = base_speed * 0.6;
                // Shorter trails for background
                let bg_max_length = max_length / 2;
                let column = RainColumn::new(x, bg_max_length, bg_speed, &mut rng);
                background_columns.push(column);
            }
        }

        Self {
            config,
            columns,
            background_columns,
            rng,
            char_width,
            char_height,
            font_size,
        }
    }

    /// Update the animation state
    pub fn update(&mut self) {
        let char_set = self.config.character_set;
        let screen_height = self.config.screen_height as f32;

        // Update foreground columns
        for column in &mut self.columns {
            column.update(&char_set, &mut self.rng);

            // Reset columns that have moved off screen
            if column.is_off_screen(screen_height, self.char_height) {
                // Random chance to start a new column or wait
                if self.rng.gen_bool(0.1) {
                    column.reset(&mut self.rng);
                } else {
                    column.active = false;
                }
            }
        }

        // Randomly activate inactive foreground columns
        for column in &mut self.columns {
            if !column.active && self.rng.gen_bool(0.01) {
                column.reset(&mut self.rng);
            }
        }

        // Update background columns (if enabled)
        if self.config.enable_background_layer {
            for column in &mut self.background_columns {
                column.update(&char_set, &mut self.rng);

                // Reset background columns with lower frequency
                if column.is_off_screen(screen_height, self.char_height) {
                    if self.rng.gen_bool(0.05) {
                        column.reset(&mut self.rng);
                    } else {
                        column.active = false;
                    }
                }
            }

            // Randomly activate inactive background columns (less frequent)
            for column in &mut self.background_columns {
                if !column.active && self.rng.gen_bool(0.005) {
                    column.reset(&mut self.rng);
                }
            }
        }
    }

    /// Render the current state
    pub fn render(&self, renderer: &mut impl Renderer) {
        // Clear screen with black
        renderer.clear(Color::BLACK);

        // Collect all characters to render
        let mut render_chars = Vec::new();

        // Render background layer first (if enabled)
        if self.config.enable_background_layer {
            for column in &self.background_columns {
                if !column.active {
                    continue;
                }

                let x_pixel = column.x as f32 * self.char_width;

                for (ch, y_pos, trail_pos) in column.get_trail_positions() {
                    if y_pos < 0.0 {
                        continue;
                    }

                    let y_pixel = y_pos * self.char_height;

                    if y_pixel > self.config.screen_height as f32 {
                        continue;
                    }

                    // Background characters are much dimmer (30% opacity, no white leader)
                    let rgba = self.config.color_scheme.get_color_with_alpha(trail_pos);
                    let mut color = Color::from_rgba_tuple(rgba);
                    color.a *= 0.3; // Reduce alpha to 30% for subtle background effect

                    render_chars.push(RenderChar {
                        character: ch,
                        x: x_pixel,
                        y: y_pixel,
                        color,
                        font_size: self.font_size * 0.9, // Slightly smaller font for depth
                    });
                }
            }
        }

        // Render foreground layer (main rain)
        for column in &self.columns {
            if !column.active {
                continue;
            }

            let x_pixel = column.x as f32 * self.char_width;

            for (ch, y_pos, trail_pos) in column.get_trail_positions() {
                // Skip characters above screen
                if y_pos < 0.0 {
                    continue;
                }

                let y_pixel = y_pos * self.char_height;

                // Skip characters below screen
                if y_pixel > self.config.screen_height as f32 {
                    continue;
                }

                // Get color based on position in trail
                let rgba = self.config.color_scheme.get_color_with_alpha(trail_pos);
                let color = Color::from_rgba_tuple(rgba);

                render_chars.push(RenderChar {
                    character: ch,
                    x: x_pixel,
                    y: y_pixel,
                    color,
                    font_size: self.font_size,
                });
            }
        }

        // Render all characters
        renderer.draw_chars(&render_chars);
        renderer.present();
    }

    /// Get the current configuration
    pub fn config(&self) -> &ScreenSaverConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: ScreenSaverConfig) {
        // If screen dimensions changed, recreate columns
        let dimensions_changed = config.screen_width != self.config.screen_width
            || config.screen_height != self.config.screen_height;

        // If speed changed, update column speeds and max lengths
        let speed_changed = config.speed != self.config.speed;

        self.config = config;

        if dimensions_changed {
            // Recalculate foreground columns
            let num_columns = (self.config.screen_width as f32 / self.char_width).ceil() as usize;
            let max_length = self.config.speed.max_trail_length();
            let base_speed = self.config.speed.speed_multiplier();

            self.columns.clear();
            for x in 0..num_columns {
                let column = RainColumn::new(x, max_length, base_speed, &mut self.rng);
                self.columns.push(column);
            }

            // Recalculate background columns
            self.background_columns.clear();
            if self.config.enable_background_layer {
                for x in (0..num_columns).step_by(3) {
                    let bg_speed = base_speed * 0.6;
                    let bg_max_length = max_length / 2;
                    let column = RainColumn::new(x, bg_max_length, bg_speed, &mut self.rng);
                    self.background_columns.push(column);
                }
            }
        } else if speed_changed {
            let max_length = self.config.speed.max_trail_length();
            let base_speed = self.config.speed.speed_multiplier();

            // Update foreground column speeds
            for column in &mut self.columns {
                column.speed = base_speed * self.rng.gen_range(0.7..=1.3);
                column.max_length = self.rng.gen_range(max_length / 2..=max_length);
            }

            // Update background column speeds
            for column in &mut self.background_columns {
                column.speed = base_speed * 0.6 * self.rng.gen_range(0.7..=1.3);
                column.max_length = self.rng.gen_range(max_length / 4..=max_length / 2);
            }
        }
    }

    /// Get the number of active columns
    pub fn active_columns(&self) -> usize {
        self.columns.iter().filter(|c| c.active).count()
    }

    /// Get total number of columns
    pub fn total_columns(&self) -> usize {
        self.columns.len()
    }

    /// Get render data without actually rendering (useful for FFI)
    pub fn get_render_data(&self) -> Vec<RenderChar> {
        let mut render_chars = Vec::new();

        // Add background layer first (if enabled)
        if self.config.enable_background_layer {
            for column in &self.background_columns {
                if !column.active {
                    continue;
                }

                let x_pixel = column.x as f32 * self.char_width;

                for (ch, y_pos, trail_pos) in column.get_trail_positions() {
                    if y_pos < 0.0 {
                        continue;
                    }

                    let y_pixel = y_pos * self.char_height;

                    if y_pixel > self.config.screen_height as f32 {
                        continue;
                    }

                    // Background characters are much dimmer
                    let rgba = self.config.color_scheme.get_color_with_alpha(trail_pos);
                    let mut color = Color::from_rgba_tuple(rgba);
                    color.a *= 0.3; // 30% opacity for subtle effect

                    render_chars.push(RenderChar {
                        character: ch,
                        x: x_pixel,
                        y: y_pixel,
                        color,
                        font_size: self.font_size * 0.9,
                    });
                }
            }
        }

        // Add foreground layer
        for column in &self.columns {
            if !column.active {
                continue;
            }

            let x_pixel = column.x as f32 * self.char_width;

            for (ch, y_pos, trail_pos) in column.get_trail_positions() {
                // Skip characters above screen
                if y_pos < 0.0 {
                    continue;
                }

                let y_pixel = y_pos * self.char_height;

                // Skip characters below screen
                if y_pixel > self.config.screen_height as f32 {
                    continue;
                }

                // Get color based on position in trail
                let rgba = self.config.color_scheme.get_color_with_alpha(trail_pos);
                let color = Color::from_rgba_tuple(rgba);

                render_chars.push(RenderChar {
                    character: ch,
                    x: x_pixel,
                    y: y_pixel,
                    color,
                    font_size: self.font_size,
                });
            }
        }

        render_chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CharacterSet, ColorScheme, RainSpeed};

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

        fn present(&mut self) {}

        fn width(&self) -> u32 {
            self.width
        }

        fn height(&self) -> u32 {
            self.height
        }
    }

    #[test]
    fn test_matrix_rain_creation() {
        let config = ScreenSaverConfig::default();
        let matrix = MatrixRain::new(config);

        assert!(matrix.total_columns() > 0);
        assert_eq!(matrix.config().character_set, CharacterSet::Japanese);
    }

    #[test]
    fn test_matrix_rain_update() {
        let config = ScreenSaverConfig::default();
        let mut matrix = MatrixRain::new(config);

        // Run a few updates
        for _ in 0..10 {
            matrix.update();
        }

        // Some columns should be active
        assert!(matrix.active_columns() > 0);
    }

    #[test]
    fn test_matrix_rain_render() {
        let config = ScreenSaverConfig::default();
        let mut matrix = MatrixRain::new(config);
        let mut renderer = MockRenderer::new(1920, 1080);

        // Update a few times to build up trails
        for _ in 0..50 {
            matrix.update();
        }

        // Render
        matrix.render(&mut renderer);

        // Should have rendered some characters
        assert!(!renderer.chars_drawn.is_empty());
    }

    #[test]
    fn test_config_update() {
        let config = ScreenSaverConfig::default();
        let mut matrix = MatrixRain::new(config);

        let original_columns = matrix.total_columns();

        // Update config with different dimensions
        let new_config = ScreenSaverConfig::new(
            CharacterSet::Korean,
            ColorScheme::Purple,
            RainSpeed::Fast,
            2560,
            1440,
        );

        matrix.set_config(new_config);

        // Should have different number of columns for different width
        assert_ne!(matrix.total_columns(), original_columns);
        assert_eq!(matrix.config().character_set, CharacterSet::Korean);
    }

    #[test]
    fn test_speed_affects_columns() {
        let config = ScreenSaverConfig::new(
            CharacterSet::Japanese,
            ColorScheme::MatrixGreen,
            RainSpeed::VerySlow,
            1920,
            1080,
        );
        let slow_matrix = MatrixRain::new(config);

        let config = ScreenSaverConfig::new(
            CharacterSet::Japanese,
            ColorScheme::MatrixGreen,
            RainSpeed::VeryFast,
            1920,
            1080,
        );
        let fast_matrix = MatrixRain::new(config);

        // Fast should have shorter max trail lengths
        assert!(
            slow_matrix.config().speed.max_trail_length()
                > fast_matrix.config().speed.max_trail_length()
        );
    }
}
