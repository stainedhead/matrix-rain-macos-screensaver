//! Diagnostic tool to check terminal capabilities

use std::env;

fn main() {
    println!("=== Matrix Rain Terminal Diagnostics ===\n");

    // Check environment
    println!("1. ENVIRONMENT VARIABLES");
    println!("   TERM: {}", env::var("TERM").unwrap_or_else(|_| "NOT SET".to_string()));
    println!("   LANG: {}", env::var("LANG").unwrap_or_else(|_| "NOT SET".to_string()));
    println!("   COLORTERM: {}", env::var("COLORTERM").unwrap_or_else(|_| "NOT SET".to_string()));

    let colorterm = env::var("COLORTERM").unwrap_or_default();
    if colorterm == "truecolor" || colorterm == "24bit" {
        println!("   ✅ Terminal supports 24-bit color (True Color)\n");
    } else {
        println!("   ⚠️  COLORTERM not set to 'truecolor' - you may have color issues\n");
    }

    // Check character data
    use matrix_rain_core::*;

    println!("2. CHARACTER SET DATA (built into binary)");
    let charsets = vec![
        ("Japanese", CharacterSet::Japanese),
        ("Hindi", CharacterSet::Hindi),
        ("Tamil", CharacterSet::Tamil),
        ("Sinhala", CharacterSet::Sinhala),
        ("Korean", CharacterSet::Korean),
        ("Jawi", CharacterSet::Jawi),
        ("Mixed", CharacterSet::Mixed),
    ];

    for (name, charset) in charsets {
        let chars = charset.get_characters();
        println!("   ✅ {}: {} characters loaded", name, chars.len());
    }

    println!("\n3. SAMPLE CHARACTERS (visual test)");
    println!("   If you see boxes (□) or question marks (?), your font is missing glyphs.\n");

    println!("   Japanese:  ハミヒーウシナモニサワツオリアホテマケメエカキムユラセネスタヌヘ");
    println!("   Hindi:     अआइईउऊऋऌएऐओऔकखगघङचछजझञटठडढण");
    println!("   Tamil:     அஆஇஈஉஊஎஏஐஒஓஔகஙசஞடணதநபமயரலவழளறன");
    println!("   Sinhala:   අආඇඈඉඊඋඌඍඎඏඐඑඒඓඔඕඖ");
    println!("   Korean:    가나다라마바사아자차카타파하");
    println!("   Jawi:      ابتثجحخدذرزسشصضطظعغفقكلمنهوي");

    println!("\n4. CONFIGURATION TEST");
    let config = ScreenSaverConfig::new(
        CharacterSet::Japanese,
        ColorScheme::MatrixGreen,
        RainSpeed::Medium,
        1920,
        1080,
    );

    println!("   Character Set: {:?}", config.character_set);
    println!("   Color Scheme: {:?}", config.color_scheme);
    println!("   Speed: {:?}", config.speed);
    println!("   Screen Size: {}x{}", config.screen_width, config.screen_height);
    println!("   Background Layer: {}", if config.enable_background_layer { "✅ ENABLED" } else { "❌ DISABLED" });

    println!("\n5. COLOR SCHEME TEST");
    println!("   Matrix Green colors:");

    let (r, g, b, a) = config.color_scheme.get_color_with_alpha(0.0);
    println!("     Leader (0.0):   RGB({:3}, {:3}, {:3}) Alpha: {:.2} (should be white)", r, g, b, a);

    let (r, g, b, a) = config.color_scheme.get_color_with_alpha(0.1);
    println!("     Bright (0.1):   RGB({:3}, {:3}, {:3}) Alpha: {:.2}", r, g, b, a);

    let (r, g, b, a) = config.color_scheme.get_color_with_alpha(0.3);
    println!("     Medium (0.3):   RGB({:3}, {:3}, {:3}) Alpha: {:.2}", r, g, b, a);

    let (r, g, b, a) = config.color_scheme.get_color_with_alpha(0.7);
    println!("     Dark (0.7):     RGB({:3}, {:3}, {:3}) Alpha: {:.2}", r, g, b, a);

    let (r, g, b, a) = config.color_scheme.get_color_with_alpha(1.0);
    println!("     Faded (1.0):    RGB({:3}, {:3}, {:3}) Alpha: {:.2}", r, g, b, a);

    println!("\n=== DIAGNOSTICS COMPLETE ===\n");

    println!("RECOMMENDATIONS:");
    println!("• If characters show as boxes: Install a font with better Unicode support");
    println!("  (Noto Sans Mono, Cascadia Code, or JetBrains Mono)");
    println!("• If COLORTERM is not 'truecolor': Update your terminal or try iTerm2");
    println!("• The background layer is subtle by design (30% opacity)");
    println!("• For best experience, use the macOS screensaver version (GPU rendered)");

    println!("\nSee FONT_REQUIREMENTS.md for detailed information.");
}
