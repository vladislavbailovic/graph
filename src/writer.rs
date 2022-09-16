use std::io::Result;

use crate::{Graph, ImageRenderer};
pub use rsound_output::{FileWriter, StdoutWriter, Writer};

pub trait ImageWriter {
    fn write_image<T, U>(&self, renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph;
}

impl ImageWriter for FileWriter {
    fn write_image<T, U>(&self, mut renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph,
    {
        graph.draw(&mut renderer);
        self.write(renderer)
    }
}

impl ImageWriter for StdoutWriter {
    fn write_image<T, U>(&self, mut renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph,
    {
        graph.draw(&mut renderer);
        self.write(renderer)
    }
}

#[cfg(test)]
mod tests_file {
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
        let w = FileWriter::new("foo.ppm");
        let renderer = Renderer::new(&graph.size());
        if let Err(e) = w.write_image(renderer, graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "File created");
            let _ = std::fs::remove_file("foo.ppm");
        }
    }
}

#[cfg(test)]
mod tests_stdout {
    use super::*;
    use crate::{svg::Renderer, Block, Roll};

    #[test]
    fn stdout_graph() {
        let graph = Roll::new(&[Block(4.0, 1.0)]);
        let renderer = Renderer::new(&graph.size());
        let w = StdoutWriter::new();

        if let Err(e) = w.write_image(renderer, graph) {
            assert!(false, "{:#?}", e);
        } else {
            assert!(true, "Out rendered");
        }
    }
}
