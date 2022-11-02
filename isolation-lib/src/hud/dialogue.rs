use gdnative::{prelude::*, api::{RichTextLabel}};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// The Player "class"
#[derive(NativeClass)]
#[inherit(RichTextLabel)]
pub struct Dialogue{
    toggle: bool,
    id: usize,
    lang: Vec<String>,
    options: u32,
}
//Non Godot functions
impl Dialogue{
    fn new(_owner:&RichTextLabel)->Self{
        Dialogue{
            toggle: true,
            id: 0,
            lang: Vec::new(),
            options: 0,
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
                self.id = Variant::to_string(&test).parse::<usize>().unwrap();
            }
        }
    }
}
fn read_file_line_by_line(filepath: &str,dialogue: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        //println!("{}", line?);
        dialogue.push(line?);
    }
    Ok(())
}
//functions godot can call
#[methods]
impl Dialogue{
    #[method]
    fn _ready(&mut self, #[base]_owner : &RichTextLabel){
        //let mut langs:Vec<String> = Vec::new();
        let _o = read_file_line_by_line("./lang.lang",&mut self.lang);
        
        //read_file_line_by_line(langs[self.options]+ )
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
        let mut bbcode = "[center][color=white][".to_owned();
        bbcode.push_str(&self.lang[self.id]);
        let end = "][/color]".to_string();
        bbcode.push_str(&end);
        owner.set_bbcode(&bbcode);
        let input = Input::godot_singleton();
        if Input::is_action_just_pressed(input, "ui_accept", false) {
            if self.id < self.lang.capacity()-1{
                self.id += 1;
            }else{
                self.id = 0;
            }
        }
        if self.toggle{
            owner.set_percent_visible(1.0);
        }else{
            owner.set_percent_visible(0.0);
        }
    }
}