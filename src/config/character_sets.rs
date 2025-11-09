//! Character sets for different languages/scripts

use serde::{Deserialize, Serialize};

/// Available character sets for the Matrix rain effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CharacterSet {
    /// Japanese Katakana characters (default Matrix style)
    #[default]
    Japanese,
    /// Hindi Devanagari script
    Hindi,
    /// Tamil script
    Tamil,
    /// Sinhala script
    Sinhala,
    /// Korean Hangul characters
    Korean,
    /// Malaysian Jawi script (Arabic-based)
    Jawi,
    /// Mixed character set (50% Japanese, 50% from other sets)
    Mixed,
}

impl CharacterSet {
    /// Get the Unicode characters for this character set
    pub fn get_characters(&self) -> Vec<char> {
        match self {
            CharacterSet::Japanese => {
                // Katakana characters (U+30A0 to U+30FF)
                // Including half-width katakana for variety
                let mut chars: Vec<char> =
                    (0x30A0..=0x30FF).filter_map(std::char::from_u32).collect();

                // Add some half-width katakana
                chars.extend((0xFF65..=0xFF9F).filter_map(std::char::from_u32));

                // Add some numbers and symbols for authenticity
                chars.extend("0123456789.:=*+-<>¦|ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ".chars());
                chars
            }
            CharacterSet::Hindi => {
                // Devanagari script (U+0900 to U+097F)
                let mut chars: Vec<char> =
                    (0x0900..=0x097F).filter_map(std::char::from_u32).collect();

                // Add Devanagari extended (U+A8E0 to U+A8FF)
                chars.extend((0xA8E0..=0xA8FF).filter_map(std::char::from_u32));
                chars
            }
            CharacterSet::Tamil => {
                // Tamil script (U+0B80 to U+0BFF)
                (0x0B80..=0x0BFF).filter_map(std::char::from_u32).collect()
            }
            CharacterSet::Sinhala => {
                // Sinhala script (U+0D80 to U+0DFF)
                let mut chars: Vec<char> =
                    (0x0D80..=0x0DFF).filter_map(std::char::from_u32).collect();

                // Add Sinhala Archaic Numbers (U+111E0 to U+111FF)
                chars.extend((0x111E0..=0x111FF).filter_map(std::char::from_u32));
                chars
            }
            CharacterSet::Korean => {
                // Hangul Syllables (U+AC00 to U+D7AF)
                // Using a subset for performance - every 10th character
                let mut chars: Vec<char> = (0xAC00..=0xD7AF)
                    .step_by(10)
                    .filter_map(std::char::from_u32)
                    .collect();

                // Add Hangul Compatibility Jamo (U+3130 to U+318F)
                chars.extend((0x3130..=0x318F).filter_map(std::char::from_u32));
                chars
            }
            CharacterSet::Jawi => {
                // Arabic script (U+0600 to U+06FF)
                let mut chars: Vec<char> =
                    (0x0600..=0x06FF).filter_map(std::char::from_u32).collect();

                // Add Arabic Supplement (U+0750 to U+077F)
                chars.extend((0x0750..=0x077F).filter_map(std::char::from_u32));

                // Add Arabic Extended-A (U+08A0 to U+08FF)
                chars.extend((0x08A0..=0x08FF).filter_map(std::char::from_u32));
                chars
            }
            CharacterSet::Mixed => {
                // Mixed set: 50% Japanese, 10% each from other 5 sets
                let mut mixed_chars = Vec::new();

                // Get all character sets
                let japanese_chars = CharacterSet::Japanese.get_characters();
                let hindi_chars = CharacterSet::Hindi.get_characters();
                let tamil_chars = CharacterSet::Tamil.get_characters();
                let sinhala_chars = CharacterSet::Sinhala.get_characters();
                let korean_chars = CharacterSet::Korean.get_characters();
                let jawi_chars = CharacterSet::Jawi.get_characters();

                // Calculate target counts (aim for ~500 total characters)
                let total_target = 500;
                let japanese_count = (total_target as f32 * 0.5) as usize; // 50%
                let other_count = (total_target as f32 * 0.1) as usize; // 10% each

                // Take Japanese characters (50%)
                let japanese_sample: Vec<char> = japanese_chars
                    .iter()
                    .step_by((japanese_chars.len() / japanese_count).max(1))
                    .copied()
                    .take(japanese_count)
                    .collect();
                mixed_chars.extend(japanese_sample);

                // Take from each other set (10% each)
                for chars in [
                    &hindi_chars,
                    &tamil_chars,
                    &sinhala_chars,
                    &korean_chars,
                    &jawi_chars,
                ] {
                    let sample: Vec<char> = chars
                        .iter()
                        .step_by((chars.len() / other_count).max(1))
                        .copied()
                        .take(other_count)
                        .collect();
                    mixed_chars.extend(sample);
                }

                mixed_chars
            }
        }
    }

