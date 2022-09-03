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
        &Dimension { w: 40.0, h: 40.0 }
    }

    /// Distance between margin and renderables
    /// Expressed relative to base block size
    fn padding(&self) -> &Block {
        &Block(0.0, 0.0)
    }
}
