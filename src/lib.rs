pub mod ppm;
mod style;
pub mod writer;

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

// Graphs
// ======

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

pub struct Roll<'a> {
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
        let mut roll = Self {
            size: Dimension {
                w: width,
                h: height,
            },
            base,
            blocks,
        };
        let &Dimension { w: pw, h: ph } = roll.padding();
        let &Dimension { w: mw, h: mh } = roll.margin();
        roll.size.w += pw + mw;
        roll.size.h += ph + mh;

        roll
    }

    fn grid(&self) -> Vec<Renderable> {
        let &Dimension { w: mw, h: mh } = self.margin();
        let &Dimension {
            w: width,
            h: height,
        } = self.size();
        let &Block(basew, baseh) = self.base();
        let mut grid = vec![Renderable::Rect(
            Point { x: 0.0, y: 0.0 },
            Dimension {
                w: width,
                h: height,
            },
            Style::color(0x060910),
        )];
        let style = Style::color(0x303030).with_border(0.25);

        for y in (((mh / 2.0) as usize)..((height - mh / 2.0) as usize)).step_by(baseh as usize) {
            for x in (((mw / 2.0) as usize)..((width - mw / 2.0) as usize)).step_by(basew as usize)
            {
                let rect = Renderable::Rect(
                    Point {
                        x: x as f64,
                        y: y as f64,
                    },
                    Dimension { w: basew, h: baseh },
                    style,
                );
                grid.push(rect);
            }
        }

        grid
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
        let &Dimension { w: pw, h: ph } = self.padding();
        let &Dimension { w: mw, h: mh } = self.margin();
        let mut prev = Point {
            x: pw / 2.0 + mw / 2.0,
            y: ph / 2.0 + mh / 2.0,
        };
        let style = Style::color(0xDEAD00)
            .with_border(2.0)
            .with_background(0xBADA55);
        let mut renderables = self.grid();
        renderables.append(
            &mut self
                .blocks
                .iter()
                .map(|block| {
                    let rect = Renderable::Rect(
                        Point {
                            x: prev.x,
                            y: (block.1 * self.base.1) + prev.y,
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
                .collect::<Vec<Renderable>>(),
        );
        renderables
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

        assert_eq!(graph.size.w, 160.0 + 20.0 + 20.0, "graph width");
        assert_eq!(graph.size.h, 35.0 + 20.0 + 20.0, "graph height");
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
        let grid = graph.grid();

        let idx = grid.len();

        assert_eq!(rects.len(), idx + 4);

        let Renderable::Rect(pos, size, _) = &rects[idx];
        assert_eq!(
            pos.x, 20.0,
            "first rect should be at x=0+half padding+half margin"
        );
        assert_eq!(
            pos.y, 25.0,
            "first rect should be at y=5+half padding+half margin"
        );
        assert_eq!(size.w, 40.0, "first rect should be at w=40");
        assert_eq!(size.h, 5.0, "first rect should be at h=5");

        let Renderable::Rect(pos, size, _) = &rects[idx + 1];
        assert_eq!(
            pos.x, 60.0,
            "second rect should be at x=40+half padding+half margin"
        );
        assert_eq!(
            pos.y, 35.0,
            "second rect should be at y=15+half padding+half margin"
        );
        assert_eq!(size.w, 40.0, "second rect should be at w=40");
        assert_eq!(size.h, 5.0, "second rect should be at h=5");

        let Renderable::Rect(pos, size, _) = &rects[idx + 2];
        assert_eq!(
            pos.x, 100.0,
            "third rect should be at x=80+half padding+half margin"
        );
        assert_eq!(
            pos.y, 25.0,
            "third rect should be at y=5+half padding+half margin"
        );
        assert_eq!(size.w, 40.0, "third rect should be at w=40");
        assert_eq!(size.h, 5.0, "third rect should be at h=5");

        let Renderable::Rect(pos, size, _) = &rects[idx + 3];
        assert_eq!(
            pos.x, 140.0,
            "fourth rect should be at x=120+half padding+half margin"
        );
        assert_eq!(
            pos.y, 30.0,
            "fourth rect should be at y=10+half padding+half margin"
        );
        assert_eq!(size.w, 40.0, "fourth rect should be at w=40");
        assert_eq!(size.h, 5.0, "fourth rect should be at h=5");
    }
}
