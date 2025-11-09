# Matrix Rain macOS Screensaver

A customizable Matrix-style digital rain screensaver for macOS, written in Rust. Experience the iconic cascading characters from The Matrix with support for multiple languages and color schemes.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)

## Features

### ✨ Authentic Matrix Visual Effects
- **Bright White Leading Characters**: Classic Matrix effect with white "leaders"
- **Smooth Color Fade**: 4-stage progression (white → bright → medium → dark)
- **Dual-Layer Depth**: Subtle background rain for atmospheric depth
  - Background layer at 30% opacity
  - Slower speed (60% of foreground)
  - Sparser coverage (every 3rd column)
  - Creates natural depth perception
- **Dynamic Effects**: Character glitching, variable speeds, smooth trails
- **Screen Resize Support**: Automatic adaptation to window/terminal size changes

### 🌐 Multiple Language Support
Choose from six different character sets:
- **Japanese** (Katakana) - The classic Matrix look
- **Hindi** (Devanagari script)
- **Tamil** script
- **Sinhala** script
- **Korean** (Hangul)
- **Jawi** (Arabic-based Malaysian script)

### 🎨 Customizable Color Schemes
Select from 11 beautiful color options:
- Matrix Green (classic)
- Dark Blue
- Purple
- Orange
- Red
- Cyan
- Yellow
- Pink
- White
- Lime Green
- Teal

### ⚡ Adjustable Speed Settings
Five speed levels to match your preference:
- Very Slow
- Slow
- Medium (default)
- Fast
- Very Fast

## Screenshots

![Matrix Rain CLI Demo](docs/images/demo-screenshot.png)

*Matrix Rain running in terminal with Korean characters and purple color scheme*

## Installation

### Quick Start (CLI)

