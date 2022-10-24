

use gdnative::{prelude::*};
use gdnative::api::{ClippedCamera };
/// The Camera "class"
#[derive(NativeClass)]
#[inherit(ClippedCamera)]
pub struct Pcamera;
impl Pcamera {
    /// The "constructor" of the class.
    fn new(_base: &ClippedCamera) -> Self {
        Pcamera
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl Pcamera {
    
    #[method]
    fn _ready(&self, #[base] _base: &ClippedCamera) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _process(&mut self, #[base] _base: &ClippedCamera, _delta: f32) {
        /* 
        unsafe{
            let view = match base.get_viewport(){
                Some(x) =>x.clone(),
                None => panic!("Couldn't get viewport"),
            };  
            godot_print!("{}",view.assume_unique().get_mouse_position().x);
        }
        */
         
        let input = Input::godot_singleton();
        if Input::is_action_pressed(input, "ui_up", false) {
          // godot_print!("key pressed");
        }
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _physics_process(&mut self, #[base] _owner: &ClippedCamera, _delta: f32) {
        
        //godot_print!("x{},y{},z{}",base.project_ray_origin(Vector2::new(512.0,300.0)).x,base.project_ray_origin(Vector2::new(512.0,300.0)).y,base.project_ray_origin(Vector2::new(512.0,300.0)).z);
        //godot_print!("Hello world from node {}!", base.to_string());
        
    }

}