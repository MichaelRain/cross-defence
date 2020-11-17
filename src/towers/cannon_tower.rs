use ggez::{graphics, Context, GameResult, ContextBuilder};

use super::tower::Tower;

use crate::{
    world::{GridPosition}
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CannonTower {
    pos: GridPosition
}

impl Tower for CannonTower {
    fn get_pos(&self) -> GridPosition{
        self.pos
    }


    fn new(pos: GridPosition) -> Self {
        CannonTower { pos }
    }
}