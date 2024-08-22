use std::{cell::RefCell, rc::Rc};
use super::{BaseNode, Agent, World};

pub struct Node {
  // Define the properties of the node here
  inputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all nodes that give input to this node
  weights: Vec<f64>,                           //weights for each input
  bias: f64,                                   //bias for the node

  outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
  output: Option<f64>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl Node {
  pub fn new() -> Node {
      Node {
          inputs: Vec::new(),
          weights: Vec::new(),
          outputs: Vec::new(),
          bias: 0.0,
          output: None,
      }
  }
}

impl BaseNode for Node {
  fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>, world: &World) {
      //calculate output of all input nodes first
      let mut output: f64 = 0.0;

      assert_eq!(self.inputs.len(), self.weights.len());
      self.output.replace(0.0); //DONT DELETE. This line is needed to prevent error, otherwise we get in an infinite loop of caluclate_output
      for (i, input) in self.inputs.iter().enumerate() {
          if input.borrow().get_output().is_none() {
              unsafe {
                  let input_mut = &mut *input.as_ptr();
                  input_mut.calculate_output(Rc::clone(&agent), world);
              }
          }
          output += input.borrow().get_output().unwrap() * self.weights[i];
      }

      //add bias to output
      output += self.bias;

      //TODO: research what activation functions I should use for this project
      //apply activation function on output
      output = output.tanh();

      //set the output
      self.output = Some(output);
  }

  fn add_input(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>) {
      self.inputs.push(Rc::clone(&node));
      self.weights.push(1.0);
  }

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