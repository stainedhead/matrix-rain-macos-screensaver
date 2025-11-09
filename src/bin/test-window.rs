//! Matrix Rain Test Window
//!
//! A standalone windowed application for testing the Matrix rain screensaver
//! Uses macOS Cocoa framework to create a native window with the Matrix effect

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivationOptions, NSApplicationActivationPolicyRegular,
    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSRunningApplication, NSWindow,
    NSWindowStyleMask,
};
use cocoa::base::{id, nil, selector, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use core_graphics::base::CGFloat;
use matrix_rain_core::*;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Shared state for the matrix engine
struct MatrixState {
    engine: MatrixRain,
    last_update: Instant,
    update_interval: Duration,
}

impl MatrixState {
    fn new(width: u32, height: u32) -> Self {
        let config = ScreenSaverConfig::new(
            CharacterSet::Japanese,
            ColorScheme::MatrixGreen,
            RainSpeed::Medium,
            width,
            height,
        );
        let engine = MatrixRain::new(config);
        let update_interval = Duration::from_millis(RainSpeed::Medium.update_interval_ms());

        Self {
            engine,
            last_update: Instant::now(),
            update_interval,
        }
    }

    fn update_if_needed(&mut self) -> bool {
        if self.last_update.elapsed() >= self.update_interval {
            self.engine.update();
            self.last_update = Instant::now();
            true
        } else {
            false
        }
    }

    fn get_render_chars(&self) -> Vec<rendering::RenderChar> {
        self.engine.get_render_data()
    }
}

// Create a custom NSView subclass for rendering
fn create_matrix_view_class() -> *const Class {
    let superclass = class!(NSView);
    let mut decl = ClassDecl::new("MatrixTestView", superclass).unwrap();

    // Add ivar to store the matrix state
    decl.add_ivar::<*mut std::ffi::c_void>("_matrixState");

    // Override drawRect:
    extern "C" fn draw_rect(this: &Object, _cmd: Sel, _dirty_rect: NSRect) {
        unsafe {
            // Get the matrix state
            let state_ptr: *mut std::ffi::c_void = *this.get_ivar("_matrixState");
            if state_ptr.is_null() {
                return;
            }
            let state = &mut *(state_ptr as *mut Arc<Mutex<MatrixState>>);
            let mut state_guard = state.lock().unwrap();

            // Update engine if needed
            state_guard.update_if_needed();

            // Fill background with black
            let black: id = msg_send![class!(NSColor), blackColor];
            let _: () = msg_send![black, setFill];
            let bounds: NSRect = msg_send![this, bounds];
            let _: () = msg_send![class!(NSBezierPath), fillRect: bounds];

            // Get render characters
            let chars = state_guard.get_render_chars();

            // Create font
            let font_size: CGFloat = 16.0;
            let font: id = msg_send![class!(NSFont), monospacedSystemFontOfSize:font_size weight:0.0];

            // Draw each character
            for render_char in chars.iter() {
                // Convert character to NSString
                let char_string = render_char.character.to_string();
                let ns_string: id = NSString::alloc(nil).init_str(&char_string);

                // Create color
                let color: id = msg_send![
                    class!(NSColor),
                    colorWithRed: render_char.color.r as CGFloat / 255.0
                    green: render_char.color.g as CGFloat / 255.0
                    blue: render_char.color.b as CGFloat / 255.0
                    alpha: render_char.color.a as CGFloat / 255.0
                ];

                // Create attributes dictionary
                let font_key: id = msg_send![class!(NSString), alloc];
                let font_key: id = msg_send![font_key, initWithUTF8String: "NSFont\0".as_ptr()];
                let color_key: id = msg_send![class!(NSString), alloc];
                let color_key: id = msg_send![color_key, initWithUTF8String: "NSColor\0".as_ptr()];

                let dict: id = msg_send![class!(NSMutableDictionary), dictionary];
                let _: () = msg_send![dict, setObject:font forKey:font_key];
                let _: () = msg_send![dict, setObject:color forKey:color_key];

                // Draw the string
                let point = NSPoint::new(render_char.x as f64, render_char.y as f64);
                let _: () = msg_send![ns_string, drawAtPoint:point withAttributes:dict];

                // Release
                let _: () = msg_send![ns_string, release];
                let _: () = msg_send![font_key, release];
                let _: () = msg_send![color_key, release];
            }
        }
    }

    unsafe {
        decl.add_method(
            sel!(drawRect:),
            draw_rect as extern "C" fn(&Object, Sel, NSRect),
        );
    }

    // Override isOpaque (return YES for better performance)
    extern "C" fn is_opaque(_this: &Object, _cmd: Sel) -> objc::runtime::BOOL {
        YES
    }

    unsafe {
        decl.add_method(sel!(isOpaque), is_opaque as extern "C" fn(&Object, Sel) -> objc::runtime::BOOL);
    }

    decl.register()
}

fn main() {
    unsafe {
        // Create autorelease pool
        let _pool = NSAutoreleasePool::new(nil);

        // Create application
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // Create menu bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // Create app menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_title = NSString::alloc(nil).init_str("Quit Matrix Rain Test");
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // Create window
        let window_width = 1280.0;
        let window_height = 800.0;
        let window_rect = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(window_width, window_height),
        );

        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                window_rect,
                NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSMiniaturizableWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();

        let title = NSString::alloc(nil).init_str("Matrix Rain Test");
        window.setTitle_(title);
        window.center();

        // Create custom view
        let view_class = create_matrix_view_class();
        let view: id = msg_send![view_class, alloc];
        let view: id = msg_send![view, initWithFrame: window_rect];

        // Create and store matrix state
        let state = Arc::new(Mutex::new(MatrixState::new(
            window_width as u32,
            window_height as u32,
        )));
        let state_ptr = Box::into_raw(Box::new(state)) as *mut std::ffi::c_void;
        (*view).set_ivar("_matrixState", state_ptr);

        window.setContentView_(view);

        // Start animation timer
        let timer_interval: f64 = 1.0 / 60.0; // 60 FPS
        let timer: id = msg_send![
            class!(NSTimer),
            scheduledTimerWithTimeInterval: timer_interval
            target: view
            selector: selector("setNeedsDisplay:")
            userInfo: nil
            repeats: YES
        ];

        // Make window key and order front
        window.makeKeyAndOrderFront_(nil);

        // Activate app
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivationOptions::NSApplicationActivateIgnoringOtherApps);

        // Run the application
        app.run();

        // Cleanup (won't be reached until app quits)
        let _: () = msg_send![timer, invalidate];
    }
}
