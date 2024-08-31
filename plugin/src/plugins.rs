use crate::Plugin;

pub struct ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn load(&mut self) {
        println!("Example plugin loaded");
    }

    fn unload(&mut self) {
        println!("Example plugin unloaded");
    }
}
