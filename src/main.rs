use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const TEXT_OFFSET: Vec2<f32> = Vec2::new(16.0, 16.0);

struct GameState {
    // vector_text: Text,
    texts: Vec<Text>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // let vector_text = Text::new(
        //     "This is some text being rendered from a TTF font.",
        //     Font::vector(ctx, "./DejaVuSansMono.ttf", 16.0)?,
        // );

        let mut texts = vec![];
        for i in 1..51 {
            texts.push(Text::new(
                format!(
                    "{}: This is some text being rendered from a vector font.",
                    i
                ),
                Font::vector(ctx, "./DejaVuSansMono.ttf", 16.0)?,
                // NOTE: same behavior with a bitmap font as well
                // Font::bmfont(ctx, "./DejaVuSansMono.fnt")?,
            ));
        }

        Ok(GameState {
            texts,
            // vector_text,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        tetra::window::set_title(
            ctx,
            &format!("Text Render Perf ({:.0} FPS)", tetra::time::get_fps(ctx)),
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        for (i, text) in self.texts.iter_mut().enumerate() {
            let x = if i < 25 { 0.0 } else { 640.0 };
            let y = if i < 25 {
                (i as f32) * 18.0
            } else {
                ((i - 25) as f32) * 18.0
            };
            text.draw(ctx, TEXT_OFFSET + Vec2::new(x, y));
        }

        // NOTE: drawing the same text over and over does not impact performance
        // for i in 1..41 {
        //     let x = if i <= 20 { 0.0 } else { 640.0 };
        //     let y = if i <= 20 {
        //         (i as f32) * 32.0
        //     } else {
        //         ((i - 20) as f32) * 32.0
        //     };
        //     self.vector_text.draw(ctx, TEXT_OFFSET + Vec2::new(x, y));
        // }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Rendering Text", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
