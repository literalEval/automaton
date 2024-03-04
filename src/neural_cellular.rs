use rand::{rngs::ThreadRng, Rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    libc::abs,
    mouse::MouseButton,
    pixels::Color,
    rect::{Point, Rect},
};
use std::cmp::min;

use crate::GlobalContext;

const DELTA: [(i32, i32); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone, Copy, Debug)]
pub(crate) struct BufData {
    pub color: i32,
    pub intensity: f32,
}

pub(crate) struct NeuralCellular {
    cur_draw_color: i32,
    max_intensity: i32,
    buf: Vec<Vec<BufData>>,
    filter: [[f32; 3]; 3],
}

impl NeuralCellular {
    pub fn new() -> Self {
        Self {
            cur_draw_color: 3,
            max_intensity: 8,
            buf: vec![vec![BufData {
                color: 0,
                intensity: 0.,
            }]],
            // filter: [
            //     [0.80, -0.85, 0.80],
            //     [-0.85, -0.20, 0.85],
            //     [0.80, -0.85, 0.80],
            // ],
            // Path
            // filter: [[0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
            // filter: [[0.2, -0.1, -0.5], [-0.3, 1.0, 0.0], [0.3, -0.1, 0.1]],
            // filter: [[1.0, 1.0, 1.0], [1.0, 9.0, 1.0], [1.0, 1.0, 1.0]],

            // Wave
            filter: [
                [0.565, -0.716, 0.565],
                [-0.716, 0.627, -0.716],
                [0.565, -0.716, 0.565],
            ],
            // Worm
            // filter: [[0.68, -0.9, 0.68], [-0.9, -0.66, -0.9], [0.68, -0.9, 0.68]],
        }
    }

    pub fn setup(&mut self, context: &GlobalContext) {
        let mut rand_gen = rand::thread_rng();
        self.buf = vec![
            vec![
                BufData {
                    color: self.cur_draw_color,
                    intensity: 0.,
                };
                context.grid_width as usize
            ];
            context.grid_height as usize
        ];

        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                self.buf[y][x].intensity = rand_gen.gen_range(-1.0..=1.0);
                // self.buf[y][x].intensity = if rand_gen.gen_bool(0.5) { 1.0 } else { 0.0 };
            }
        }

        // println!("{:?}", self.buf);
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
                        [(x / context.block_size) as usize] = BufData {
                        color: self.cur_draw_color,
                        intensity: 1.,
                    };
                }
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                // println!("{:?}", self.buf);
                if mouse_btn == &MouseButton::Left {
                    self.buf[(y / context.block_size) as usize]
                        [(x / context.block_size) as usize] = BufData {
                        color: self.cur_draw_color,
                        intensity: 1.,
                    };
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

    pub fn draw(&mut self, context: &mut GlobalContext) -> Result<(), String> {
        let mut old_buf: Vec<Vec<BufData>> = self.buf.clone();

        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let mut new_intensity: f32 = 0.;

                for delta in DELTA {
                    let neighbour_y = y as i32 + delta.0;
                    let neighbour_x = x as i32 + delta.1;

                    // let neighbour_y =
                    //     ((y as i32 + delta.0) + context.grid_height) % context.grid_height;
                    // let neighbour_x =
                    //     ((x as i32 + delta.1) + context.grid_width) % context.grid_width;

                    if !Self::in_bounds(
                        &neighbour_y,
                        &neighbour_x,
                        &context.grid_height,
                        &context.grid_width,
                    ) {
                        continue;
                    }

                    new_intensity += self.buf[neighbour_y as usize][neighbour_x as usize].intensity
                        * self.filter[(delta.0 + 1) as usize][(delta.1 + 1) as usize];
                }

                old_buf[y][x].intensity = new_intensity;

                // if new_intensity > 1.0 {
                //     println!("{:?}", new_intensity);
                //     panic!("err");
                // }
            }
        }

        self.buf = old_buf;
        self.activate();

        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let rect_color: Color;

                // println!("{:?}", self.buf[y][x].intensity);
                let new_intensity = (self.buf[y][x].intensity * 200.).round() as u8;

                match self.buf[y][x].color {
                    0 => rect_color = context.bg_color,
                    1 | 10 => rect_color = Color::RGBA(246, 122, 17, new_intensity),
                    2 | 20 => rect_color = Color::RGBA(208, 37, 37, new_intensity),
                    3 | 30 => rect_color = Color::RGBA(35, 119, 181, new_intensity),
                    _ => rect_color = Color::RGBA(20, 20, 20, new_intensity),
                }

                if self.buf[y][x].color != 0 {
                    // println!("{:?}", new_intensity);
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

        // context.canvas.set_draw_color(Color::RGB(0, 0, 0));

        // for x in 0..self.buf[0].len() {
        //     context.canvas.draw_line(
        //         Point::new(
        //             (x as i32) * context.block_size,
        //             (0 as i32) * context.block_size,
        //         ),
        //         Point::new(
        //             (x as i32) * context.block_size,
        //             (context.scr_height as i32) * context.block_size,
        //         ),
        //     )?;
        // }

        // for y in 0..self.buf.len() {
        //     context.canvas.draw_line(
        //         Point::new(
        //             (0 as i32) * context.block_size,
        //             (y as i32) * context.block_size,
        //         ),
        //         Point::new(
        //             (context.scr_width as i32) * context.block_size,
        //             (y as i32) * context.block_size,
        //         ),
        //     )?;
        // }

        Ok(())
    }

    fn gaussian(&mut self, x: f32, b: f32) -> f32 {
        1. / (2.0_f32).powf((x - b).powf(2.))
    }

    fn inverse_gaussian(&mut self, x: f32) -> f32 {
        -1. / 2.0_f32.powf(0.6 * (x.powf(2.0))) + 1.
    }

    fn activate_conway(&mut self, x: f32) -> f32 {
        if x == 3. || x == 11. || x == 12. {
            return 1.;
        }
        return 0.;
    }

    fn activate_wave(&mut self, x: f32) -> f32 {
        f32::min(6., (1.2 * x).abs())
    }

    fn activate(&mut self) {
        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                // self.buf[y][x].intensity =
                //     -1. / (0.89 * self.buf[y][x].intensity.powf(2.0) + 1.) + 1.;

                // Path
                // self.buf[y][x].intensity = self.gaussian(self.buf[y][x].intensity, 0.35);

                // Worm
                // self.buf[y][x].intensity = self.inverse_gaussian(self.buf[y][x].intensity);

                // Wave
                self.buf[y][x].intensity = self.activate_wave(self.buf[y][x].intensity);
            }
        }
    }

    fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
