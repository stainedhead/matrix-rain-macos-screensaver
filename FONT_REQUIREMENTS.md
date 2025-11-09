# Font and Character Set Requirements

## Short Answer

**The character data IS built into the application** - you don't need to install anything extra. However, your **terminal font** must support the Unicode scripts you want to display.

## How It Works

### What's Built-In ✅
- **All character data** is compiled into the Rust binary
- Unicode codepoints for all 7 character sets (Japanese, Hindi, Tamil, Sinhala, Korean, Jawi, Mixed)
- **207** Japanese characters (Katakana)
- **160** Hindi characters (Devanagari)
- **128** Tamil characters
- **160** Sinhala characters
- **1215** Korean characters (Hangul)
- **400** Jawi/Arabic characters
- **457** Mixed characters

### What's NOT Built-In ⚠️
- **Font glyphs** - your terminal's font must have the shapes for these characters
- This is similar to how a web page can specify emoji 🚀, but your font needs the emoji glyph to display it

## macOS Terminal Fonts

### Built-in Fonts (Usually Work)
Most macOS terminal fonts support a wide range of Unicode:

- **SF Mono** ✅ (default in macOS Terminal, supports most scripts)
- **Menlo** ✅ (supports most scripts)
- **Monaco** ⚠️ (limited Unicode support)

### Recommended Fonts for Full Support
If you want perfect support for all character sets:

1. **Noto Sans Mono** - Google's font designed for comprehensive Unicode coverage
   - Download: https://fonts.google.com/noto/specimen/Noto+Sans+Mono

2. **Cascadia Code** - Microsoft's programmer font with good Unicode support
   - Download: https://github.com/microsoft/cascadia-code

3. **JetBrains Mono** - Good general programming font
   - Download: https://www.jetbrains.com/lp/mono/

## Terminal Requirements

Your terminal needs to support:

1. **24-bit True Color** (16.7 million colors)
   - Most modern terminals support this
   - macOS Terminal.app: ✅ YES
   - iTerm2: ✅ YES
   - VSCode integrated terminal: ✅ YES

2. **UTF-8 encoding**
   - Should be default on macOS
   - Check: `echo $LANG` should show `*.UTF-8`

## What You're Likely Seeing

Based on your description of "flashing colors" and "missing background layer":

### 1. Terminal Color Support
Some older terminals don't support RGB colors (24-bit color). They fall back to 256-color mode or even 16-color mode, which can cause:
- Wrong colors
- Flashing as the terminal tries to approximate colors
- Inconsistent rendering

**Check**: Your terminal should support "True Color" or "24-bit color"

### 2. Font Glyph Coverage
If your terminal font doesn't have glyphs for certain Unicode ranges:
- Characters may appear as boxes (□)
- Characters may appear as replacement characters (�)
- Characters may be invisible
- Wrong spacing/alignment

**Fix**: Switch to a font with better Unicode coverage (see recommendations above)

### 3. Rendering Artifacts
The terminal might be:
- Not clearing the screen properly between frames
- Not setting the background color correctly
- Having issues with character positioning

**Fix**: Recent code changes fixed the background color handling

## Testing Your Setup

Run these test programs to diagnose issues:

```bash
# Test 1: Check character sets are available
cargo run --example test_charset

# Test 2: Visual terminal color test (requires TTY)
cargo run --example test_terminal_display --features cli
```

## CLI Usage

```bash
# Build the CLI
cargo build --release --features cli --bin matrix-rain

# Test with Japanese (most compatible)
./target/release/matrix-rain -c japanese -o matrix-green

# Test with Mixed (requires better font coverage)
./target/release/matrix-rain -c mixed -o purple
```

## Troubleshooting

### Issue: Characters appear as boxes (□)
**Cause**: Font doesn't have glyphs for that Unicode range
**Fix**: Use a font with better coverage (Noto Sans Mono recommended)

### Issue: Colors are wrong or flashing
**Cause**: Terminal doesn't support 24-bit color
**Fix**: Use a modern terminal (iTerm2, macOS Terminal.app 10.15+)

### Issue: Background layer not visible
**Cause**: Background layer is intentionally subtle (30% opacity)
**Note**: This is by design for the depth effect - it should be barely visible

### Issue: Performance is slow
**Cause**: Terminal rendering is slow compared to GPU rendering
**Fix**:
- Use a faster terminal (iTerm2 with GPU rendering)
- Reduce speed setting: `-s slow` or `-s very-slow`
- Use macOS screensaver version instead (uses Metal GPU rendering)

## macOS Screensaver vs CLI

### CLI (Terminal)
- ✅ Quick to test
- ✅ Works in any terminal
- ⚠️ Limited by terminal font support
- ⚠️ Performance depends on terminal
- ⚠️ Subtle effects (like 30% opacity) may not render well

### macOS Screensaver
- ✅ Full GPU rendering via Metal
- ✅ Perfect color accuracy
- ✅ Smooth 60 FPS animation
- ✅ Proper alpha blending for background layer
- ✅ Works without font dependencies (renders glyphs from system fonts)
- ℹ️ Requires macOS 10.15+

**Recommendation**: For the best experience, use the macOS screensaver version. The CLI is great for testing and development, but the screensaver uses the full GPU and system font rendering.

## Summary

| Feature | Built Into Binary | Requires Terminal Font |
|---------|------------------|------------------------|
| Character data (Unicode codepoints) | ✅ YES | ❌ NO |
| Character glyphs (visual shapes) | ❌ NO | ✅ YES |
| Color data | ✅ YES | ❌ NO |
| 24-bit color rendering | ❌ NO | ✅ YES |
| Alpha blending | ✅ YES (simulated in terminal) | ⚠️ LIMITED |

**Bottom line**: The app has everything it needs. Your terminal needs a good Unicode font and 24-bit color support.
