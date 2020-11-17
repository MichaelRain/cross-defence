use ggez;
use rand;

use ggez::{graphics, Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler};
use ggez::input::mouse;
use ggez::input::mouse::button_pressed;

use super::{GRID_CELL_SIZE};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

impl From<(f32, f32)> for GridPosition {
    fn from(pos: (f32, f32)) -> Self {
        GridPosition { x: pos.0 as i16 / GRID_CELL_SIZE.0, y: pos.1 as i16 / GRID_CELL_SIZE.1 }
    }
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}