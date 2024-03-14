use nannou::{color::BLACK, glam::Vec2, Draw};

use crate::BuildContext;

pub fn draw_grid(draw: &mut Draw, context: &BuildContext) {
    for y in (context.window_info.grid_height / -2)..(context.window_info.grid_height / 2) {
        draw.line()
            .color(BLACK)
            .start(Vec2::new(
                ((context.window_info.width as i32) / -2 * context.window_info.block_size) as f32,
                ((y as i32) * context.window_info.block_size) as f32,
            ))
            .end(Vec2::new(
                ((context.window_info.width as i32) / 2 * context.window_info.block_size) as f32,
                ((y as i32) * context.window_info.block_size) as f32,
            ));
    }

    for x in (context.window_info.grid_width / -2)..(context.window_info.grid_width / 2) {
        draw.line()
            .color(BLACK)
            .start(Vec2::new(
                ((x as i32) * context.window_info.block_size)
                    as f32,
                ((context.window_info.height as i32) / -2 * context.window_info.block_size) as f32,
            ))
            .end(Vec2::new(
                ((x as i32) * context.window_info.block_size)
                    as f32,
                ((context.window_info.height as i32) / 2 * context.window_info.block_size) as f32,
            ));
        // context.canvas.draw_line(
        //     Point::new(
        //         (x as i32) * context.block_size,
        //         (0 as i32) * context.block_size,
        //     ),
        //     Point::new(
        //         (x as i32) * context.block_size,
        //         (context.scr_height as i32) * context.block_size,
        //     ),
        // )?;
    }
}
