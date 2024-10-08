use std::{cell::RefCell, rc::Rc};

use super::{Agent, BaseNode, World};

enum MoveDirection {
    X,
    Y,
}
pub struct MoveNode {
    // Define the properties of the node here
    inputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all nodes that give input to this node
    weights: Vec<f32>,                           //weights for each input
    bias: f32,                                   //bias for the node
    move_direction: MoveDirection, //determines if this move nodes moves the x or y direction

    outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
    output: Option<f32>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl MoveNode {
    pub fn new() -> MoveNode {
        MoveNode {
            inputs: Vec::new(),
            weights: Vec::new(),
            outputs: Vec::new(),
            bias: 0.0,

            move_direction: if rand::random() {
                MoveDirection::X
            } else {
                MoveDirection::Y
            },
            output: None,
        }
    }
}

impl BaseNode for MoveNode {
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

        //apply activation function on output
        output = output.tanh();

        //set the output
        self.output = Some(output);

        //move the agent
        match self.move_direction {
            MoveDirection::X => unsafe { (*agent.as_ptr()).x += output },
            MoveDirection::Y => unsafe { (*agent.as_ptr()).y += output },
        }

        //bounds check
        if agent.borrow().x > world.borrow().width as f32 {
            unsafe { (*agent.as_ptr()).x = world.borrow().width as f32 };
        }

        if agent.borrow().y > world.borrow().height as f32 {
            unsafe { (*agent.as_ptr()).y = world.borrow().height as f32 };
        }

        if agent.borrow().x < 0.0 {
            unsafe { (*agent.as_ptr()).x = 0.0 };
        }

        if agent.borrow().y < 0.0 {
            unsafe { (*agent.as_ptr()).y = 0.0 };
        }
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
