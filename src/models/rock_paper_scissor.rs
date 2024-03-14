use nannou::{
    color::{self, hsl, Rgb, Rgba}, draw, event::{Key, MouseButton}, geom::Point2, state::{mouse::ButtonMap, Keys, Window}, App, Draw
};
use rand::{rngs::ThreadRng, Rng};
use std::cmp::min;

use crate::{BuildContext, WindowInfo};

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
    pub fn new(window_info: &WindowInfo) -> Self {
        Self {
            cur_draw_color: 1,
            max_intensity: 6,
            rand_gen: rand::thread_rng(),
            buf: vec![
                vec![
                    BufData {
                        color: 0,
                        intensity: 6
                    };
                    window_info.grid_width as usize
                ];
                window_info.grid_height as usize
            ],
        }
    }

    pub fn handle_mouse_pressed(
        &mut self,
        window_info: &WindowInfo,
        button: MouseButton,
        pos: Point2,
    ) {
        if button == MouseButton::Left {
            self.buf[((pos.y + (window_info.height as f32 / -2.)).abs() as i32 / window_info.block_size)
                as usize][((pos.x + (window_info.width as f32 / 2.)) as i32
                / window_info.block_size) as usize]
                .color = self.cur_draw_color;
        }
    }

    pub fn handle_mouse_moved(
        &mut self,
        window_info: &WindowInfo,
        button: &ButtonMap,
        pos: Point2,
    ) {
        if button.left().is_down() {
            self.buf[((pos.y + (window_info.height as f32 / -2.)).abs() as i32 / window_info.block_size)
                as usize][((pos.x + (window_info.width as f32 / 2.)) as i32
                / window_info.block_size) as usize]
                .color = self.cur_draw_color;
        }
    }

    pub fn handle_key_pressed(&mut self, key: &Key) {
        match key {
            Key::R => self.cur_draw_color = 1,
            Key::G => self.cur_draw_color = 2,
            Key::B => self.cur_draw_color = 3,
            _ => self.cur_draw_color = 0,
        }
    }

    pub fn mutate(&mut self, window_info: &WindowInfo) {
        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let neighbour_y = self.rand_gen.gen_range(-1..=1) + (y as i32);
                let neighbour_x = self.rand_gen.gen_range(-1..=1) + (x as i32);

                if Self::in_bounds(
                    &neighbour_y,
                    &neighbour_x,
                    &window_info.grid_height,
                    &window_info.grid_width,
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
    }

    pub fn draw(&self, window_info: &WindowInfo, draw: &Draw, app: &App) {
        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let rect_color: Rgba<u8>;
                let new_intensity =
                    (150 + (0 + (100 * self.buf[y][x].intensity) / self.max_intensity)) as u8;

                match self.buf[y][x].color {
                    0 => rect_color = window_info.bg_color,
                    1 | 10 => rect_color = Rgba::new(246, 122, 17, new_intensity),
                    2 | 20 => rect_color = Rgba::new(208, 37, 37, new_intensity),
                    3 | 30 => rect_color = Rgba::new(35, 119, 181, new_intensity),
                    _ => rect_color = Rgba::new(20, 20, 20, new_intensity),
                }

                if self.buf[y][x].color != 0 {
                    draw.rect()
                        .color(rect_color)
                        .x_y(
                            ((x as i32) * window_info.block_size - window_info.width / 2) as f32,
                            (-(y as i32) * window_info.block_size + window_info.height / 2) as f32,
                        )
                        .width(window_info.block_size as f32)
                        .height(window_info.block_size as f32);
                }
            }
        }
    }

    fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
