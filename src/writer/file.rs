use std::fs::File;
use std::io::{BufWriter, Result, Write};

use super::Writer;
use crate::{Graph, ImageRenderer};

pub struct FileWriter {
    fname: String,
}
impl Writer for FileWriter {
    fn write<T, U>(&self, renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph,
    {
        let mut p = BufWriter::new(File::create(&self.fname)?);

        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            let _ = p.write(header)?;
        }
        let buffer = graph.draw(Box::new(renderer));
        let _ = p.write(&buffer)?;
        if let Some(footer) = footer {
            let _ = p.write(footer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ppm::Renderer, Block, Roll};

    #[test]
    fn graph_draw_save() {
        let graph = Roll::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);
        let w = FileWriter {
            fname: String::from("foo.ppm"),
        };
        let renderer = Renderer::new(&graph.size());
        if let Err(e) = w.write(renderer, graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "File created");
        }
    }
}