    /// Get a random character from this character set
    pub fn random_character(&self, rng: &mut impl rand::Rng) -> char {
        let chars = self.get_characters();
        chars[rng.gen_range(0..chars.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_japanese_characters() {
        let chars = CharacterSet::Japanese.get_characters();
        assert!(!chars.is_empty());
        assert!(chars.len() > 100); // Should have plenty of characters
    }

    #[test]
    fn test_all_character_sets() {
        let sets = vec![
            CharacterSet::Japanese,
            CharacterSet::Hindi,
            CharacterSet::Tamil,
            CharacterSet::Sinhala,
            CharacterSet::Korean,
            CharacterSet::Jawi,
            CharacterSet::Mixed,
        ];

        for set in sets {
            let chars = set.get_characters();
            assert!(
                !chars.is_empty(),
                "Character set {:?} should not be empty",
                set
            );
        }
    }

    #[test]
    fn test_random_character() {
        let mut rng = thread_rng();
        let char_set = CharacterSet::Japanese;

        for _ in 0..10 {
            let ch = char_set.random_character(&mut rng);
            assert!(char_set.get_characters().contains(&ch));
        }
    }

    #[test]
    fn test_default_character_set() {
        assert_eq!(CharacterSet::default(), CharacterSet::Japanese);
    }

    #[test]
    fn test_mixed_character_set() {
        let chars = CharacterSet::Mixed.get_characters();

        // Should have around 500 characters
        assert!(
            chars.len() >= 400 && chars.len() <= 600,
            "Mixed set should have ~500 characters, got {}",
            chars.len()
        );

        // Should not be empty
        assert!(!chars.is_empty());

        // Get individual sets for comparison
        let japanese_chars = CharacterSet::Japanese.get_characters();
        let hindi_chars = CharacterSet::Hindi.get_characters();
        let tamil_chars = CharacterSet::Tamil.get_characters();
        let sinhala_chars = CharacterSet::Sinhala.get_characters();
        let korean_chars = CharacterSet::Korean.get_characters();
        let jawi_chars = CharacterSet::Jawi.get_characters();

        // Count how many characters from each set appear in mixed
        let japanese_count = chars.iter().filter(|c| japanese_chars.contains(c)).count();
        let hindi_count = chars.iter().filter(|c| hindi_chars.contains(c)).count();
        let tamil_count = chars.iter().filter(|c| tamil_chars.contains(c)).count();
        let sinhala_count = chars.iter().filter(|c| sinhala_chars.contains(c)).count();
        let korean_count = chars.iter().filter(|c| korean_chars.contains(c)).count();
        let jawi_count = chars.iter().filter(|c| jawi_chars.contains(c)).count();

        // Japanese should be roughly 50% (within tolerance)
        let total = chars.len() as f32;
        let japanese_ratio = japanese_count as f32 / total;
        assert!(
            (0.45..=0.55).contains(&japanese_ratio),
            "Japanese should be ~50%, got {:.1}%",
            japanese_ratio * 100.0
        );

        // Each other set should be roughly 10% (within tolerance)
        for (name, count) in [
            ("Hindi", hindi_count),
            ("Tamil", tamil_count),
            ("Sinhala", sinhala_count),
            ("Korean", korean_count),
            ("Jawi", jawi_count),
        ] {
            let ratio = count as f32 / total;
            assert!(
                (0.05..=0.15).contains(&ratio),
                "{} should be ~10%, got {:.1}%",
                name,
                ratio * 100.0
            );
        }
    }

    #[test]
    fn test_mixed_random_character() {
        let mut rng = thread_rng();
        let char_set = CharacterSet::Mixed;

        // Generate 100 random characters and ensure they're all valid
        for _ in 0..100 {
            let ch = char_set.random_character(&mut rng);
            assert!(char_set.get_characters().contains(&ch));
        }
    }
}
