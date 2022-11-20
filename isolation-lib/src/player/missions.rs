use gdnative::prelude::Vector3;


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
    quests: [Quest;4],
    pub should_tp: bool,
    pub location: Vector3,
    pub dialogue: i32,
    pub should_delete: bool,
    pub id_to_dlt: i32,
    pub expire: f32,
}
impl Missions{
    pub fn new()->Self{
        Missions {  
            q_id: 0,
            quests: [Quest::new();4],
            should_tp: false,
            location: Vector3::new(0.0,0.0,0.0),
            dialogue: -1,
            should_delete: false,
            id_to_dlt: 1,
            expire: 0.0,
        }
    }
    ///takes array and checks if everything was picked up.
    fn is_collected(&mut self,ids:&[usize])->bool{
        for i in ids{
            if !self.has_picked(i.clone()){
                return false;
            }
        }
        return true;
    }
    ///Ends the mission and starts the next. TPs the player to home location.
    pub fn end_mission(&mut self){
        self.dialogue = -1;
        self.location = Vector3::new(1.0,1.0,1.0);
        self.should_tp = true;
        self.quests[self.q_id].complete = true;
        self.q_id += 1;
    }
    ///Checks if item has been used
    fn check_toggle(&mut self,id:usize)->bool{
        let output =  self.quests[self.q_id].toggles[id];
        self.quests[self.q_id].toggles[id] = true;
        return !output;
    }
    ///Takes id and returns if it has been picked up.
    fn has_picked(&mut self,id:usize)->bool{
        return self.quests[self.q_id].toggles[id];
    }
    ///Hides the object of that id.
    fn delete_id(&mut self,id:i32){
        self.id_to_dlt = id;
        self.should_delete = true;
    }
    ///Teleports the player to the specified location.
    fn teleport(&mut self,x:f32,y:f32,z:f32){
        self.location = Vector3::new(x,y,z);
        self.should_tp = true;
    }
    ///Display the dialogue and reset the counter
    fn dialogue_trigger(&mut self, id:i32){
        self.dialogue = id;
        self.expire = 75.0;
    }
    ///Runs every tick and decreases the dialogue timer by delta.
    pub fn update_dialogue(&mut self, delta:f64){
        if self.expire - delta as f32 >0.0{
            self.expire -= delta as f32 * 10.0;
        }else{
            self.dialogue = -1;
        }

    }
    pub fn can_pickup(&mut self, item: &String)->bool{
        match self.q_id{
            0 => self.mission_one_can_pickup(&item),
            1 => self.m_ci_2(&item),
            2 => self.m_ci_3(&item),
            _ => false,
        }
    }
    pub fn on_used(&mut self, item: &String){
        match self.q_id{
            0 => self.mission_one_on_used(&item),
            1 => self.m_on_used_2(&item),
            2 => self.m_on_used_3(&item),
            _ => print!("no mission"),
        }
    }
    pub fn look(&mut self, item: &String){
        match self.q_id{
            0 => self.mission_one_look(&item),
            1 => self.m_look_2(&item),
            2 => self.m_look_3(&item),
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
            self.dialogue_trigger(1);
            
        }else if id == 2 && self.quests[self.q_id].progress == 1{
            //use car
            self.quests[self.q_id]._add_progress();
            self.dialogue_trigger(3);
        }else if id == 3 && self.quests[self.q_id].progress ==2{
            //pick up rock?
            self.id_to_dlt = 0;
            self.should_delete = true;
            self.dialogue_trigger(5);
            self.quests[self.q_id]._add_progress();
        }else if id == 2 && self.quests[self.q_id].progress ==3{
            //use rock on window
            self.dialogue_trigger(6);
            self.quests[self.q_id]._add_progress();
            
        }else if id == 2 && self.quests[self.q_id].progress ==4{
            self.end_mission();
        }

    }
    fn mission_one_look(&mut self,item: &String){
        let id = get_id(item);
        if id == 1 && !self.quests[self.q_id].toggles[0]{//keys
            self.dialogue_trigger(0);
            self.quests[self.q_id].toggles[0] = true;
        }else if id == 2  && !self.quests[self.q_id].toggles[1]{//car
            self.dialogue_trigger(2);
            self.quests[self.q_id].toggles[1] = true;
        }else if id == 3 && self.quests[self.q_id].progress >=2 && !self.quests[self.q_id].toggles[2]{//rock
            self.dialogue_trigger(4);
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
            "Bean"=> !self.has_picked(5),
            "CerealI"=> !self.has_picked(6),
            "Yams"=> !self.has_picked(7),
            _ => false,
        }
    }
    fn m_look_2(&mut self,item:&String){
        match item.as_str(){
            "Fridge"=> if !self.check_toggle(0) {self.dialogue_trigger(8);},
            "Mannequin"=> if !self.check_toggle(1) {self.dialogue_trigger(10); },
            _ => (),
        }
    }
    fn m_on_used_2(&mut self,item:&String){
        match item.as_str(){
            "Fridge"=> if self.check_toggle(2) {self.dialogue_trigger(9); self.teleport(-6.0,1.0,16.0)},
            "Mannequin"=> if self.check_toggle(3) {self.dialogue_trigger(11); },
            "G_Door"=> if !self.is_collected(&[5 as usize,6 as usize,7 as usize]){self.dialogue_trigger(12)}else{self.end_mission()},
            "Bean"=> if self.check_toggle(5){self.dialogue_trigger(13); self.delete_id(2)},
            "CerealI"=> if self.check_toggle(6){self.dialogue_trigger(14); self.delete_id(3)},
            "Yams"=> if self.check_toggle(7){self.dialogue_trigger(15); self.delete_id(4)},
            _ => (),
        }
    }

    fn m_ci_3(&mut self,item:&String)->bool{
        match item.as_str(){
            "ChristmasTree"=> self.quests[self.q_id].progress <= 2,
            "ChristmasDoor"=> true,
            _ => false,
        }
    }
    fn m_look_3(&mut self,item:&String){
        match item.as_str(){
            "ChristmasTree"=> self.dialogue_trigger(16),
            "ChristmasDoor"=> if self.expire < 20.0{self.dialogue_trigger(18)},
            _ => (),
        }
    }
    fn m_on_used_3(&mut self,item:&String){
        match item.as_str(){
            "ChristmasTree"=> if self.check_toggle(0) {self.dialogue_trigger(17); self.teleport(-27.0,1.0,30.0)},
            "ChristmasDoor"=> {self.check_toggle(1); self.dialogue_trigger(19);},
            _ => (),
        }
    }
   
   
}