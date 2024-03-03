mod rock_paper_scissor;

use rock_paper_scissor::RockPaperScissor;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

struct GlobalContext<'a> {
    scr_width: u32,
    scr_height: u32,
    grid_width: i32,
    grid_height: i32,
    block_size: i32,
    screen_area: Rect,
    bg_color: Color,
    running: bool,
    canvas: &'a mut Canvas<Window>,
}

impl<'a> GlobalContext<'a> {
    fn new(
        scr_w: u32,
        scr_h: u32,
        block_size: i32,
        bg_color: Color,
        canvas: &'a mut Canvas<Window>,
    ) -> Self {
        GlobalContext {
            scr_width: scr_w,
            scr_height: scr_h,
            block_size,
            grid_width: scr_w as i32 / block_size,
            grid_height: scr_h as i32 / block_size,
            screen_area: Rect::new(0, 0, scr_w, scr_h),
            bg_color,
            running: true,
            canvas,
        }
    }
}

fn main() -> Result<(), String> {
    let scr_width = 1080;
    let scr_height = 720;
    let bg_color = Color::RGBA(197, 214, 220, 255);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Automaton", scr_width, scr_height)
        .position_centered()
        .build()
        .expect("Failed to build window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to build context");

    let mut global_c = GlobalContext::new(scr_width, scr_height, 4, bg_color, &mut canvas);
    global_c
        .canvas
        .set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut rps = RockPaperScissor::new();
    rps.setup(&global_c);

    while global_c.running {
        global_c.canvas.set_draw_color(global_c.bg_color);
        global_c.canvas.fill_rect(global_c.screen_area)?;

        for event in event_queue.poll_iter() {
            rps.handle_event(&event, &mut global_c);
        }

        rps.draw(&mut global_c)?;
        global_c.canvas.present();
    }

    Ok(())
}
