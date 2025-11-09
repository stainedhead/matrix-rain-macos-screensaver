# macOS ScreenSaver Implementation Plan

## Overview

This document outlines the complete plan to implement the remaining roadmap items for the Matrix Rain macOS Screensaver project.

---

## Phase 1: FFI Bridge & Core Graphics Renderer ✅ (In Progress)

### 1.1 FFI Interface (COMPLETED)
- ✅ Created `src/ffi.rs` with C-compatible exports
- ✅ Implemented `MatrixRainHandle` opaque pointer pattern
- ✅ Added lifecycle methods: `matrix_rain_new`, `matrix_rain_update`, `matrix_rain_destroy`
- ✅ Added configuration update method: `matrix_rain_set_config`
- ✅ Added 4 FFI tests for safety and functionality
- ✅ Added `ffi` feature flag to Cargo.toml

### 1.2 Build FFI Library
```bash
# Build the dynamic library for macOS
cargo build --release --features ffi
# Produces: target/release/libmatrix_rain_core.dylib

# For universal binary (Intel + ARM)
cargo build --release --features ffi --target x86_64-apple-darwin
cargo build --release --features ffi --target aarch64-apple-darwin
lipo -create \
  target/x86_64-apple-darwin/release/libmatrix_rain_core.dylib \
  target/aarch64-apple-darwin/release/libmatrix_rain_core.dylib \
  -output libmatrix_rain_core_universal.dylib
```

### 1.3 Core Graphics Renderer (TODO)
Create `src/rendering/coregraphics.rs`:

```rust
#[cfg(target_os = "macos")]
use core_graphics::context::CGContext;
use super::{Color, RenderChar, Renderer};

pub struct CoreGraphicsRenderer {
    context: CGContext,
    width: u32,
    height: u32,
}

impl CoreGraphicsRenderer {
    pub fn new(context: CGContext, width: u32, height: u32) -> Self {
        Self { context, width, height }
    }
}

impl Renderer for CoreGraphicsRenderer {
    fn clear(&mut self, color: Color) {
        // Use CGContext to fill background
    }

    fn draw_char(&mut self, render_char: &RenderChar) {
        // Use CGContext to draw text
    }

    fn present(&mut self) {
        // Flush context
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
```

---

## Phase 2: Xcode ScreenSaver Project

### 2.1 Create Xcode Project

1. **Open Xcode** → New Project
2. **macOS** → Screen Saver
3. **Product Name**: MatrixRainSaver
4. **Language**: Swift
5. **Bundle Identifier**: `com.stainedhead.MatrixRainSaver`

### 2.2 Project Structure
```
macos-screensaver/
├── MatrixRainSaver.xcodeproj
├── MatrixRainSaver/
│   ├── MatrixRainView.swift        # Main screensaver view
│   ├── ConfigurationView.swift     # Settings panel
│   ├── Preferences.swift            # User defaults manager
│   ├── BridgingHeader.h            # Objective-C bridge
│   ├── Info.plist
│   └── Assets.xcassets/
└── Frameworks/
    └── libmatrix_rain_core.dylib   # Rust library
```

### 2.3 Bridging Header (`BridgingHeader.h`)

```objc
#ifndef BridgingHeader_h
#define BridgingHeader_h

#include <stdint.h>

// Opaque type
typedef struct MatrixRainHandle MatrixRainHandle;

// FFI function declarations
MatrixRainHandle* matrix_rain_new(uint32_t width, uint32_t height,
                                   uint8_t charset, uint8_t color, uint8_t speed);
void matrix_rain_update(MatrixRainHandle* handle);
void matrix_rain_set_config(MatrixRainHandle* handle, uint32_t width, uint32_t height,
                             uint8_t charset, uint8_t color, uint8_t speed);
void matrix_rain_destroy(MatrixRainHandle* handle);
uint64_t matrix_rain_get_update_interval_ms(uint8_t speed);

#endif
```

### 2.4 Swift ScreenSaverView (`MatrixRainView.swift`)

