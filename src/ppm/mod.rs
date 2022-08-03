use std::fs::File;
use std::io::{BufWriter, Write};

use crate::{Graph, GraphFileWriter};

pub struct Writer;
impl GraphFileWriter for Writer {
    #[allow(clippy::unused_io_amount)]
    fn write(&self, fname: &str, graph: &Graph) -> std::io::Result<()> {
        let mut p = BufWriter::new(File::create(fname)?);
        p.write(format!("P6 {} {} 255\n", graph.size.w, graph.size.h).as_bytes())?;

        let buffer = graph.draw();
        p.write(&buffer)?;

        Ok(())
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
