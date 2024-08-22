pub mod node;
pub mod random_node;
pub mod move_node;

use std::{cell::RefCell, rc::Rc};
use super::World;
use crate::agent::Agent;
pub trait BaseNode {
    fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>, world: &World);
    fn add_input(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>);
    fn add_output(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>);
    fn get_output(&self) -> Option<f64>;
    fn reset_output(&mut self);
}
