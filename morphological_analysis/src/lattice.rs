#[derive(Debug)]
pub struct LatticeNode {
    begin_nodes: Vec<usize>,
    end_nodes: Vec<usize>,
}

#[derive(Debug)]
pub struct Lattice {
    indexes: Vec<LatticeNode>,
}

impl Lattice {
    pub fn new() -> Lattice {
        Lattice {}
    }

    pub fn parse(string: &str) -> Lattice {}
}
