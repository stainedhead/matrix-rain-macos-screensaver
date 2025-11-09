//! Matrix Rain CLI Application
//!
//! A terminal-based version of the Matrix rain screensaver

use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use matrix_rain_core::rendering::TerminalRenderer;
use matrix_rain_core::*;
use std::time::{Duration, Instant};

/// Matrix Rain - Digital rain effect in your terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Character set to use
    #[arg(short, long, default_value = "japanese")]
    charset: String,

    /// Color scheme to use
    #[arg(short = 'o', long, default_value = "matrix-green")]
    color: String,

    /// Speed setting
    #[arg(short, long, default_value = "medium")]
    speed: String,

    /// Run for specified duration (seconds), or indefinitely if not specified
    #[arg(short, long)]
    duration: Option<u64>,

    /// List available options
    #[arg(short, long)]
    list: bool,
}

fn parse_charset(s: &str) -> Result<CharacterSet> {
    match s.to_lowercase().as_str() {
        "japanese" | "jp" => Ok(CharacterSet::Japanese),
        "hindi" | "hi" => Ok(CharacterSet::Hindi),
        "tamil" | "ta" => Ok(CharacterSet::Tamil),
        "sinhala" | "si" => Ok(CharacterSet::Sinhala),
        "korean" | "ko" => Ok(CharacterSet::Korean),
        "jawi" | "jw" => Ok(CharacterSet::Jawi),
        "mixed" | "mix" => Ok(CharacterSet::Mixed),
        _ => Err(anyhow::anyhow!("Unknown character set: {}", s)),
    }
}

fn parse_color(s: &str) -> Result<ColorScheme> {
    match s.to_lowercase().as_str() {
        "matrix-green" | "green" => Ok(ColorScheme::MatrixGreen),
        "dark-blue" | "blue" => Ok(ColorScheme::DarkBlue),
        "purple" => Ok(ColorScheme::Purple),
        "orange" => Ok(ColorScheme::Orange),
        "red" => Ok(ColorScheme::Red),
        "cyan" => Ok(ColorScheme::Cyan),
        "yellow" => Ok(ColorScheme::Yellow),
        "pink" => Ok(ColorScheme::Pink),
        "white" => Ok(ColorScheme::White),
        "lime-green" | "lime" => Ok(ColorScheme::LimeGreen),
        "teal" => Ok(ColorScheme::Teal),
        _ => Err(anyhow::anyhow!("Unknown color scheme: {}", s)),
    }
}

fn parse_speed(s: &str) -> Result<RainSpeed> {
    match s.to_lowercase().as_str() {
        "very-slow" | "veryslow" | "vs" => Ok(RainSpeed::VerySlow),
        "slow" | "s" => Ok(RainSpeed::Slow),
        "medium" | "med" | "m" => Ok(RainSpeed::Medium),
        "fast" | "f" => Ok(RainSpeed::Fast),
        "very-fast" | "veryfast" | "vf" => Ok(RainSpeed::VeryFast),
        _ => Err(anyhow::anyhow!("Unknown speed: {}", s)),
    }
}

