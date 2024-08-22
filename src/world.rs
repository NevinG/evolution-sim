use std::{cell::RefCell, rc::Rc};

use super::Agent;
pub struct World {
    pub agents: Vec<Rc<RefCell<Agent>>>,
    pub width: u32,
    pub height: u32,
}

impl World {
    pub fn new() -> World {
        World {
            agents: Vec::new(),
            width: 1000,
            height: 600,
        }
    }

    pub fn add_n_agents(&mut self, n: usize) {
        for _ in 0..n {
            let agent = Rc::new(RefCell::new(Agent::new()));

            //TODO: change the default agent brain, and let you customize this
            for _ in 0..100 {
                agent.borrow_mut().add_random_node();
            }

            for _ in 0..200 {
                agent.borrow_mut().connect_random_nodes();
            }

            self.agents.push(agent);
        }
    }

    pub fn simulate_frame(&mut self) {
        for agent in &self.agents {
            for node in &agent.borrow().brain {
                if node.borrow().get_output().is_some() {
                    continue;
                }
                unsafe {
                    let node_mut: &mut _ = &mut *node.as_ptr();
                    node_mut.calculate_output(Rc::clone(&agent), self);
                }
            }
        }

        //reset all nodes some values
        for agent in &self.agents {
            for node in &agent.borrow().brain {
                node.borrow_mut().reset_output();
            }
        }
    }
}
