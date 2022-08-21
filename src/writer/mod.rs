mod file;
pub use file::*;

use std::io::Result;

use crate::{Graph, ImageRenderer};

pub trait Writer {
    fn write<T, U>(&self, renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph;
}

pub struct StdoutWriter;
impl Writer for StdoutWriter {
    fn write<T, U>(&self, _renderer: T, _graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph,
    {
        Ok(())
    }
}
