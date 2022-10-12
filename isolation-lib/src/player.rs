use gdnative::prelude::*;
use gdnative::api::{MeshInstance, InputEventMouse, InputEventMouseMotion};
use gdnative::export::hint::{EnumHint, IntHint, StringHint};
/// The Player "class"
#[derive(gdnative::derive::NativeClass)]
#[inherit(MeshInstance)]
#[register_with(register_members)]
pub struct Player{
    start: Vector3,
    time: f32,
}
fn register_members(builder: &ClassBuilder<Player>) {
    builder
        .property::<String>("test/test_enum")
        .with_hint(StringHint::Enum(EnumHint::new(vec![
            "Hello".into(),
            "World".into(),
            "Testing".into(),
        ])))
        .with_getter(|_: &Player, _| "Hello".to_string())
        .done();

    builder
        .property("test/test_flags")
        .with_hint(IntHint::Flags(EnumHint::new(vec![
            "A".into(),
            "B".into(),
            "C".into(),
            "D".into(),
        ])))
        .with_getter(|_: &Player, _| 0)
        .done();
}
impl Player {
    /// The "constructor" of the class.
    fn new( _owner: &MeshInstance) -> Self {
        Player{
            start: Vector3::new(0.1,0.1,0.1),
            time: 0.0,
        }
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl Player {
    #[method]
    fn _ready(&mut self, #[base]owner: &MeshInstance) {
        owner.set_physics_process(true);
        
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.s
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _physics_process(&mut self,#[base]owner: &MeshInstance, delta: f64) {
     
        let input = Input::godot_singleton();
        //godot_print!("{}", input.get_last_mouse_speed().x);
        let mouse = InputEventMouseMotion::new();
        godot_print!("{}",mouse.global_position().x);
        //owner.rotate_y(input.get_last_mouse_speed().x as f64 * delta);
        //owner.set_rotation(Vector3::new(m.0 as f32,0.0,0.0));
        if Input::is_action_pressed(input, "ui_up", false) {
            //godot_print!("key pressed");
            
            //self.Transform = Vector3::new(0.0, 1.0, 0.0) * self.time.cos() * 0.5;
            //let trans = Transform::translated(&self, offset);
            owner.translate(Vector3::new(0.5* delta as f32,0.0,0.0));
            
            //owner.set_translation(self.start);
        }
        if Input::is_action_pressed(input, "ui_down", false) {
            owner.translate(Vector3::new(-0.5* delta as f32,0.0,0.0));
        }
        if Input::is_action_pressed(input, "ui_left", false) {
            owner.translate(Vector3::new(0.0,0.0,-0.5* delta as f32));
        }
        if Input::is_action_pressed(input, "ui_right", false) {
            owner.translate(Vector3::new(0.0,0.0,0.5* delta as f32));
        }
        if Input::is_action_pressed(input, "ui_q", false) {
            owner.rotate_y(0.5 * delta);
        }
        if Input::is_action_pressed(input, "ui_e", false) {
            owner.rotate_y(-0.5 * delta);
        }
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    

}