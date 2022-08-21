mod file;
pub use file::*;

use std::io::Result;

use crate::{Graph, ImageRenderer};

trait Writer {
    fn write(&self, renderer: impl ImageRenderer + 'static, graph: Box<dyn Graph>) -> Result<()>;
}

pub struct StdoutWriter;
impl Writer for StdoutWriter {
    fn write(&self, _renderer: impl ImageRenderer + 'static, _graph: Box<dyn Graph>) -> Result<()> {
        Ok(())
    }
}