Download the pre-built binary for your system from the [releases page](https://github.com/iggybdda/matrix-rain-macos-screensaver/releases):

```bash
# macOS (Apple Silicon)
curl -L https://github.com/iggybdda/matrix-rain-macos-screensaver/releases/latest/download/matrix-rain-macos-aarch64 -o matrix-rain
chmod +x matrix-rain
./matrix-rain

# macOS (Intel)
curl -L https://github.com/iggybdda/matrix-rain-macos-screensaver/releases/latest/download/matrix-rain-macos-x86_64 -o matrix-rain
chmod +x matrix-rain
./matrix-rain

# Linux
curl -L https://github.com/iggybdda/matrix-rain-macos-screensaver/releases/latest/download/matrix-rain-linux-x86_64 -o matrix-rain
chmod +x matrix-rain
./matrix-rain
```

### Building from Source

#### Prerequisites
- macOS 10.15+ or Linux
- Rust 1.70 or later

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/iggybdda/matrix-rain-macos-screensaver.git
cd matrix-rain-macos-screensaver

# Build the library
cargo build --release

# Build the CLI application
cargo build --release --features cli --bin matrix-rain

# Run tests
cargo test
cargo test --features cli

# Install CLI globally (optional)
cargo install --path . --features cli --bin matrix-rain
```

## Usage

### CLI Application

Run the Matrix rain effect in your terminal:

```bash
# Basic usage with defaults (Japanese characters, Matrix green, medium speed)
matrix-rain

# Custom configuration
matrix-rain --charset korean --color purple --speed fast

# Short aliases
matrix-rain -c hindi -o cyan -s slow

# Run for specific duration (30 seconds)
matrix-rain --duration 30

# List all available options
matrix-rain --list
```

**Available Options:**

| Option | Short | Values | Description |
|--------|-------|--------|-------------|
| `--charset` | `-c` | japanese, hindi, tamil, sinhala, korean, jawi | Character set to use |
| `--color` | `-o` | matrix-green, blue, purple, orange, red, cyan, yellow, pink, white, lime, teal | Color scheme |
| `--speed` | `-s` | very-slow, slow, medium, fast, very-fast | Animation speed |
| `--duration` | `-d` | seconds | Run for specified duration (omit for indefinite) |
| `--list` | `-l` | - | Show all available options |

**Controls:**
- Press `q`, `Q`, or `Esc` to exit
- Press `Ctrl+C` to exit

### As a Library

The core rain logic is available as a Rust library that can be integrated into your own projects:

```rust
use matrix_rain_core::{MatrixRain, ScreenSaverConfig, CharacterSet, ColorScheme, RainSpeed};

// Create configuration
let config = ScreenSaverConfig::new(
    CharacterSet::Japanese,
    ColorScheme::MatrixGreen,
    RainSpeed::Medium,
    1920,
    1080,
);

// Create the rain engine
let mut matrix = MatrixRain::new(config);

// In your render loop:
matrix.update();
matrix.render(&mut your_renderer);
```

### As a macOS Screensaver

#### Building the Screensaver

The macOS screensaver implementation is complete and ready to build:

```bash
# Build the Rust FFI library
cargo build --release --features ffi

# Build the screensaver bundle
cd macos-screensaver
./build-screensaver.sh
```

This creates `macos-screensaver/build/MatrixRainSaver.saver`.

#### Installation

Once built, install the screensaver:

**User Installation**
```bash
# Copy the built screensaver to your Screen Savers folder
cp -r macos-screensaver/build/MatrixRainSaver.saver ~/Library/Screen\ Savers/

# Or double-click the .saver file to install via Finder
open MatrixRain.saver
```

**Option 2: System-Wide Installation** (requires admin)
```bash
# Install for all users
sudo cp -r MatrixRain.saver /Library/Screen\ Savers/
```

**Configuring in System Preferences:**
1. Open System Preferences (or System Settings on macOS 13+)
2. Navigate to:
   - **macOS 13+**: Desktop & Screen Saver → Screen Saver
   - **macOS 12 and earlier**: Desktop & Screen Saver
3. Select "Matrix Rain" from the screensaver list
4. Click "Screen Saver Options" to customize:
   - Character set (Japanese, Hindi, Tamil, Sinhala, Korean, Jawi)
   - Color scheme (11 options)
   - Animation speed (5 levels)

**Testing Your Screensaver:**
```bash
# Preview in System Preferences
open /System/Library/PreferencePanes/DesktopScreenEffectsPref.prefPane

# Test directly (macOS 12 and earlier)
/System/Library/CoreServices/ScreenSaverEngine.app/Contents/MacOS/ScreenSaverEngine -module MatrixRain

# Test directly (macOS 13+)
/System/Library/Frameworks/ScreenSaver.framework/Versions/A/Resources/ScreenSaverEngine.app/Contents/MacOS/ScreenSaverEngine
```

#### Custom Development

To create your own macOS screensaver using this library:

1. **Create a new Xcode project** → macOS → Screen Saver
2. **Add the Rust library** as a dependency
3. **Implement the ScreenSaverView** subclass:

```swift
import ScreenSaver
import MatrixRainCore  // Via FFI bridge

class MatrixRainView: ScreenSaverView {
    var matrixEngine: OpaquePointer?

    override init?(frame: NSRect, isPreview: Bool) {
        super.init(frame: frame, isPreview: isPreview)

        // Initialize Rust engine via FFI
        matrixEngine = matrix_rain_new(
            UInt32(frame.width),
            UInt32(frame.height)
        )
    }

    override func animateOneFrame() {
        // Update and render via Rust engine
        matrix_rain_update(matrixEngine)
        matrix_rain_render(matrixEngine, /* Core Graphics context */)
    }

    deinit {
        matrix_rain_destroy(matrixEngine)
    }
}
```

4. **Build the screensaver bundle**
5. **Install** to `~/Library/Screen Savers/`

See [documents/technical-details.md](documents/technical-details.md) for more information on creating custom integrations.

## Project Structure

```
matrix-rain-macos-screensaver/
├── src/
│   ├── config/           # Configuration types
│   │   ├── character_sets.rs
│   │   ├── colors.rs
│   │   └── speed.rs
│   ├── engine/           # Core rain logic
│   │   ├── column.rs
│   │   └── matrix_rain.rs
│   ├── rendering/        # Rendering abstractions
│   │   ├── color.rs
│   │   └── renderer.rs
│   └── lib.rs
├── documents/            # Product documentation
│   ├── product-summary.md
│   ├── product-details.md
│   └── technical-details.md
├── AGENTS.md            # Development guidelines
└── README.md
```

## Development

### Testing

We follow Test-Driven Development (TDD) practices. All changes should include appropriate tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_matrix_rain_creation
```

### Contributing

We welcome contributions! Please see our [AGENTS.md](AGENTS.md) file for development guidelines and best practices.

Before submitting a pull request:

1. Ensure all tests pass (`cargo test`)
2. Format your code (`cargo fmt`)
3. Run clippy for linting (`cargo clippy`)
4. Update documentation as needed
5. Add tests for new functionality

### Code Quality

- **Test Coverage**: All modules include comprehensive unit tests
- **Documentation**: Public APIs are fully documented
- **Type Safety**: Leverages Rust's type system for correctness
- **No Unsafe Code**: Pure safe Rust implementation

### Debugging

For debugging and CLI testing, you can create example executables:

```bash
# Create examples directory
mkdir -p examples

# Create examples/demo.rs with your test code
# See AGENTS.md for detailed debugging instructions

# Run the example
cargo run --example demo
```

Or run tests with debug output:

```bash
# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_matrix_rain_creation -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test
```

See [AGENTS.md](AGENTS.md) for comprehensive debugging and CLI development instructions.

## Architecture

The project is organized into three main modules:

- **Config**: Manages character sets, colors, speeds, and overall configuration
- **Engine**: Contains the core rain logic including column management and animation
- **Rendering**: Provides abstractions for drawing characters to screen

For detailed technical information, see [documents/technical-details.md](documents/technical-details.md).

## Performance

- Efficient rendering with minimal CPU usage
- Configurable update intervals based on speed settings
- Smart column activation to balance visual density
- Optimized character set handling

## Roadmap

### Completed ✅
- [x] Core rain engine with authentic Matrix visuals
- [x] **Bright white leading character effect** (classic Matrix look)
- [x] **Smooth color fade progression** (white → bright → medium → dark)
- [x] **Dual-layer background rain** for depth effect (30% opacity, slower speed)
- [x] **Full screen resize support** (CLI, engine, FFI, screensaver)
- [x] Multiple character set support (6 languages)
- [x] Color customization (11 color schemes)
- [x] Speed settings (5 levels)
- [x] Terminal CLI application with resize support
- [x] **FFI render data export** for external renderers
- [x] **macOS ScreenSaver bundle** (Swift + Rust)
- [x] **User preferences UI** (SwiftUI configuration panel)
- [x] **Automated build system** for screensaver
- [x] CI/CD with GitHub Actions
- [x] Cross-platform binary builds
- [x] Comprehensive test suite (49 tests)
- [x] Full documentation

### Ready for Testing 🧪
- [ ] Test screensaver on physical macOS device
- [ ] Capture screenshot/video for README
- [ ] Verify configuration panel functionality
- [ ] Test multi-monitor behavior

### Future Enhancements 🔮
- [ ] Installer package (.pkg) with code signing
- [ ] Universal binary (Intel + Apple Silicon)
- [ ] Additional character sets (Arabic, Hebrew, Thai, etc.)
- [ ] Custom character set support (user-defined)
- [ ] Per-monitor configuration
- [ ] Metal GPU acceleration for 4K+ displays
- [ ] Audio reactivity using AVFoundation
- [ ] Screen recording export to video/GIF
- [ ] Audio reactivity mode
- [ ] Interactive mode (mouse/keyboard patterns)
- [ ] Configuration profiles
- [ ] Snapshot/export feature

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the iconic digital rain from The Matrix films
- Built with Rust for performance and safety
- Special thanks to all contributors

## Support

- **Issues**: [GitHub Issues](https://github.com/iggybdda/matrix-rain-macos-screensaver/issues)
- **Discussions**: [GitHub Discussions](https://github.com/iggybdda/matrix-rain-macos-screensaver/discussions)

## Screenshots

*Coming soon - screenshots will be added once the macOS screensaver integration is complete.*

---

**Made with ❤️ and Rust**