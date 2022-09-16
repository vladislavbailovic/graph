use rsound_output::*;
pub mod ppm;
pub mod svg;
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

#[derive(Debug)]
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

pub trait ShapeRenderer: Buffer {
    fn draw(&mut self, shape: Renderable);
}

pub trait ImageRenderer: ShapeRenderer + OutputRenderer {}

pub enum Renderable {
    Rect(Point, Dimension, Style),
    Line(Point, Point, Style),
}
