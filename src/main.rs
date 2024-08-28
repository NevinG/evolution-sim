mod agent;
mod nodes;
mod renderer;
mod util;
mod world;

use agent::Agent;
use winit::dpi::PhysicalPosition;
use world::World;

use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
fn main() {
    //channel used for sending the world to the rendering thread
    let (send, recv) = mpsc::channel();

    //game logic thread
    thread::spawn(move || {
        let mut world = World::new();
        world.add_n_agents(500);

        const FRAME_RATE: u32 = 60;
        let frame_duration = Duration::from_secs(1) / FRAME_RATE;
        let mut last_frame_time = Instant::now();

        loop {
            world.simulate_frame();
            if last_frame_time.elapsed() >= frame_duration {
                send.send(World::renderable_clone(&world)).unwrap();
                last_frame_time = Instant::now();
            }
        }
    });

    //rendering loop & thread
    let event_loop = EventLoop::new();
    let (context, gl_display, window, surface) = renderer::create_window(&event_loop);

    let renderer =
        unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Cannot create renderer");

    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(1000, 600, window.scale_factor() as f32);

    let mut drag: PhysicalPosition<f32> = PhysicalPosition{x: 0.0, y: 0.0};
    let mut last_position: PhysicalPosition<f64> = PhysicalPosition{x: 0.0, y: 0.0};
    let mut left_mouse_down = false;
    event_loop.run(move |event, _target, control_flow| {
        //check if new thing to render
        if let Ok(world) = recv.try_recv() {
            renderer::render(&context, &surface, &window, &mut canvas, world, drag);
        }

        //close window on exit
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput { button, state, .. } => {
                    match state {
                        ElementState::Pressed => {
                            if let MouseButton::Left = button {
                                left_mouse_down = true;
                            }
                        }
                        ElementState::Released => {
                            if let MouseButton::Left = button {
                                left_mouse_down = false;
                            }
                        }
                    }
                },
                WindowEvent::CursorMoved { position, .. } => {
                    if left_mouse_down {
                        drag.x += (position.x - last_position.x) as f32;
                        drag.y += (position.y - last_position.y) as f32;
                    }
                    last_position = position;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
