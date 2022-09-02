pub mod ppm;
pub mod writer;

mod graph;
pub use crate::graph::*;

mod style;
use style::*;

// Primitives
// ==========

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Dimension {
    w: f64,
    h: f64,
}

pub struct Block(f64, f64);
impl Block {
    pub fn new(w: f64, h: f64) -> Self {
        Self(w, h)
    }

    pub fn intensity(&self) -> Option<&f64> {
        Some(&self.1)
    }

    pub fn duration(&self) -> &f64 {
        &self.0
    }
}

// Shapes
// ======

pub trait ShapeRenderer {
    fn draw(&mut self, shape: Renderable);
    fn get_buffer(&self) -> &[u8];
}

pub trait ImageRenderer: ShapeRenderer {
    fn get_header(&self) -> Option<Vec<u8>>;
    fn get_footer(&self) -> Option<Vec<u8>>;
}

pub enum Renderable {
    Rect(Point, Dimension, Style),
}
