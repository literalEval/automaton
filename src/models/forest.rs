use rand::Rng;
use sfml::{
    graphics::{CircleShape, Color, RenderTarget, RenderTexture, Shape, Texture, Transformable},
    window::{event::Event, Key, MouseButton},
};

use crate::GlobalContext;

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

    pub fn setup(&mut self, _context: &GlobalContext) {}

    pub fn handle_event(&mut self, event: &Event, context: &mut GlobalContext) {
        let mut rand_gen = rand::thread_rng();

        match event {
            Event::Closed { .. } => context.running = false,
            Event::MouseMoved { x, y } => {
                if MouseButton::Left.is_pressed() {
                    for _ in 0..1 {
                        self.buf.push(Root {
                            x: *x as f32,
                            y: *y as f32,
                            dx: rand_gen.gen_range(-2.0..=2.0),
                            dy: rand_gen.gen_range(-2.0..=2.0),
                            size: 0.1,
                            ds: rand_gen.gen_range(0.1..0.2),
                            angle_x: 0.,
                            angle_y: 0.,
                            dax: rand_gen.gen_range(0.1..=0.9),
                            day: rand_gen.gen_range(0.1..=0.9),
                            max_size: rand_gen.gen_range(3.0..=8.0),
                        });
                    }
                }
            }
            Event::MouseButtonPressed { button, x, y } => {
                if button == &MouseButton::Left {
                    for _ in 0..9 {
                        self.buf.push(Root {
                            x: *x as f32,
                            y: *y as f32,
                            dx: rand_gen.gen_range(-2.0..=2.0),
                            dy: rand_gen.gen_range(-2.0..=2.0),
                            size: 0.1,
                            ds: rand_gen.gen_range(0.1..0.2),
                            angle_x: 0.,
                            angle_y: 0.,
                            dax: rand_gen.gen_range(0.1..=0.9),
                            day: rand_gen.gen_range(0.1..=0.9),
                            max_size: rand_gen.gen_range(8.0..=9.0),
                        });
                    }
                }
            }
            Event::KeyPressed { code, .. } => match code {
                Key::R => self.cur_draw_color = 1,
                Key::G => self.cur_draw_color = 2,
                Key::B => self.cur_draw_color = 3,
                Key::P => context.is_playing = !context.is_playing,
                _ => self.cur_draw_color = 0,
            },
            _ => {}
        }
    }

    pub fn mutate(&mut self, context: &GlobalContext) {
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

    pub fn draw(
        &mut self,
        _context: &mut GlobalContext,
        canvas: &mut RenderTexture,
    ) -> Result<(), String> {
        for cur_buf in &self.buf {
            let mut circle = CircleShape::new().unwrap();
            circle.set_radius(cur_buf.size);
            circle.set_position2f(cur_buf.x, cur_buf.y);
            circle.set_fill_color(&Color::new_rgb(10, (17. * cur_buf.size) as u8, 10));
            circle.set_outline_thickness(1.2);
            circle.set_outline_color(&Color::black());

            canvas.draw(&circle);
        }

        Ok(())
    }

    pub fn _activate(&mut self) {}

    fn _in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
        y >= &0 && x >= &0 && y < grid_h && x < grid_w
    }
}
