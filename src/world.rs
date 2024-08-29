use crate::agent::RenderableAgent;
use std::{cell::RefCell, rc::Rc};

use super::Agent;
pub struct World {
    pub agents: Vec<Rc<RefCell<Agent>>>,
    pub width: u32,
    pub height: u32,
    pub food: Vec<Vec<f32>>,
}

pub struct RenderableWorld {
    pub agents: Vec<RenderableAgent>,
    pub width: u32,
    pub height: u32,
    pub food: Vec<Vec<f32>>,
}

impl World {
    pub fn new() -> World {
        World {
            agents: Vec::new(),
            width: 100,
            height: 65,
            food: (0..100)
                .map(|_| (0..65).map(|_| rand::random::<f32>()).collect())
                .collect(), //random food grid
        }
    }

    pub fn renderable_clone(&self) -> RenderableWorld {
        let mut renderable_agents = vec![];
        for agent in &self.agents {
            renderable_agents.push(Agent::renderable_clone(&agent.borrow()));
        }
        RenderableWorld {
            agents: renderable_agents,
            width: self.width,
            height: self.height,
            food: self.food.clone(),
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

    pub fn simulate_frame(world: Rc<RefCell<World>>) {
        for agent in &world.borrow().agents {
            for node in &agent.borrow().brain {
                if node.borrow().get_output().is_some() {
                    continue;
                }
                unsafe {
                    let node_mut: &mut _ = &mut *node.as_ptr();
                    node_mut.calculate_output(Rc::clone(&agent), Rc::clone(&world));
                }
            }
        }

        //reset all nodes some values
        for agent in &world.borrow().agents {
            for node in &agent.borrow().brain {
                node.borrow_mut().reset_output();
            }
        }
    }
}
