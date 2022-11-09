
use gdnative::api::{RayCast, MeshInstance};
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
    pickupable: bool,
    item: String,
    dialogue: i32,
    mission: Missions,
    mouse_confined: bool,
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
            pickupable: false,
            item: "".to_string(),
            dialogue: 0,
            mission: Missions::new(),
            mouse_confined: true,
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
        return self.pickupable;
    }
    #[method]
    fn _get_dialogue(&self,#[base] _owner: &KinematicBody)->i32{
        return self.dialogue;
    }
    ///Handles the inputs to move the player
    fn movement(&self,input:&Input)-> Vector3{
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
        return new_velocity;
    }
    ///Lerped crouch went input is given
    fn crouch(&mut self,input:&Input){
        //crouch 
        if Input::is_action_pressed(input, "ui_q", false) {
            self.position.y = lerp(self.position.y, 0.90,0.09);
            self.crouched = true;
        }else{
            self.position.y = lerp(self.position.y, 1.48,0.09);
            self.crouched = false;
        }
    }
    fn quest_loop(&mut self,input:&Input,owner:&KinematicBody){
        self.mission.look(&self.item);
        
        self.dialogue = self.mission.dialogue;
            
        
        if Input::is_action_pressed(input, "ui_use", false){
            self.mission.on_used(&self.item);
        }
        if self.mission.should_tp{
            
            self.unhide_obj(owner, "../Hideables/Chrismas");
            
            self.mission.should_tp = false;
            self.position = self.mission.location;
            
        }
    }
    fn quest_can_pickup(&mut self){
        self.pickupable = self.mission.can_pickup(&self.item);
    }
    ///Needs owner for viewport, sets rotation of player using mouse position.
    fn mouse_rotation(&mut self,owner:&KinematicBody,input:&Input){
        //mouse movement system- unsafe due to undetermined viewport
        unsafe{
            let view = match owner.get_viewport(){
                Some(x) =>x.clone(),
                None => panic!("Couldn't get viewport"),
            };  
            
            let mut mouse_x = view.assume_unique().get_mouse_position().x/view.assume_unique().size().x-0.5;
            let mut mouse_y = view.assume_unique().get_mouse_position().y/view.assume_unique().size().y-0.5;
            //godot_print!("{:?}",view.assume_unique().get_mouse_position());
            if mouse_y > 0.06{
                mouse_y = 0.06;
            }else if mouse_y < -0.1{
                mouse_y = -0.1;
            }
            if mouse_x < -0.25{
                let mouse = Vector2::new((mouse_x+1.0)*view.assume_unique().size().x,view.assume_unique().get_mouse_position().y);//
                input.warp_mouse_position(mouse);
                mouse_x +=0.5;
            }else if mouse_x > 0.25{
                let mouse = Vector2::new((mouse_x)*view.assume_unique().size().x,view.assume_unique().get_mouse_position().y);
                input.warp_mouse_position(mouse);
                mouse_x -=0.5;
            }
            
            self.rotation = Vector3::new(0.0,self.sensitivity * mouse_x,self.sensitivity * mouse_y);
        }
    }
    ///Needs owner to get the raycast grandchild object.
    ///Then it gets the name of the obj its looking at and sets it self.item
    fn raycast_item(&mut self,owner:&KinematicBody){
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
            self.item = item; 
        }
    }
    
    fn unhide_obj(&self,owner:&KinematicBody,name:&str){
        unsafe{
            let tmp = Node::get_node(&owner,NodePath::from_str(name));
            let obj = match tmp{
                Some(x)=>  x.assume_unique(),
                None=> Node::new(),
            };
            
            match obj.cast::<gdnative::api::Spatial>(){
                Some(x) => x.show(),
                None => godot_error!("Fail"),
            };
        }
    }
    //End Of Hover Functions
    #[method]
    fn _physics_process(&mut self,#[base]owner: &KinematicBody, delta: f64) {
        let input = Input::godot_singleton();
        //enable/ disable mouse interaction with camera rotation
        if Input::is_action_just_pressed(input,"ui_cancel", false){
            self.mouse_confined = !self.mouse_confined;
            if self.mouse_confined{
                let m = Vector2::new((self.rotation.y/self.sensitivity +0.5)*1280.0,(self.rotation.z/self.sensitivity+0.5)*720.0 );
                input.warp_mouse_position(m);
            }
            //godot_print!("{}",self.mouse_confined);

            
        }
        //if game is paused
        if self.mouse_confined{
            //hide and lock mouse to screen
            input.set_mouse_mode(Input::MOUSE_MODE_CONFINED);
            input.set_mouse_mode(Input::MOUSE_MODE_HIDDEN);
            //get raycast item
            self.raycast_item(owner);
            //mouse Movement
            self.mouse_rotation(owner,input);
            owner.set_rotation(self.rotation);
            //Movement
            let mut _new_velocity = self.movement(input);
            self.velocity.x = lerp(self.velocity.x,_new_velocity.x* if self.crouched {self.move_speed/2.0}else{self.move_speed} * delta as f32,0.09);
            self.velocity.z = lerp(self.velocity.z,_new_velocity.z*if self.crouched {self.move_speed/2.0}else{self.move_speed} * delta as f32,0.09);
            owner.move_and_collide(self.velocity, false, true, false);
            self.position = owner.translation();
            
            self.crouch(input);
            
            self.quest_loop(input,owner);
            self.quest_can_pickup();
            owner.set_translation(self.position);//if quest requires a TP also handles crouch pos
        }else{
            input.set_mouse_mode(Input::MOUSE_MODE_VISIBLE);
        }
    }
    

}