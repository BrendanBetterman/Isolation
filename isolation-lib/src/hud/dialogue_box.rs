use gdnative::{prelude::*, api::{Panel}};

/// The Player "class"
#[derive(NativeClass)]
#[inherit(Panel)]
pub struct DialogueBox{
    toggle: bool,
}
//Non Godot functions
impl DialogueBox{
    fn new(_owner:&Panel)->Self{
        DialogueBox{
            toggle: true,
        }
    }
    ///Checks the player info.
    fn check_player_info(&mut self,node : &Node){
        self.get_dialogue(node);    
    }
    ///When Given a Node it calls the can_use function to get the boolean
    fn get_dialogue(&mut self,node : &Node){
        if node.has_method("_get_dialogue"){
            unsafe{
                let test = node.call("_get_dialogue", &[]);
                //self.toggle = Variant::to_string(&test) == "True";
                let id = Variant::to_string(&test).parse::<i32>().unwrap();
                if id == -1{
                    self.toggle = false;
                }else{
                    self.toggle = true;
                }
            }
        }
    }
}
//functions godot can call
#[methods]
impl DialogueBox{
    #[method]
    fn _ready(&mut self, #[base]_owner : &Panel){
        //let mut langs:Vec<String> = Vec::new();
        
        //read_file_line_by_line(langs[self.options]+ )
    }
    #[method]
    fn _physics_process(&mut self, #[base]owner:&Panel , _delta:f32){
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
        owner.set_visible(self.toggle);
        
    }
}