```swift
import ScreenSaver
import Foundation
import CoreGraphics

class MatrixRainView: ScreenSaverView {
    private var matrixEngine: OpaquePointer?
    private var preferences: Preferences

    // MARK: - Initialization

    override init?(frame: NSRect, isPreview: Bool) {
        self.preferences = Preferences()
        super.init(frame: frame, isPreview: isPreview)

        setupEngine()

        // Set animation interval based on speed preference
        let speed = preferences.speed
        let intervalMs = matrix_rain_get_update_interval_ms(speed)
        self.animationTimeInterval = TimeInterval(intervalMs) / 1000.0
    }

    required init?(coder: NSCoder) {
        self.preferences = Preferences()
        super.init(coder: coder)
        setupEngine()
    }

    private func setupEngine() {
        let width = UInt32(bounds.width)
        let height = UInt32(bounds.height)

        matrixEngine = matrix_rain_new(
            width,
            height,
            preferences.characterSet,
            preferences.colorScheme,
            preferences.speed
        )
    }

    // MARK: - Animation

    override func startAnimation() {
        super.startAnimation()
    }

    override func stopAnimation() {
        super.stopAnimation()
    }

    override func animateOneFrame() {
        super.animateOneFrame()

        // Update the Rust engine
        if let engine = matrixEngine {
            matrix_rain_update(engine)
        }

        // Trigger redraw
        setNeedsDisplay(bounds)
    }

    // MARK: - Drawing

    override func draw(_ rect: NSRect) {
        super.draw(rect)

        // Fill background
        NSColor.black.setFill()
        rect.fill()

        // TODO: Get render data from Rust and draw characters
        // This requires extending the FFI to return render data

        // For now, we can draw a simple test
        let attrs: [NSAttributedString.Key: Any] = [
            .font: NSFont.monospacedSystemFont(ofSize: 16, weight: .regular),
            .foregroundColor: NSColor.green
        ]

        let testString = "Matrix Rain (Rust Core)"
        let nsString = NSString(string: testString)
        nsString.draw(at: NSPoint(x: 20, y: 20), withAttributes: attrs)
    }

    // MARK: - Configuration

    override var hasConfigureSheet: Bool {
        return true
    }

    override var configureSheet: NSWindow? {
        let configView = ConfigurationView(preferences: preferences) { [weak self] in
            self?.updateConfiguration()
        }

        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 400, height: 300),
            styleMask: [.titled, .closable],
            backing: .buffered,
            defer: false
        )
        window.title = "Matrix Rain Configuration"
        window.contentView = NSHostingView(rootView: configView)
        return window
    }

    private func updateConfiguration() {
        if let engine = matrixEngine {
            matrix_rain_set_config(
                engine,
                UInt32(bounds.width),
                UInt32(bounds.height),
                preferences.characterSet,
                preferences.colorScheme,
                preferences.speed
            )
        }

        // Update animation interval
        let intervalMs = matrix_rain_get_update_interval_ms(preferences.speed)
        self.animationTimeInterval = TimeInterval(intervalMs) / 1000.0
    }

    // MARK: - Cleanup

    deinit {
        if let engine = matrixEngine {
            matrix_rain_destroy(engine)
        }
    }
}
```

### 2.5 Preferences Manager (`Preferences.swift`)

```swift
import Foundation

class Preferences {
    private let defaults = UserDefaults.standard

    private enum Keys {
        static let characterSet = "MatrixRainCharacterSet"
        static let colorScheme = "MatrixRainColorScheme"
        static let speed = "MatrixRainSpeed"
    }

    var characterSet: UInt8 {
        get { UInt8(defaults.integer(forKey: Keys.characterSet)) }
        set { defaults.set(Int(newValue), forKey: Keys.characterSet) }
    }

    var colorScheme: UInt8 {
        get { UInt8(defaults.integer(forKey: Keys.colorScheme)) }
        set { defaults.set(Int(newValue), forKey: Keys.colorScheme) }
    }

    var speed: UInt8 {
        get {
            let value = defaults.integer(forKey: Keys.speed)
            return value == 0 ? 2 : UInt8(value) // Default to Medium (2)
        }
        set { defaults.set(Int(newValue), forKey: Keys.speed) }
    }
}
```

### 2.6 Configuration View (`ConfigurationView.swift`)

```swift
import SwiftUI

struct ConfigurationView: View {
    @ObservedObject var preferences: Preferences
    var onSave: () -> Void

    private let charsets = ["Japanese", "Hindi", "Tamil", "Sinhala", "Korean", "Jawi"]
    private let colors = ["Matrix Green", "Dark Blue", "Purple", "Orange", "Red",
                          "Cyan", "Yellow", "Pink", "White", "Lime Green", "Teal"]
    private let speeds = ["Very Slow", "Slow", "Medium", "Fast", "Very Fast"]

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text("Matrix Rain Configuration")
                .font(.title2)
                .bold()

            GroupBox(label: Text("Character Set")) {
                Picker("", selection: $preferences.characterSet) {
                    ForEach(0..<charsets.count, id: \.self) { index in
                        Text(charsets[index]).tag(UInt8(index))
                    }
                }
                .pickerStyle(.radioGroup)
            }

            GroupBox(label: Text("Color Scheme")) {
                Picker("", selection: $preferences.colorScheme) {
                    ForEach(0..<colors.count, id: \.self) { index in
                        Text(colors[index]).tag(UInt8(index))
                    }
                }
                .pickerStyle(.menu)
            }

            GroupBox(label: Text("Speed")) {
                Picker("", selection: $preferences.speed) {
                    ForEach(0..<speeds.count, id: \.self) { index in
                        Text(speeds[index]).tag(UInt8(index))
                    }
                }
                .pickerStyle(.segmented)
            }

            Spacer()

            HStack {
                Spacer()
                Button("Save") {
                    onSave()
                }
                .keyboardShortcut(.defaultAction)
            }
        }
        .padding()
        .frame(width: 400, height: 300)
    }
}
```

