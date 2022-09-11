use super::{Graph, Hits, Roll};
use crate::{Block, Dimension, Renderable, ShapeRenderer};

pub struct Track<'a> {
    size: Dimension,
    base: Block,
    hits: Hits<'a>,
    roll: Roll<'a>,
}

impl<'a> Track<'a> {
    pub fn new(hsrc: &'a [Block], rsrc: &'a [Block]) -> Self {
        let base = Block(20.0, 20.0);
        let hits = Hits::new(hsrc);
        let roll = Roll::new(rsrc);
        let width = hits.size().w;
        let height = hits.size().h + roll.size().h;
        let mut track = Self {
            size: Dimension {
                w: width,
                h: height,
            },
            base,
            hits,
            roll,
        };
        let &Block(dw, dh) = track.padding();
        let &Dimension { w: mw, h: mh } = track.margin();
        track.size.w += (track.base.0 * dw * 2.0) + mw * 2.0;
        track.size.h += (track.base.1 * dh * 2.0) + mh * 2.0;

        track
    }
}

impl<'a> Graph for Track<'a> {
    fn size(&self) -> &Dimension {
        &self.size
    }

    fn base(&self) -> &Block {
        &self.base
    }

    /// Children will take care of this
    fn margin(&self) -> &Dimension {
        &Dimension { w: 0.0, h: 0.0 }
    }

    /// Children will take care of this
    fn padding(&self) -> &Block {
        &Block(0.0, 0.0)
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
        let mut renderables = self.roll.renderables();
        let offset = self.roll.size().h;
        renderables.append(
            &mut self
                .hits
                .renderables()
                .iter_mut()
                .filter_map(|x| match x {
                    Renderable::Rect(p, d, s) => {
                        p.y += offset;
                        Some(Renderable::Rect(*p, *d, *s))
                    },
                    Renderable::Line(p1, p2, s) => {
                        p1.y += offset;
                        p2.y += offset;
                        Some(Renderable::Line(*p1, *p2, *s))
                    },
                })
                .collect(),
        );

        renderables
    }
}

#[cfg(test)]
mod tests {}
