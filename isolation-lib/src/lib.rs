use gdnative::prelude::*;
mod helloworld;
mod player;
mod camera;
mod shadow;
mod hud;
// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<helloworld::HelloWorld>();
    handle.add_class::<player::Player>();
    handle.add_class::<camera::Pcamera>();
    handle.add_class::<shadow::Shadow>();
    handle.add_class::<hud::use_obj::Use>();
    handle.add_class::<hud::dialogue::Dialogue>();
    handle.add_class::<hud::dialogue_box::DialogueBox>();
}
// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
*/
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mission_one_can_pickup() {
        let mut mission = player::missions::Missions::new();  
        assert_eq!(mission.can_pickup(&"CarKey".to_string()), true);
        assert_eq!(mission.can_pickup(&"CarTrigger".to_string()),true);
    }
    #[test]
    fn mission_one_cant_pickup(){
        let mut mission = player::missions::Missions::new();  
        assert_eq!(mission.can_pickup(&"Fridge".to_string()), false);
        assert_eq!(mission.can_pickup(&"Mannequin".to_string()), false);
    }
    #[test]
    fn mission_two_can_pickup() {
        let mut mission = player::missions::Missions::new();  
        mission.end_mission();
        assert_eq!(mission.can_pickup(&"Fridge".to_string()), true);
        assert_eq!(mission.can_pickup(&"Mannequin".to_string()),true);
        assert_eq!(mission.can_pickup(&"G_Door".to_string()),true);
        assert_eq!(mission.can_pickup(&"Bean".to_string()),true);
        assert_eq!(mission.can_pickup(&"CerealI".to_string()),true);
        assert_eq!(mission.can_pickup(&"Yams".to_string()),true);
    }
    #[test]
    fn mission_two_cant_pickup() {
        let mut mission = player::missions::Missions::new();  
        mission.end_mission();
        mission.on_used(&"Bean".to_string());
        mission.on_used(&"Yams".to_string());
        mission.on_used(&"CerealI".to_string());
        assert_eq!(mission.can_pickup(&"Bean".to_string()),false);
        assert_eq!(mission.can_pickup(&"CerealI".to_string()),false);
        assert_eq!(mission.can_pickup(&"Yams".to_string()),false);
    }
}