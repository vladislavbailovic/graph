mod file;
pub use file::*;
pub mod stdout;
pub use stdout::*;

use std::io::Result;

use crate::{Graph, ImageRenderer};

pub trait Writer {
    fn write<T, U>(&self, renderer: T, graph: U) -> Result<()>
    where
        T: ImageRenderer + 'static,
        U: Graph;
}
