use rand::{rngs::ThreadRng, Rng};
use sfml::{
    graphics::{
        BlendMode, Color, RectangleShape, RenderStates, RenderTarget, RenderTexture, Shape,
        Transformable,
    },
    system::Vector2f,
    window::{
        event::Event::{self, MouseLeft},
        Key, MouseButton,
    },
};
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
            max_intensity: 6,
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
            Event::Closed { .. } => context.running = false,
            Event::MouseMoved { x, y } | Event::MouseButtonPressed { x, y, .. } => {
                if MouseButton::Left.is_pressed() {
                    self.buf[(y / context.block_size) as usize]
                        [(x / context.block_size) as usize]
                        .color = self.cur_draw_color;
                }
            }
            Event::KeyPressed { code, .. } => match code {
                Key::R => self.cur_draw_color = 1,
                Key::G => self.cur_draw_color = 2,
                Key::B => self.cur_draw_color = 3,
                _ => self.cur_draw_color = 0,
            },
            _ => {}
        }
    }

    pub fn draw(
        &mut self,
        context: &mut GlobalContext,
        canvas: &mut RenderTexture,
    ) -> Result<(), String> {
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
                        0 | 10 | 20 | 30 => {
                            if neighbour_pixel.color != 0 && neighbour_pixel.intensity > 0 {
                                self.buf[y][x].color = neighbour_pixel.color;
                                self.buf[y][x].intensity = neighbour_pixel.intensity - 1;
                            }
                        }
                        1 => {
                            if neighbour_pixel.color == 2 {
                                neighbour_pixel.color = 20;
                                neighbour_pixel.intensity = self.max_intensity;
                                self.buf[y][x].intensity =
                                    min(self.buf[y][x].intensity + 1, self.max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        2 => {
                            if neighbour_pixel.color == 3 {
                                neighbour_pixel.color = 30;
                                neighbour_pixel.intensity = self.max_intensity;
                                self.buf[y][x].intensity =
                                    min(self.buf[y][x].intensity + 1, self.max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        3 => {
                            if neighbour_pixel.color == 1 {
                                neighbour_pixel.color = 10;
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
                let new_intensity =
                    (200 + (50 - (50 * self.buf[y][x].intensity) / self.max_intensity)) as u8;

                match self.buf[y][x].color {
                    // 0 => rect_color = context.bg_color,
                    1 | 10 => rect_color = Color::new_rgba(246, 122, 17, new_intensity),
                    2 | 20 => rect_color = Color::new_rgba(208, 37, 37, new_intensity),
                    3 | 30 => rect_color = Color::new_rgba(35, 119, 181, new_intensity),
                    _ => rect_color = Color::new_rgba(20, 20, 20, new_intensity),
                }

                if self.buf[y][x].color != 0 {
                    let mut rect = RectangleShape::new_init(&Vector2f::new(
                        context.block_size as f32,
                        context.block_size as f32,
                    ))
                    .unwrap();
                    rect.set_position(&Vector2f {
                        x: ((x as i32) * context.block_size) as f32,
                        y: ((y as i32) * context.block_size) as f32,
                    });
                    rect.set_fill_color(&rect_color);

                    let mut rs = RenderStates::default();
                    rs.blend_mode = BlendMode::blend_alpha();

                    canvas.draw_rectangle_shape(&rect, &mut rs);
                }
            }
        }

        Ok(())
    }

    fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
