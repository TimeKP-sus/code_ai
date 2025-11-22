pub mod chatbot;
// pub mod compile_py;
mod main_node;
pub mod pythonrun;
mod thanh_menu;
mod viet_code;
pub mod data;

// mod db_mo_dau;



use godot::prelude::*;



pub struct RustExtension;


#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
