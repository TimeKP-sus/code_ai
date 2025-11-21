pub mod chatbot;
// pub mod compile_py;
mod main_node;
pub mod pythonrun;
mod thanh_menu;
mod viet_code;

use godot::prelude::*;



pub struct RustExtension;


#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
