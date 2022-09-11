use crate::{Color, Dimension, ImageRenderer, Point, Renderable, ShapeRenderer};

pub struct Renderer {
    size: Dimension,
    buffer: Vec<u8>,
}

impl ShapeRenderer for Renderer {
    fn draw(&mut self, shape: Renderable) {
        match shape {
            Renderable::Rect(pos, size, style) => {
                if style.has_fill() {
                    self.rect(pos, size, style.get_color());
                }
                if let Some((color, thickness)) = style.get_frame() {
                    self.frame(pos, size, color, thickness);
                }
            },
            Renderable::Line(p1, p2, style) => {
                if let Some((color, thickness)) = style.get_frame() {
                    self.line(p1, p2, color, thickness);
                } else if style.has_fill() {
                    self.line(p1, p2, style.get_color(), 1.0);
                }
            }
        };
    }

    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl ImageRenderer for Renderer {
    fn get_header(&self) -> Option<Vec<u8>> {
        Some(format!("P6 {} {} 255\n", self.size.w as usize, self.size.h as usize).into_bytes())
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

    fn rect(&mut self, pos: Point, size: Dimension, color: &Color) {
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

    fn frame(&mut self, pos: Point, size: Dimension, color: &Color, thickness: f64) {
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
    }

    fn line(&mut self, p1: Point, p2: Point, color: &Color, thickness: f64) {
        let mut line = |x1: f64, x2: f64, y1: f64, y2: f64| {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let mut delta = 2.0 * dy - dx;
            let mut y = y1;

            for x in (x1 as usize)..(x2 as usize) {
                let offset = (y as usize * (self.size.w as usize) * 3) + (x * 3);
                self.buffer[offset] = color.0;
                self.buffer[offset + 1] = color.1;
                self.buffer[offset + 2] = color.2;

                if delta > 0.0 {
                    y = y + 1.0;
                    delta = delta - 2.0*dx;
                }
                delta = delta + 2.0*dy;
            }
        };
        line(p1.x, p2.x, p1.y, p2.y);
        for i in 0..=(thickness as usize) {
            line(p1.x-(i as f64), p2.x, p1.y, p2.y);
            line(p1.x+(i as f64), p2.x, p1.y, p2.y);
        }
    }

    // https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
    // fn line(&mut self, p1: Point, p2: Point, color: &Color, thickness: f64) {
        // let dx = p1.x - p2.x;
        // let dy = p2.y - p2.y;
        // let mut delta = 2.0 * dy - dx;
        // let mut y = p1.y;

        // for x in (p1.x as usize)..(p2.x as usize) {
        //     let offset = (y as usize * (self.size.w as usize) * 3) + (x * 3);
        //     self.buffer[offset] = color.0;
        //     self.buffer[offset + 1] = color.1;
        //     self.buffer[offset + 2] = color.2;

        //     if delta > 0.0 {
        //         y = y + 1.0;
        //         delta = delta - 2.0*dx;
        //     }
        //     delta = delta + 2.0*dy;
        // }
    // }
}
