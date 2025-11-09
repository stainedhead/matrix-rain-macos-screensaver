//! Test character set rendering capabilities

use matrix_rain_core::*;

fn main() {
    println!("Testing Character Sets\n");
    println!("======================\n");

    let charsets = vec![
        ("Japanese (Katakana)", CharacterSet::Japanese),
        ("Hindi (Devanagari)", CharacterSet::Hindi),
        ("Tamil", CharacterSet::Tamil),
        ("Sinhala", CharacterSet::Sinhala),
        ("Korean (Hangul)", CharacterSet::Korean),
        ("Jawi (Arabic)", CharacterSet::Jawi),
        ("Mixed", CharacterSet::Mixed),
    ];

    for (name, charset) in charsets {
        let chars = charset.get_characters();
        println!("{}: {} characters", name, chars.len());

        // Show first 20 characters
        print!("  Sample: ");
        for ch in chars.iter().take(20) {
            print!("{} ", ch);
        }
        println!("\n");

        // Check for replacement characters (�) which indicate missing glyphs
        let has_replacement = chars.iter().any(|&ch| ch == '\u{FFFD}');
        if has_replacement {
            println!("  ⚠️  WARNING: Contains replacement characters (missing glyphs)");
        }
    }

    println!("\n======================");
    println!("Unicode Ranges Used:");
    println!("======================\n");
    println!("Japanese:");
    println!("  - Katakana: U+30A0 to U+30FF");
    println!("  - Half-width Katakana: U+FF65 to U+FF9F");
    println!("  - ASCII numbers and symbols");

    println!("\nHindi:");
    println!("  - Devanagari: U+0900 to U+097F");
    println!("  - Devanagari Extended: U+A8E0 to U+A8FF");

    println!("\nTamil:");
    println!("  - Tamil: U+0B80 to U+0BFF");

    println!("\nSinhala:");
    println!("  - Sinhala: U+0D80 to U+0DFF");
    println!("  - Sinhala Archaic Numbers: U+111E0 to U+111FF");

    println!("\nKorean:");
    println!("  - Hangul Syllables: U+AC00 to U+D7AF (sampled)");
    println!("  - Hangul Compatibility Jamo: U+3130 to U+318F");

    println!("\nJawi:");
    println!("  - Arabic: U+0600 to U+06FF");
    println!("  - Arabic Supplement: U+0750 to U+077F");
    println!("  - Arabic Extended-A: U+08A0 to U+08FF");

    println!("\n======================");
    println!("Terminal Requirements:");
    println!("======================\n");
    println!("For proper display, your terminal needs:");
    println!("  1. A font that supports Unicode");
    println!("  2. Support for the specific scripts you want to use");
    println!("\nRecommended fonts:");
    println!("  - macOS: SF Mono, Menlo (built-in)");
    println!("  - Cross-platform: Noto Sans Mono, Cascadia Code");
    println!("\nNote: The application bundles the CHARACTER DATA,");
    println!("but your terminal font must have the GLYPHS to display them.");
}
