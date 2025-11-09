//
//  MatrixRainView.swift
//  MatrixRainSaver
//
//  Main screensaver view integrating with Rust engine via FFI
//

import ScreenSaver
import Foundation
import AppKit
import SwiftUI

class MatrixRainView: ScreenSaverView {
    private var matrixEngine: OpaquePointer?
    private var preferences: Preferences
    private var font: NSFont

    // MARK: - Initialization

    override init?(frame: NSRect, isPreview: Bool) {
        self.preferences = Preferences()
        self.font = NSFont.monospacedSystemFont(ofSize: 16, weight: .regular)
        super.init(frame: frame, isPreview: isPreview)

        setupEngine()

        // Set animation interval based on speed preference
        let speed = preferences.speed
        let intervalMs = matrix_rain_get_update_interval_ms(speed)
        self.animationTimeInterval = TimeInterval(intervalMs) / 1000.0
    }

    required init?(coder: NSCoder) {
        self.preferences = Preferences()
        self.font = NSFont.monospacedSystemFont(ofSize: 16, weight: .regular)
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
        bounds.fill()

        // Get render data from Rust engine
        guard let engine = matrixEngine else { return }

        var count: size_t = 0
        guard let dataPtr = matrix_rain_get_render_chars(engine, &count) else { return }

        // Safety check
        guard count > 0 else { return }

        // Convert pointer to array
        let renderChars = UnsafeBufferPointer(start: dataPtr, count: count)

        // Draw each character
        for renderChar in renderChars {
            // Convert Unicode codepoint to character
            guard let scalar = UnicodeScalar(renderChar.character) else { continue }
            let character = String(Character(scalar))

            // Create color with alpha
            let color = NSColor(
                red: CGFloat(renderChar.r) / 255.0,
                green: CGFloat(renderChar.g) / 255.0,
                blue: CGFloat(renderChar.b) / 255.0,
                alpha: CGFloat(renderChar.a)
            )

            // Create attributes
            let attrs: [NSAttributedString.Key: Any] = [
                .font: NSFont.monospacedSystemFont(ofSize: CGFloat(renderChar.font_size), weight: .regular),
                .foregroundColor: color
            ]

            // Draw character
            let point = NSPoint(x: CGFloat(renderChar.x), y: CGFloat(renderChar.y))
            (character as NSString).draw(at: point, withAttributes: attrs)
        }
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
            contentRect: NSRect(x: 0, y: 0, width: 400, height: 350),
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

        // Update font if needed
        self.font = NSFont.monospacedSystemFont(ofSize: 16, weight: .regular)
    }

    // MARK: - Cleanup

    deinit {
        if let engine = matrixEngine {
            matrix_rain_destroy(engine)
        }
    }
}
