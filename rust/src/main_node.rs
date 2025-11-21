// use crate::chatbot::hoi_dap;
use godot::{
    classes::{Control, IControl},
    obj::Base,
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainNode {
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl IControl for MainNode {
    fn ready(&mut self) {
        godot::global::godot_print!("MainNode is ready!");

        // hoi_dap("give me answer only yes or no: is rust a programming language?");
    }
}
