mod models;
mod utils;

use models::forest::Forest;
use models::neural_cellular::NeuralCellular;
use models::rock_paper_scissor::RockPaperScissor;

use sfml::graphics::{Color, RenderTarget, RenderTexture, RenderWindow, Sprite};
use sfml::window::{ContextSettings, WindowStyle};
use utils::*;

struct GlobalContext<'a> {
    scr_width: u32,
    scr_height: u32,
    grid_width: i32,
    grid_height: i32,
    block_size: i32,
    bg_color: &'a Color,
    running: bool,
    is_playing: bool,
    draw_grid_lines: bool,
    // canvas: &'a mut Canvas<Window>,
    // canvas: &'a mut RenderTexture,
}

impl<'a> GlobalContext<'a> {
    fn new(
        scr_w: u32,
        scr_h: u32,
        block_size: i32,
        bg_color: &'a Color,
        // canvas: &'a mut RenderTexture,
    ) -> Self {
        GlobalContext {
            scr_width: scr_w,
            scr_height: scr_h,
            block_size,
            grid_width: scr_w as i32 / block_size,
            grid_height: scr_h as i32 / block_size,
            bg_color,
            running: true,
            is_playing: true,
            draw_grid_lines: false,
            // canvas,
        }
    }
}

fn main() -> Result<(), String> {
    let scr_width = 1080;
    let scr_height = 720;
    let bg_color = Color::new_rgba(255, 255, 255, 255);

    let mut sfml_win = RenderWindow::new(
        sfml::window::VideoMode {
            width: 1080,
            height: 720,
            bits_per_pixel: 100,
        },
        "Automaton",
        WindowStyle::default(),
        &ContextSettings::default(),
    )
    .unwrap();

    let mut canvas = RenderTexture::new(1080, 720, false).unwrap();
    canvas.set_smooth(true);
    canvas.clear(&bg_color);
    canvas.display();

    sfml_win.get_settings().0.antialiasing_level = 10;

    let mut global_c = GlobalContext::new(scr_width, scr_height, 6, &bg_color);

    let mut rps = RockPaperScissor::new();
    let mut nca = NeuralCellular::new();
    let mut forest = Forest::new();

    rps.setup(&global_c);
    nca.setup(&global_c);
    forest.setup(&global_c);

    let mut frame = 0;

    while global_c.running {
        for ev in sfml_win.events() {
            forest.handle_event(&ev, &mut global_c);
            // nca.handle_event(&ev, &mut global_c);
            // rps.handle_event(&ev, &mut global_c);
        }

        if frame == 20 {
            forest.mutate(&global_c);
        }

        forest.draw(&mut global_c, &mut canvas)?;

        // canvas.clear(&bg_color);
        // nca.mutate(&mut global_c);
        // nca.activate();
        // nca.draw(&mut global_c, &mut canvas)?;

        // rps.draw(&mut global_c, &mut canvas)?;

        sfml_win.clear(&Color::white());
        sfml_win.draw(&Sprite::new_with_texture(&canvas.get_texture().unwrap()).unwrap());

        if global_c.draw_grid_lines {
            render::draw_grid(&mut global_c, &mut canvas)?;
        }

        sfml_win.display();

        frame = (frame + 1) % 21;
    }

    sfml_win.close();
    Ok(())
}
