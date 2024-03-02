use rand::{rngs::ThreadRng, Rng};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::{Point, Rect}};
use std::cmp::min;

use crate::GlobalContext;

#[derive(Clone, Copy)]
pub(crate) struct BufData {
    pub color: i32,
    pub intensity: i32,
}

pub(crate) struct RockPaperScissor {
    rand_gen: ThreadRng,
    cur_draw_color: i32,
    max_intensity: i32,
    pub buf: Vec<Vec<BufData>>,
}

impl RockPaperScissor {
    pub fn new() -> Self {
        Self {
            cur_draw_color: 1,
            max_intensity: 8,
            rand_gen: rand::thread_rng(),
            buf: vec![vec![BufData {
                color: 0,
                intensity: 0,
            }]],
        }
    }

    pub fn setup(&mut self, context: &GlobalContext) {
        self.rand_gen = rand::thread_rng();
        self.buf = vec![
            vec![
                BufData {
                    color: 0,
                    intensity: self.max_intensity
                };
                context.grid_width as usize
            ];
            context.grid_height as usize
        ];
    }

    pub fn handle_event(&mut self, event: &Event, context: &mut GlobalContext) {
        match event {
            Event::Quit { .. } => context.running = false,
            Event::MouseMotion {
                mousestate, x, y, ..
            } => {
                // println!("{:?}", mousestate.left());
                if mousestate.left() {
                    self.buf[(y / context.block_size) as usize]
                        [(x / context.block_size) as usize]
                        .color = self.cur_draw_color;
                }
            }
            Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                Keycode::R => self.cur_draw_color = 1,
                Keycode::G => self.cur_draw_color = 2,
                Keycode::B => self.cur_draw_color = 3,
                _ => self.cur_draw_color = 0,
            },
            _ => {}
        }
    }

    pub fn draw(&mut self, context: &mut GlobalContext) -> Result<(), String>{
        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let neighbour_y = self.rand_gen.gen_range(-1..=1) + (y as i32);
                let neighbour_x = self.rand_gen.gen_range(-1..=1) + (x as i32);

                // println!("y: {:?}, x: {:?}", neighbour_y - (y as i32), neighbour_x - (x as i32));

                if Self::in_bounds(
                    &neighbour_y,
                    &neighbour_x,
                    &context.grid_height,
                    &context.grid_width,
                ) {
                    let mut neighbour_pixel = self.buf[neighbour_y as usize][neighbour_x as usize];

                    match self.buf[y][x].color {
                        0 => {
                            if neighbour_pixel.color != 0 && neighbour_pixel.intensity > 0 {
                                self.buf[y][x].color = neighbour_pixel.color;
                                self.buf[y][x].intensity = neighbour_pixel.intensity - 1;
                            }
                        }
                        1 => {
                            if neighbour_pixel.color == 2 {
                                neighbour_pixel.color = 0;
                                neighbour_pixel.intensity = self.max_intensity;
                                self.buf[y][x].intensity =
                                    min(self.buf[y][x].intensity + 1, self.max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        2 => {
                            if neighbour_pixel.color == 3 {
                                neighbour_pixel.color = 0;
                                neighbour_pixel.intensity = self.max_intensity;
                                self.buf[y][x].intensity =
                                    min(self.buf[y][x].intensity + 1, self.max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        3 => {
                            if neighbour_pixel.color == 1 {
                                neighbour_pixel.color = 0;
                                neighbour_pixel.intensity = self.max_intensity;
                                self.buf[y][x].intensity =
                                    min(self.buf[y][x].intensity + 1, self.max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        _ => {}
                    }

                    self.buf[neighbour_y as usize][neighbour_x as usize] = neighbour_pixel;
                }
            }
        }

        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let rect_color: Color;

                match self.buf[y][x].color {
                    0 => rect_color = Color::RGBA(200, 200, 200, 0),
                    1 => rect_color = Color::RGB(246, 122, 17),
                    2 => rect_color = Color::RGB(208, 37, 37),
                    3 => rect_color = Color::RGB(35, 119, 181),
                    _ => rect_color = Color::RGB(20, 20, 20),
                }

                if self.buf[y][x].color != 0 {
                    context.canvas.set_draw_color(rect_color);
                    context.canvas.fill_rect(Rect::new(
                        (x as i32) * context.block_size,
                        (y as i32) * context.block_size,
                        context.block_size as u32,
                        context.block_size as u32,
                    ))?;
                }
            }
        }

        context.canvas.set_draw_color(Color::RGB(0, 0, 0));

        for x in 0..self.buf[0].len() {
            context.canvas.draw_line(
                Point::new(
                    (x as i32) * context.block_size,
                    (0 as i32) * context.block_size,
                ),
                Point::new(
                    (x as i32) * context.block_size,
                    (context.scr_height as i32) * context.block_size,
                ),
            )?;
        }

        for y in 0..self.buf.len() {
            context.canvas.draw_line(
                Point::new(
                    (0 as i32) * context.block_size,
                    (y as i32) * context.block_size,
                ),
                Point::new(
                    (context.scr_width as i32) * context.block_size,
                    (y as i32) * context.block_size,
                ),
            )?;
        }

        Ok(())
    }

    fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
