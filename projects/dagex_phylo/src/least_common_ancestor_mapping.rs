use std::collections::HashMap;

use dagex_core::Node;

use crate::{GenesOverSpecies, PhylogeneticNetworkId};

pub(crate) type NodeMap = HashMap<Node, Node>;
pub(crate) type PhyloMap = HashMap<PhylogeneticNetworkId, NodeMap>;

pub struct LeastCommonAncestorMapping {
    genes_over_species: GenesOverSpecies,
    mapping: PhyloMap,
}

impl LeastCommonAncestorMapping {
    /// Constructs `LeastCommonAncestorMapping` directly.
    /// 
    /// # Safety
    /// This method is unsafe since it doesn't verify invariants:
    /// * `mapping` has to be a valid LCA mapping.
    #[inline(always)]
    pub unsafe fn from_unchecked(
        genes_over_species: GenesOverSpecies,
        mapping: PhyloMap) -> Self
    {
        Self { genes_over_species, mapping }
    }

    #[inline(always)]
    pub fn get_genes_over_species(&self) -> &GenesOverSpecies {
        &self.genes_over_species
    }

    #[inline(always)]
    pub fn get_mapping_for_network(&self, id: PhylogeneticNetworkId)
        -> Option<&NodeMap>
    {
        self.mapping.get(&id)
    }
}
