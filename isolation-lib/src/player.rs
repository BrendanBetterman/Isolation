
use gdnative::api::RayCast;
use gdnative::prelude::*;

use self::missions::*;
mod missions;
/// The Player "class"
#[derive(gdnative::derive::NativeClass)]
#[inherit(KinematicBody)]

pub struct Player{
    sensitivity: f32,
    position: Vector3,
    rotation: Vector3,
    velocity: Vector3,
    move_speed: f32,
    crouched: bool,
    key: [bool;4],
    dialogue: usize,
    mission: Missions,
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
            key: [false;4],
            dialogue: 0,
            mission: Missions::new(),
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
///Takes raycast obj and outputs the name of the obj its colliding with.
fn cast_ray(ref ray : &gdnative::prelude::Ref<RayCast, gdnative::prelude::Unique>) ->String{
    ray.force_raycast_update();
    //godot_print!("{}",ray.get_collision_point().x);
    
    let var = ray.get_collider().to_variant().to_string();
    let split = var.clone();
    let mut sp = split.split(":");
    let output = match sp.next(){
        Some(x) => x,
        None => "None",
    };
    
    return output.to_string();
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
        let node = Node::get_path(owner);
        godot_print!("{:?}",node);
        //create Missions
        self.mission = Missions::new();

    }
   
    #[method]
    fn _can_use(&self,#[base]_owner: &KinematicBody)->bool{
        let mut can = false;
        for i in 0..self.key.len(){
            if self.key[i]{
                can = true;
            }
        }
        return can;
    }
    #[method]
    fn _get_dialogue(&self,#[base] _owner: &KinematicBody)->usize{
        return self.dialogue;
    }
    //End Of Hover Functions
    #[method]
    fn _physics_process(&mut self,#[base]owner: &KinematicBody, delta: f64) {
        //mouse movement system- unsafe due to undetermined viewport
        unsafe{
            //gets the child of current object as a refernce so you can modify it.
            let node = match owner.get_child(0){//
                Some(x) => x.assume_unique(),
                None => Node::new(),
            };
            //gets the child of the child which is the ray cast obj.
            let raycast = match node.get_child(0){
                Some(x) => x.assume_unique(),
                None => Node::new(),
            };
            //cast the node grandchild to a raycast obj then push to cast_ray function.
            let item = match raycast.cast::<gdnative::api::RayCast>(){//let ray = 
                Some(x) => cast_ray(&x),
                None => "None".to_string(),//RayCast::new(),
            };
            //check what is being hovered over
            match item.as_str(){
                "CarKey"=> self.key[0] = true,
                "CarTrigger"=> self.key[1] = true,
                "Rock"=> self.key[2] = true,
                _ => {for i in 0..self.key.len(){
                    self.key[i] = false;
                };}
            }   
        }
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
        input.set_mouse_mode(Input::MOUSE_MODE_HIDDEN);
        
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
        if Input::is_action_pressed(input, "ui_use", false){
            self.mission.on_used(&self.key);
            //Car Keys
            if self.key[0]{
                self.position.x = -24.0;
                //self.dialogue = 0;
            //Car Locked
            }else if self.key[1]{
                self.dialogue = 1;
            //Rock
            }else if self.key[2]{
                self.dialogue = 2;
            }
        }
        //Look Dialogue
        if self.key[0]{
            self.dialogue = 0;
        }else if self.key[2]{
            self.dialogue = 2;
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