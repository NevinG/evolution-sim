use super::{Agent, BaseNode, World};
use std::{cell::RefCell, rc::Rc};

pub struct EatNode {
    // Define the properties of the node here
    inputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all nodes that give input to this node
    weights: Vec<f32>,                           //weights for each input
    bias: f32,                                   //bias for the node

    outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
    output: Option<f32>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl EatNode {
    pub fn new() -> EatNode {
        EatNode {
            inputs: Vec::new(),
            weights: Vec::new(),
            outputs: Vec::new(),
            bias: 0.0,
            output: None,
        }
    }
}

impl BaseNode for EatNode {
    fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>, world: Rc<RefCell<World>>) {
        //calculate output of all input nodes first
        let mut output: f32 = 0.0;

        assert_eq!(self.inputs.len(), self.weights.len());
        self.output.replace(0.0); //DONT DELETE. This line is needed to prevent error, otherwise we get in an infinite loop of caluclate_output
        for (i, input) in self.inputs.iter().enumerate() {
            if input.borrow().get_output().is_none() {
                unsafe {
                    let input_mut = &mut *input.as_ptr();
                    input_mut.calculate_output(Rc::clone(&agent), Rc::clone(&world));
                }
            }
            output += input.borrow().get_output().unwrap() * self.weights[i];
        }

        //add bias to output
        output += self.bias;

        //TODO: research what activation functions I should use for this project
        //apply activation function on output
        output = output.tanh();

        //eat output amount of food
        //TODO:
        let new_food = world.borrow().food[agent.borrow().x.min(99.0) as usize]
            [agent.borrow().y.min(64.0) as usize]
            - (output + 1.0) / 2.0 / 1000.0;
        unsafe { &mut *world.as_ptr() }.food[agent.borrow().x.min(99.0) as usize]
            [agent.borrow().y.min(64.0) as usize] = new_food.max(0.0);

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

    fn get_output(&self) -> Option<f32> {
        self.output
    }

    fn reset_output(&mut self) {
        self.output = None;
    }
}
