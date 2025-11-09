//
//  ConfigurationView.swift
//  MatrixRainSaver
//
//  SwiftUI configuration panel for screensaver settings
//

import SwiftUI

struct ConfigurationView: View {
    @ObservedObject var preferences: Preferences
    var onSave: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text("Matrix Rain Configuration")
                .font(.title2)
                .bold()

            GroupBox(label: Text("Character Set")) {
                Picker("", selection: $preferences.characterSet) {
                    ForEach(0..<Preferences.characterSetNames.count, id: \.self) { index in
                        Text(Preferences.characterSetNames[index]).tag(UInt8(index))
                    }
                }
                .pickerStyle(.radioGroup)
            }

            GroupBox(label: Text("Color Scheme")) {
                Picker("", selection: $preferences.colorScheme) {
                    ForEach(0..<Preferences.colorSchemeNames.count, id: \.self) { index in
                        Text(Preferences.colorSchemeNames[index]).tag(UInt8(index))
                    }
                }
                .pickerStyle(.menu)
            }

            GroupBox(label: Text("Speed")) {
                Picker("", selection: $preferences.speed) {
                    ForEach(0..<Preferences.speedNames.count, id: \.self) { index in
                        Text(Preferences.speedNames[index]).tag(UInt8(index))
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
        .frame(width: 400, height: 350)
    }
}

#if DEBUG
struct ConfigurationView_Previews: PreviewProvider {
    static var previews: some View {
        ConfigurationView(preferences: Preferences()) {
            print("Saved")
        }
    }
}
#endif
