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
    handle.add_class::<hud::Use>();
}
// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/