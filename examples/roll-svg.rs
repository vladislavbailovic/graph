use graph::svg::Renderer;
use graph::writer::{FileWriter, Writer};
use graph::{Block, Graph, Roll};

fn main() -> std::io::Result<()> {
    let blocks = [
        Block::new(4.0, 1.0),
        Block::new(4.0, 3.0),
        Block::new(4.0, 1.0),
        Block::new(4.0, 2.0),
    ];
    let graph = Roll::new(&blocks);
    let w = FileWriter::new("foo.svg");
    let renderer = Renderer::new(&graph.size());
    w.write(renderer, graph)?;

    Ok(())
}
