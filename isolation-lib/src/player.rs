use gdnative::prelude::*;
use gdnative::api::MeshInstance;
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
            start: Vector3::new(0.0,0.0,0.0),
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
        if Input::is_action_pressed(input, "ui_up", false) {
            //godot_print!("key pressed");
            self.time += delta as f32;
            //self.Transform = Vector3::new(0.0, 1.0, 0.0) * self.time.cos() * 0.5;
            //let trans = Transform::translated(&self, offset);
            owner.rotate_y(0.01);
            owner.set_translation(self.start);
        }
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    

}