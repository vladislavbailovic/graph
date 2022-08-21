mod roll;
pub use roll::Roll;

use crate::{Block, Dimension, Renderable, ShapeRenderer};

pub trait Graph {
    fn get_blocks(&self) -> &[Block];
    fn renderables(&self) -> Vec<Renderable>;
    fn draw<T>(&self, renderer: T) -> Vec<u8>
    where
        T: ShapeRenderer;
    fn size(&self) -> &Dimension;
    fn base(&self) -> &Block;

    /// Distance between edge and padding
    fn margin(&self) -> &Dimension {
        &Dimension { w: 20.0, h: 20.0 }
    }

    /// Distance between margin and renderables
    fn padding(&self) -> &Dimension {
        &Dimension { w: 20.0, h: 20.0 }
    }
}
