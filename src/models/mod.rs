use nannou::{event::MouseButton, geom::Point2, Event};

use crate::BuildContext;

pub mod forest;
pub mod neural_cellular;
pub mod rock_paper_scissor;
pub mod gravitation;

// pub trait Model {
//     fn new() -> Self;
//     fn setup(&mut self, _context: &BuildContext<dyn Model>);
//     fn handle_event(&mut self, event: &Event, context: &mut BuildContext<dyn Model>);
//     fn handle_mouse_pressed(&mut self, button: MouseButton, position: Point2);
//     fn mutate(&mut self, context: &BuildContext<dyn Model>);
//     fn draw(&mut self, context: &mut BuildContext<dyn Model>) -> Result<(), String>;
//     fn _activate(&mut self);
// }
