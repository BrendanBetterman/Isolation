use gdnative::prelude::*;
mod helloworld;
// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<helloworld::HelloWorld>();
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