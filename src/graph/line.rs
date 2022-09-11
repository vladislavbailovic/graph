use super::Graph;
use crate::{Block, Dimension, Point, Renderable, ShapeRenderer, Style};

pub struct Line<'a> {
    size: Dimension,
    base: Block,
    blocks: &'a [Block],
}

impl<'a> Line<'a> {
    pub fn new(blocks: &'a [Block]) -> Self {
        let base = Block(20.0, 20.0);
        let width = blocks
            .iter()
            .fold(0.0, |total, block| total + block.0);
        let maximum = blocks
            .iter()
            .map(|x| x.1 * base.1)
            .reduce(f64::max)
            .expect("there has to be maximum");
        let height = ((maximum / base.1).floor() + 1.0) * base.1;
        let mut roll = Self {
            size: Dimension {
                w: width,
                h: height,
            },
            base,
            blocks,
        };
        let &Block(dw, dh) = roll.padding();
        let &Dimension { w: mw, h: mh } = roll.margin();
        roll.size.w += (roll.base.0 * dw * 2.0) + mw * 2.0;
        roll.size.h += (roll.base.1 * dh * 2.0) + mh * 2.0;

        roll
    }
}

impl<'a> Graph for Line<'a> {
    fn size(&self) -> &Dimension {
        &self.size
    }

    fn base(&self) -> &Block {
        &self.base
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
        let vpad = (self.base.1 * dh) + mh;
        let hpad = (self.base.0 * dw) + mw;
        let mut prev = Point {
            x: hpad,
            y: vpad,
        };
        let style = Style::color(0x6495ED);
        let mut renderables = self.grid();
        renderables.append(
            &mut self
                .blocks
                .iter()
                .filter_map(|block| {
                    let mut delta_y = block.1 * self.base.1;
                    delta_y = (height - vpad) - delta_y;
                    let rect = if prev.x != hpad {
                        Some(Renderable::Line(
                            Point {
                                x: prev.x,
                                y: prev.y,
                            },
                            Point {
                                x: prev.x + block.0,
                                y: delta_y,
                            },
                            style,
                        ))
                    } else {
                        None
                    };

                    prev.x += block.0;
                    prev.y = delta_y;
                    rect
                })
                .collect::<Vec<Renderable>>(),
        );
        renderables
    }
}

#[cfg(test)]
mod tests {}
