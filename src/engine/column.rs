//! Individual rain column implementation

use crate::config::CharacterSet;
use rand::Rng;

/// A single column of falling characters
#[derive(Debug, Clone)]
pub struct RainColumn {
    /// X position of the column (in character units, not pixels)
    pub x: usize,
    /// Current Y position of the head of the rain (in character units)
    pub y: f32,
    /// The trail of characters in this column
    pub characters: Vec<char>,
    /// Speed multiplier for this specific column
    pub speed: f32,
    /// Maximum length of the trail
    pub max_length: usize,
    /// Whether this column is currently active
    pub active: bool,
}

impl RainColumn {
    /// Create a new rain column
    pub fn new(x: usize, max_length: usize, base_speed: f32, rng: &mut impl Rng) -> Self {
        // Randomize starting position above screen
        let y = -(rng.gen_range(5..=20) as f32);

        // Randomize speed slightly for visual variety
        let speed = base_speed * rng.gen_range(0.7..=1.3);

        Self {
            x,
            y,
            characters: Vec::with_capacity(max_length),
            speed,
            max_length: rng.gen_range(max_length / 2..=max_length),
            active: true,
        }
    }

    /// Update the column's position
    pub fn update(&mut self, char_set: &CharacterSet, rng: &mut impl Rng) {
        if !self.active {
            return;
        }

        // Move the column down
        self.y += self.speed;

        // Add new characters to the trail
        if self.characters.len() < self.max_length && rng.gen_bool(0.8) {
            self.characters.push(char_set.random_character(rng));
        }

        // Occasionally change a character in the trail for the "glitch" effect
        if !self.characters.is_empty() && rng.gen_bool(0.05) {
            let idx = rng.gen_range(0..self.characters.len());
            self.characters[idx] = char_set.random_character(rng);
        }
    }

    /// Check if the column has moved off screen
    pub fn is_off_screen(&self, screen_height: f32, char_height: f32) -> bool {
        let max_chars = (screen_height / char_height) as usize;
        self.y > (max_chars + self.characters.len()) as f32
    }

    /// Reset the column to start over
    pub fn reset(&mut self, rng: &mut impl Rng) {
        self.y = -(rng.gen_range(5..=20) as f32);
        self.characters.clear();
        self.active = true;
    }

    /// Get the position of each character in the trail
    /// Returns Vec<(character, y_position, position_in_trail)>
    /// position_in_trail is 0.0 at the head, 1.0 at the tail
    pub fn get_trail_positions(&self) -> Vec<(char, f32, f32)> {
        self.characters
            .iter()
            .enumerate()
            .map(|(i, &ch)| {
                let y_pos = self.y - i as f32;
                let trail_pos = if self.characters.len() <= 1 {
                    0.0
                } else {
                    i as f32 / (self.characters.len() - 1) as f32
                };
                (ch, y_pos, trail_pos)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_column_creation() {
        let mut rng = thread_rng();
        let column = RainColumn::new(5, 20, 1.0, &mut rng);

        assert_eq!(column.x, 5);
        assert!(column.y < 0.0); // Should start above screen
        assert!(column.active);
        assert!(column.max_length <= 20);
    }

    #[test]
    fn test_column_update() {
        let mut rng = thread_rng();
        let char_set = CharacterSet::Japanese;
        let mut column = RainColumn::new(5, 20, 1.0, &mut rng);

        let initial_y = column.y;
        column.update(&char_set, &mut rng);

        // Column should have moved down
        assert!(column.y > initial_y);
    }

    #[test]
    fn test_column_builds_trail() {
        let mut rng = thread_rng();
        let char_set = CharacterSet::Japanese;
        let mut column = RainColumn::new(5, 20, 1.0, &mut rng);

        // Update multiple times to build trail
        for _ in 0..50 {
            column.update(&char_set, &mut rng);
        }

        // Should have some characters in the trail
        assert!(!column.characters.is_empty());
        assert!(column.characters.len() <= column.max_length);
    }

    #[test]
    fn test_off_screen_detection() {
        let mut rng = thread_rng();
        let mut column = RainColumn::new(5, 20, 1.0, &mut rng);

        // Column starts above screen, so not off screen yet
        assert!(!column.is_off_screen(1000.0, 20.0));

        // Move column way down
        column.y = 100.0;
        assert!(column.is_off_screen(1000.0, 20.0));
    }

    #[test]
    fn test_column_reset() {
        let mut rng = thread_rng();
        let char_set = CharacterSet::Japanese;
        let mut column = RainColumn::new(5, 20, 1.0, &mut rng);

        // Build up the column
        for _ in 0..50 {
            column.update(&char_set, &mut rng);
        }

        let had_characters = !column.characters.is_empty();

        // Reset
        column.reset(&mut rng);

        assert!(had_characters); // Confirm we had something to reset
        assert!(column.characters.is_empty());
        assert!(column.y < 0.0);
        assert!(column.active);
    }

    #[test]
    fn test_trail_positions() {
        let mut rng = thread_rng();
        let char_set = CharacterSet::Japanese;
        let mut column = RainColumn::new(5, 20, 1.0, &mut rng);

        // Build up some trail
        for _ in 0..10 {
            column.update(&char_set, &mut rng);
        }

        let positions = column.get_trail_positions();

        // Should have positions for all characters
        assert_eq!(positions.len(), column.characters.len());

        if positions.len() > 1 {
            // First character (head) should have trail_pos near 0.0
            assert!(positions[0].2 < 0.1);

            // Last character (tail) should have trail_pos near 1.0
            let last_idx = positions.len() - 1;
            assert!(positions[last_idx].2 > 0.9);
        }
    }
}
