mod models;
mod utils;

use models::forest::Forest;
use models::neural_cellular::NeuralCellular;
use models::rock_paper_scissor::RockPaperScissor;

use nannou::color::Rgba;
use nannou::prelude::*;
use nannou::wgpu::BlendComponent;
use utils::*;

#[derive(Clone)]
struct WindowInfo {
    width: i32,
    height: i32,
    grid_width: i32,
    grid_height: i32,
    block_size: i32,
    bg_color: Rgba<u8>,
    is_playing: bool,
}

enum Model {
    FOREST(Forest),
    NCA(NeuralCellular),
    RPS(RockPaperScissor),
}

struct BuildContext {
    window_info: WindowInfo,
    draw_grid_lines: bool,
    model: Model,
    window_id: WindowId,
}

fn build_context(app: &App) -> BuildContext {
    let window_id = app
        .new_window()
        .size(640, 480)
        // .clear_color(Rgb::new(1., 1., 1.))
        .mouse_pressed(handle_mouse_pressed)
        .mouse_moved(handle_mouse_moved)
        .key_pressed(handle_key_pressed)
        .view(view)
        .build()
        .unwrap();

    let window_info = WindowInfo {
        width: 640,
        height: 480,
        grid_width: 640 / 3,
        grid_height: 480 / 3,
        block_size: 3,
        bg_color: Rgba::new(255, 255, 255, 255),
        is_playing: true,
    };

    BuildContext {
        window_info: window_info.clone(),
        draw_grid_lines: false,
        // model: Model::forest(Forest::new()),
        // model: Model::rps(RockPaperScissor::new(&window_info)),
        model: Model::NCA(NeuralCellular::new(&window_info)),
        window_id,
    }
}

fn handle_mouse_pressed(app: &App, context: &mut BuildContext, button: MouseButton) {
    match &mut context.model {
        Model::FOREST(ref mut f) => f.handle_mouse_pressed(button, app.mouse.position()),
        Model::RPS(ref mut r) => {
            r.handle_mouse_pressed(&context.window_info, button, app.mouse.position())
        }
        Model::NCA(ref mut n) => {
            n.handle_mouse_pressed(&context.window_info, button, app.mouse.position())
        }
        _ => {}
    }
}

fn handle_mouse_moved(app: &App, context: &mut BuildContext, pos: Point2) {
    match &mut context.model {
        Model::FOREST(ref mut f) => f.handle_mouse_moved(&app.mouse.buttons, pos),
        Model::RPS(ref mut r) => {
            r.handle_mouse_moved(&context.window_info, &app.mouse.buttons, pos)
        }
        Model::NCA(ref mut n) => {
            n.handle_mouse_moved(&context.window_info, &app.mouse.buttons, pos)
        }
        _ => {}
    }
}

fn handle_key_pressed(app: &App, context: &mut BuildContext, key: Key) {
    match &mut context.model {
        Model::RPS(ref mut r) => r.handle_key_pressed(&key),
        Model::NCA(ref mut n) => n.handle_key_pressed(&mut context.window_info, &key),
        _ => {}
    }
}

fn update(_app: &App, context: &mut BuildContext, _update: Update) {
    if !context.window_info.is_playing {
        return;
    }

    match &mut context.model {
        Model::FOREST(ref mut f) => f.mutate(),
        Model::RPS(ref mut r) => r.mutate(&context.window_info),
        Model::NCA(ref mut n) => {
            n.mutate(&context.window_info);
            n.activate();
        }
        _ => {}
    }
}

fn view(app: &App, context: &BuildContext, frame: Frame) {
    let mut draw = app.draw().blend(BlendComponent::OVER);

    match &context.model {
        Model::FOREST(ref f) => f.draw(&mut draw),
        Model::RPS(ref r) => r.draw(&context.window_info, &mut draw, &app),
        Model::NCA(ref n) => {
            frame.clear(context.window_info.bg_color);
            n.draw(&context.window_info, &mut draw, &app)
        }
        _ => {}
    }

    if context.draw_grid_lines {
        render::draw_grid(&mut draw, &context);
    }

    match draw.to_frame(&app, &frame) {
        Result::Ok(_) => {}
        Result::Err(e) => println!("{:?}", e),
    }
}

fn main() {
    nannou::app(build_context).update(update).view(view).run();
}
