use std::{cell::RefCell, rc::Rc};

//TODO use weak to prevent ref cycles
trait BaseNode {
    fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>);
    fn add_input(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>);
    fn add_output(&mut self, node: Rc<RefCell<Box<dyn BaseNode>>>);
    fn get_output(&self) -> Option<f64>;
    fn reset_output(&mut self);
}

struct Node {
    // Define the properties of the node here
    inputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all nodes that give input to this node
    weights: Vec<f64>, //weights for each input
    bias: f64, //bias for the node

    outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
    output: Option<f64>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl Node {
    fn new() -> Node {
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
    fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>) {
        //calculate output of all input nodes first
        let mut output: f64 = 0.0;

        assert_eq!(self.inputs.len(), self.weights.len());
        self.output.replace(0.0); //DONT DELETE. This line is needed to prevent error, otherwise we get in an infinite loop of caluclate_output
        for (i, input) in self.inputs.iter().enumerate() {
            if input.borrow().get_output().is_none() {
                unsafe {
                    let input_mut = &mut *input.as_ptr();
                    input_mut.calculate_output(Rc::clone(&agent));
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

struct RandomNode {
    // Define the properties of the node here
    outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
    output: Option<f64>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl RandomNode {
    fn new() -> RandomNode {
        RandomNode {
            outputs: Vec::new(),
            output: None,
        }
    }
}

impl BaseNode for RandomNode {
    fn calculate_output(&mut self, _agent: Rc<RefCell<Agent>>) {
        self.output = Some(rand::random::<f64>() * 2.0 - 1.0); //random number [-1,1]
    }

    fn add_input(&mut self, _node: Rc<RefCell<Box<dyn BaseNode>>>) { }

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

enum MoveDirection {
    X,
    Y
}
struct MoveNode {
    // Define the properties of the node here
    inputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all nodes that give input to this node
    weights: Vec<f64>, //weights for each input
    bias: f64, //bias for the node
    move_direction: MoveDirection, //determines if this move nodes moves the x or y direction

    outputs: Vec<Rc<RefCell<Box<dyn BaseNode>>>>, //all node this node gives output to
    output: Option<f64>, //this is the value all nodes in the outputs array use, if the Option is None this Node has not been calculated yet
}

impl MoveNode {
    fn new() -> MoveNode {
        MoveNode {
            inputs: Vec::new(),
            weights: Vec::new(),
            outputs: Vec::new(),
            bias: 0.0,

            move_direction: if rand::random() { MoveDirection::X } else { MoveDirection::Y },
            output: None,
        }
    }
}

impl BaseNode for MoveNode {
    fn calculate_output(&mut self, agent: Rc<RefCell<Agent>>) {
        //calculate output of all input nodes first
        let mut output: f64 = 0.0;

        assert_eq!(self.inputs.len(), self.weights.len());
        self.output.replace(0.0); //DONT DELETE. This line is needed to prevent error, otherwise we get in an infinite loop of caluclate_output
        for (i, input) in self.inputs.iter().enumerate() {
            if input.borrow().get_output().is_none() {
                unsafe {
                    let input_mut = &mut *input.as_ptr();
                    input_mut.calculate_output(Rc::clone(&agent));
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
struct Agent {
    brain: Vec<Rc<RefCell<Box<dyn BaseNode>>>>,

    //attributes that affect the way the NN moves
    x: f64, //how much we move in the x direction each frame
    y: f64, //how much we move in the y direction each frame
}
//TODO: let mut rng = rand::thread_rng(); instead of rand::random()
impl Agent {
    fn new() -> Agent {
        Agent {
            brain: Vec::new(),
            x: 0.0,
            y: 0.0,
        }
    }

    fn add_random_node(&mut self) {
        match rand::random::<usize>() % 3 {
            0 => self.brain.push(Rc::new(RefCell::new(Box::new(RandomNode::new())))),
            1 => self.brain.push(Rc::new(RefCell::new(Box::new(Node::new())))),
            2 => self.brain.push(Rc::new(RefCell::new(Box::new(MoveNode::new())))),
            _ => panic!("Random number generator failed"),
        } 
    }

    fn connect_random_nodes(&mut self) {
        let node1 = self.get_random_node();
        let node2 = self.get_random_node();

        self.connect_nodes(node1, node2);
    }

    fn connect_nodes(&mut self, node1: Rc<RefCell<Box<dyn BaseNode>>>, node2: Rc<RefCell<Box<dyn BaseNode>>>) {
        //Node can't be input to itself!!!
        if Rc::ptr_eq(&node1, &node2) { return; }

        //connect the nodes
        node1.borrow_mut().add_output(Rc::clone(&node2));
        node2.borrow_mut().add_input(Rc::clone(&node1));
    }
    
    fn get_random_node(&self) -> Rc<RefCell<Box<dyn BaseNode>>> {
        Rc::clone(&self.brain[rand::random::<usize>() % self.brain.len()])
    }
}

struct World {
    agents: Vec<Rc<RefCell<Agent>>>,
}

impl World {
    fn new() -> World {
        World {
            agents: Vec::new(),
        }
    }

    fn add_n_agents(&mut self, n: usize) {
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

    fn simulate_frame(&mut self) {
        for agent in &self.agents {
            for node in &agent.borrow().brain {
                if node.borrow().get_output().is_some() { continue; }
                unsafe {
                    let node_mut: &mut _ = &mut *node.as_ptr();
                    node_mut.calculate_output(Rc::clone(&agent));
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

use std::num::NonZeroU32;

use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, Renderer};
use glutin::surface::Surface;
use glutin::{context::PossiblyCurrentContext, display::Display};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::{dpi::PhysicalSize, window::Window};
use std::time::{Duration, Instant};

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
fn main() {
    let mut world = World::new();
    world.add_n_agents(100);

    let event_loop = EventLoop::new();
    let (context, gl_display, window, surface) = create_window(&event_loop);

    let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
        .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(1000, 600, window.scale_factor() as f32);

    const FRAME_RATE: u32 = 30;
    let frame_duration = Duration::from_secs(1) / FRAME_RATE;
    let mut last_frame_time = Instant::now();
    event_loop.run(move |event, _target, control_flow| {
        world.simulate_frame();

        if last_frame_time.elapsed() >= frame_duration {
            render(&context, &surface, &window, &mut canvas, &world);
            last_frame_time = Instant::now();
        }

        //close window on exit
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            }
            _ => {}
    }})
}

fn create_window(event_loop: &EventLoop<()>) -> (PossiblyCurrentContext, Display, Window, Surface<WindowSurface>) {
    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1000., 600.))
        .with_title("Femtovg");

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));

    let mut not_current_gl_context =
        Some(unsafe { gl_display.create_context(&gl_config, &context_attributes).unwrap() });

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

    (
        not_current_gl_context.take().unwrap().make_current(&surface).unwrap(),
        gl_display,
        window,
        surface,
    )
}

fn render<T: Renderer>(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<T>,
    world : &World,
) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    canvas.clear_rect(0, 0, size.width, size.height, Color::black());

    // Make smol red rectangle
    for agent in &world.agents {
        let square_position = PhysicalPosition::new(agent.borrow().x as f64, agent.borrow().y as f64);
        canvas.clear_rect(
            square_position.x as u32,
            square_position.y as u32,
            15,
            15,
            Color::rgbf(1., 0., 0.),
        );
    }
    // Tell renderer to execute all drawing commands
    canvas.flush();
    // Display what we've just rendered
    surface.swap_buffers(context).expect("Could not swap buffers");
}