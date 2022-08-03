use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{Graph, GraphFileWriter};

pub struct Writer;
impl GraphFileWriter for Writer {
    #[allow(clippy::unused_io_amount)]
    fn write(&self, fname: &str, graph: &Graph) -> std::io::Result<()> {
        let mut p = BufWriter::new(File::create(fname)?);
        p.write(format!("P6 {} {} 255\n", graph.size.w, graph.size.h).as_bytes())?;

        let renderer = Renderer {};
        let buffer = graph.draw(renderer);
        p.write(&buffer)?;

        Ok(())
    }
}

use crate::{Color, Dimension, Point, Renderable, ShapeRenderer};
pub struct Renderer;
impl ShapeRenderer for Renderer {
    fn draw(&self, shape: Renderable, buffer_size: &Dimension, buffer: &mut Vec<u8>) {
        match shape {
            Renderable::Rect(pos, size, col) => self.rect(pos, size, col, buffer_size, buffer),
            Renderable::Frame(pos, size, col, thickness) => {
                self.frame(pos, size, col, thickness, buffer_size, buffer)
            }
        };
    }
}

impl Renderer {
    fn rect(
        &self,
        pos: Point,
        size: Dimension,
        color: Color,
        buffer_size: &Dimension,
        buffer: &mut Vec<u8>,
    ) {
        let ystart = pos.y as usize;
        let yend = (pos.y + size.h) as usize;
        let xstart = pos.x as usize;
        let xend = (pos.x + size.w) as usize;
        let width = buffer_size.w as usize;

        for y in ystart..yend {
            for x in xstart..xend {
                let offset = (y * width * 3) + (x * 3);
                buffer[offset] = color.0;
                buffer[offset + 1] = color.1;
                buffer[offset + 2] = color.2;
            }
        }
    }

    fn frame(
        &self,
        pos: Point,
        size: Dimension,
        color: Color,
        thickness: f64,
        buffer_size: &Dimension,
        buffer: &mut Vec<u8>,
    ) {
        let mut pixel = |x: usize, y: usize| {
            let offset = (y * (buffer_size.w as usize) * 3) + (x * 3);
            buffer[offset] = color.0;
            buffer[offset + 1] = color.1;
            buffer[offset + 2] = color.2;
        };
        // top
        for y in (pos.y as usize)..((pos.y + thickness) as usize) {
            for x in (pos.x as usize)..((pos.x + size.w) as usize) {
                pixel(x, y);
            }
        }
        // bottom
        for y in (((pos.y + size.h) - thickness) as usize)..((pos.y + size.h) as usize) {
            for x in (pos.x as usize)..((pos.x + size.w) as usize) {
                pixel(x, y);
            }
        }
        // sides
        for y in (pos.y as usize)..((pos.y + size.h) as usize) {
            // left
            for x in (pos.x as usize)..((pos.x + thickness) as usize) {
                pixel(x, y);
            }
            // right
            for x in (((pos.x + size.w) - thickness) as usize)..((pos.x + size.w) as usize) {
                pixel(x, y);
            }
        }
        // let ystart = pos.y as usize;
        // let yend = (pos.y + size.h) as usize;
        // let xstart = pos.x as usize;
        // let xend = (pos.x + size.w) as usize;
        // let t = thickness as usize;
        // let width = buffer_size.w as usize;

        // for y in ystart..yend {
        //     for x in xstart..xend {
        //         if (y < ystart + t || y >= yend - t) || (x < xstart + t || x >= xend - t) {
        //             let offset = (y * width * 3) + (x * 3);
        //             buffer[offset] = color.0;
        //             buffer[offset+1] = color.1;
        //             buffer[offset+2] = color.2;
        //         }
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Block, Graph};

    #[test]
    fn graph_draw_save() {
        let graph = Graph::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);
        let w = Writer {};
        if let Err(e) = w.write("foo.ppm", &graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "File created");
        }
    }
}
