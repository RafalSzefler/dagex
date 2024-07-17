use std::collections::{hash_map::Entry, HashMap, HashSet};

use raf_immutable_string::ImmutableString;
use raf_newick::ast::{NewickGraph, NewickNodeId};

use crate::{core::{ArrowDTO, DirectedGraphDTO}, phylo::{PhylogeneticNetwork, PhylogeneticNetworkDTO}};

use super::NewickParseError;


pub(super) struct NewickParseContext<'a> {
    graph: &'a NewickGraph,
    number_of_nodes: i32,
    reticulation_map: HashMap<u32, HashSet<NewickNodeId>>,
    reticulation_ids: HashMap<u32, i32>,
    node_map: HashMap<NewickNodeId, i32>,
    taxa: HashMap<i32, ImmutableString>,
    arrows: Vec<ArrowDTO>,
}


macro_rules! perr {
    ( $e: expr ) => {
        {
            let msg = format!($e);
            return Err(NewickParseError::ContentError(msg));
        }
    };
}


impl<'a> NewickParseContext<'a> {
    pub fn new(graph: &'a NewickGraph) -> Self {
        Self {
            graph: graph,
            reticulation_map: calculate_reticulation_map(graph),
            reticulation_ids: HashMap::new(),
            number_of_nodes: 0,
            node_map: HashMap::new(),
            taxa: HashMap::new(),
            arrows: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn parse(mut self) -> Result<PhylogeneticNetwork, NewickParseError>
    {
        self.calculate_reticulation_ids()?;
        self.calculate_arrows();
        let dag_dto = DirectedGraphDTO::new(self.number_of_nodes, self.arrows);
        let phylo_dto = PhylogeneticNetworkDTO::new(dag_dto, self.taxa);
        let network = PhylogeneticNetwork::from_dto(&phylo_dto)?;
        Ok(network)
    }

    fn calculate_reticulation_ids(&mut self) -> Result<(), NewickParseError> {
        for key in self.reticulation_map.keys() {
            let idx = self.number_of_nodes;
            self.number_of_nodes += 1;
            self.reticulation_ids.insert(*key, idx);
        }
        for node in self.graph.nodes() {
            let idx = if let Some(reticulation) = node.reticulation() {
                let ret_id = reticulation.id();
                let ret_node_id = *self.reticulation_ids.get(&ret_id).unwrap();
                self.node_map.insert(node.id(), ret_node_id);
                ret_node_id
            } else {
                let idx = self.number_of_nodes;
                self.number_of_nodes += 1;
                self.node_map.insert(node.id(), idx);
                idx
            };

            let node_name = node.name().as_immutable_string();
            if !node_name.is_empty() {
                if let Some(old_value) = self.taxa.insert(idx, node_name.clone()) {
                    if &old_value != node_name {
                        perr!("Conflict in node names, likely two reticulation entries have different name but point to the same id.");
                    }
                }
            }
        }
        Ok(())
    }

    fn calculate_arrows(&mut self) {
        for node in self.graph.nodes() {
            let source_id = *self.node_map.get(&node.id()).unwrap();
            let successors = self.graph.get_children(node.id());
            for successor in successors {
                let target_id = *self.node_map.get(successor).unwrap();
                self.arrows.push(ArrowDTO::new(source_id, target_id));
            }
        }
    }
}

fn calculate_reticulation_map(graph: &NewickGraph)
    -> HashMap<u32, HashSet<NewickNodeId>>
{
    let mut reticulation_map: HashMap<u32, HashSet<NewickNodeId>>
        = HashMap::new();

    for node in graph.nodes() {
        if let Some(reticulation) = node.reticulation() {
            match reticulation_map.entry(reticulation.id()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(node.id());
                },
                Entry::Vacant(e) => {
                    let set = HashSet::from([node.id()]);
                    e.insert(set);
                }
            }
        }
    }
    reticulation_map
}
