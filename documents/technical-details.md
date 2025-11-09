# Technical Details

## Matrix Rain macOS Screensaver - Technical Architecture

### Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Module Design](#module-design)
3. [Data Structures](#data-structures)
4. [Algorithms](#algorithms)
5. [Threading Model](#threading-model)
6. [Testing Strategy](#testing-strategy)
7. [Build System](#build-system)

---

## Architecture Overview

### High-Level Design

```
┌─────────────────────────────────────────────────────┐
│                   User/System                        │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│              macOS ScreenSaver                       │
│              (Future Integration)                    │
└──────────────────┬──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────┐
│           Matrix Rain Core Library                   │
├─────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐          │
│  │  Config  │  │  Engine  │  │Rendering │          │
│  └──────────┘  └──────────┘  └──────────┘          │
└─────────────────────────────────────────────────────┘
```

### Design Principles

1. **Separation of Concerns**: Three distinct modules (Config, Engine, Rendering)
2. **Dependency Inversion**: Renderer trait allows platform-specific implementations
3. **Immutability by Default**: Configuration is immutable, updates create new instances
4. **Type Safety**: Strong typing eliminates entire classes of errors
5. **Test-Driven**: All functionality backed by comprehensive tests

---

## Module Design

### Config Module

**Purpose**: Manage all configuration aspects of the screensaver

**Structure**:
```
config/
├── mod.rs              # Module root, ScreenSaverConfig
├── character_sets.rs   # CharacterSet enum and Unicode ranges
├── colors.rs           # ColorScheme enum and RGB values
└── speed.rs            # RainSpeed enum and timing values
```

**Key Types**:

```rust
pub struct ScreenSaverConfig {
    pub character_set: CharacterSet,
    pub color_scheme: ColorScheme,
    pub speed: RainSpeed,
    pub screen_width: u32,
    pub screen_height: u32,
}

pub enum CharacterSet {
    Japanese, Hindi, Tamil, Sinhala, Korean, Jawi
}

pub enum ColorScheme {
    MatrixGreen, DarkBlue, Purple, Orange, Red,
    Cyan, Yellow, Pink, White, LimeGreen, Teal
}

pub enum RainSpeed {
    VerySlow, Slow, Medium, Fast, VeryFast
}
```

**Serialization**: Uses serde for JSON support

### Engine Module

**Purpose**: Core rain animation logic

**Structure**:
```
engine/
├── mod.rs           # Module root
├── column.rs        # RainColumn struct
└── matrix_rain.rs   # MatrixRain main engine
```

**Key Types**:

```rust
pub struct RainColumn {
    pub x: usize,
    pub y: f32,
    pub characters: Vec<char>,
    pub speed: f32,
    pub max_length: usize,
    pub active: bool,
}

pub struct MatrixRain {
    config: ScreenSaverConfig,
    columns: Vec<RainColumn>,
    rng: StdRng,
    char_width: f32,
    char_height: f32,
    font_size: f32,
}
```

**Responsibilities**:
- Column lifecycle management
- Character generation and mutation
- Position updates
- Off-screen detection and recycling

### Rendering Module

**Purpose**: Abstract rendering layer

**Structure**:
```
rendering/
├── mod.rs       # Module root
├── color.rs     # Color struct
└── renderer.rs  # Renderer trait and RenderChar
```

**Key Types**:

```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

pub struct RenderChar {
    pub character: char,
    pub x: f32,
    pub y: f32,
    pub color: Color,
    pub font_size: f32,
}

pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn draw_char(&mut self, render_char: &RenderChar);
    fn draw_chars(&mut self, chars: &[RenderChar]);
    fn present(&mut self);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
```

---

## Data Structures

### RainColumn State Machine

```
┌──────────┐
│ Creation │ (y < 0, active = true)
└────┬─────┘
     │
     ▼
┌──────────┐
│ Active   │ (y >= 0, building trail)
│ Falling  │
└────┬─────┘
     │
     ▼
┌──────────┐
│ Off      │ (y > screen_height + trail_length)
│ Screen   │
└────┬─────┘
     │
     ├─10%─────► Reset (new position, y < 0)
     │
     └─90%─────► Inactive (active = false)
                      │
                      └─1%/frame─► Reset
```

### Character Trail Structure

```
Trail (oldest to newest):

characters[n-1] ─┐
characters[n-2]  │ Tail (tertiary color, high alpha)
characters[n-3]  │
      ...        │ Middle (secondary color, medium alpha)
characters[2]    │
characters[1]    │
characters[0] ───┘ Head (primary color, low alpha)
                   ^ Current y position
```

### Memory Layout

**Per-Column Overhead**:
- `x`: 8 bytes (usize)
- `y`: 4 bytes (f32)
- `characters`: 24 bytes (Vec header) + N*4 bytes (chars)
- `speed`: 4 bytes (f32)
- `max_length`: 8 bytes (usize)
- `active`: 1 byte (bool)

**Total per column**: ~49 bytes + character storage

**Example (1920px width)**:
- Columns: 120
- Average trail: 15 characters
- Memory: 120 * (49 + 15*4) = ~13 KB

---

## Algorithms

### Column Update Algorithm

```rust
fn update_column(column: &mut RainColumn, char_set: CharacterSet) {
    // 1. Move column down
    column.y += column.speed;

    // 2. Add new character (80% probability)
    if column.characters.len() < column.max_length && random() < 0.8 {
        column.characters.push(char_set.random_character());
    }

    // 3. Glitch effect (5% probability per character)
    for char in &mut column.characters {
        if random() < 0.05 {
            *char = char_set.random_character();
        }
    }

    // 4. Check if off-screen
    if column.is_off_screen() {
        if random() < 0.1 {
            column.reset();
        } else {
            column.active = false;
        }
    }
}
```

**Time Complexity**: O(L) where L = trail length (typically 12-30)
**Space Complexity**: O(1) additional space

### Rendering Algorithm

```rust
fn render(matrix: &MatrixRain, renderer: &mut Renderer) {
    // 1. Clear screen: O(1)
    renderer.clear(BLACK);

    // 2. Collect visible characters: O(C * L)
    //    C = columns, L = avg trail length
    let mut render_chars = Vec::new();
    for column in &matrix.columns {
        if !column.active { continue; }

        for (char, y_pos, trail_pos) in column.trail_positions() {
            if y_pos < 0.0 || y_pos > screen_height { continue; }

            let color = color_scheme.get_with_alpha(trail_pos);
            render_chars.push(RenderChar { char, x, y, color, ... });
        }
    }

    // 3. Batch render: O(N) where N = visible chars
    renderer.draw_chars(&render_chars);

    // 4. Present: O(1)
    renderer.present();
}
```

**Total Complexity**: O(C * L) where typically C*L ≈ 1000-2000

### Character Generation

Uses Unicode range sampling:

```rust
fn get_characters(set: CharacterSet) -> Vec<char> {
    match set {
        Japanese => {
            // Katakana range
            (0x30A0..=0x30FF).filter_map(char::from_u32).collect()

            // Plus half-width katakana
            + (0xFF65..=0xFF9F).filter_map(char::from_u32)

            // Plus symbols
            + "0123456789.:=*+-<>".chars()
        },
        // ... other sets
    }
}
```

**Optimization**: Character vectors generated once and cached

---

## Threading Model

### Current Implementation (Single-threaded)

```
Main Thread:
  ├─ Update Loop (timer-driven)
  │    ├─ Update all columns
  │    └─ Collect render data
  │
  └─ Render Loop
       ├─ Clear screen
       ├─ Draw characters
       └─ Present frame
```

**Justification**:
- Rendering is fast enough single-threaded
- No data races or synchronization overhead
- Simpler code, easier to maintain

### Future Multi-threading Potential

```
Timer Thread:
  └─ Trigger updates at interval

Update Thread:
  └─ Update all columns
       ├─ Could parallelize with rayon
       └─ Send RenderChars to render thread

Render Thread:
  └─ Receive RenderChars
       ├─ Batch draw
       └─ Present
```

**When to consider**:
- Ultra-high resolutions (8K+)
- Very fast update rates (>120 FPS)
- Hundreds of columns with long trails

---

## Testing Strategy

### Test Organization

```
src/
├── config/
│   ├── mod.rs (6 tests)
│   ├── character_sets.rs (4 tests)
│   ├── colors.rs (5 tests)
│   └── speed.rs (6 tests)
├── engine/
│   ├── column.rs (7 tests)
│   └── matrix_rain.rs (5 tests)
├── rendering/
│   ├── color.rs (8 tests)
│   └── renderer.rs (3 tests)
└── lib.rs (1 test)

Total: 42 tests
```

### Test Categories

**Unit Tests**:
- Individual function behavior
- Edge case handling
- Type constraints

**Integration Tests**:
- Module interaction
- Configuration serialization
- End-to-end engine behavior

**Property Tests** (Potential):
- Character set completeness
- Color value ranges
- Speed interval ordering

### Test Patterns

**Mock Renderer**:
```rust
struct MockRenderer {
    chars_drawn: Vec<RenderChar>,
    clear_count: usize,
    present_count: usize,
}
```

**Deterministic RNG** (for testing):
```rust
let rng = StdRng::seed_from_u64(42);  // Fixed seed
```

### Coverage Goals

- **Line Coverage**: >90%
- **Branch Coverage**: >85%
- **Public API**: 100%
- **Critical Paths**: 100%

### Continuous Testing

```bash
# Local development
cargo test
cargo test -- --nocapture  # With output
cargo test --release       # Optimized

# CI/CD (GitHub Actions planned)
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test --all-features
- cargo doc --no-deps
```

---

## Build System

### Cargo Configuration

**Dependencies**:
- Core: `rand`, `serde`, `serde_json`
- macOS-only: `objc`, `cocoa`, `core-graphics`

**Build Profiles**:
```toml
[profile.release]
opt-level = 3
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
strip = true         # Smaller binary
```

**Target Configuration**:
```toml
[lib]
crate-type = ["lib", "cdylib"]  # Static + dynamic lib
```

### Build Commands

```bash
# Development build
cargo build

# Optimized release
cargo build --release

# Documentation
cargo doc --open

# Linting
cargo clippy

# Formatting
cargo fmt

# Check without building
cargo check
```

### Platform-Specific Compilation

```rust
#[cfg(target_os = "macos")]
use cocoa::*;

#[cfg(target_os = "macos")]
impl MacOSRenderer {
    // macOS-specific code
}
```

### Dependency Tree

```
matrix_rain_core
├── rand 0.8
│   ├── rand_core
│   └── rand_chacha
├── serde 1.0
│   └── serde_derive
└── serde_json 1.0
    ├── serde
    ├── itoa
    └── ryu

[target.macos]
├── objc 0.2
├── cocoa 0.25
│   ├── cocoa-foundation
│   └── core-graphics
└── core-graphics 0.23
    └── core-foundation
```

---

## Performance Optimization

### Current Optimizations

1. **Character Set Caching**: Unicode ranges computed once
2. **Vector Pre-allocation**: Capacity hints for character storage
3. **Batch Rendering**: All characters collected before drawing
4. **Smart Culling**: Off-screen characters skipped
5. **Random Speed Variation**: Prevents synchronization artifacts

### Profiling

```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin screensaver_demo

# Memory profiling
cargo install valgrind
valgrind --tool=massif target/release/screensaver_demo

# Benchmarking
cargo bench  # (requires benchmark setup)
```

### Bottleneck Analysis

**Expected hot paths**:
1. Column updates (O(C*L))
2. Character rendering (O(visible chars))
3. Color calculations (O(visible chars))
4. Random number generation (O(C*L))

**Optimization opportunities**:
- SIMD for color calculations
- GPU rendering for high-resolution displays
- Thread pool for column updates at 4K+

---

## Future Technical Enhancements

### macOS Integration

**ScreenSaverView subclass**:
```swift
import ScreenSaver

class MatrixRainView: ScreenSaverView {
    var matrixRain: MatrixRain?

    override func startAnimation() {
        // Initialize Rust engine via FFI
    }

    override func animateOneFrame() {
        // Call Rust update and render
    }
}
```

**FFI Bridge**:
```rust
#[no_mangle]
pub extern "C" fn matrix_rain_new(
    width: u32,
    height: u32
) -> *mut MatrixRain {
    // Create and return engine
}
```

### GPU Acceleration

Potential Metal/OpenGL rendering:
- Vertex buffers for character quads
- Texture atlas for glyph rendering
- Shader-based color effects
- Instanced rendering for performance

### Advanced Features

- **Particle effects**: Occasional glitches, waves
- **Audio reactivity**: Respond to system audio
- **Camera effects**: Depth of field, motion blur
- **Interactive mode**: Mouse interaction, patterns
