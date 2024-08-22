use std::{cell::RefCell, rc::Rc};
use super::{BaseNode, Agent, World};
pub struct RandomNode {
  // Define the properties of the node here
  outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
  output: Option<f64>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl RandomNode {
  pub fn new() -> RandomNode {
      RandomNode {
          outputs: Vec::new(),
          output: None,
      }
  }
}

impl BaseNode for RandomNode {
  fn calculate_output(&mut self, _agent: Rc<RefCell<Agent>>, _world: &World) {
      self.output = Some(rand::random::<f64>() * 2.0 - 1.0); //random number [-1,1]
  }

  fn add_input(&mut self, _node: Rc<RefCell<Box<dyn BaseNode>>>) {}

  fn add_output(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>) {
      self.outputs.push(Rc::clone(&node));
  }

  fn get_output(&self) -> Option<f64> {
      self.output
  }

  fn reset_output(&mut self) {
      self.output = None;
  }
}