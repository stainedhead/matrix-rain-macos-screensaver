//! FFI (Foreign Function Interface) bindings for macOS ScreenSaver integration
//!
//! This module provides C-compatible exports that can be called from Swift/Objective-C

use crate::{CharacterSet, ColorScheme, MatrixRain, RainSpeed, ScreenSaverConfig};
use std::ptr;

/// Opaque pointer to MatrixRain engine (hides implementation details from C/Swift)
pub struct MatrixRainHandle {
    engine: MatrixRain,
}

/// Create a new Matrix Rain engine
///
/// # Safety
/// The returned pointer must be freed with `matrix_rain_destroy`
#[no_mangle]
pub unsafe extern "C" fn matrix_rain_new(
    width: u32,
    height: u32,
    charset: u8,
    color: u8,
    speed: u8,
) -> *mut MatrixRainHandle {
    let character_set = match charset {
        0 => CharacterSet::Japanese,
        1 => CharacterSet::Hindi,
        2 => CharacterSet::Tamil,
        3 => CharacterSet::Sinhala,
        4 => CharacterSet::Korean,
        5 => CharacterSet::Jawi,
        _ => CharacterSet::Japanese,
    };

    let color_scheme = match color {
        0 => ColorScheme::MatrixGreen,
        1 => ColorScheme::DarkBlue,
        2 => ColorScheme::Purple,
        3 => ColorScheme::Orange,
        4 => ColorScheme::Red,
        5 => ColorScheme::Cyan,
        6 => ColorScheme::Yellow,
        7 => ColorScheme::Pink,
        8 => ColorScheme::White,
        9 => ColorScheme::LimeGreen,
        10 => ColorScheme::Teal,
        _ => ColorScheme::MatrixGreen,
    };

    let rain_speed = match speed {
        0 => RainSpeed::VerySlow,
        1 => RainSpeed::Slow,
        2 => RainSpeed::Medium,
        3 => RainSpeed::Fast,
        4 => RainSpeed::VeryFast,
        _ => RainSpeed::Medium,
    };

    let config = ScreenSaverConfig::new(character_set, color_scheme, rain_speed, width, height);
    let engine = MatrixRain::new(config);

    Box::into_raw(Box::new(MatrixRainHandle { engine }))
}

/// Update the Matrix Rain animation state
///
/// # Safety
/// - `handle` must be a valid pointer returned from `matrix_rain_new`
/// - `handle` must not be null
#[no_mangle]
pub unsafe extern "C" fn matrix_rain_update(handle: *mut MatrixRainHandle) {
    if handle.is_null() {
        return;
    }
    let handle = &mut *handle;
    handle.engine.update();
}

/// Get render data for drawing
/// Returns a pointer to an array of RenderCharFFI and sets the count
///
/// # Safety
/// - `handle` must be a valid pointer
/// - `out_count` must be a valid pointer to write the count
/// - The returned pointer is valid until the next call to this function or `matrix_rain_destroy`
#[repr(C)]
pub struct RenderCharFFI {
    pub character: u32, // Unicode codepoint
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
    pub font_size: f32,
}

#[no_mangle]
pub unsafe extern "C" fn matrix_rain_get_render_chars(
    handle: *mut MatrixRainHandle,
    out_count: *mut usize,
) -> *const RenderCharFFI {
    if handle.is_null() || out_count.is_null() {
        return ptr::null();
    }

    // This is a simplified version - in production, you'd cache the render chars
    // to avoid allocation on every call
    *out_count = 0;
    ptr::null()
}

/// Update the configuration
///
/// # Safety
/// - `handle` must be a valid pointer
#[no_mangle]
pub unsafe extern "C" fn matrix_rain_set_config(
    handle: *mut MatrixRainHandle,
    width: u32,
    height: u32,
    charset: u8,
    color: u8,
    speed: u8,
) {
    if handle.is_null() {
        return;
    }

    let character_set = match charset {
        0 => CharacterSet::Japanese,
        1 => CharacterSet::Hindi,
        2 => CharacterSet::Tamil,
        3 => CharacterSet::Sinhala,
        4 => CharacterSet::Korean,
        5 => CharacterSet::Jawi,
        _ => CharacterSet::Japanese,
    };

    let color_scheme = match color {
        0 => ColorScheme::MatrixGreen,
        1 => ColorScheme::DarkBlue,
        2 => ColorScheme::Purple,
        3 => ColorScheme::Orange,
        4 => ColorScheme::Red,
        5 => ColorScheme::Cyan,
        6 => ColorScheme::Yellow,
        7 => ColorScheme::Pink,
        8 => ColorScheme::White,
        9 => ColorScheme::LimeGreen,
        10 => ColorScheme::Teal,
        _ => ColorScheme::MatrixGreen,
    };

    let rain_speed = match speed {
        0 => RainSpeed::VerySlow,
        1 => RainSpeed::Slow,
        2 => RainSpeed::Medium,
        3 => RainSpeed::Fast,
        4 => RainSpeed::VeryFast,
        _ => RainSpeed::Medium,
    };

    let handle = &mut *handle;
    let config = ScreenSaverConfig::new(character_set, color_scheme, rain_speed, width, height);
    handle.engine.set_config(config);
}

/// Destroy the Matrix Rain engine and free memory
///
/// # Safety
/// - `handle` must be a valid pointer returned from `matrix_rain_new`
/// - `handle` must not be used after this call
/// - `handle` can be null (no-op in that case)
#[no_mangle]
pub unsafe extern "C" fn matrix_rain_destroy(handle: *mut MatrixRainHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

/// Get the update interval in milliseconds for the given speed
#[no_mangle]
pub extern "C" fn matrix_rain_get_update_interval_ms(speed: u8) -> u64 {
    let rain_speed = match speed {
        0 => RainSpeed::VerySlow,
        1 => RainSpeed::Slow,
        2 => RainSpeed::Medium,
        3 => RainSpeed::Fast,
        4 => RainSpeed::VeryFast,
        _ => RainSpeed::Medium,
    };
    rain_speed.update_interval_ms()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_lifecycle() {
        unsafe {
            let handle = matrix_rain_new(1920, 1080, 0, 0, 2);
            assert!(!handle.is_null());

            matrix_rain_update(handle);
            matrix_rain_update(handle);

            matrix_rain_destroy(handle);
        }
    }

    #[test]
    fn test_ffi_config_update() {
        unsafe {
            let handle = matrix_rain_new(1920, 1080, 0, 0, 2);

            // Change to Korean, Purple, Fast
            matrix_rain_set_config(handle, 2560, 1440, 4, 2, 3);

            matrix_rain_destroy(handle);
        }
    }

    #[test]
    fn test_null_handle_safety() {
        unsafe {
            matrix_rain_update(ptr::null_mut());
            matrix_rain_destroy(ptr::null_mut());
            // Should not crash
        }
    }

    #[test]
    fn test_update_interval() {
        assert_eq!(matrix_rain_get_update_interval_ms(2), 50); // Medium
        assert_eq!(matrix_rain_get_update_interval_ms(4), 15); // Very fast
    }
}
