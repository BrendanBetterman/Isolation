use std::clone;

use gdnative::prelude::*;
use gdnative::api::{MeshInstance, InputEventMouse, InputEventMouseMotion, Area};
use gdnative::export::hint::{EnumHint, IntHint, StringHint};
/// The Player "class"
#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody)]
#[register_with(register_members)]
pub struct Player{
    sensitivity: f32,
    position: Vector3,
    rotation: Vector3,
    move_speed: f32,
    crouched: bool,
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
    fn new( _owner: &KinematicBody) -> Self {
        Player{
            sensitivity: -12.5,
            position: Vector3::new(0.0,0.0,0.0),
            rotation: Vector3::new(0.0,0.0,0.0),
            move_speed: 1.0,
            crouched: false,
        }
    }
}
fn lerp(start:f32,end:f32,time:f32)->f32{
    return start+(end-start)*time;
}
// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl Player {
    #[method]
    fn _ready(&mut self, #[base]owner: &KinematicBody) {
        owner.set_physics_process(true);
        self.position = owner.translation();
        self.rotation = owner.rotation();
        
        unsafe{
            
            match owner.get_child(0){
                Some(x)=>match x.clone().assume_unique().get_child(0){
                    //get the camera parent
                    Some(y)=>godot_print!("{}",y.clone().assume_unique().name()),
                    None=>godot_print!("Failed"),
                },
                None=>godot_print!("failed"),
            };
        }
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.s
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _on_bed_entered(&self,#[base]owner: &KinematicBody, body: Ref<KinematicBody>){
        unsafe{
            godot_print!("collided with {}", body.clone().assume_unique().name());
        }
    }
    
    #[method]
    fn _physics_process(&mut self,#[base]owner: &KinematicBody, delta: f64) {
        //mouse movement system- unsafe due to undetermined viewport
        
        unsafe{
            let view = match owner.get_viewport(){
                Some(x) =>x.clone(),
                None => panic!("Couldn't get viewport"),
            };  
            //godot_print!("{}",);
            let mouse_x = view.assume_unique().get_mouse_position().x/view.assume_unique().size().x-0.5;
            let mut mouse_y = view.assume_unique().get_mouse_position().y/view.assume_unique().size().y-0.5;
            if mouse_y > 0.06{
                mouse_y = 0.06;
            }else if mouse_y < -0.1{
                mouse_y = -0.1;
            }
            self.rotation = Vector3::new(0.0,self.sensitivity * mouse_x,self.sensitivity * mouse_y);
            owner.set_rotation(self.rotation);
            //godot_print!("{}",mouse_y);
        }
        let input = Input::godot_singleton();
        //godot_print!("{}", input.get_last_mouse_speed().x);
       
        //godot_print!("{}",mouse.global_position().x);
        //owner.rotate_y(input.get_last_mouse_speed().x as f64 * delta);
        //owner.set_rotation(Vector3::new(m.0 as f32,0.0,0.0));
        if Input::is_action_pressed(input, "ui_up", false) {
            self.position.z -= self.rotation.y.sin()*self.move_speed * delta as f32;
            self.position.x -= (self.rotation.y + 3.14).cos()*self.move_speed*delta as f32;
           
        }
        if Input::is_action_pressed(input, "ui_down", false) {
            self.position.z += self.rotation.y.sin()*self.move_speed*delta as f32;
            self.position.x += (self.rotation.y + 3.14).cos()*self.move_speed*delta as f32;
            
        }
        if Input::is_action_pressed(input, "ui_left", false) {
            self.position.z -= (self.rotation.y + 1.57).sin()*self.move_speed*delta as f32;
            self.position.x -= (self.rotation.y + 4.71).cos()*self.move_speed*delta as f32;
            
        }
        if Input::is_action_pressed(input, "ui_right", false) {
            self.position.z += (self.rotation.y + 1.57).sin()*self.move_speed*delta as f32;
            self.position.x += (self.rotation.y + 4.71).cos()*self.move_speed*delta as f32;   
        }
        if Input::is_action_pressed(input, "ui_q", false) {
            self.position.y = lerp(self.position.y, 0.90,0.09);
        }else{
            self.position.y = lerp(self.position.y, 1.48,0.09);
        }
        owner.set_translation(self.position);
        
        /*
        if Input::is_action_pressed(input, "ui_e", false) {
            owner.rotate_y(-0.5 * delta);
        }
        */
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    

}