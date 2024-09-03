use femtovg::{Canvas, Color, FontId, Paint, Renderer};
use glutin::context::PossiblyCurrentContext;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use winit::dpi::PhysicalPosition;
use winit::window::Window;

use crate::gui;
use crate::renderer::ClickAction;
use crate::world::{GameSpeed, RenderableWorld};

use super::{GUILocation, GraphicsWindow};

const UNIT_SIZE: u32 = 15;
const UNIT_SIZE_F: f32 = UNIT_SIZE as f32;
pub struct Game {
    buttons: Vec<(GUILocation, Box<dyn FnMut() -> ()>, ClickAction)>,
}

impl Game {
    pub fn new() -> Game {
        Game { buttons: vec![] }
    }

    pub fn button_right<T: Renderer>(
        &mut self,
        text: &str,
        height: f32,
        right_pad: f32,
        callback: Box<dyn FnMut() -> ()>,
        click_action: ClickAction,
        canvas: &mut Canvas<T>,
        window: &Window,
        paint: &Paint,
    ) -> GUILocation {
        let gui_location = gui::button_right(text, height, right_pad, canvas, window, paint);

        //add button to window
        self.buttons.push((gui_location, callback, click_action));

        gui_location
    }
}

impl<T: Renderer> GraphicsWindow<T> for Game {
    fn draw(
        &mut self,
        context: &PossiblyCurrentContext,
        surface: &Surface<WindowSurface>,
        window: &Window,
        canvas: &mut Canvas<T>,
        font_id: FontId,
        drag: PhysicalPosition<f32>,
        world: Option<&RenderableWorld>,
    ) {
        //remove old buttons
        self.buttons = vec![];

        // Make sure the canvas has the right size:
        let size = window.inner_size();
        canvas.set_size(size.width, size.height, window.scale_factor() as f32);

        //clear the canvas
        canvas.clear_rect(0, 0, size.width, size.height, Color::white());

        //caluclate offset for all renders
        let x_offset: f32 = 15.0 + drag.x;
        let y_offset: f32 = 15.0 + drag.y;

        //make paint for drawing text
        let mut fill_paint = Paint::color(Color::black());
        fill_paint.set_font(&[font_id]);
        fill_paint.set_font_size(32.0);

        //game could be rendered before world is created
        match world {
            Some(world) => {
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

                //gui buttons
                let gui_location = if !world.controls.lock().unwrap().paused {
                    let gui_location = self.button_right(
                        match world.controls.lock().unwrap().speed {
                            GameSpeed::Slow => ">",
                            GameSpeed::Medium => ">>",
                            GameSpeed::Fast => ">>>",
                        },
                        5.0,
                        5.0,
                        Box::new(|| {}),
                        ClickAction::SpeedChange,
                        canvas,
                        window,
                        &fill_paint,
                    );
                    gui_location
                } else {
                    let gui_location = self.button_right(
                        "Step",
                        5.0,
                        5.0,
                        Box::new(|| {}),
                        ClickAction::Step,
                        canvas,
                        window,
                        &fill_paint,
                    );
                    gui_location
                };

                self.button_right(
                    if world.controls.lock().unwrap().paused {
                        "Play"
                    } else {
                        "Pause"
                    },
                    5.0,
                    (size.width - gui_location.x + 5) as f32,
                    Box::new(|| (println!("play/pause"))),
                    ClickAction::PlayPause,
                    canvas,
                    window,
                    &fill_paint,
                );
            }
            _ => {
                //render start button gui
                self.button_right(
                    "Start",
                    5.0,
                    5.0,
                    Box::new(|| {}),
                    ClickAction::StartGame,
                    canvas,
                    window,
                    &fill_paint,
                );
            }
        }

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
