use ggez::{graphics, Context, GameResult, ContextBuilder};

use super::tower::Tower;

use crate::{
    world::{GridPosition}
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ArrowTower {
    pos: GridPosition
}

impl Tower for ArrowTower {
    fn new(pos: GridPosition) -> Self {
        ArrowTower { pos }
    }

    fn get_pos(&self) -> GridPosition{
        self.pos
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color = [0.3, 0.2, 0.3, 1.0].into();

        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.pos.into(), color)?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        Ok(())
    }
}