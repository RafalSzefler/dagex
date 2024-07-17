#![allow(clippy::cast_sign_loss)]
use std::collections::{hash_map::Entry, HashMap};

use dagex_impl::{
    core::{DirectedGraph, Node},
    phylo::{PhylogeneticNetwork, Taxon},
    raf_immutable_string::ImmutableString,
};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn convert(network: &PhylogeneticNetwork) -> TokenStream {
    assert!(network.graph().number_of_nodes() > 0, "Graph has to have positive number of nodes.");
    let dg_stream = convert_dg(network.graph());
    let taxa_stream = convert_taxa(network.taxa());

    quote! {
        unsafe {
            use std::collections::{HashMap, HashSet};
            use dagex::{
                macro_helpers,
                core::{DirectedGraph, Node, DirectedGraphBasicProperties},
                phylo::{PhylogeneticNetwork, Taxon}};

            #dg_stream
            #taxa_stream

            PhylogeneticNetwork::new_unchecked(graph, taxa)
        }
    }
}

fn convert_taxa(taxa: &HashMap<Node, Taxon>) -> TokenStream {
    let len = taxa.len();
    let mut stream = TokenStream::new();
    let mut seen = HashMap::<ImmutableString, Vec<Node>>::with_capacity(taxa.len());
    for (key, value) in taxa {
        match seen.entry(value.value().clone()) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(*key);
            },
            Entry::Vacant(e) => {
                e.insert(Vec::from([*key]));
            }
        }
    }
    
    for (key, nodes) in seen {
        let mut substream = TokenStream::new();
        let nodes_len = nodes.len();
        if nodes_len == 0 {
            continue;
        }

        for node in nodes.iter().take(nodes_len-1) {
            let node_id = node.id();
            substream.extend(quote! {
                result.insert(Node::from(#node_id), taxon.clone());
            });
        }

        let node_id = nodes[nodes_len-1].id();
        substream.extend(quote! {
            result.insert(Node::from(#node_id), taxon);
        });

        let str = key.as_str();
        stream.extend(quote! {
            {
                let taxon = Taxon::new(#str).unwrap();
                #substream
            }
        });

    }

    if len > 0 {
        quote! {
            let taxa = {
                let mut result = HashMap::<Node, Taxon>::with_capacity(#len);
                #stream
                result
            };
        }
    } else {
        quote! {
            let taxa = HashMap::<Node, Taxon>::new();
        }
    }
}

fn convert_dg(graph: &DirectedGraph) -> TokenStream {
    let no = graph.number_of_nodes();
    let root_node_stream = if let Some(node) = graph.root() {
        let id = node.id();
        quote! { let root = Option::Some(Node::from(#id)); }
    } else {
        quote! { let root = Option::<Node>::None; }
    };

    let props_stream = convert_basic_properties(graph);
    let leaves = convert_leaves(graph);
    let (predecessors, successors) = convert_predecessors_and_successors(graph);

    quote! {
        let graph = {
            #props_stream
            #root_node_stream
            #leaves
            #predecessors
            #successors
            DirectedGraph::new_unchecked(
                #no,
                succs,
                preds,
                props,
                root,
                leaves
            )
        };
    }
}

fn convert_predecessors_and_successors(graph: &DirectedGraph) -> (TokenStream, TokenStream) {
    let mut preds_stream = TokenStream::new();
    let mut succs_stream = TokenStream::new();
    for node in graph.iter_nodes() {
        let src_id = node.id() as usize;
        let preds = graph.get_predecessors(node);
        let preds_len = preds.len();
        if preds_len > 0 {
            let mut substream = TokenStream::new();
            for (idx, pred) in preds.iter().enumerate() {
                let trg_id = pred.id();
                substream.extend(quote! {
                    subarray[#idx] = Node::from(#trg_id);
                });
            }
            preds_stream.extend(quote! {
                {
                    let subarray = &mut data[#src_id];
                    subarray.resize_with(#preds_len, macro_helpers::default_node);
                    #substream
                }
            });
        }

        let succs = graph.get_successors(node);
        let succs_len = succs.len();
        if succs_len > 0 {
            let mut substream = TokenStream::new();
            for (idx, successor) in succs.iter().enumerate() {
                let trg_id = successor.id();
                substream.extend(quote! {
                    subarray[#idx] = Node::from(#trg_id);
                });
            }
            succs_stream.extend(quote! {
                {
                    let subarray = &mut data[#src_id];
                    subarray.resize_with(#succs_len, macro_helpers::default_node);
                    #substream
                }
            });
        }
    }

    let graph_size = graph.number_of_nodes() as usize;

    let preds_result = quote! {
        let preds = {
            let mut data = macro_helpers::empty_arow_map();
            data.resize_with(#graph_size, Default::default);
            #preds_stream
            data
        };
    };
    let succs_result = quote! {
        let succs = {
            let mut data = macro_helpers::empty_arow_map();
            data.resize_with(#graph_size, Default::default);
            #succs_stream
            data
        };
    };
    (preds_result, succs_result)
}

fn convert_leaves(graph: &DirectedGraph) -> TokenStream {
    let graph_leaves = graph.leaves();
    let len = graph_leaves.len();
    assert!(len > 0, "Graph needs positive number of leaves.");

    let mut substream = TokenStream::new();
    for leaf in graph_leaves {
        let id = leaf.id();
        let subquote = quote! {
            result.insert(Node::from(#id));
        };
        substream.extend(subquote);
    }

    quote! {
        let leaves = {
            let mut result = HashSet::<Node>::with_capacity(#len);
            #substream
            result
        };
    }
}

fn convert_basic_properties(graph: &DirectedGraph) -> TokenStream {
    let props = graph.basic_properties();
    let acyclic = props.acyclic;
    let connected = props.connected;
    let rooted = props.rooted;
    let binary = props.binary;
    let tree = props.tree;
    quote! {
        let props = DirectedGraphBasicProperties {
            acyclic: #acyclic,
            connected: #connected,
            rooted: #rooted,
            binary: #binary,
            tree: #tree,
        };
    }
}
