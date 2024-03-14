use nannou::{
    color::Rgba,
    event::{Key, MouseButton},
    geom::Point2,
    state::mouse::ButtonMap,
    App, Draw,
};
use rand::Rng;

use crate::{BuildContext, WindowInfo};

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
    buf: Vec<Vec<BufData>>,
    filter: [[f32; 3]; 3],
    wrap_on_edge: bool,
}

impl NeuralCellular {
    pub fn new(window_info: &WindowInfo) -> Self {
        let mut rand_gen = rand::thread_rng();

        let mut buf = vec![
            vec![
                BufData {
                    color: 2,
                    intensity: 0.,
                };
                window_info.grid_width as usize
            ];
            window_info.grid_height as usize
        ];

        for y in 0..buf.len() {
            for x in 0..buf[0].len() {
                buf[y][x].intensity = rand_gen.gen_range(-1.0..=1.0);
            }
        }

        Self {
            cur_draw_color: 2,
            buf,
            wrap_on_edge: true,
            // filter: [
            //     [0.80, -0.85, 0.80],
            //     [-0.85, -0.20, 0.85],
            //     [0.80, -0.85, 0.80],
            // ],
            // Path
            // filter: [[0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]],
            // filter: [[0.2, -0.1, -0.5], [-0.3, 1.0, 0.0], [0.3, -0.1, 0.1]],
            // Conway
            // filter: [[1.0, 1.0, 1.0], [1.0, 9.0, 1.0], [1.0, 1.0, 1.0]],
            // Wave
            // filter: [
            //     [0.565, -0.716, 0.565],
            //     [-0.716, 0.627, -0.716],
            //     [0.565, -0.716, 0.565],
            // ],
            // Worm
            filter: [[0.68, -0.9, 0.68], [-0.9, -0.66, -0.9], [0.68, -0.9, 0.68]],
        }
    }

    pub fn setup(&mut self, window_info: &WindowInfo) {
        let mut rand_gen = rand::thread_rng();
        self.buf = vec![
            vec![
                BufData {
                    color: self.cur_draw_color,
                    intensity: 0.,
                };
                window_info.grid_width as usize
            ];
            window_info.grid_height as usize
        ];

        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                self.buf[y][x].intensity = rand_gen.gen_range(-1.0..=1.0);
            }
        }
    }

    pub fn handle_mouse_pressed(
        &mut self,
        window_info: &WindowInfo,
        button: MouseButton,
        pos: Point2,
    ) {
        if button == MouseButton::Left {
            self.buf[((pos.y + (window_info.height as f32 / -2.)).abs() as i32
                / window_info.block_size) as usize][((pos.x
                + (window_info.width as f32 / 2.))
                as i32
                / window_info.block_size)
                as usize] = BufData {
                color: self.cur_draw_color,
                intensity: 1.,
            }
        }
    }

    pub fn handle_mouse_moved(
        &mut self,
        window_info: &WindowInfo,
        button: &ButtonMap,
        pos: Point2,
    ) {
        if button.left().is_down() {
            self.buf[((pos.y + (window_info.height as f32 / -2.)).abs() as i32
                / window_info.block_size) as usize][((pos.x
                + (window_info.width as f32 / 2.))
                as i32
                / window_info.block_size)
                as usize] = BufData {
                color: self.cur_draw_color,
                intensity: 1.,
            }
        }
    }

    pub fn handle_key_pressed(&mut self, window_info: &mut WindowInfo, key: &Key) {
        match key {
            Key::R => self.cur_draw_color = 1,
            Key::G => self.cur_draw_color = 2,
            Key::B => self.cur_draw_color = 3,
            Key::P => window_info.is_playing = !window_info.is_playing,
            _ => self.cur_draw_color = 0,
        }
    }

    pub fn mutate(&mut self, window_info: &WindowInfo) {
        let mut old_buf: Vec<Vec<BufData>> = self.buf.clone();

        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let mut new_intensity: f32 = 0.;

                for delta in DELTA {
                    let (neighbour_y, neighbour_x): (i32, i32);

                    if self.wrap_on_edge {
                        neighbour_y = ((y as i32 + delta.0) + window_info.grid_height)
                            % window_info.grid_height;
                        neighbour_x = ((x as i32 + delta.1) + window_info.grid_width)
                            % window_info.grid_width;
                    } else {
                        neighbour_y = y as i32 + delta.0;
                        neighbour_x = x as i32 + delta.1;

                        if !Self::in_bounds(
                            &neighbour_y,
                            &neighbour_x,
                            &window_info.grid_height,
                            &window_info.grid_width,
                        ) {
                            continue;
                        }
                    }

                    new_intensity += self.buf[neighbour_y as usize][neighbour_x as usize].intensity
                        * self.filter[(delta.0 + 1) as usize][(delta.1 + 1) as usize];
                }

                old_buf[y][x].intensity = new_intensity;
            }
        }

        self.buf = old_buf;
    }

    pub fn draw(&self, window_info: &WindowInfo, draw: &Draw, app: &App) {
        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                let rect_color: Rgba<u8>;
                let new_intensity = (self.buf[y][x].intensity * 200.).round() as u8;

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
        0.
    }

    fn activate_wave(&mut self, x: f32) -> f32 {
        f32::min(6., (1.2 * x).abs())
    }

    pub fn activate(&mut self) {
        for y in 0..self.buf.len() {
            for x in 0..self.buf[0].len() {
                // self.buf[y][x].intensity =
                //     -1. / (0.89 * self.buf[y][x].intensity.powf(2.0) + 1.) + 1.;

                // Path
                // self.buf[y][x].intensity = self.gaussian(self.buf[y][x].intensity, 0.35);

                // Worm
                self.buf[y][x].intensity = self.inverse_gaussian(self.buf[y][x].intensity);

                // Wave
                // self.buf[y][x].intensity = self.activate_wave(self.buf[y][x].intensity);

                // Conway
                // self.buf[y][x].intensity = self.activate_conway(self.buf[y][x].intensity);
            }
        }
    }

    fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
