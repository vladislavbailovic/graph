use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{Graph,ImageRenderer};
trait Writer {
    fn write(&self, renderer: impl ImageRenderer, graph: &Graph) -> std::io::Result<()>;
}


pub struct FileWriter {
    fname: String,
}
impl Writer for FileWriter {
    fn write(&self, renderer: impl ImageRenderer, graph: &Graph) -> std::io::Result<()> {
        let mut p = BufWriter::new(File::create(&self.fname)?);

        let header = &renderer.get_header();
        let footer = &renderer.get_footer();

        if let Some(header) = header {
            let _ =p.write(&header)?;
        }
        let buffer = graph.draw(renderer);
        let _ = p.write(&buffer)?;
        if let Some(footer) = footer {
            let _ = p.write(&footer)?;
        }

        Ok(())
    }
}

pub struct StdoutWriter;
impl Writer for StdoutWriter {
    fn write(&self, _renderer: impl ImageRenderer, _graph: &Graph) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Block, Graph, ppm::Renderer};

    #[test]
    fn graph_draw_save() {
        let graph = Graph::new(&[
            Block(4.0, 1.0),
            Block(4.0, 3.0),
            Block(4.0, 1.0),
            Block(4.0, 2.0),
        ]);
        let w = FileWriter{fname: String::from("foo.ppm")};
        let renderer = Renderer::new(&graph.size);
        if let Err(e) = w.write(renderer, &graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "File created");
        }
    }
}
