//
//  BridgingHeader.h
//  MatrixRainSaver
//
//  Bridge between Swift and Rust FFI
//

#ifndef BridgingHeader_h
#define BridgingHeader_h

#include <stdint.h>
#include <stddef.h>

// Opaque type for Rust engine
typedef struct MatrixRainHandle MatrixRainHandle;

// FFI structure for render data
typedef struct {
    uint32_t character;  // Unicode codepoint
    float x;
    float y;
    uint8_t r;
    uint8_t g;
    uint8_t b;
    float a;
    float font_size;
} RenderCharFFI;

// Lifecycle functions
MatrixRainHandle* matrix_rain_new(uint32_t width, uint32_t height,
                                   uint8_t charset, uint8_t color, uint8_t speed);
void matrix_rain_update(MatrixRainHandle* handle);
void matrix_rain_destroy(MatrixRainHandle* handle);

// Configuration
void matrix_rain_set_config(MatrixRainHandle* handle, uint32_t width, uint32_t height,
                             uint8_t charset, uint8_t color, uint8_t speed);

// Rendering
const RenderCharFFI* matrix_rain_get_render_chars(MatrixRainHandle* handle, size_t* out_count);

// Utilities
uint64_t matrix_rain_get_update_interval_ms(uint8_t speed);

#endif /* BridgingHeader_h */
