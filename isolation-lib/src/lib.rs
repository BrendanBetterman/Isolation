use gdnative::prelude::*;
/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;
impl HelloWorld {
    /// The "constructor" of the class.
    fn new(_base: &Node) -> Self {
        HelloWorld
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    #[method]
    fn _ready(&self, #[base] base: &Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello world from node {}!", base.to_string());
    }
    #[method]
    fn _process(&mut self, #[base] base: &Node, delta: f32) {
        
        godot_print!("Hello world from node {}!", base.to_string());
    }

}
// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);

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
}


