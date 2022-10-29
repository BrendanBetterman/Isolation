use gdnative::{prelude::*, api::{RichTextLabel}};
/// The Player "class"
#[derive(NativeClass)]
#[inherit(RichTextLabel)]
pub struct Use{
    toggle: bool,
}
impl Use{
    fn new(_owner:&RichTextLabel)->Self{
        Use{
            toggle: false,
        }
    }
}

#[methods]
impl Use{
    ///Checks the player info.
    fn check_player_info(&mut self,node : &Node){
        self.can_use(node);    
    }
    ///When Given a Node it calls the can_use function to get the boolean
    fn can_use(&mut self,node : &Node){
        if node.has_method("_can_use"){
            unsafe{
                let test = node.call("_can_use", &[]);
                self.toggle = Variant::to_string(&test) == "True";
            }
        }
    }
    #[method]
    fn _physics_process(&mut self, #[base]owner:&RichTextLabel , _delta:f32){
        //let input = Input::godot_singleton();
        //Unsafe because rust doesn't know if it can get the object at runtime.
        //gets the player so it can call the _can_use function so it may display itself.
        unsafe{
            let tmp = Node::get_node(&owner,NodePath::from_str("/root/Spatial/KinematicBody"));
            match tmp{
                Some(x)=>  self.check_player_info(&x.clone().assume_unique()) ,
                None=>panic!("Failed"),
            }
        }
        if self.toggle{
            owner.set_percent_visible(1.0);
        }else{
            owner.set_percent_visible(0.0);
        }
    }
}