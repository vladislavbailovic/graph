mod roll;
pub use roll::Roll;
mod hits;
pub use hits::Hits;
mod track;
pub use track::Track;
mod line;
pub use line::Line;

use crate::{Block, Dimension, Point, Renderable, ShapeRenderer, Style};

pub trait Graph {
    fn renderables(&self) -> Vec<Renderable>;
    fn draw<T>(&self, renderer: &mut T) -> Vec<u8>
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

        for y in ((mh as usize)..((height - mh) as usize)).step_by(baseh as usize) {
            grid.push(Renderable::Rect(
                Point { x: mw, y: y as f64 },
                Dimension {
                    w: width - mw * 2.0,
                    h: 1.0,
                },
                style,
            ));
        }
        grid.push(Renderable::Rect(
            Point {
                x: mw,
                y: height - mh,
            },
            Dimension {
                w: width - mw * 2.0,
                h: 1.0,
            },
            style,
        ));
        for x in ((mw as usize)..((width - mw) as usize)).step_by(basew as usize) {
            grid.push(Renderable::Rect(
                Point { x: x as f64, y: mh },
                Dimension {
                    w: 1.0,
                    h: height - mh * 2.0,
                },
                style,
            ));
        }
        grid.push(Renderable::Rect(
            Point {
                x: width - mw,
                y: mh,
            },
            Dimension {
                w: 1.0,
                h: height - mh * 2.0,
            },
            style,
        ));

        grid
    }
}
