use ggez::{graphics, Context, GameResult, ContextBuilder};

use crate::{
    world::GridPosition
};

pub trait Tower {
    fn new(pos: GridPosition) -> Self;

    fn get_pos(&self) -> GridPosition;

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color = [1.0, 1.0, 1.0, 1.0].into();

        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.get_pos().into(), color)?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 }, ));
        Ok(())
    }
}