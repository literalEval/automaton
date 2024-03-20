use std::f32::consts::PI;

use nannou::{
    color::Rgba,
    event::{Key, MouseButton},
    geom::Point2,
    glam::Vec2,
    math::num_traits::Pow,
    state::mouse::ButtonMap,
    Draw,
};
use rand::Rng;

use crate::WindowInfo;

#[derive(Copy, Clone)]
pub(self) struct Particle {
    pos: Point2,
    v: Vec2,
    mass: f32,
    color: Rgba<u8>,
    radius: f32,
    particle_type: u8,
    elasticity: f32,
}

pub(crate) struct Gravitation {
    pub bg_color: Rgba<u8>,
    particles: Vec<Vec<Particle>>,
    fallback_colors: Vec<Rgba<u8>>,
    force_rules: Vec<Vec<f32>>,
    particle_type_cnt: u8,
    cur_particle_type: u8,
}

impl Gravitation {
    pub fn new() -> Self {
        Self {
            particles: vec![vec![], vec![], vec![], vec![]],
            fallback_colors: vec![
                Rgba::new(246, 122, 17, 255),
                Rgba::new(208, 37, 37, 255),
                Rgba::new(35, 119, 181, 255),
                Rgba::new(150, 150, 150, 255),
            ],
            bg_color: Rgba::new(0, 0, 0, 255),
            force_rules: vec![
                vec![-0.1, -0.00, -0.34, -0.1],
                vec![-0.00, 0.15, -0.2, 0.4],
                vec![-0.17, 0.34, -0.32, 0.2],
                vec![-0.1, -0.1, -0.1, 0.1],
            ],
            particle_type_cnt: 4,
            cur_particle_type: 0,
        }
    }

    fn create_particle(&self, particle_type: u8, pos: Point2) -> Particle {
        let mut rand_gen = rand::thread_rng();
        let mass = 16.;

        Particle {
            pos,
            v: Point2::new(0., 0.),
            mass,
            color: self.fallback_colors[particle_type as usize],
            radius: 4.,
            particle_type: self.cur_particle_type,
            elasticity: rand_gen.gen_range(0.2..0.9),
        }
    }

    pub fn setup(&mut self) {
        self.particles = vec![vec![], vec![], vec![], vec![]];

        let mut rand_gen = rand::thread_rng();

        for p_type in 0..4 {
            for _ in 0..500 {
                let particle = self.create_particle(
                    p_type,
                    Vec2::new(
                        rand_gen.gen_range(-320.0..320.0),
                        rand_gen.gen_range(-240.0..240.0),
                    ),
                );

                self.particles[p_type as usize].push(particle);
            }
        }
    }

    pub fn handle_mouse_pressed(&mut self, button: MouseButton, pos: Point2) {
        if button == MouseButton::Left {
            let new_particle = self.create_particle(self.cur_particle_type, pos);
            self.particles[new_particle.particle_type as usize].push(new_particle);
        }
    }

    pub fn handle_mouse_moved(&mut self, button: &ButtonMap, pos: Point2) {
        if button.left().is_down() {
            let new_particle = self.create_particle(self.cur_particle_type, pos);
            // self.particles[new_particle.particle_type as usize].push(new_particle);
        }
    }

    pub fn handle_key_pressed(&mut self, window_info: &mut WindowInfo, key: &Key) {
        match key {
            Key::P => window_info.is_playing = !window_info.is_playing,
            Key::R => self.setup(),
            Key::Key1 => self.cur_particle_type = 0,
            Key::Key2 => self.cur_particle_type = 1,
            Key::Key3 => self.cur_particle_type = 2,
            Key::Key4 | _ => self.cur_particle_type = 3,
        }
    }

    pub fn mutate(&mut self) {
        for p_type in 0..self.particle_type_cnt as usize {
            for particle_one_ind in 0..self.particles[p_type].len() {
                for n_type in 0..(self.particle_type_cnt - 0) as usize {
                    for particle_two_ind in 0..self.particles[n_type].len() {
                        if p_type == n_type && particle_one_ind == particle_two_ind {
                            continue;
                        }

                        let particle_one = self.particles[p_type][particle_one_ind];
                        let mut particle_two = self.particles[n_type][particle_two_ind];

                        let perpendicular = particle_one.pos.y - particle_two.pos.y;
                        let base = particle_one.pos.x - particle_two.pos.x;

                        let angle = if (base - 0.).abs() <= f32::EPSILON {
                            if (perpendicular - 0.).abs() <= f32::EPSILON {
                                0.
                            } else {
                                PI / 2.
                            }
                        } else {
                            (perpendicular / base).abs().atan()
                        };

                        let dist: f32 = (perpendicular.pow(2) as f32 + base.pow(2) as f32).sqrt();
                        let center_dist = particle_one.radius + particle_two.radius;

                        if dist <= 0.1 || dist >= 80. {
                            continue;
                        }

                        let mutual_force = (self.force_rules[p_type as usize][n_type as usize]
                            * 0.01
                            * particle_one.mass
                            * particle_two.mass)
                            / dist;

                        let mut new_v = Vec2::new(
                            mutual_force * dist * angle.cos() * base.signum() * -1.,
                            mutual_force * dist * angle.sin() * perpendicular.signum() * -1.,
                        );

                        if dist <= (center_dist + 1.) {
                            let mutual_elasticity = (particle_one.elasticity.pow(2) as f32
                                + particle_two.elasticity.pow(2) as f32)
                                .sqrt();

                            new_v *= mutual_elasticity;
                        }

                        particle_two.v += new_v;

                        self.particles[n_type][particle_two_ind] = particle_two;
                    }
                }
            }
        }

        for p_type in 0..self.particle_type_cnt as usize {
            for particle in self.particles[p_type].iter_mut() {
                particle.v *= 0.5;
                particle.pos += particle.v;

                if particle.pos.x < -320. + particle.radius
                    || particle.pos.x >= 320. - particle.radius
                {
                    particle.v.x *= -1.;
                    particle.pos.x += 2. * particle.v.x;
                }

                if particle.pos.y < -240. + particle.radius
                    || particle.pos.y >= 240. - particle.radius
                {
                    particle.v.y *= -1.;
                    particle.pos.y += 2. * particle.v.y;
                }
            }
        }
    }

    pub fn draw(&self, draw: &mut Draw) {
        for p_type in 0..self.particle_type_cnt as usize {
            for particle in self.particles[p_type].iter() {
                draw.ellipse()
                    .w_h(particle.radius, particle.radius)
                    .color(particle.color)
                    .xy(particle.pos);
            }
        }
    }
}
