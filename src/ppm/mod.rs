use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{Graph, GraphFileWriter};

pub struct Writer;
impl GraphFileWriter for Writer {
    #[allow(clippy::unused_io_amount)]
    fn write(&self, fname: &str, graph: &Graph) -> std::io::Result<()> {
        let mut p = BufWriter::new(File::create(fname)?);

        let renderer = Renderer::new(&graph.size);
        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            p.write(&header)?;
        }
        let buffer = graph.draw(renderer);
        p.write(&buffer)?;
        if let Some(footer) = footer {
            p.write(&footer)?;
        }

        Ok(())
    }
}

use crate::{Color, Dimension, ImageRenderer, Point, Renderable, ShapeRenderer};
pub(crate) struct Renderer {
    size: Dimension,
    buffer: Vec<u8>,
}
impl ShapeRenderer for Renderer {
    fn draw(&mut self, shape: Renderable) {
        match shape {
            Renderable::Rect(pos, size, col) => self.rect(pos, size, col),
            Renderable::Frame(pos, size, col, thickness) => self.frame(pos, size, col, thickness),
        };
    }

    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl ImageRenderer for Renderer {
    fn get_header(&self) -> Option<Vec<u8>> {
        Some(format!("P6 {} {} 255\n", self.size.w, self.size.h).into_bytes())
    }
    fn get_footer(&self) -> Option<Vec<u8>> {
        None
    }
}

impl Renderer {
    pub fn new(size: &Dimension) -> Self {
        let max_size = (size.w * size.h) as usize * 3;
        let buffer = vec![0; max_size];
        Self {
            size: Dimension {
                w: size.w,
                h: size.h,
            },
            buffer,
        }
    }

    fn rect(&mut self, pos: Point, size: Dimension, color: Color) {
        let ystart = pos.y as usize;
        let yend = (pos.y + size.h) as usize;
        let xstart = pos.x as usize;
        let xend = (pos.x + size.w) as usize;
        let width = self.size.w as usize;

        for y in ystart..yend {
            for x in xstart..xend {
                let offset = (y * width * 3) + (x * 3);
                self.buffer[offset] = color.0;
                self.buffer[offset + 1] = color.1;
                self.buffer[offset + 2] = color.2;
            }
        }
    }

    fn frame(&mut self, pos: Point, size: Dimension, color: Color, thickness: f64) {
        let mut pixel = |x: usize, y: usize| {
            let offset = (y * (self.size.w as usize) * 3) + (x * 3);
            self.buffer[offset] = color.0;
            self.buffer[offset + 1] = color.1;
            self.buffer[offset + 2] = color.2;
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
        // let width = self.size.w as usize;

        // for y in ystart..yend {
        //     for x in xstart..xend {
        //         if (y < ystart + t || y >= yend - t) || (x < xstart + t || x >= xend - t) {
        //             let offset = (y * width * 3) + (x * 3);
        //             self.buffer[offset] = color.0;
        //             self.buffer[offset+1] = color.1;
        //             self.buffer[offset+2] = color.2;
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
