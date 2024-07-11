use std::{collections::HashMap, hash::Hasher, marker::PhantomData};

use dagex::phylo::PhylogeneticNetworkId;

#[derive(Debug, PartialEq, Eq)]
pub struct EpisodeFeasabilityOutput<'a> {
    result: HashMap<PhylogeneticNetworkId, bool>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> EpisodeFeasabilityOutput<'a> {
    pub fn new(
        result: HashMap<PhylogeneticNetworkId, bool>) -> Self
    {
        Self {
            result,
            phantom: PhantomData,
        }
    }
    
    pub fn result(&self) -> &HashMap<PhylogeneticNetworkId, bool> {
        &self.result
    }
}

impl<'a> core::hash::Hash for EpisodeFeasabilityOutput<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.result.len().hash(state);

        let mut total_hash = self.result.len() as u64;
        for node in &self.result {
            let mut fnv1 = raf_fnv1a_hasher::FNV1a32Hasher::new();
            node.hash(&mut fnv1);
            total_hash ^= fnv1.finish();
        }
        state.write_u64(total_hash);
    }
}
