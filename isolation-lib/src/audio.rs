
use gdnative;
use gdnative::api::AudioStreamPlayer;
use gdnative::prelude::*;
/// The Player "class"
#[derive(gdnative::derive::NativeClass)]
#[inherit(AudioStreamPlayer)]
pub struct Audio;


impl Audio{
    fn new(owner:&AudioStreamPlayer)->Self{
        Audio
        
    }
}
#[methods]
impl Audio{
   
    #[method]
    fn _process(&mut self, #[base]owner: &AudioStreamPlayer,_delta: f32){
        if !owner.is_playing(){
            
            owner.play(0.0);
        }
    }
}