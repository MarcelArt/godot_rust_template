use godot::prelude::*;

// Import modules here
mod rust_example;

struct GdRust;

#[gdextension]
unsafe impl ExtensionLibrary for GdRust {}
