use sdl2::{event::Event, pixels::Color, rect::Rect};

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

    while running {

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                _ => {},
            }
        }

        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(screen_area)?;

        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.fill_rect(Rect::new(100, 100, 100, 100))?;

        canvas.present();
    }

    Ok(())
}
