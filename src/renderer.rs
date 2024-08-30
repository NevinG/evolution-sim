use super::gui::{game::Game, menu::Menu, GraphicsWindow};
use super::world::RenderableWorld;

use femtovg::{Canvas, FontId, Renderer};
use glutin::surface::Surface;
use glutin::{context::PossiblyCurrentContext, display::Display};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use resource::resource;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
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

pub enum ClickAction {
    MoveWindow,
    StartGame,
    None,
}

impl Clone for ClickAction {
    fn clone(&self) -> Self {
        match self {
            ClickAction::MoveWindow => ClickAction::MoveWindow,
            ClickAction::StartGame => ClickAction::StartGame,
            ClickAction::None => ClickAction::None,
        }
    }
}
pub struct GraphicsRenderer<T: Renderer> {
    game_started: Arc<Mutex<bool>>,
    cur_window: usize,
    windows: Vec<Box<dyn GraphicsWindow<T>>>,
    font_id: FontId,
}

impl<T: Renderer> GraphicsRenderer<T> {
    pub fn new(canvas: &mut Canvas<T>, game_started: Arc<Mutex<bool>>) -> GraphicsRenderer<T> {
        GraphicsRenderer {
            game_started,
            cur_window: 0,
            windows: vec![Box::new(Menu::new()), Box::new(Game::new())],
            font_id: canvas
                .add_font_mem(&resource!("src/fonts/Roboto-Regular.ttf"))
                .expect("Cannot add font"),
        }
    }

    pub fn render(
        &mut self,
        context: &PossiblyCurrentContext,
        surface: &Surface<WindowSurface>,
        window: &Window,
        canvas: &mut Canvas<T>,
        drag: PhysicalPosition<f32>,
        world: Option<&RenderableWorld>,
    ) {
        self.windows[self.cur_window].draw(
            context,
            surface,
            window,
            canvas,
            self.font_id,
            drag,
            world,
        );
    }

    pub fn click(
        &mut self,
        context: &PossiblyCurrentContext,
        surface: &Surface<WindowSurface>,
        window: &Window,
        canvas: &mut Canvas<T>,
        pos: PhysicalPosition<f64>,
    ) {
        //check current windows gui for click on button
        match self.windows[self.cur_window].click(pos) {
            ClickAction::MoveWindow => {
                self.cur_window += 1;

                if self.cur_window == 1 {
                    self.render(
                        context,
                        surface,
                        window,
                        canvas,
                        PhysicalPosition { x: 0.0, y: 0.0 },
                        None,
                    );
                }
            }
            ClickAction::StartGame => *self.game_started.lock().unwrap() = true,
            _ => {}
        }
    }
}
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
