use rand::Rng;
use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::{Point, Rect},
};

use crate::utils::*;
use crate::GlobalContext;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Root {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    size: f32,
    ds: f32,
    angle: f32,
    da: f32,
    max_size: f32,
}

pub(crate) struct Forest {
    cur_draw_color: i32,
    buf: Vec<Root>,
    wrap_on_edge: bool,
}

impl Forest {
    pub fn new() -> Self {
        Self {
            cur_draw_color: 2,
            buf: vec![],
            wrap_on_edge: true,
        }
    }

    pub fn setup(&mut self, context: &GlobalContext) {}

    pub fn handle_event(&mut self, event: &Event, context: &mut GlobalContext) {
        let mut rand_gen = rand::thread_rng();

        match event {
            Event::Quit { .. } => context.running = false,
            Event::MouseMotion {
                mousestate, x, y, ..
            } => {
                if mousestate.left() {
                    for _ in 0..2 {
                        self.buf.push(Root {
                            x: *x as f32,
                            y: *y as f32,
                            dx: rand_gen.gen_range(-2.0..=2.0),
                            dy: rand_gen.gen_range(-2.0..=2.0),
                            size: 0.1,
                            ds: rand_gen.gen_range(0.0..1.0),
                            angle: 0.,
                            da: rand_gen.gen_range(0.1..=0.3),
                            max_size: rand_gen.gen_range(3.0..=6.0),
                        });
                    }
                }
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if mouse_btn == &MouseButton::Left {
                    for _ in 0..5 {
                        self.buf.push(Root {
                            x: *x as f32,
                            y: *y as f32,
                            dx: rand_gen.gen_range(-2.0..=2.0),
                            dy: rand_gen.gen_range(-2.0..=2.0),
                            size: 0.1,
                            ds: rand_gen.gen_range(0.0..1.0),
                            angle: 0.,
                            da: rand_gen.gen_range(0.1..=0.3),
                            max_size: rand_gen.gen_range(3.0..=6.0),
                        });
                    }
                }
            }
            Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                Keycode::R => self.cur_draw_color = 1,
                Keycode::G => self.cur_draw_color = 2,
                Keycode::B => self.cur_draw_color = 3,
                Keycode::P => context.is_playing = !context.is_playing,
                _ => self.cur_draw_color = 0,
            },
            _ => {}
        }
    }

    pub fn mutate(&mut self, context: &GlobalContext) {
        let mut old_buf = vec![];

        for cur_buf in &mut self.buf {
            if cur_buf.size < cur_buf.max_size {
                cur_buf.size += cur_buf.ds;
                cur_buf.x += cur_buf.dx + cur_buf.angle.sin();
                cur_buf.y += cur_buf.dy + cur_buf.angle.sin();
                cur_buf.angle += cur_buf.da;
                old_buf.push(cur_buf.clone());
            }
        }

        self.buf = old_buf;
    }

    pub fn draw(&mut self, context: &mut GlobalContext) -> Result<(), String> {
        for cur_buf in &self.buf {
            // context
            //     .canvas
            //     .draw_point(Point::new(cur_buf.x as i32, cur_buf.y as i32))?;

            // render::draw_circle(
            //     context.canvas,
            //     cur_buf.x as i32,
            //     cur_buf.y as i32,
            //     cur_buf.size as i32,
            // )?;

            context.canvas.filled_circle(
                cur_buf.x as i16,
                cur_buf.y as i16,
                cur_buf.size as i16,
                Color::RGB(10, (40. * cur_buf.size) as u8, 10),
            )?;

            // context.canvas.draw_rect(Rect::new(
            //     cur_buf.x as i32,
            //     cur_buf.y as i32,
            //     cur_buf.size as u32,
            //     cur_buf.size as u32,
            // ))?;
        }

        Ok(())
    }

    pub fn activate(&mut self) {}

    fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
