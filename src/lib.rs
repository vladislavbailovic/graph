mod ppm;
mod style;
mod writer;

use style::*;

// Primitives
// ==========

#[derive(Debug, Copy, Clone)]
pub(crate) struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Dimension {
    w: f64,
    h: f64,
}

pub(crate) struct Block(f64, f64);

// Shapes
// ======

pub(crate) trait ShapeRenderer {
    fn draw(&mut self, shape: Renderable);
    fn get_buffer(&self) -> &[u8];
}

pub(crate) trait ImageRenderer: ShapeRenderer {
    fn get_header(&self) -> Option<Vec<u8>>;
    fn get_footer(&self) -> Option<Vec<u8>>;
}

pub(crate) enum Renderable {
    Rect(Point, Dimension, Style),
}

// Graphs
// ======

pub(crate) trait Graph {
    fn get_blocks(&self) -> &[Block];
    fn renderables(&self) -> Vec<Renderable>;
    fn draw<T>(&self, renderer: T) -> Vec<u8>
    where
        T: ShapeRenderer;
    fn size(&self) -> &Dimension;
    fn base(&self) -> &Block;
}

pub(crate) struct Roll<'a> {
    size: Dimension,
    base: Block,
    blocks: &'a [Block],
}

impl<'a> Roll<'a> {
    pub fn new(blocks: &'a [Block]) -> Self {
        let base = Block(10.0, 5.0);
        let width = blocks
            .iter()
            .fold(0.0, |total, block| total + block.0 * base.0);
        let height = blocks
            .iter()
            .fold(0.0, |total, block| total + block.1 * base.1);
        Self {
            size: Dimension {
                w: width,
                h: height,
            },
            base,
            blocks,
        }
    }
}

impl<'a> Graph for Roll<'a> {
    fn size(&self) -> &Dimension {
        &self.size
    }

    fn base(&self) -> &Block {
        &self.base
    }

    fn get_blocks(&self) -> &[Block] {
        self.blocks
    }

    fn draw<T>(&self, mut renderer: T) -> Vec<u8>
    where
        T: ShapeRenderer,
    {
        for rect in self.renderables() {
            renderer.draw(rect);
        }

        renderer.get_buffer().to_vec()
    }

    fn renderables(&self) -> Vec<Renderable> {
        let mut prev = Point { x: 0.0, y: 0.0 };
        let style = Style::color(0xDEAD00)
            .with_border(2.0)
            .with_background(0xBADA55);
        self.blocks
            .iter()
            .map(|block| {
                let rect = Renderable::Rect(
                    Point {
                        x: prev.x,
                        y: block.1 * self.base.1,
                    },
                    Dimension {
                        w: block.0 * self.base.0,
                        h: self.base.1,
                    },
                    style,
                );

                prev.x += block.0 * self.base.0;
                rect
            })
            .collect::<Vec<Renderable>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_dimensions_from_blocks() {
        let graph = Roll::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);

        assert_eq!(graph.size.w, 160.0, "graph width");
        assert_eq!(graph.size.h, 35.0, "graph height");
    }

    #[test]
    fn graph_draw() {
        let graph = Roll::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);
        let renderer = ppm::Renderer::new(&graph.size);
        let buf = graph.draw(renderer);

        assert_eq!(buf.len(), (graph.size.w * graph.size.h) as usize * 3);
    }

    #[test]
    fn graph_rects() {
        let graph = Roll::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);
        let rects = graph.renderables();

        assert_eq!(rects.len(), 4);

        if let Renderable::Rect(pos, size, _) = &rects[0] {
            assert_eq!(pos.x, 0.0, "first rect should be at x=0");
            assert_eq!(pos.y, 5.0, "first rect should be at y=5");
            assert_eq!(size.w, 40.0, "first rect should be at w=40");
            assert_eq!(size.h, 5.0, "first rect should be at h=5");
        }

        if let Renderable::Rect(pos, size, _) = &rects[1] {
            assert_eq!(pos.x, 40.0, "second rect should be at x=40");
            assert_eq!(pos.y, 15.0, "second rect should be at y=15");
            assert_eq!(size.w, 40.0, "second rect should be at w=40");
            assert_eq!(size.h, 5.0, "second rect should be at h=5");
        }

        if let Renderable::Rect(pos, size, _) = &rects[2] {
            assert_eq!(pos.x, 80.0, "third rect should be at x=80");
            assert_eq!(pos.y, 5.0, "third rect should be at y=5");
            assert_eq!(size.w, 40.0, "third rect should be at w=40");
            assert_eq!(size.h, 5.0, "third rect should be at h=5");
        }

        if let Renderable::Rect(pos, size, _) = &rects[3] {
            assert_eq!(pos.x, 120.0, "fourth rect should be at x=120");
            assert_eq!(pos.y, 10.0, "fourth rect should be at y=10");
            assert_eq!(size.w, 40.0, "fourth rect should be at w=40");
            assert_eq!(size.h, 5.0, "fourth rect should be at h=5");
        }
    }
}
