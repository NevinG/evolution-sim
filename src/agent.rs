use std::{cell::RefCell, rc::Rc};
use crate::nodes::{BaseNode, move_node::MoveNode, node::Node, random_node::RandomNode};
pub struct Agent {
    pub brain: Vec<Rc<RefCell<Box<dyn BaseNode>>>>,

    //attributes that affect the way the agent interacts with environment
    pub x: f64,     //how much we move in the x direction each frame
    pub y: f64,     //how much we move in the y direction each frame
    pub color: f64, //color of the agent 0 is black, 1 is white
}
//TODO: let mut rng = rand::thread_rng(); instead of rand::random()
impl Agent {
    pub fn new() -> Agent {
        Agent {
            brain: Vec::new(),
            x: 0.0,
            y: 0.0,
            color: rand::random(),
        }
    }

    pub fn add_random_node(&mut self) {
        match rand::random::<usize>() % 3 {
            0 => self
                .brain
                .push(Rc::new(RefCell::new(Box::new(RandomNode::new())))),
            1 => self
                .brain
                .push(Rc::new(RefCell::new(Box::new(Node::new())))),
            2 => self
                .brain
                .push(Rc::new(RefCell::new(Box::new(MoveNode::new())))),
            _ => panic!("Random number generator failed"),
        }
    }

    pub fn connect_random_nodes(&mut self) {
        let node1 = self.get_random_node();
        let node2 = self.get_random_node();

        self.connect_nodes(node1, node2);
    }

    fn connect_nodes(
        &mut self,
        node1: Rc<RefCell<Box<dyn BaseNode>>>,
        node2: Rc<RefCell<Box<dyn BaseNode>>>,
    ) {
        //Node can't be input to itself!!!
        if Rc::ptr_eq(&node1, &node2) {
            return;
        }

        //connect the nodes
        node1.borrow_mut().add_output(Rc::clone(&node2));
        node2.borrow_mut().add_input(Rc::clone(&node1));
    }

    fn get_random_node(&self) -> Rc<RefCell<Box<dyn BaseNode>>> {
        Rc::clone(&self.brain[rand::random::<usize>() % self.brain.len()])
    }
}
