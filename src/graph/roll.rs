use super::Graph;
use crate::{Block, Dimension, Point, Renderable, ShapeRenderer, Style};

pub struct Roll<'a> {
    size: Dimension,
    minimum: f64,
    base: Block,
    blocks: &'a [Block],
}

impl<'a> Roll<'a> {
    pub fn new(blocks: &'a [Block]) -> Self {
        let base = Block(20.0, 20.0);
        let width = blocks
            .iter()
            .fold(0.0, |total, block| total + block.0 * base.0);
        let minimum = blocks
            .iter()
            .filter_map(|x| if x.1 > 0.0 { Some(x.1) } else { None })
            .reduce(f64::min)
            .expect("there has to be minimum");
        let maximum = blocks
            .iter()
            .map(|x| x.1)
            .reduce(f64::max)
            .expect("there has to be maximum");
        let height = ((maximum - minimum) + 1.0) * base.1;
        let mut roll = Self {
            size: Dimension {
                w: width,
                h: height,
            },
            minimum,
            base,
            blocks,
        };
        let &Block(dw, dh) = roll.padding();
        let &Dimension { w: mw, h: mh } = roll.margin();
        roll.size.w += (roll.base.0 * dw * 2.0) + mw * 2.0;
        roll.size.h += (roll.base.1 * dh * 2.0) + mh * 2.0;

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
        let style = Style::color(0x303030);

        for y in (((mh) as usize)..((height - mh) as usize) + 1_usize).step_by(baseh as usize) {
            grid.push(Renderable::Rect(
                Point { x: mw, y: y as f64 },
                Dimension {
                    w: width - mw * 2.0,
                    h: 1.0,
                },
                style,
            ));
        }
        for x in (((mw) as usize)..((width - mw) as usize) + 1).step_by(basew as usize) {
            grid.push(Renderable::Rect(
                Point { x: x as f64, y: mh },
                Dimension {
                    w: 1.0,
                    h: height - mh * 2.0,
                },
                style,
            ));
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
        let &Dimension {
            h: height,
            w: _width,
        } = self.size();
        let &Block(dw, dh) = self.padding();
        let &Dimension { w: mw, h: mh } = self.margin();
        let mut prev = Point {
            x: (self.base.0 * dw) + mw,
            y: (self.base.1 * dh) + mh,
        };
        let style = Style::color(0xDEAD00)
            .with_border(2.0)
            .with_background(0xBADA55);
        let mut renderables = self.grid();
        renderables.append(
            &mut self
                .blocks
                .iter()
                .filter_map(|block| {
                    if block.1 == 0.0 {
                        prev.x += block.0 * self.base.0;
                        return None;
                    }
                    let mut delta_y = block.1 * self.base.1;
                    delta_y -= self.minimum * self.base.1;
                    delta_y = (height - prev.y * 2.0 - self.base.1) - delta_y;
                    let rect = Renderable::Rect(
                        Point {
                            x: prev.x,
                            y: delta_y + prev.y,
                        },
                        Dimension {
                            w: block.0 * self.base.0,
                            h: self.base.1,
                        },
                        style,
                    );

                    prev.x += block.0 * self.base.0;
                    Some(rect)
                })
                .collect::<Vec<Renderable>>(),
        );
        renderables
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ppm;

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
