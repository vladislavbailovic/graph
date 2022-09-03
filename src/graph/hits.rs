use super::Graph;
use crate::{Block, Dimension, Point, Renderable, ShapeRenderer, Style};

pub struct Hits<'a> {
    size: Dimension,
    base: Block,
    blocks: &'a [Block],
}

impl<'a> Hits<'a> {
    pub fn new(blocks: &'a [Block]) -> Self {
        let base = Block(20.0, 20.0);
        let width = blocks
            .iter()
            .fold(0.0, |total, block| total + block.0 * base.0);
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

impl<'a> Graph for Hits<'a> {
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
        let style = Style::color(0xBADA55)
            .with_border(2.0)
            .with_background(0x33EF33);
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
                    delta_y = (height - prev.y * 2.0) - delta_y;
                    let rect = Renderable::Rect(
                        Point {
                            x: prev.x,
                            y: delta_y + prev.y,
                        },
                        Dimension {
                            w: self.base.0,
                            h: block.1 * self.base.1,
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
}
