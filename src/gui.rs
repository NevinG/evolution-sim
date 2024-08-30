use femtovg::{Canvas, Color, FontId, Paint, Renderer};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::{Surface, WindowSurface};
use winit::dpi::PhysicalPosition;
use winit::window::Window;

use crate::renderer::ClickAction;
use crate::world::RenderableWorld;

pub mod game;
pub mod menu;
pub struct GUILocation {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}
pub trait GraphicsWindow<T: Renderer> {
    fn draw(
        &mut self,
        context: &PossiblyCurrentContext,
        surface: &Surface<WindowSurface>,
        window: &Window,
        canvas: &mut Canvas<T>,
        font_id: FontId,
        drag: PhysicalPosition<f32>,
        world: Option<&RenderableWorld>,
    );

    fn click(&mut self, pos: PhysicalPosition<f64>) -> ClickAction;
}

///returns (height, GUILocation) this function should be used by GraphicsWindow with a button
pub fn button_centered<T: Renderer>(
    text: &str,
    height: f32,
    canvas: &mut Canvas<T>,
    window: &Window,
    paint: &Paint,
) -> (f32, GUILocation) {
    let size = window.inner_size();
    let text_metrics = canvas.measure_text(0.0, 0.0, text, &paint);
    let text_height = text_metrics.as_ref().unwrap().height();
    let text_width = text_metrics.as_ref().unwrap().width();

    //add button background
    //black rectangle
    canvas.clear_rect(
        ((size.width as f32 - text_width) / 2.0) as u32 - 5,
        height as u32,
        text_width as u32 + 10,
        text_height as u32 + 10,
        Color::black(),
    );
    //smaller gray rectangle
    canvas.clear_rect(
        ((size.width as f32 - text_width) / 2.0) as u32 - 4,
        height as u32 + 1,
        text_width as u32 + 8,
        text_height as u32 + 8,
        Color::rgb(180, 180, 180),
    );

    //add text to the button
    canvas
        .fill_text(
            (size.width as f32 - text_width) / 2.0,
            text_height + height + 4.0,
            text,
            &paint,
        )
        .unwrap();

    return (
        text_height + height + 10.0,
        GUILocation {
            x: ((size.width as f32 - text_width) / 2.0) as u32 - 5,
            y: height as u32,
            width: text_width as u32 + 10,
            height: text_height as u32 + 10,
        },
    );
}

///returns (height, GUILocation) this function should be used by GraphicsWindow with a button
pub fn button_right<T: Renderer>(
    text: &str,
    height: f32,
    right_pad: f32,
    canvas: &mut Canvas<T>,
    window: &Window,
    paint: &Paint,
) -> (f32, GUILocation) {
    let size = window.inner_size();
    let text_metrics = canvas.measure_text(0.0, 0.0, text, &paint);
    let text_height = text_metrics.as_ref().unwrap().height();
    let text_width = text_metrics.as_ref().unwrap().width();

    //add button background
    //black rectangle
    canvas.clear_rect(
        (size.width as f32 - text_width - right_pad) as u32 - 10,
        height as u32,
        text_width as u32 + 10,
        text_height as u32 + 10,
        Color::black(),
    );
    //smaller gray rectangle
    canvas.clear_rect(
        (size.width as f32 - text_width - right_pad) as u32 - 9,
        height as u32 + 1,
        text_width as u32 + 8,
        text_height as u32 + 8,
        Color::rgb(180, 180, 180),
    );

    //add text to the button
    canvas
        .fill_text(
            size.width as f32 - text_width - right_pad - 5.0,
            text_height + height + 4.0,
            text,
            &paint,
        )
        .unwrap();

    return (
        text_height + height + 10.0,
        GUILocation {
            x: (size.width as f32 - text_width - right_pad) as u32 - 10,
            y: height as u32,
            width: text_width as u32 + 10,
            height: text_height as u32 + 10,
        },
    );
}

///returns height where it finsihed drawing to the canvas
pub fn text_centered<T: Renderer>(
    text: &str,
    height: f32,
    canvas: &mut Canvas<T>,
    window: &Window,
    paint: &Paint,
) -> f32 {
    let size = window.inner_size();
    let text_metrics = canvas.measure_text(0.0, 0.0, text, &paint);
    let text_height = text_metrics.as_ref().unwrap().height();
    let text_width = text_metrics.as_ref().unwrap().width();
    canvas
        .fill_text(
            (size.width as f32 - text_width) / 2.0,
            text_height + height,
            text,
            &paint,
        )
        .unwrap();

    return text_height + height;
}

///returns height where it finsihed drawing to the canvas
#[allow(dead_code)]
pub fn text_right<T: Renderer>(
    text: &str,
    height: f32,
    right_pad: f32,
    canvas: &mut Canvas<T>,
    window: &Window,
    paint: &Paint,
) -> f32 {
    let size = window.inner_size();
    let text_metrics = canvas.measure_text(0.0, 0.0, text, &paint);
    let text_height = text_metrics.as_ref().unwrap().height();
    let text_width = text_metrics.as_ref().unwrap().width();
    canvas
        .fill_text(
            size.width as f32 - text_width - right_pad,
            text_height + height,
            text,
            &paint,
        )
        .unwrap();

    return text_height + height;
}

///returns height where it finsihed drawing to the canvas
#[allow(dead_code)]
pub fn text_left<T: Renderer>(
    text: &str,
    height: f32,
    left_pad: f32,
    canvas: &mut Canvas<T>,
    paint: &Paint,
) -> f32 {
    let text_metrics = canvas.measure_text(0.0, 0.0, text, &paint);
    let text_height = text_metrics.as_ref().unwrap().height();
    canvas
        .fill_text(left_pad, text_height + height, text, &paint)
        .unwrap();

    return text_height + height;
}
