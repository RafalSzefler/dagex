use std::collections::{HashMap, HashSet};

use super::{PhylogeneticNetwork, PhylogeneticNetworkId, Taxon};

pub struct GenesOverSpecies {
    gene_networks: Vec<PhylogeneticNetwork>,
    gene_networks_by_id: HashMap<PhylogeneticNetworkId, i32>,
    species_network: PhylogeneticNetwork,
}

pub enum GenesOverSpeciesFromResult {
    /// Correct [`GenesOverSpecies`] object.
    Ok(GenesOverSpecies),

    /// Collection of gene networks is empty. This is not allowed.
    EmptyGeneNetworks(Vec<PhylogeneticNetwork>, PhylogeneticNetwork),

    /// Incorrect taxa on some gene network, i.e. not a subset of species
    /// network's taxa. Attached values are returned parameters.
    IncorrectTaxa(Vec<PhylogeneticNetwork>, PhylogeneticNetwork),

    /// Collection of gene networks contains duplicate networks, or at least
    /// networks with duplicate ids. This is not allowed.
    DuplicatedIds(Vec<PhylogeneticNetwork>, PhylogeneticNetwork),

    /// Species network cannot have differen leaves with the same taxon.
    SpeciesContainsTaxaDuplicates(Vec<PhylogeneticNetwork>, PhylogeneticNetwork),
}

impl GenesOverSpeciesFromResult {
    /// Unwraps [`GenesOverSpeciesFromResult::Ok`] value.
    /// 
    /// # Panics
    /// Only and always when `self` is not [`GenesOverSpeciesFromResult::Ok`].
    #[inline(always)]
    pub fn unwrap(self) -> GenesOverSpecies {
        if let GenesOverSpeciesFromResult::Ok(genes_over_species) = self {
            genes_over_species
        }
        else
        {
            let name = core::any::type_name::<GenesOverSpeciesFromResult>();
            panic!("{name} not Ok.");
        }
    }
}

impl GenesOverSpecies {
    /// Creates an unchecked [`GenesOverSpecies`].
    /// 
    /// # Safety
    /// It is up to caller to ensure that all properties and invariants are
    /// satisfied and consistent. The following invariants have to
    /// be satisfied:
    /// * `gene_network` taxa has to be a subset of `species_network` taxa
    ///   for each `gene_network` in `gene_networks`.
    /// * `gene_networks_by_id` is a mapping from `PhyologeneticNetworkId`
    ///   to the index of given network in `gene_networks`.
    pub unsafe fn new_unchecked(
        gene_networks: Vec<PhylogeneticNetwork>,
        gene_networks_by_id: HashMap<PhylogeneticNetworkId, i32>,
        species_network: PhylogeneticNetwork) -> Self
    {
        Self { gene_networks, gene_networks_by_id, species_network }
    }

    pub fn from_networks(
        gene_networks: Vec<PhylogeneticNetwork>,
        species_network: PhylogeneticNetwork) -> GenesOverSpeciesFromResult
    {
        if gene_networks.is_empty() {
            return GenesOverSpeciesFromResult::EmptyGeneNetworks(gene_networks, species_network);
        }

        let species_taxa_map = species_network.taxa();
        let mut species_taxa = HashSet::<Taxon>::with_capacity(species_taxa_map.len());
        for taxon in species_taxa_map.values() {
            if !species_taxa.insert(taxon.clone()) {
                return GenesOverSpeciesFromResult::SpeciesContainsTaxaDuplicates(gene_networks, species_network);
            }
        }

        let mut by_id = HashMap::<PhylogeneticNetworkId, i32>::with_capacity(gene_networks.len());

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        for (idx, gene_network) in gene_networks.iter().enumerate() {
            if !has_valid_taxa(gene_network, &species_taxa) {
                return GenesOverSpeciesFromResult::IncorrectTaxa(gene_networks, species_network);
            }
            if by_id.insert(gene_network.id(), idx as i32).is_some() {
                return GenesOverSpeciesFromResult::DuplicatedIds(gene_networks, species_network);
            }
        }

        let result = unsafe {
            Self::new_unchecked(gene_networks, by_id, species_network)
        };

        return GenesOverSpeciesFromResult::Ok(result);
    }

    pub fn from_single_network(
        gene_network: PhylogeneticNetwork,
        species_network: PhylogeneticNetwork) -> GenesOverSpeciesFromResult
    {
        Self::from_networks(vec![gene_network], species_network)
    }

    #[inline(always)]
    pub fn get_gene_networks(&self) -> &[PhylogeneticNetwork] {
        &self.gene_networks
    }

    #[inline(always)]
    pub fn get_gene_network_by_id(&self, id: PhylogeneticNetworkId)
        -> Option<&PhylogeneticNetwork>
    {
        #[allow(clippy::cast_sign_loss)]
        if let Some(idx) = self.gene_networks_by_id.get(&id) {
            Some(&self.gene_networks[*idx as usize])
        }
        else
        {
            None
        }
    }

    #[inline(always)]
    pub fn get_species_network(&self) -> &PhylogeneticNetwork {
        &self.species_network
    }
}


fn has_valid_taxa(
    gene_network: &PhylogeneticNetwork,
    species_taxa: &HashSet<Taxon>) -> bool
{
    let gene_taxa: HashSet<Taxon>
        = gene_network.taxa().values().cloned().collect();
    gene_taxa.is_subset(species_taxa)
}
