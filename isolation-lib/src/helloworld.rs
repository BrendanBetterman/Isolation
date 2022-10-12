use core::panicking::panic;

use gdnative::{prelude::*, api::viewport};
/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;
impl HelloWorld {
    /// The "constructor" of the class.
    fn new(_base: &Node) -> Self {
        HelloWorld
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    #[method]
    fn _ready(&self, #[base] base: &Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _process(&mut self, #[base] base: &Node, _delta: f32) {
        let tmp = match base.get_viewport().clone(){
            None=> panic!("break"),
            Some(x) => godot_print!("{}",x.get),
        };
        
        
        let input = Input::godot_singleton();
        if Input::is_action_pressed(input, "ui_up", false) {
           godot_print!("key pressed");
        }
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _physics_process(&mut self, #[base] _base: &Node, _delta: f32) {
        
        //godot_print!("Hello world from node {}!", base.to_string());
    }

}