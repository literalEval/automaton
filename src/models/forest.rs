use std::f32::consts::PI;

use nannou::{
    color::{self, hsl, Rgb, Rgba},
    event::MouseButton,
    geom::Point2,
    state::mouse::ButtonMap,
    Draw, Event, Frame,
};
use rand::Rng;

use crate::BuildContext;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Root {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    size: f32,
    ds: f32,
    angle_x: f32,
    angle_y: f32,
    dax: f32,
    day: f32,
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

    fn create_root(pos: Point2) -> Root {
        let mut rand_gen = rand::thread_rng();

        Root {
            x: pos.x,
            y: pos.y,
            dx: rand_gen.gen_range(-2.0..=2.0),
            dy: rand_gen.gen_range(-2.0..=2.0),
            size: 0.1,
            ds: rand_gen.gen_range(0.1..0.2),
            angle_x: 0.,
            angle_y: 0.,
            dax: rand_gen.gen_range(0.1..=0.9),
            day: rand_gen.gen_range(0.1..=0.9),
            max_size: rand_gen.gen_range(3.0..=8.0),
        }
    }

    pub fn setup(&mut self, _context: &BuildContext) {}

    pub fn handle_mouse_pressed(&mut self, button: MouseButton, pos: Point2) {
        if button == MouseButton::Left {
            for _ in 0..9 {
                self.buf.push(Forest::create_root(pos));
            }
        }
    }

    pub fn handle_mouse_moved(&mut self, button: &ButtonMap, pos: Point2) {
        if button.left().is_down() {
            for _ in 0..=1 {
                self.buf.push(Forest::create_root(pos));
            }
        }
    }

    pub fn mutate(&mut self) {
        let mut old_buf = vec![];

        for cur_buf in &mut self.buf {
            if cur_buf.size.le(&cur_buf.max_size) {
                cur_buf.size += cur_buf.ds;
                cur_buf.x += cur_buf.dx + cur_buf.angle_x.sin();
                cur_buf.y += cur_buf.dy + cur_buf.angle_y.sin();
                cur_buf.angle_x += cur_buf.dax;
                cur_buf.angle_y += cur_buf.day;
                old_buf.push(cur_buf.clone());
            }
        }

        self.buf = old_buf;
    }

    pub fn draw(&self, draw: &mut Draw) {
        for cur_buf in &self.buf {
            draw.ellipse()
                .radius(cur_buf.size)
                .x_y(cur_buf.x, cur_buf.y)
                // Lightness
                //
                // .hsl(
                //     0.25,
                //     (cur_buf.size / cur_buf.max_size) / 1.5,
                //     1. - (cur_buf.size / cur_buf.max_size),
                // )
                //
                .hsl(
                    (cur_buf.size / cur_buf.max_size).cos(),
                    (cur_buf.size / cur_buf.max_size).sin(),
                    (cur_buf.size / cur_buf.max_size) / 2.,
                )
                .stroke(Rgba::new(
                    0.,
                    0.,
                    0.,
                    (cur_buf.size / cur_buf.max_size) * 0.5,
                ))
                .stroke_weight((cur_buf.size / cur_buf.max_size) * 0.5);
        }
    }

    pub fn _activate(&mut self) {}
}
