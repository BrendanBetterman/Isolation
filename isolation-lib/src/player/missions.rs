use gdnative::prelude::{Vector3, NodePath,Node};


#[derive(Copy)]
#[derive(Clone)]
pub struct Quest{
    progress: i16,
    toggles: [bool;8],
    complete: bool,
}
impl Quest{
    pub fn new()-> Self{
        Quest{
            progress: 0,
            toggles: [false; 8],
            complete: false,
        }
    }
    #[allow(dead_code)]
    pub fn clone(&self)->Self{
        Quest{
            progress:self.progress,
            toggles:self.toggles,
            complete:self.complete,
        }
    }
    pub fn _add_progress(&mut self){
        self.progress += 1;
    }
}
fn get_id(item: &String)->usize{
    return match item.as_str(){
        "CarKey"=>      1,
        "CarTrigger"=>  2,
        "Rock"=>        3,
        "Fridge"=>      4,
        "Mannequin"=>   5,
        _ => 0
    }
}
pub struct Missions{
    q_id: usize,
    quests: [Quest;2],
    pub should_tp: bool,
    pub location: Vector3,
    pub dialogue: i32,
}
impl Missions{
    pub fn new()->Self{
        Missions {  
            q_id: 0,
            quests: [Quest::new();2],
            should_tp: false,
            location: Vector3::new(0.0,0.0,0.0),
            dialogue: -1,
        }
    }
    
    ///Checks if item has been used
    fn check_toggle(&mut self,id:usize)->bool{
        let output =  self.quests[self.q_id].toggles[id];
        self.quests[self.q_id].toggles[id] = true;
        return !output;
    }
    fn teleport(&mut self,x:f32,y:f32,z:f32){
        self.location = Vector3::new(x,y,z);
        self.should_tp = true;
    }
    pub fn can_pickup(&mut self, item: &String)->bool{
        match self.q_id{
            0 => self.mission_one_can_pickup(&item),
            1 => self.m_ci_2(&item),
            _ => false,
        }
    }
    pub fn on_used(&mut self, item: &String){
        match self.q_id{
            0 => self.mission_one_on_used(&item),
            1 => self.m_on_used_2(&item),
            _ => print!("no mission"),
        }
    }
    pub fn look(&mut self, item: &String){
        match self.q_id{
            0 => self.mission_one_look(&item),
            1 => self.m_look_2(&item),
            _ => print!("no mission"),
        }
    }
    //mission
    fn mission_one_on_used(&mut self,item: &String){
        let id = get_id(item);
        if id == 1 && self.quests[self.q_id].progress <=1{
            //use car keys
            self.quests[self.q_id]._add_progress();
            self.location = Vector3::new(-28.0,0.0,4.0);
            self.should_tp = true;
            self.dialogue = 1;
            
        }else if id == 2 && self.quests[self.q_id].progress == 1{
            //use car
            self.quests[self.q_id]._add_progress();
            self.dialogue = 3;
        }else if id == 3 && self.quests[self.q_id].progress ==2{
            //pick up rock
            self.dialogue = 5;
            self.quests[self.q_id]._add_progress();
        }else if id == 2 && self.quests[self.q_id].progress ==3{
            //use rock on window
            self.dialogue = 6;
            self.quests[self.q_id]._add_progress();
            
        }else if id == 2 && self.quests[self.q_id].progress ==4{
            self.dialogue = -1;
            self.location = Vector3::new(1.0,1.0,1.0);
            self.should_tp = true;
            self.quests[self.q_id].complete = true;
            self.q_id += 1;
        }

    }
    fn mission_one_look(&mut self,item: &String){
        let id = get_id(item);
        if id == 1 && !self.quests[self.q_id].toggles[0]{//keys
            self.dialogue = 0;
            self.quests[self.q_id].toggles[0] = true;
        }else if id == 2  && !self.quests[self.q_id].toggles[1]{//car
            self.dialogue = 2;
            self.quests[self.q_id].toggles[1] = true;
        }else if id == 3 && self.quests[self.q_id].progress >=2 && !self.quests[self.q_id].toggles[2]{//rock
            self.dialogue = 4;
            self.quests[self.q_id].toggles[2] = true;
        }
    }
    fn mission_one_can_pickup(&mut self,item:&String)->bool{
        match get_id(item){
            1=> self.quests[self.q_id].progress == 0,
            2=> self.quests[self.q_id].progress <=4,
            3 => self.quests[self.q_id].progress ==2,
            _ => false,
        }
    }
    
    ///Mission 2 can interact
    fn m_ci_2(&mut self,item:&String)->bool{
        match item.as_str(){
            "Fridge"=> self.quests[self.q_id].progress == 0,
            "Mannequin"=> self.quests[self.q_id].progress <=3,
            "G_Door"=> self.quests[self.q_id].progress <= 4,
            _ => false,
        }
    }
    fn m_look_2(&mut self,item:&String){
        match item.as_str(){
            "Fridge"=> if !self.check_toggle(0) {self.dialogue = 8;},
            "Mannequin"=> if !self.check_toggle(1) {self.dialogue = 10; },
            _ => (),
        }
    }
    fn m_on_used_2(&mut self,item:&String){
        match item.as_str(){
            "Fridge"=> if self.check_toggle(2) {self.dialogue = 9; self.teleport(-6.0,1.0,16.0)},
            "Mannequin"=> if self.check_toggle(3) {self.dialogue = 11; },
            "G_Door"=> if self.check_toggle(4) { self.dialogue = 12;},
            
            _ => (),
        }
    }

   
   
}