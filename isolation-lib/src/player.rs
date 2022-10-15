use gdnative::api::MeshInstance;
use gdnative::prelude::*;
use gdnative::export::hint::{EnumHint, IntHint, StringHint};
/// The Player "class"
#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody)]
#[register_with(register_members)]
pub struct Player{
    sensitivity: f32,
    position: Vector3,
    rotation: Vector3,
    velocity: Vector3,
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
            velocity: Vector3::new(0.0,0.0,0.0),
            move_speed: 2.0,
            crouched: false,
        }
    }
}
fn lerp(start:f32,end:f32,time:f32)->f32{
    let mut out = start+(end-start)*time; 
    //so end can be achieved
    if end > start{
        if out > end - 0.01{
            out = end;
        }
    }else{
        if out < end + 0.01{
            out = end;
        }
    }
    return out;
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
        
        /*unsafe{
            
            match owner.get_child(0){
                Some(x)=>match x.clone().assume_unique().get_child(0){
                    //get the camera parent
                    Some(y)=>godot_print!("{}",y.clone().assume_unique().name()),
                    None=>godot_print!("Failed"),
                },
                None=>godot_print!("failed"),
            };
        }*/
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.s
        //godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _on_collision(&mut self,#[base]owner: &KinematicBody, body: Ref<Node>){
        self.position.x -= self.velocity.x*2.0* if self.crouched {self.move_speed/2.0}else{self.move_speed};
        self.position.z -= self.velocity.z*2.0* if self.crouched {self.move_speed/2.0}else{self.move_speed};
        
        owner.set_translation(self.position);
        self.velocity = Vector3::new(0.0,0.0,0.0);
        unsafe{
            godot_print!("hovered with {}", body.clone().assume_unique().name());
        }
    }
    #[method]
    fn _on_mouse_hover(&self,#[base]_owner: &KinematicBody){
        //godot_print!("hit");
        //unsafe{
            //godot_print!("hovered with {}", body.clone().assume_unique().name());
        //}
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
        
        let mut new_velocity = Vector3::new(0.0,0.0,0.0);
        if Input::is_action_pressed(input, "ui_up", false) {
            new_velocity.z -= self.rotation.y.sin();
            new_velocity.x -= (self.rotation.y + 3.141592).cos();
        }else if Input::is_action_pressed(input, "ui_down", false) {
            new_velocity.z += self.rotation.y.sin();
            new_velocity.x += (self.rotation.y + 3.141592).cos();
        }
        if Input::is_action_pressed(input, "ui_left", false) {
            new_velocity.z -= (self.rotation.y+1.570796).sin();
            new_velocity.x -= (self.rotation.y + 4.712388).cos();
            
        }else if Input::is_action_pressed(input, "ui_right", false) {
            new_velocity.z += (self.rotation.y+1.570796).sin();
            new_velocity.x += (self.rotation.y + 4.712388).cos();
        }
        self.velocity.x = lerp(self.velocity.x,new_velocity.x* if self.crouched {self.move_speed/2.0}else{self.move_speed} * delta as f32,0.09);
        self.velocity.z = lerp(self.velocity.z,new_velocity.z*if self.crouched {self.move_speed/2.0}else{self.move_speed} * delta as f32,0.09);
        
        //self.position.x += self.velocity.x;
        //self.position.z += self.velocity.z;
        
        owner.move_and_collide(self.velocity, false, true, false);
        self.position = owner.translation();
        if Input::is_action_pressed(input, "ui_q", false) {
            self.position.y = lerp(self.position.y, 0.90,0.09);
            self.crouched = true;
        }else{
            self.position.y = lerp(self.position.y, 1.48,0.09);
            self.crouched = false;
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