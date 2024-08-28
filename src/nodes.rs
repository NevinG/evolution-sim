pub mod move_node;
pub mod node;
pub mod random_node;

use super::World;
use crate::agent::Agent;
use std::{cell::RefCell, rc::Rc};
pub trait BaseNode {
    fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>, world: &World);
    fn add_input(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>);
    fn add_output(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>);
    fn get_output(&self) -> Option<f32>;
    fn reset_output(&mut self);
}
