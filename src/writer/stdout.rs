use std::io::{self, Write, Result};

use super::Writer;
use crate::{Graph, ImageRenderer};

pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self { Self }
}

impl Writer for StdoutWriter {
    fn write<T, U>(&self, renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph,
    {
        let mut stdout = io::stdout();
        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            stdout.write(header)?;
        }
        let buffer = graph.draw(renderer);
        stdout.write(&buffer)?;
        if let Some(footer) = footer {
            stdout.write(footer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ppm::Renderer, Block, Roll};

    #[test]
    fn simple_graph() {
        let graph = Roll::new(&[
            Block(4.0, 1.0),
        ]);
        let renderer = Renderer::new(&graph.size());
        let w = StdoutWriter::new();

        if let Err(e) = w.write(renderer, graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "Out rendered");
        }
    }
}
