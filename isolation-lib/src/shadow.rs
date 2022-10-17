use gdnative::{prelude::*, api::MeshInstance};

/// The Player "class"
#[derive(NativeClass)]
#[inherit(MeshInstance)]
pub struct Shadow{
    position: Vector3,
}
impl Shadow{
    fn new(owner:&MeshInstance)->Self{
        Shadow{
            position: owner.translation(),
        }
        
    }
}
#[methods]
impl Shadow{
    #[method]
    fn _physics_process(&mut self, #[base]owner:&MeshInstance , delta:f32){
        let input = Input::godot_singleton();
        if Input::is_action_pressed(input, "ui_accept", false){
            self.position.z -= 1.0 * delta;
            owner.set_translation(self.position);
        }
    }
}