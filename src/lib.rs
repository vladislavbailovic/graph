// Primitives
// ==========

struct Color(u8, u8, u8);

impl From<u32> for Color {
    fn from(raw: u32) -> Self{
        Self(
            ((raw >> 16) & 255) as u8,
            ((raw >> 8) & 255) as u8,
            (raw & 255) as u8,
        )
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Dimension {
    w: f64,
    h: f64,
}

struct Block(f64, f64);

// Shapes
// ======

trait Shape {
    fn fill(&self, color: Color, buffer_size: &Dimension, buffer: &mut Vec<u8>);
    fn frame(&self, color: Color, thickness: f64, buffer_size: &Dimension, buffer: &mut Vec<u8>);
}


#[derive(Debug)]
struct Rect {
    position: Point,
    size: Dimension,
}

impl Shape for Rect {
    fn fill(&self, color: Color, buffer_size: &Dimension, buffer: &mut Vec<u8>) {
        let ystart = self.position.y as usize;
        let yend = (self.position.y + self.size.h) as usize;
        let xstart = self.position.x as usize;
        let xend = (self.position.x + self.size.w) as usize;
        let width = buffer_size.w as usize;

        for y in ystart..yend {
            for x in xstart..xend {
                let offset = (y * width * 3) + (x * 3);
                buffer[offset] = color.0;
                buffer[offset+1] = color.1;
                buffer[offset+2] = color.2;
            }
        }
    }
    fn frame(&self, color: Color, thickness: f64, buffer_size: &Dimension, buffer: &mut Vec<u8>) {
        let ystart = self.position.y as usize;
        let yend = (self.position.y + self.size.h) as usize;
        let xstart = self.position.x as usize;
        let xend = (self.position.x + self.size.w) as usize;
        let t = thickness as usize;
        let width = buffer_size.w as usize;

        for y in ystart..yend {
            for x in xstart..xend {
                if (y < ystart + t || y >= yend - t) || (x < xstart + t || x >= xend - t) {
                    let offset = (y * width * 3) + (x * 3);
                    buffer[offset] = color.0;
                    buffer[offset+1] = color.1;
                    buffer[offset+2] = color.2;
                }
            }
        }
    }
}

// Graphs
// ======

mod ppm;

struct Graph<'a> {
    size: Dimension,
    base: Block,
    blocks: &'a [Block],
}

impl<'a> Graph<'a> {

    pub fn new(blocks: &'a [Block]) -> Self {
        let base = Block(10.0, 5.0);
        let width = blocks.iter().fold(0.0, |total, block| total + block.0 * base.0);
        let height = blocks.iter().fold(0.0, |total, block| total + block.1 * base.1);
        Self{
            size: Dimension {
                w: width,
                h: height,
            },
            base,
            blocks,
        }
    }

    fn draw(&self) -> Vec<u8> {
        let max_size = (self.size.w * self.size.h) as usize * 3;
        let mut buffer = vec![0; max_size];
        for rect in self.rects() {
            rect.frame(Color::from(0xDEAF00), 1.0, &self.size, &mut buffer);
            // rect.fill(Color::from(0xDEAD00), &self.size, &mut buffer);
        }

        buffer
    }

    fn rects(&self) -> Vec<Rect> {
        let mut prev = Point{ x: 0.0, y: 0.0 };
        self.blocks.iter().map(|block| {
            let rect = Rect{
                position: Point{
                    x: prev.x,
                    y: block.1 * self.base.1,
                },
                size: Dimension {
                    w: block.0 * self.base.0,
                    h: self.base.1,
                }
            };

            prev.x += rect.size.w;
            rect
        }).collect::<Vec<Rect>>()
    }
}

trait GraphFileWriter {
    fn write(&self, fname: &str, graph: &Graph) -> std::io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_rects() {
        let graph = Graph::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);
        let rects = graph.rects();

        assert_eq!(rects.len(), 4);

        assert_eq!(rects[0].position.x, 0.0, "first rect should be at x=0");
        assert_eq!(rects[1].position.x, 40.0, "second rect should be at x=40");
        assert_eq!(rects[2].position.x, 80.0, "third rect should be at x=80");
        assert_eq!(rects[3].position.x, 120.0, "fourth rect should be at x=120");

        assert_eq!(rects[0].position.y, 5.0, "first rect should be at y=5");
        assert_eq!(rects[1].position.y, 15.0, "second rect should be at y=15");
        assert_eq!(rects[2].position.y, 5.0, "third rect should be at y=5");
        assert_eq!(rects[3].position.y, 10.0, "fourth rect should be at y=10");

        assert_eq!(rects[0].size.w, 40.0, "first rect should be at w=40");
        assert_eq!(rects[1].size.w, 40.0, "second rect should be at w=40");
        assert_eq!(rects[2].size.w, 40.0, "third rect should be at w=40");
        assert_eq!(rects[3].size.w, 40.0, "fourth rect should be at w=40");

        assert_eq!(rects[0].size.h, 5.0, "first rect should be at h=5");
        assert_eq!(rects[1].size.h, 5.0, "second rect should be at h=5");
        assert_eq!(rects[2].size.h, 5.0, "third rect should be at h=5");
        assert_eq!(rects[3].size.h, 5.0, "fourth rect should be at h=5");
    }

    #[test]
    fn it_works() {
        let base = Block(10.0, 5.0);
        let mut prev = Point{ x: 0.0, y: 0.0 };
        let rects = vec![
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ].iter().map(|block| {
            let rect = Rect{
                position: Point{
                    x: prev.x,
                    y: block.1 * base.1,
                },
                size: Dimension {
                    w: block.0 * base.0,
                    h: base.1,
                }
            };

            prev.x += rect.size.w;
            rect
        }).collect::<Vec<Rect>>();

        assert_eq!(rects.len(), 4);

        assert_eq!(rects[0].position.x, 0.0, "first rect should be at x=0");
        assert_eq!(rects[1].position.x, 40.0, "second rect should be at x=40");
        assert_eq!(rects[2].position.x, 80.0, "third rect should be at x=80");
        assert_eq!(rects[3].position.x, 120.0, "fourth rect should be at x=120");

        assert_eq!(rects[0].position.y, 5.0, "first rect should be at y=5");
        assert_eq!(rects[1].position.y, 15.0, "second rect should be at y=15");
        assert_eq!(rects[2].position.y, 5.0, "third rect should be at y=5");
        assert_eq!(rects[3].position.y, 10.0, "fourth rect should be at y=10");

        assert_eq!(rects[0].size.w, 40.0, "first rect should be at w=40");
        assert_eq!(rects[1].size.w, 40.0, "second rect should be at w=40");
        assert_eq!(rects[2].size.w, 40.0, "third rect should be at w=40");
        assert_eq!(rects[3].size.w, 40.0, "fourth rect should be at w=40");

        assert_eq!(rects[0].size.h, 5.0, "first rect should be at h=5");
        assert_eq!(rects[1].size.h, 5.0, "second rect should be at h=5");
        assert_eq!(rects[2].size.h, 5.0, "third rect should be at h=5");
        assert_eq!(rects[3].size.h, 5.0, "fourth rect should be at h=5");
    }
}
