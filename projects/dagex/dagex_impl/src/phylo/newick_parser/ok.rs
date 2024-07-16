use crate::phylo::PhylogeneticNetwork;

#[derive(Debug)]
pub struct NewickParseOk {
    pub network: PhylogeneticNetwork,
    pub read_bytes: usize,
}
