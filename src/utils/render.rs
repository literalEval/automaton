use sfml::{
    graphics::{Color, RenderStates, RenderTarget, RenderTexture, Vertex},
    system::Vector2f,
};

use crate::GlobalContext;

pub fn draw_grid(context: &mut GlobalContext, canvas: &mut RenderTexture) -> Result<(), String> {
    for y in 0..context.grid_height {
        let mut line = [
            Vertex::new_with_pos_color(
                &Vector2f {
                    x: ((0 as i32) * context.block_size) as f32,
                    y: ((y as i32) * context.block_size) as f32,
                },
                &Color::black(),
            ),
            Vertex::new_with_pos_color(
                &Vector2f {
                    x: ((context.scr_width as i32) * context.block_size) as f32,
                    y: ((y as i32) * context.block_size) as f32,
                },
                &Color::black(),
            ),
        ];

        canvas.draw_primitives(
            &line,
            sfml::graphics::PrimitiveType::sfLines,
            &mut RenderStates::default(),
        );

        // context.canvas.draw_line(
        //     Point::new(
        //         (0 as i32) * context.block_size,
        //         (y as i32) * context.block_size,
        //     ),
        //     Point::new(
        //         (context.scr_width as i32) * context.block_size,
        //         (y as i32) * context.block_size,
        //     ),
        // )?;
    }

    for x in 0..context.grid_width {
        let mut line =
            sfml::graphics::VertexArray::new_init(sfml::graphics::PrimitiveType::sfLines, 2)
                .unwrap();

        line.append(&Vertex::new_with_pos_color(
            &Vector2f {
                x: ((x as i32) * context.block_size) as f32,
                y: ((0 as i32) * context.block_size) as f32,
            },
            &Color::black(),
        ));
        line.append(&Vertex::new_with_pos_color(
            &Vector2f {
                x: ((x as i32) * context.block_size) as f32,
                y: ((context.scr_height as i32) * context.block_size) as f32,
            },
            &Color::black(),
        ));

        canvas.draw(&line);

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

    Ok(())
}
