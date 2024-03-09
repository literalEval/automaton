use sdl2::{pixels::Color, rect::Point};

use crate::GlobalContext;

pub fn draw_grid(context: &mut GlobalContext) -> Result<(), String> {
    context.canvas.set_draw_color(Color::RGB(0, 0, 0));

    for y in 0..context.grid_height {
        context.canvas.draw_line(
            Point::new(
                (0 as i32) * context.block_size,
                (y as i32) * context.block_size,
            ),
            Point::new(
                (context.scr_width as i32) * context.block_size,
                (y as i32) * context.block_size,
            ),
        )?;
    }

    for x in 0..context.grid_width {
        context.canvas.draw_line(
            Point::new(
                (x as i32) * context.block_size,
                (0 as i32) * context.block_size,
            ),
            Point::new(
                (x as i32) * context.block_size,
                (context.scr_height as i32) * context.block_size,
            ),
        )?;
    }

    Ok(())
}
