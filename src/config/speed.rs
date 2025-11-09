//! Rain speed settings

use serde::{Deserialize, Serialize};

/// Available speed settings for the rain animation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RainSpeed {
    /// Very slow rain
    VerySlow,
    /// Slow rain
    Slow,
    /// Medium speed (default)
    #[default]
    Medium,
    /// Fast rain
    Fast,
    /// Very fast rain
    VeryFast,
}

impl RainSpeed {
    /// Get the update interval in milliseconds
    /// This determines how often the rain animation updates
    pub fn update_interval_ms(&self) -> u64 {
        match self {
            RainSpeed::VerySlow => 150,
            RainSpeed::Slow => 100,
            RainSpeed::Medium => 50,
            RainSpeed::Fast => 30,
            RainSpeed::VeryFast => 15,
        }
    }

    /// Get the base speed multiplier
    /// Higher values mean faster falling characters
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            RainSpeed::VerySlow => 0.5,
            RainSpeed::Slow => 0.75,
            RainSpeed::Medium => 1.0,
            RainSpeed::Fast => 1.5,
            RainSpeed::VeryFast => 2.0,
        }
    }

    /// Get the maximum trail length for this speed
    /// Faster speeds have shorter trails for visual balance
    pub fn max_trail_length(&self) -> usize {
        match self {
            RainSpeed::VerySlow => 30,
            RainSpeed::Slow => 25,
            RainSpeed::Medium => 20,
            RainSpeed::Fast => 15,
            RainSpeed::VeryFast => 12,
        }
    }

    /// Get all available speed settings
    pub fn all_speeds() -> Vec<RainSpeed> {
        vec![
            RainSpeed::VerySlow,
            RainSpeed::Slow,
            RainSpeed::Medium,
            RainSpeed::Fast,
            RainSpeed::VeryFast,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_intervals() {
        let very_slow = RainSpeed::VerySlow.update_interval_ms();
        let slow = RainSpeed::Slow.update_interval_ms();
        let medium = RainSpeed::Medium.update_interval_ms();
        let fast = RainSpeed::Fast.update_interval_ms();
        let very_fast = RainSpeed::VeryFast.update_interval_ms();

        // Slower speeds should have longer intervals
        assert!(very_slow > slow);
        assert!(slow > medium);
        assert!(medium > fast);
        assert!(fast > very_fast);
    }

    #[test]
    fn test_speed_multipliers() {
        let very_slow = RainSpeed::VerySlow.speed_multiplier();
        let slow = RainSpeed::Slow.speed_multiplier();
        let medium = RainSpeed::Medium.speed_multiplier();
        let fast = RainSpeed::Fast.speed_multiplier();
        let very_fast = RainSpeed::VeryFast.speed_multiplier();

        // Faster speeds should have higher multipliers
        assert!(very_slow < slow);
        assert!(slow < medium);
        assert!(medium < fast);
        assert!(fast < very_fast);
    }

    #[test]
    fn test_trail_lengths() {
        let very_slow = RainSpeed::VerySlow.max_trail_length();
        let slow = RainSpeed::Slow.max_trail_length();
        let medium = RainSpeed::Medium.max_trail_length();
        let fast = RainSpeed::Fast.max_trail_length();
        let very_fast = RainSpeed::VeryFast.max_trail_length();

        // Slower speeds should have longer trails
        assert!(very_slow > slow);
        assert!(slow > medium);
        assert!(medium > fast);
        assert!(fast > very_fast);
    }

    #[test]
    fn test_all_speeds() {
        let speeds = RainSpeed::all_speeds();
        assert_eq!(speeds.len(), 5);
    }

    #[test]
    fn test_default_speed() {
        assert_eq!(RainSpeed::default(), RainSpeed::Medium);
    }

    #[test]
    fn test_medium_baseline() {
        assert_eq!(RainSpeed::Medium.speed_multiplier(), 1.0);
    }
}
