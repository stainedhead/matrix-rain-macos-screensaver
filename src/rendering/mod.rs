//! Rendering module for drawing the Matrix rain effect

mod color;
mod renderer;

#[cfg(feature = "cli")]
pub mod terminal;

pub use color::Color;
pub use renderer::{RenderChar, Renderer};

#[cfg(feature = "cli")]
pub use terminal::TerminalRenderer;
