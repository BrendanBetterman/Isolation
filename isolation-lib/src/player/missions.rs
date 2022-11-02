
#[derive(Copy)]
#[derive(Clone)]
pub struct Quest{
    progress: f32,
    complete: bool,
}
impl Quest{
    pub fn new()-> Self{
        Quest{
            progress: 0.0,
            complete: false,
        }
    }
    pub fn clone(&self)->Self{
        Quest{
            progress:self.progress,
            complete:self.complete,
        }
    }
    pub fn _add_progress(&mut self,percentage:f32){
        self.progress += percentage;
    }
    pub fn on_used<const s:usize>(&mut self,keys: &[bool;s]){

    }
    pub fn on_look<const s:usize>(&mut self,keys: &[bool;s]){

    }

}
pub struct Missions{
    q_id: usize,
    quests: [Quest;2],
}
impl Missions{
    pub fn new()->Self{
        
        Missions {  
            q_id: 0,
            quests: [Quest::new();2],
        }
    }
    pub fn set_quest(&mut self,id:usize,quest:Quest){
        self.quests[id] = quest;
    }
    pub fn get_quest(&mut self)->Quest{
        return self.quests[self.q_id];
    }
    pub fn set_progress(&mut self,progress:f32){
        self.quests[self.q_id].progress = progress;
    }
    pub fn add_progress(&mut self,percentage: f32){
        self.quests[self.q_id]._add_progress(percentage);
    }
    pub fn check_complete(&mut self) -> bool{
        if self.quests[self.q_id].complete{
            return true;
        }else{
            if self.quests[self.q_id].progress>=1.0{
                self.quests[self.q_id].complete = true;
                return true;
            }else{
                return false;
            }
        }
    }
    ///run when use key is pressed
    pub fn on_used<const s:usize>(&mut self,keys: &[bool;s]){
        self.quests[self.q_id].on_used(&keys);
    }
    pub fn on_look<const s:usize>(&mut self,keys: &[bool;s]){
        self.quests[self.q_id].on_look(&keys);
    }
   
}