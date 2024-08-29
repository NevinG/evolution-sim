use super::world::RenderableWorld;

use femtovg::{Canvas, Color, Renderer};
use glutin::surface::Surface;
use glutin::{context::PossiblyCurrentContext, display::Display};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::dpi::PhysicalPosition;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{dpi::PhysicalSize, window::Window};

use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

//contains some helper functions for creating a window and rendering to it
const UNIT_SIZE: u32 = 15;
const UNIT_SIZE_F: f32 = UNIT_SIZE as f32;

pub fn create_window(
    event_loop: &EventLoop<()>,
) -> (
    PossiblyCurrentContext,
    Display,
    Window,
    Surface<WindowSurface>,
) {
    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1000., 600.))
        .with_title("Evolution Simulator");

    let template = ConfigTemplateBuilder::new().with_alpha_size(8);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(event_loop, template, |mut configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    let context_attributes =
        ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));

    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap()
    });

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window.raw_window_handle(),
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    (
        not_current_gl_context
            .take()
            .unwrap()
            .make_current(&surface)
            .unwrap(),
        gl_display,
        window,
        surface,
    )
}

pub fn render<T: Renderer>(
    context: &PossiblyCurrentContext,
    surface: &Surface<WindowSurface>,
    window: &Window,
    canvas: &mut Canvas<T>,
    world: RenderableWorld,
    drag: PhysicalPosition<f32>,
) {
    // Make sure the canvas has the right size:
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, window.scale_factor() as f32);

    //clear the canvas
    canvas.clear_rect(0, 0, size.width, size.height, Color::white());

    //caluclate offset for all renders
    let x_offset: f32 = 15.0 + drag.x;
    let y_offset: f32 = 15.0 + drag.y;

    //render the food
    // render the food
    for i in 0..world.width {
        for j in 0..world.height {
            let food_amount = world.food[i as usize][j as usize];
            canvas.clear_rect(
                i * UNIT_SIZE + x_offset as u32,
                j * UNIT_SIZE + y_offset as u32,
                UNIT_SIZE,
                UNIT_SIZE,
                Color::rgbf(
                    1.0 - food_amount,
                    1.0 - food_amount / 2.0,
                    1.0 - food_amount,
                ),
            );
        }
    }

    //render all the agents
    for agent in &world.agents {
        canvas.clear_rect(
            (agent.x * UNIT_SIZE_F - UNIT_SIZE_F / 2.0 + x_offset) as u32,
            (agent.y * UNIT_SIZE_F - UNIT_SIZE_F / 2.0 + y_offset) as u32,
            UNIT_SIZE,
            UNIT_SIZE,
            Color::rgbf(agent.color.r, agent.color.g, agent.color.b),
        );
    }
    // Tell renderer to execute all drawing commands
    canvas.flush();
    // Display what we've just rendered
    surface
        .swap_buffers(context)
        .expect("Could not swap buffers");
}
