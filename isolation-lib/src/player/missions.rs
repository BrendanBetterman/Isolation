use gdnative::prelude::Vector3;


#[derive(Copy)]
#[derive(Clone)]
pub struct Quest{
    progress: i16,
    toggles: [bool;8],
    complete: bool,
    size: i16,
}
impl Quest{
    pub fn new()-> Self{
        Quest{
            progress: 0,
            toggles: [false; 8],
            complete: false,
            size: 10,
        }
    }
    pub fn clone(&self)->Self{
        Quest{
            progress:self.progress,
            toggles:self.toggles,
            complete:self.complete,
            size:self.size,
        }
    }
    pub fn _add_progress(&mut self){
        self.progress += 1;
    }
    

}
fn get_id(item: &String)->usize{
    return match item.as_str(){
        "CarKey"=> 1,
        "CarTrigger"=> 2,
        "Rock"=> 3,
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
    pub fn set_quest(&mut self,id:usize,quest:Quest){
        self.quests[id] = quest;
    }
    pub fn get_quest(&mut self)->Quest{
        return self.quests[self.q_id];
    }
    pub fn check_complete(&mut self) -> bool{
        if self.quests[self.q_id].complete{
            return true;
        }else{
            if self.quests[self.q_id].progress>=self.quests[self.q_id].size{
                self.quests[self.q_id].complete = true;
                return true;
            }else{
                return false;
            }
        }
    }
    ///run when use key is pressed
    /// 
    /* 
    pub fn on_used<const s:usize>(&mut self,keys: &[bool;s]){
        self.quests[self.q_id].on_used(&keys);
    }
    pub fn on_look<const s:usize>(&mut self,keys: &[bool;s]){
        self.quests[self.q_id].on_look(&keys);
    }
    */
    
    fn mission_one_on_used(&mut self,item: &String){
        let id = get_id(item);
        if id == 1 && self.quests[self.q_id].progress <=1{
            //use car keys
            self.quests[self.q_id]._add_progress();
            self.location = Vector3::new(-24.0,0.0,0.0);
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
    pub fn on_used(&mut self, item: &String){

        match self.q_id{
            0 => self.mission_one_on_used(&item),
            _ => print!("no mission"),
        }
    }
    pub fn look(&mut self, item: &String){

        match self.q_id{
            0 => self.mission_one_look(&item),
            _ => print!("no mission"),
        }
    }
   
}