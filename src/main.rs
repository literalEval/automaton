use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    sys::{KeyCode, SDL_KeyCode},
};

fn main() -> Result<(), String> {
    let (scr_width, scr_height) = (640, 480);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Automaton", scr_width, scr_height)
        .position_centered()
        .build()
        .expect("Failed to build window");

    let screen_area = Rect::new(0, 0, scr_width, scr_height);

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to build context");

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut buf: Vec<Vec<i32>> = vec![vec![0; 640]; 480];

    while running {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(screen_area)?;

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                Event::MouseMotion {
                    mousestate, x, y, ..
                } => {
                    // println!("{:?}", mousestate.left());
                    if mousestate.left() {
                        buf[((y / 4) * 4) as usize][((x / 4) * 4) as usize] = 1;
                    }
                }
                Event::KeyDown {
                    keycode,
                    scancode,
                    keymod,
                    ..
                } => if keycode.unwrap() == Keycode::R {},
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(20, 20, 20));

        for y in 0..buf.len() {
            for x in 0..buf[0].len() {
                if buf[y][x] == 1 {
                    canvas.fill_rect(Rect::new(x as i32, y as i32, 4, 4))?;
                }
            }
        }

        // canvas.fill_rect(Rect::new(100, 100, 100, 100))?;

        canvas.present();
    }

    Ok(())
}
