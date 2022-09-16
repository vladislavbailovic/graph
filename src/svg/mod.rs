use crate::{Color, Dimension, ImageRenderer, Point, Renderable, ShapeRenderer};
use rsound_output::*;

pub struct Renderer {
    size: Dimension,
    buffer: Vec<u8>,
}

impl OutputRenderer for Renderer {
    fn get_header(&self) -> Option<Vec<u8>> {
        Some(
            format!(
                "<svg version='1.1' width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>\n",
                self.size.w as usize, self.size.h as usize
            )
            .into_bytes(),
        )
    }

    fn get_footer(&self) -> Option<Vec<u8>> {
        Some("</svg>".as_bytes().to_vec())
    }
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
            }
            Renderable::Line(p1, p2, style) => {
                if let Some((color, thickness)) = style.get_frame() {
                    self.line(p1, p2, color, thickness);
                } else if style.has_fill() {
                    self.line(p1, p2, style.get_color(), 1.0);
                }
            }
        }
    }
}

impl Buffer for Renderer {
    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl ImageRenderer for Renderer {}

impl Renderer {
    pub fn new(size: &Dimension) -> Self {
        Self {
            size: Dimension {
                w: size.w,
                h: size.h,
            },
            buffer: Vec::new(),
        }
    }

    fn rect(&mut self, pos: Point, size: Dimension, color: &Color) {
        let rect = format!(
            "<rect x='{}' y='{}' width='{}' height='{}' fill='{}' />\n",
            pos.x,
            pos.y,
            size.w,
            size.h,
            color.rgb()
        );
        self.buffer.append(&mut rect.into_bytes());
    }

    fn frame(&mut self, pos: Point, size: Dimension, color: &Color, thickness: f64) {
        let rect = format!(
            "<rect x='{}' y='{}' width='{}' height='{}' color='{}' stroke='{}' fill='transparent'/>\n",
            pos.x,
            pos.y,
            size.w,
            size.h,
            color.rgb(),
            thickness
        );
        self.buffer.append(&mut rect.into_bytes());
    }

    fn line(&mut self, p1: Point, p2: Point, color: &Color, thickness: f64) {
        let rect = format!(
            "<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{}' stroke-width='{}' />\n",
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            color.rgb(),
            thickness,
        );
        self.buffer.append(&mut rect.into_bytes());
    }
}
