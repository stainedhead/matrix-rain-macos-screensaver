//! Character sets for different languages/scripts

use serde::{Deserialize, Serialize};

/// Available character sets for the Matrix rain effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterSet {
    /// Japanese Katakana characters (default Matrix style)
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
}

impl CharacterSet {
    /// Get the Unicode characters for this character set
    pub fn get_characters(&self) -> Vec<char> {
        match self {
            CharacterSet::Japanese => {
                // Katakana characters (U+30A0 to U+30FF)
                // Including half-width katakana for variety
                let mut chars: Vec<char> = (0x30A0..=0x30FF)
                    .filter_map(std::char::from_u32)
                    .collect();

                // Add some half-width katakana
                chars.extend((0xFF65..=0xFF9F).filter_map(std::char::from_u32));

                // Add some numbers and symbols for authenticity
                chars.extend("0123456789.:=*+-<>¦|ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ".chars());
                chars
            }
            CharacterSet::Hindi => {
                // Devanagari script (U+0900 to U+097F)
                let mut chars: Vec<char> = (0x0900..=0x097F)
                    .filter_map(std::char::from_u32)
                    .collect();

                // Add Devanagari extended (U+A8E0 to U+A8FF)
                chars.extend((0xA8E0..=0xA8FF).filter_map(std::char::from_u32));
                chars
            }
            CharacterSet::Tamil => {
                // Tamil script (U+0B80 to U+0BFF)
                (0x0B80..=0x0BFF)
                    .filter_map(std::char::from_u32)
                    .collect()
            }
            CharacterSet::Sinhala => {
                // Sinhala script (U+0D80 to U+0DFF)
                let mut chars: Vec<char> = (0x0D80..=0x0DFF)
                    .filter_map(std::char::from_u32)
                    .collect();

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
                let mut chars: Vec<char> = (0x0600..=0x06FF)
                    .filter_map(std::char::from_u32)
                    .collect();

                // Add Arabic Supplement (U+0750 to U+077F)
                chars.extend((0x0750..=0x077F).filter_map(std::char::from_u32));

                // Add Arabic Extended-A (U+08A0 to U+08FF)
                chars.extend((0x08A0..=0x08FF).filter_map(std::char::from_u32));
                chars
            }
        }
    }

    /// Get a random character from this character set
    pub fn random_character(&self, rng: &mut impl rand::Rng) -> char {
        let chars = self.get_characters();
        chars[rng.gen_range(0..chars.len())]
    }
}

impl Default for CharacterSet {
    fn default() -> Self {
        CharacterSet::Japanese
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
        ];

        for set in sets {
            let chars = set.get_characters();
            assert!(!chars.is_empty(), "Character set {:?} should not be empty", set);
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
}
