use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::cmp::min;

fn draw() {
    // canvas.set_draw_color(background_color);
    // canvas.fill_rect(screen_area)?;

    // for event in event_queue.poll_iter() {
    //     match event {
    //         Event::Quit { .. } => running = false,
    //         Event::MouseMotion {
    //             mousestate, x, y, ..
    //         } => {
    //             // println!("{:?}", mousestate.left());
    //             if mousestate.left() {
    //                 buf[(y / block_size) as usize][(x / block_size) as usize].color =
    //                     cur_draw_color;
    //             }
    //         }
    //         Event::KeyDown { keycode, .. } => match keycode.unwrap() {
    //             Keycode::R => cur_draw_color = 1,
    //             Keycode::G => cur_draw_color = 2,
    //             Keycode::B => cur_draw_color = 3,
    //             _ => cur_draw_color = 0,
    //         },
    //         _ => {}
    //     }
    // }

    // for y in 0..buf.len() {
    //     for x in 0..buf[0].len() {
    //         let neighbour_y = rand_gen.gen_range(-1..=1) + (y as i32);
    //         let neighbour_x = rand_gen.gen_range(-1..=1) + (x as i32);

    //         // println!("y: {:?}, x: {:?}", neighbour_y - (y as i32), neighbour_x - (x as i32));

    //         if in_bounds(&neighbour_y, &neighbour_x, &grid_height, &grid_width) {
    //             let mut neighbour_pixel = buf[neighbour_y as usize][neighbour_x as usize];

    //             match buf[y][x].color {
    //                 0 => {
    //                     if neighbour_pixel.color != 0 && neighbour_pixel.intensity > 0 {
    //                         buf[y][x].color = neighbour_pixel.color;
    //                         buf[y][x].intensity = neighbour_pixel.intensity - 1;
    //                     }
    //                 }
    //                 1 => {
    //                     if neighbour_pixel.color == 2 {
    //                         neighbour_pixel.color = 0;
    //                         neighbour_pixel.intensity = max_intensity;
    //                         buf[y][x].intensity = min(buf[y][x].intensity + 1, max_intensity);
    //                         // buf[y][x].intensity = 10;
    //                     }
    //                 }
    //                 2 => {
    //                     if neighbour_pixel.color == 3 {
    //                         neighbour_pixel.color = 0;
    //                         neighbour_pixel.intensity = max_intensity;
    //                         buf[y][x].intensity = min(buf[y][x].intensity + 1, max_intensity);
    //                         // buf[y][x].intensity = 10;
    //                     }
    //                 }
    //                 3 => {
    //                     if neighbour_pixel.color == 1 {
    //                         neighbour_pixel.color = 0;
    //                         neighbour_pixel.intensity = max_intensity;
    //                         buf[y][x].intensity = min(buf[y][x].intensity + 1, max_intensity);
    //                         // buf[y][x].intensity = 10;
    //                     }
    //                 }
    //                 _ => {}
    //             }

    //             buf[neighbour_y as usize][neighbour_x as usize] = neighbour_pixel;
    //         }
    //     }
    // }

    // for y in 0..buf.len() {
    //     for x in 0..buf[0].len() {
    //         let rect_color: Color;

    //         match buf[y][x].color {
    //             0 => rect_color = Color::RGBA(200, 200, 200, 0),
    //             1 => rect_color = Color::RGB(246, 122, 17),
    //             2 => rect_color = Color::RGB(208, 37, 37),
    //             3 => rect_color = Color::RGB(35, 119, 181),
    //             _ => rect_color = Color::RGB(20, 20, 20),
    //         }

    //         if buf[y][x].color != 0 {
    //             canvas.set_draw_color(rect_color);
    //             canvas.fill_rect(Rect::new(
    //                 (x as i32) * block_size,
    //                 (y as i32) * block_size,
    //                 block_size as u32,
    //                 block_size as u32,
    //             ))?;
    //         }
    //     }
    // }

    // canvas.set_draw_color(Color::RGB(0, 0, 0));

    // for x in 0..buf[0].len() {
    //     canvas.draw_line(
    //         Point::new((x as i32) * block_size, (0 as i32) * block_size),
    //         Point::new((x as i32) * block_size, (scr_height as i32) * block_size),
    //     )?;
    // }

    // for y in 0..buf.len() {
    //     canvas.draw_line(
    //         Point::new((0 as i32) * block_size, (y as i32) * block_size),
    //         Point::new((scr_width as i32) * block_size, (y as i32) * block_size),
    //     )?;
    // }

    // canvas.present();
}
