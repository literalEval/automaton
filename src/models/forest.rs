use std::f32::consts::PI;

use nannou::{
    color::{self, hsl, Rgb, Rgba, BLACK},
    event::MouseButton,
    geom::{Point2, Rect},
    glam::Vec2,
    state::mouse::ButtonMap,
    wgpu::{self, Texture},
    App, Draw, Event, Frame,
};
use rand::Rng;

use crate::BuildContext;

#[derive(Clone, Debug, Copy)]
enum RootType {
    STEM,
    FLOWER,
}

#[derive(Clone, Copy, Debug)]
struct Root {
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
    root_type: RootType,
    flower_type: u8,
}

pub(crate) struct Forest {
    cur_draw_color: i32,
    buf: Vec<Root>,
    wrap_on_edge: bool,
    flowers: Texture,
}

impl Forest {
    pub fn new(app: &App) -> Self {
        let assets = app.assets_path().unwrap();
        let img_path = assets.join("flowers.png");
        let flowers = wgpu::Texture::from_path(app, img_path).unwrap();

        Self {
            cur_draw_color: 2,
            buf: vec![],
            wrap_on_edge: true,
            flowers,
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
            root_type: RootType::STEM,
            flower_type: 0,
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
        let mut new_buf = vec![];

        for cur_buf in &mut self.buf {
            match &cur_buf.root_type {
                RootType::STEM => {
                    cur_buf.size += cur_buf.ds;
                    cur_buf.x += cur_buf.dx + cur_buf.angle_x.sin();
                    cur_buf.y += cur_buf.dy + cur_buf.angle_y.sin();
                    cur_buf.angle_x += cur_buf.dax;
                    cur_buf.angle_y += cur_buf.day;

                    if cur_buf.size.ge(&cur_buf.max_size) {
                        cur_buf.size = 1.;
                        cur_buf.root_type = RootType::FLOWER;

                        let mut rand_gen = rand::thread_rng();
                        cur_buf.flower_type = rand_gen.gen_range(0..=8);
                    }

                    new_buf.push(cur_buf.clone());
                }

                RootType::FLOWER => {
                    cur_buf.size += cur_buf.ds * 2.;
                    cur_buf.angle_x += cur_buf.dax;

                    if cur_buf.size.le(&(cur_buf.max_size * 4.)) {
                        new_buf.push(cur_buf.clone());
                    }
                }
            }
        }

        self.buf = new_buf;
    }

    pub fn draw(&self, app: &App, draw: &mut Draw) {
        for cur_buf in &self.buf {
            match &cur_buf.root_type {
                RootType::STEM => {
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
                            0.35,
                            (cur_buf.size / cur_buf.max_size).cos(),
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

                RootType::FLOWER => {
                    // TODO: Find out why this 16.5% translation is needed.
                    let crop_area = Rect::from_xy_wh(
                        Point2::new(
                            (cur_buf.flower_type % 3) as f32 / 3. + 0.165,
                            (cur_buf.flower_type / 3) as f32 / 3. + 0.165,
                        ),
                        Vec2::new(0.3333, 0.3333),
                    );

                    draw.texture(&self.flowers)
                        .area(crop_area)
                        .width(cur_buf.size)
                        .height(cur_buf.size)
                        // .rotate(cur_buf.angle_x)
                        .x(cur_buf.x)
                        .y(cur_buf.y);
                }
            }
        }
    }

    pub fn _activate(&mut self) {}
}
