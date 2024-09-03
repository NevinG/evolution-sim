use femtovg::{Canvas, Color, FontId, Paint, Renderer};
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use winit::dpi::PhysicalPosition;
use winit::window::Window;

use crate::gui;
use crate::renderer::ClickAction;
use crate::world::RenderableWorld;

use super::{GUILocation, GraphicsWindow};

pub struct Menu {
    buttons: Vec<(GUILocation, Box<dyn FnMut() -> ()>, ClickAction)>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu { buttons: vec![] }
    }

    pub fn button_centered<T: Renderer>(
        &mut self,
        text: &str,
        height: f32,
        callback: Box<dyn FnMut() -> ()>,
        click_action: ClickAction,
        canvas: &mut Canvas<T>,
        window: &Window,
        paint: &Paint,
    ) -> GUILocation {
        let gui_location = gui::button_centered(text, height, canvas, window, paint);

        //add button to window
        self.buttons.push((gui_location, callback, click_action));

        gui_location
    }
}

impl<T: Renderer> GraphicsWindow<T> for Menu {
    fn draw(
        &mut self,
        context: &PossiblyCurrentContext,
        surface: &Surface<WindowSurface>,
        window: &Window,
        canvas: &mut Canvas<T>,
        font_id: FontId,
        _drag: PhysicalPosition<f32>,
        _world: Option<&RenderableWorld>,
    ) {
        //remove old buttons
        self.buttons = vec![];

        // Make sure the canvas has the right size:
        let size = window.inner_size();
        canvas.set_size(size.width, size.height, window.scale_factor() as f32);

        //clear the canvas
        canvas.clear_rect(0, 0, size.width, size.height, Color::white());

        //render menu stuff
        let mut fill_paint = Paint::color(Color::black());
        fill_paint.set_font(&[font_id]);
        fill_paint.set_font_size(32.0);

        let gui_location =
            gui::text_centered("Evolution Simulator", 5.0, canvas, window, &fill_paint);
        self.button_centered(
            "Start",
            (gui_location.height + gui_location.y) as f32 + 5.0,
            Box::new(|| {}),
            ClickAction::MoveWindow,
            canvas,
            window,
            &fill_paint,
        );
        fill_paint.set_font_size(16.0);
        gui::text_right(
            "By: Nevin Gilday. 08/2024",
            (window.inner_size().height - 20) as f32,
            5.0,
            canvas,
            window,
            &fill_paint,
        );
        // Tell renderer to execute all drawing commands
        canvas.flush();
        // Display what we've just rendered
        surface
            .swap_buffers(context)
            .expect("Could not swap buffers");
    }

    fn click(&mut self, pos: PhysicalPosition<f64>) -> ClickAction {
        for button in &mut self.buttons {
            if pos.x >= button.0.x as f64
                && pos.x <= (button.0.x + button.0.width) as f64
                && pos.y >= button.0.y as f64
                && pos.y <= (button.0.y + button.0.height) as f64
            {
                button.1(); //calls the closure on the button
                return button.2.clone();
            }
        }
        ClickAction::None
    }
}
