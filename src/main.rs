mod agent;
mod nodes;
mod renderer;
mod world;

use agent::Agent;
use world::World;

use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use std::time::{Duration, Instant};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use glutin::prelude::*;
fn main() {
    let mut world = World::new();
    world.add_n_agents(100);

    let event_loop = EventLoop::new();
    let (context, gl_display, window, surface) = renderer::create_window(&event_loop);

    let renderer =
        unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(1000, 600, window.scale_factor() as f32);

    const FRAME_RATE: u32 = 30;
    let frame_duration = Duration::from_secs(1) / FRAME_RATE;
    let mut last_frame_time = Instant::now();
    event_loop.run(move |event, _target, control_flow| {
        world.simulate_frame();

        if last_frame_time.elapsed() >= frame_duration {
            renderer::render(&context, &surface, &window, &mut canvas, &world);
            last_frame_time = Instant::now();
        }

        //close window on exit
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }
    })
}
