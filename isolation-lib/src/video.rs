
use gdnative;
use gdnative::api::VideoPlayer;
use gdnative::prelude::*;
/// The Player "class"
#[derive(gdnative::derive::NativeClass)]
#[inherit(VideoPlayer)]
pub struct Video;


impl Video{
    fn new(owner:&VideoPlayer)->Self{
        Video
        
    }
}
#[methods]
impl Video{
   
    #[method]
    fn _process(&mut self, #[base]owner: &VideoPlayer,_delta: f32){
        if !owner.is_playing(){
            
            owner.play()
        }
    }
}