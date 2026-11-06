use winit_input_helper::WinitInputHelper;

use crate::template::Template;

mod template;

pub struct SampleTemplate {}

impl Template for SampleTemplate {
    fn create() -> Self {
        println!("Template created.");
        Self {}
    }

    fn step(&mut self, input: &mut WinitInputHelper) {}

    fn draw(&mut self) {}
}

fn main() {
    SampleTemplate::run();
}
