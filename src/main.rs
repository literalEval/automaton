mod rock_paper_scissor;

use std::cmp::min;

use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

#[derive(Clone, Copy)]
struct BufData {
    color: i32,
    intensity: i32,
}

fn in_bounds(y: &i32, x: &i32, grid_h: &i32, grid_w: &i32) -> bool {
    y >= &0 && x >= &0 && y < grid_h && x < grid_w
}

fn main() -> Result<(), String> {
    let (scr_width, scr_height) = (1080, 720);
    let block_size: i32 = 10;
    let max_intensity: i32 = 3;

    let (grid_width, grid_height) = (
        scr_width as i32 / block_size,
        scr_height as i32 / block_size,
    );

    let background_color = Color::RGBA(54, 69, 79, 255);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Automaton", scr_width, scr_height)
        .position_centered()
        .build()
        .expect("Failed to build window");

    let mut rand_gen = rand::thread_rng();

    let screen_area = Rect::new(0, 0, scr_width, scr_height);

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to build context");

    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();
    let mut cur_draw_color = 1;

    let mut buf: Vec<Vec<BufData>> = vec![
        vec![
            BufData {
                color: 0,
                intensity: max_intensity
            };
            grid_width as usize
        ];
        grid_height as usize
    ];

    while running {
        canvas.set_draw_color(background_color);
        canvas.fill_rect(screen_area)?;

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::MouseMotion {
                    mousestate, x, y, ..
                } => {
                    // println!("{:?}", mousestate.left());
                    if mousestate.left() {
                        buf[(y / block_size) as usize][(x / block_size) as usize].color =
                            cur_draw_color;
                    }
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::R => cur_draw_color = 1,
                    Keycode::G => cur_draw_color = 2,
                    Keycode::B => cur_draw_color = 3,
                    _ => cur_draw_color = 0,
                },
                _ => {}
            }
        }

        for y in 0..buf.len() {
            for x in 0..buf[0].len() {
                let neighbour_y = rand_gen.gen_range(-1..=1) + (y as i32);
                let neighbour_x = rand_gen.gen_range(-1..=1) + (x as i32);

                // println!("y: {:?}, x: {:?}", neighbour_y - (y as i32), neighbour_x - (x as i32));

                if in_bounds(&neighbour_y, &neighbour_x, &grid_height, &grid_width) {
                    let mut neighbour_pixel = buf[neighbour_y as usize][neighbour_x as usize];

                    match buf[y][x].color {
                        0 => {
                            if neighbour_pixel.color != 0 && neighbour_pixel.intensity > 0 {
                                buf[y][x].color = neighbour_pixel.color;
                                buf[y][x].intensity = neighbour_pixel.intensity - 1;
                            }
                        }
                        1 => {
                            if neighbour_pixel.color == 2 {
                                neighbour_pixel.color = 0;
                                neighbour_pixel.intensity = max_intensity;
                                buf[y][x].intensity = min(buf[y][x].intensity + 1, max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        2 => {
                            if neighbour_pixel.color == 3 {
                                neighbour_pixel.color = 0;
                                neighbour_pixel.intensity = max_intensity;
                                buf[y][x].intensity = min(buf[y][x].intensity + 1, max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        3 => {
                            if neighbour_pixel.color == 1 {
                                neighbour_pixel.color = 0;
                                neighbour_pixel.intensity = max_intensity;
                                buf[y][x].intensity = min(buf[y][x].intensity + 1, max_intensity);
                                // buf[y][x].intensity = 10;
                            }
                        }
                        _ => {}
                    }

                    buf[neighbour_y as usize][neighbour_x as usize] = neighbour_pixel;
                }
            }
        }

        for y in 0..buf.len() {
            for x in 0..buf[0].len() {
                let rect_color: Color;

                match buf[y][x].color {
                    0 => rect_color = Color::RGBA(200, 200, 200, 0),
                    1 => {
                        rect_color =
                            Color::RGBA(200, 20, 20, 130 + 30 * (buf[y][x].intensity) as u8)
                    }
                    2 => {
                        rect_color =
                            Color::RGBA(20, 200, 20, 130 + 30 * (buf[y][x].intensity) as u8)
                    }
                    3 => {
                        rect_color =
                            Color::RGBA(20, 20, 200, 130 + 30 * (buf[y][x].intensity) as u8)
                    }
                    _ => {
                        rect_color = Color::RGBA(20, 20, 20, 130 + 30 * (buf[y][x].intensity) as u8)
                    }
                }

                canvas.set_draw_color(rect_color);
                canvas.fill_rect(Rect::new(
                    (x as i32) * block_size,
                    (y as i32) * block_size,
                    block_size as u32,
                    block_size as u32,
                ))?;
            }
        }

        canvas.present();
    }

    Ok(())
}
