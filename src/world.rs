use crate::agent::RenderableAgent;
use std::{
    cell::RefCell, rc::Rc, sync::{Arc, Mutex}, thread, time::Duration
};

use super::Agent;

#[derive(Copy, Clone)]
pub enum GameSpeed {
    Slow,
    Medium,
    Fast
}
pub struct WorldControls {
    pub paused: bool,
    pub started: bool,
    pub step: bool,
    pub speed: GameSpeed,
}
impl WorldControls {
    pub fn new() -> WorldControls {
        WorldControls {
            paused: true,
            started: false,
            step: false,
            speed: GameSpeed::Slow,
        }
    }
}
pub struct World {
    pub controls: Arc<Mutex<WorldControls>>,
    pub agents: Vec<Rc<RefCell<Agent>>>,
    pub width: u32,
    pub height: u32,
    pub food: Vec<Vec<f32>>,
}

pub struct RenderableWorld {
    pub controls: Arc<Mutex<WorldControls>>,
    pub agents: Vec<RenderableAgent>,
    pub width: u32,
    pub height: u32,
    pub food: Vec<Vec<f32>>,
}

impl World {
    pub fn new(controls: Arc<Mutex<WorldControls>>) -> World {
        World {
            controls,
            agents: Vec::new(),
            width: 25,
            height: 25,
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
            controls: Arc::clone(&self.controls),
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
            for _ in 0..15 {
                agent.borrow_mut().add_random_node();
            }

            for _ in 0..25 {
                agent.borrow_mut().connect_random_nodes();
            }

            self.agents.push(agent);
        }
    }

    pub fn simulate_frame(world: Rc<RefCell<World>>) {
        //skip if paused
        if world.borrow().controls.lock().unwrap().paused && ! world.borrow().controls.lock().unwrap().step {
            return;
        }

        //use for stepping one frame at a time
        if world.borrow().controls.lock().unwrap().step {
            world.borrow().controls.lock().unwrap().step = false;
        } else {
            //control speed of slow, medium, fast
            let mut sleep_time = None;
            match world.borrow().controls.lock().unwrap().speed {
                GameSpeed::Slow => sleep_time = Some(Duration::from_millis(400)),
                GameSpeed::Medium => sleep_time = Some(Duration::from_millis(33)),
                GameSpeed::Fast => {},
            }
            if let Some(duration) = sleep_time {
                thread::sleep(duration);
            }
        }

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