---

## Phase 3: Building & Testing

### 3.1 Build Steps

1. **Build Rust Library**:
   ```bash
   cargo build --release --features ffi
   ```

2. **Copy to Xcode Project**:
   ```bash
   cp target/release/libmatrix_rain_core.dylib macos-screensaver/Frameworks/
   ```

3. **Configure Xcode**:
   - Add `libmatrix_rain_core.dylib` to "Link Binary With Libraries"
   - Set library search path to `$(PROJECT_DIR)/Frameworks`
   - Add run script phase to copy dylib to bundle

4. **Build in Xcode**: Cmd+B

### 3.2 Testing

**Test in Xcode**:
- Run scheme should launch System Preferences

**Manual Testing**:
```bash
# Build the screensaver
xcodebuild -project macos-screensaver/MatrixRainSaver.xcodeproj \
           -scheme MatrixRainSaver \
           -configuration Release

# Copy to Screen Savers
cp -r build/Release/MatrixRainSaver.saver ~/Library/Screen\ Savers/

# Open System Preferences
open /System/Library/PreferencePanes/DesktopScreenEffectsPref.prefPane
```

---

## Phase 4: Installer Package

### 4.1 Create PKG Installer

Create `macos-screensaver/create-installer.sh`:

```bash
#!/bin/bash

set -e

# Build the screensaver
xcodebuild -project MatrixRainSaver.xcodeproj \
           -scheme MatrixRainSaver \
           -configuration Release \
           clean build

# Create package structure
mkdir -p pkgroot/Library/Screen\ Savers
cp -r build/Release/MatrixRainSaver.saver pkgroot/Library/Screen\ Savers/

# Build package
pkgbuild --root pkgroot \
         --identifier com.stainedhead.MatrixRainSaver \
         --version 0.1.0 \
         --install-location / \
         MatrixRainSaver.pkg

# Clean up
rm -rf pkgroot

echo "Created MatrixRainSaver.pkg"
```

Make executable:
```bash
chmod +x macos-screensaver/create-installer.sh
```

---

## Phase 5: Multi-Monitor Support

### 5.1 Update Swift Code

```swift
override init?(frame: NSRect, isPreview: Bool) {
    // Detect if on secondary screen
    let screenIndex = NSScreen.screens.firstIndex(of: self.window?.screen ?? NSScreen.main!) ?? 0

    // Could use different configs per screen
    // For now, just use same config

    super.init(frame: frame, isPreview: isPreview)
}
```

---

## Phase 6: Documentation & Release

### 6.1 Update Documentation

Add to README.md:
- Installation instructions for .saver file
- Installation instructions for .pkg
- Uninstallation instructions
- Troubleshooting section

### 6.2 GitHub Release Workflow

Update `.github/workflows/release.yml` to include:
- Build Rust library with FFI
- Build Xcode project
- Create .pkg installer
- Upload to release

---

## Implementation Checklist

### Immediate Next Steps
- [ ] Test FFI build: `cargo build --release --features ffi`
- [ ] Create Xcode project following structure above
- [ ] Implement MatrixRainView.swift
- [ ] Test basic rendering
- [ ] Implement configuration UI
- [ ] Test preferences persistence

### Before Release
- [ ] Add proper render data export in FFI
- [ ] Implement Core Graphics drawing
- [ ] Test on multiple macOS versions
- [ ] Create installer package
- [ ] Update all documentation
- [ ] Create release with binaries

### Future Enhancements
- [ ] GPU-accelerated rendering with Metal
- [ ] Audio reactivity using AVFoundation
- [ ] Screen saver transitions
- [ ] Particle effects
- [ ] Export frames as video/GIF

---

## Resources

- [Apple ScreenSaver Documentation](https://developer.apple.com/documentation/screensaver)
- [Swift FFI Guide](https://theswiftdev.com/how-to-call-c-code-from-swift/)
- [Creating PKG Installers](https://www.techrepublic.com/article/how-to-create-a-simple-package-installer-for-macos/)

---

## Notes

1. **Code Signing**: For distribution, the .saver bundle must be code signed
2. **Notarization**: macOS 10.15+ requires notarization for distribution
3. **Privacy**: Screen recording permission may be required on newer macOS
4. **Performance**: Consider Metal rendering for 4K+ displays
5. **Testing**: Test on macOS 12, 13, 14, and 15 (Sequoia)

---

## Timeline Estimate

- **Phase 1** (FFI + Core Graphics): 2-4 hours
- **Phase 2** (Xcode Project): 4-6 hours
- **Phase 3** (Testing): 2-3 hours
- **Phase 4** (Installer): 1-2 hours
- **Phase 5** (Multi-monitor): 1-2 hours
- **Phase 6** (Documentation): 2-3 hours

**Total Estimated Time**: 12-20 hours of focused development

---

*Last Updated: 2025-01-08*
