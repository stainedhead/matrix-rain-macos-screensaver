# Matrix Rain macOS Screensaver

This directory contains the macOS screensaver implementation that uses the Rust core library via FFI.

## Structure

```
macos-screensaver/
├── MatrixRainSaver/
│   ├── BridgingHeader.h          # Objective-C bridge for FFI
│   ├── Preferences.swift          # User preferences manager
│   ├── ConfigurationView.swift   # SwiftUI settings panel
│   ├── MatrixRainView.swift      # Main screensaver view
│   └── Info.plist                # Bundle configuration
├── Frameworks/
│   └── libmatrix_rain_core.dylib # Rust FFI library
├── build-screensaver.sh          # Build script
└── README.md                      # This file
```

## Building

### Prerequisites

- macOS 11.0 or later
- Swift compiler (included with Xcode Command Line Tools)
- Rust toolchain (to build the core library)

### Build Steps

1. **Build the Rust FFI library** (from project root):
   ```bash
   cargo build --release --features ffi
   ```

2. **Build the screensaver bundle**:
   ```bash
   cd macos-screensaver
   ./build-screensaver.sh
   ```

This creates `build/MatrixRainSaver.saver`

## Installation

### Manual Installation

Copy the built screensaver to your Screen Savers directory:

```bash
cp -r build/MatrixRainSaver.saver ~/Library/Screen\ Savers/
```

Then open System Preferences > Screen Saver and select "MatrixRainSaver".

### System-wide Installation

For all users (requires admin):

```bash
sudo cp -r build/MatrixRainSaver.saver /Library/Screen\ Savers/
```

## Configuration

Click the "Screen Saver Options..." button in System Preferences to configure:

- **Character Set**: Choose from Japanese, Hindi, Tamil, Sinhala, Korean, or Jawi
- **Color Scheme**: Select from 11 color schemes (classic Matrix green is default)
- **Speed**: Adjust animation speed from Very Slow to Very Fast

Settings are saved per-user in UserDefaults.

## Features

### Visual Effects

- **Bright white leading character**: Classic Matrix effect with white character at the head of each trail
- **Smooth color fading**: Characters transition from bright to dark as they age
- **Variable column speeds**: Each rain column falls at a slightly different rate
- **Character glitching**: Random character changes for authentic Matrix look
- **Multiple character sets**: Authentic Unicode scripts from various languages

### Technical

- **Rust-powered engine**: High-performance animation core
- **FFI integration**: Seamless Swift ↔ Rust communication
- **Efficient rendering**: Cached render data to minimize allocations
- **Memory safe**: Proper lifecycle management with automatic cleanup

## Troubleshooting

### Build Errors

If you get library linking errors:
```bash
# Rebuild the Rust library
cd ..
cargo clean
cargo build --release --features ffi
cp target/release/libmatrix_rain_core.dylib macos-screensaver/Frameworks/

# Then rebuild the screensaver
cd macos-screensaver
./build-screensaver.sh
```

### Screensaver Not Appearing

1. Check that the bundle is in the correct location:
   ```bash
   ls ~/Library/Screen\ Savers/MatrixRainSaver.saver
   ```

2. Restart System Preferences

3. Check Console.app for error messages

### Performance Issues

- Try selecting a slower speed in settings
- Reduce screen resolution (for 4K+ displays)
- Consider GPU-accelerated version (future enhancement)

## Development

### Xcode Integration (Optional)

You can also build this as an Xcode project:

1. Create new Screen Saver project in Xcode
2. Add the Swift files from `MatrixRainSaver/`
3. Link against `libmatrix_rain_core.dylib`
4. Set library search paths and rpath

### Debugging

Run the CLI version for easier debugging:
```bash
cd ..
cargo run --release --features cli --bin matrix-rain
```

## Future Enhancements

- [ ] Metal rendering for GPU acceleration
- [ ] Multi-monitor independent configuration
- [ ] Audio reactivity
- [ ] Screen recording export
- [ ] Custom user-defined character sets

## License

MIT License - See LICENSE file in project root

## Credits

Built with Rust 🦀 and Swift, inspired by the iconic Matrix digital rain effect.
