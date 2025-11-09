//
//  Preferences.swift
//  MatrixRainSaver
//
//  Manages user preferences using UserDefaults
//

import Foundation

class Preferences: ObservableObject {
    private let defaults = UserDefaults.standard

    private enum Keys {
        static let characterSet = "MatrixRainCharacterSet"
        static let colorScheme = "MatrixRainColorScheme"
        static let speed = "MatrixRainSpeed"
    }

    @Published var characterSet: UInt8 {
        didSet {
            defaults.set(Int(characterSet), forKey: Keys.characterSet)
        }
    }

    @Published var colorScheme: UInt8 {
        didSet {
            defaults.set(Int(colorScheme), forKey: Keys.colorScheme)
        }
    }

    @Published var speed: UInt8 {
        didSet {
            defaults.set(Int(speed), forKey: Keys.speed)
        }
    }

    init() {
        // Load saved preferences or use defaults
        self.characterSet = UInt8(defaults.integer(forKey: Keys.characterSet))
        self.colorScheme = UInt8(defaults.integer(forKey: Keys.colorScheme))

        let savedSpeed = defaults.integer(forKey: Keys.speed)
        self.speed = savedSpeed == 0 ? 2 : UInt8(savedSpeed) // Default to Medium (2)
    }

    // Character set names for UI
    static let characterSetNames = [
        "Japanese",
        "Hindi",
        "Tamil",
        "Sinhala",
        "Korean",
        "Jawi",
        "Mixed"
    ]

    // Color scheme names for UI
    static let colorSchemeNames = [
        "Matrix Green",
        "Dark Blue",
        "Purple",
        "Orange",
        "Red",
        "Cyan",
        "Yellow",
        "Pink",
        "White",
        "Lime Green",
        "Teal"
    ]

    // Speed names for UI
    static let speedNames = [
        "Very Slow",
        "Slow",
        "Medium",
        "Fast",
        "Very Fast"
    ]
}
