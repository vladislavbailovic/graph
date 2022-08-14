mod file;
pub use file::*;

use std::io::Result;

use crate::{Graph, ImageRenderer};

trait Writer {
    fn write(&self, renderer: impl ImageRenderer, graph: &Graph) -> Result<()>;
}

pub struct StdoutWriter;
impl Writer for StdoutWriter {
    fn write(&self, _renderer: impl ImageRenderer, _graph: &Graph) -> Result<()> {
        Ok(())
    }
}
