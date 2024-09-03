mod agent;
mod gui;
mod nodes;
mod renderer;
mod util;
mod world;

use agent::Agent;
use renderer::GraphicsRenderer;
use winit::dpi::PhysicalPosition;
use world::{World, WorldControls};

use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
fn main() {
    //channel used for sending the world to the rendering thread
    let (send, recv) = mpsc::channel();

    let world_controls = Arc::new(Mutex::new(WorldControls::new()));
    let world_controls_clone = Arc::clone(&world_controls);
    //game logic thread
    thread::spawn(move || {
        let world_controls_clone_2 = Arc::clone(&world_controls_clone);
        let world = Rc::new(RefCell::new(World::new(world_controls_clone_2)));
        world.borrow_mut().add_n_agents(100);

        const FRAME_RATE: u32 = 60;
        let frame_duration = Duration::from_secs(1) / FRAME_RATE;
        let mut last_frame_time = Instant::now();

        //wait for game to start
        while !(*world_controls_clone.lock().unwrap()).started {
            thread::sleep(Duration::from_millis(12));
        }

        loop {
            World::simulate_frame(Rc::clone(&world));
            if last_frame_time.elapsed() >= frame_duration {
                send.send(World::renderable_clone(&world.borrow())).unwrap();
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
    let mut graphics_renderer =
        GraphicsRenderer::<OpenGl>::new(&mut canvas, Arc::clone(&world_controls));

    let mut drag: PhysicalPosition<f32> = PhysicalPosition { x: 0.0, y: 0.0 };
    let mut last_position: PhysicalPosition<f64> = PhysicalPosition { x: 0.0, y: 0.0 };
    let mut left_mouse_down = false;

    graphics_renderer.render(&context, &surface, &window, &mut canvas, drag, None);
    event_loop.run(move |event, _target, control_flow| {
        //check if new thing to render
        if let Ok(world) = recv.try_recv() {
            //renderer::render(&context, &surface, &window, &mut canvas, world, drag);
            //renderer::render_menu(&context, &surface, &window, &mut canvas, font_id);
            graphics_renderer.render(&context, &surface, &window, &mut canvas, drag, Some(&world));
        }

        //close window on exit
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput { button, state, .. } => match state {
                    ElementState::Pressed => {
                        if let MouseButton::Left = button {
                            left_mouse_down = true;
                        }
                    }
                    ElementState::Released => {
                        if let MouseButton::Left = button {
                            left_mouse_down = false;

                            //check if user clicked anything
                            graphics_renderer.click(
                                &context,
                                &surface,
                                &window,
                                &mut canvas,
                                last_position,
                            );
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
