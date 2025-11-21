use godot::{
    classes::{Control, IControl},
    global::godot_print,
    obj::Base,
    prelude::{GodotClass, godot_api},
};

#[derive(GodotClass)]
#[class(base = Control, init)]
pub struct ThanhMenu {
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl IControl for ThanhMenu {
    fn ready(&mut self) {
        godot_print!("ThanhMenu is ready!");
        godot_print!("hi");
        godot_print!("dsa")
    }
}
