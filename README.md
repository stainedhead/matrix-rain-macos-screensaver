# Matrix Rain macOS Screensaver

A customizable Matrix-style digital rain screensaver for macOS, written in Rust. Experience the iconic cascading characters from The Matrix with support for multiple languages and color schemes.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)

## Features

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

## Installation

### Prerequisites
- macOS 10.15 (Catalina) or later
- Rust 1.70 or later (for building from source)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/iggybdda/matrix-rain-macos-screensaver.git
cd matrix-rain-macos-screensaver

# Build the project
cargo build --release

# Run tests
cargo test
```

## Usage

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

*Note: macOS screensaver integration is currently in development. The core library is fully functional and tested.*

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

- [x] Core rain engine
- [x] Multiple character set support
- [x] Color customization
- [x] Speed settings
- [ ] macOS ScreenSaver bundle integration
- [ ] User preferences UI
- [ ] Installer package
- [ ] Additional character sets
- [ ] Custom character set support
- [ ] Multi-monitor support

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