fn print_available_options() {
    println!("Matrix Rain - Available Options\n");

    println!("Character Sets:");
    println!("  japanese (jp)  - Japanese Katakana (default Matrix style)");
    println!("  hindi (hi)     - Hindi Devanagari script");
    println!("  tamil (ta)     - Tamil script");
    println!("  sinhala (si)   - Sinhala script");
    println!("  korean (ko)    - Korean Hangul");
    println!("  jawi (jw)      - Malaysian Jawi (Arabic-based)");
    println!("  mixed (mix)    - Mixed scripts (50% Japanese, 10% each other)");

    println!("\nColor Schemes:");
    println!("  matrix-green   - Classic Matrix green (default)");
    println!("  dark-blue      - Deep ocean blue");
    println!("  purple         - Royal purple");
    println!("  orange         - Warm amber");
    println!("  red            - Alert red");
    println!("  cyan           - Electric cyan");
    println!("  yellow         - Pure yellow");
    println!("  pink           - Hot pink");
    println!("  white          - Pure white");
    println!("  lime-green     - Bright lime");
    println!("  teal           - Ocean teal");

    println!("\nSpeed Settings:");
    println!("  very-slow (vs) - Contemplative pace");
    println!("  slow (s)       - Relaxed viewing");
    println!("  medium (m)     - Balanced (default)");
    println!("  fast (f)       - Energetic movement");
    println!("  very-fast (vf) - High intensity");

    println!("\nExamples:");
    println!("  matrix-rain");
    println!("  matrix-rain --charset korean --color purple --speed fast");
    println!("  matrix-rain -c hindi -o cyan -s slow --duration 30");
    println!("\nPress 'q' or Ctrl+C to exit when running");
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle list option
    if args.list {
        print_available_options();
        return Ok(());
    }

    // Parse configuration
    let charset = parse_charset(&args.charset)?;
    let color = parse_color(&args.color)?;
    let speed = parse_speed(&args.speed)?;

    // Create terminal renderer
    let mut renderer = TerminalRenderer::new()?;
    renderer.init()?;

    // Get terminal size
    let width = renderer.width();
    let height = renderer.height();

    // Create configuration
    let config = ScreenSaverConfig::new(charset, color, speed, width, height);

    // Create rain engine
    let mut matrix = MatrixRain::new(config);

    // Calculate update interval
    let update_interval = Duration::from_millis(speed.update_interval_ms());

    // Track start time if duration is specified
    let start_time = Instant::now();
    let duration = args.duration.map(Duration::from_secs);

    // Main render loop
    let mut last_update = Instant::now();
    let result = loop {
        // Check for exit conditions
        if let Some(d) = duration {
            if start_time.elapsed() >= d {
                break Ok(());
            }
        }

        // Check for events (key press or resize)
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => break Ok(()),
                    _ => {}
                },
                Event::Resize(new_width, new_height) => {
                    // Handle terminal resize
                    let new_config = ScreenSaverConfig::new(
                        charset,
                        color,
                        speed,
                        new_width as u32 * 8,   // Approximate pixel width
                        new_height as u32 * 16, // Approximate pixel height
                    );
                    matrix.set_config(new_config);
                }
                _ => {}
            }
        }

        // Update and render at the appropriate interval
        if last_update.elapsed() >= update_interval {
            matrix.update();
            matrix.render(&mut renderer);
            last_update = Instant::now();
        }
    };

    // Cleanup terminal
    renderer.cleanup()?;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charset_parsing() {
        assert!(matches!(
            parse_charset("japanese"),
            Ok(CharacterSet::Japanese)
        ));
        assert!(matches!(parse_charset("jp"), Ok(CharacterSet::Japanese)));
        assert!(matches!(parse_charset("hindi"), Ok(CharacterSet::Hindi)));
        assert!(matches!(parse_charset("mixed"), Ok(CharacterSet::Mixed)));
        assert!(matches!(parse_charset("mix"), Ok(CharacterSet::Mixed)));
        assert!(parse_charset("invalid").is_err());
    }

    #[test]
    fn test_color_parsing() {
        assert!(matches!(
            parse_color("matrix-green"),
            Ok(ColorScheme::MatrixGreen)
        ));
        assert!(matches!(parse_color("green"), Ok(ColorScheme::MatrixGreen)));
        assert!(matches!(parse_color("purple"), Ok(ColorScheme::Purple)));
        assert!(parse_color("invalid").is_err());
    }

    #[test]
    fn test_speed_parsing() {
        assert!(matches!(parse_speed("medium"), Ok(RainSpeed::Medium)));
        assert!(matches!(parse_speed("m"), Ok(RainSpeed::Medium)));
        assert!(matches!(parse_speed("fast"), Ok(RainSpeed::Fast)));
        assert!(parse_speed("invalid").is_err());
    }
}
