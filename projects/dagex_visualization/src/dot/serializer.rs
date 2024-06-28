#![allow(clippy::cast_sign_loss)]
use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}, io::{Error, Write}};

use dagex::core::DirectedGraphDTO;

use super::{
    graph_model_serializer::DotLangSerializer,
    models::{
        EdgeOp,
        EdgeRHS,
        EdgeStatement,
        EdgeStatementItem,
        Graph,
        Id,
        NodeId,
        NodeStatement,
        Statement,
        Subgraph},
    traits::{DotSerializable, DotSerializer}};

pub struct DefaultDotSerializer<TWrite: Write> {
    stream: TWrite,
}

impl<TWrite: Write> From<TWrite> for DefaultDotSerializer<TWrite> {
    fn from(value: TWrite) -> Self {
        Self { stream: value }
    }
}

impl<TWrite: Write> DotSerializer<TWrite> for DefaultDotSerializer<TWrite> {
    fn release(self) -> TWrite { self.stream }
}

impl<TWrite> DotSerializable<TWrite, DefaultDotSerializer<TWrite>> for DirectedGraphDTO
    where TWrite: Write
{
    fn serialize(&self, ser: &mut DefaultDotSerializer<TWrite>) -> Result<usize, Error> {
        let graph = self;

        let hash = {
            let mut hasher = fnv1a_hasher::FNV1a32Hasher::new();
            graph.hash(&mut hasher);
            hasher.finish()
        };

        let mut stmts = Vec::new();        

        let no = graph.number_of_nodes();
        let mut arrows_map = HashMap::<i32, Vec<i32>>
            ::with_capacity(no as usize);
        let mut seen = HashSet::with_capacity(no as usize);

        for arrow in graph.arrows() {
            let src = arrow.source();
            let targets = if let Some(targs) = arrows_map.get_mut(&src) { targs } else {
                let targs = Vec::with_capacity(2);
                arrows_map.insert(src, targs);
                arrows_map.get_mut(&src).unwrap()
            };
            targets.push(arrow.target());
            seen.insert(src);
            seen.insert(arrow.target());
        }

        for idx in 0..no {
            let source_id = NodeId::new(Id::new(idx.to_string().as_str()), None);
            match arrows_map.get(&idx) {
                Some(targets) => {
                    let source = EdgeStatementItem::Node(source_id);
                    let mut subgraph_stmts = Vec::with_capacity(targets.len());
                    for target in targets {
                        let target_id = NodeId::new(Id::new(target.to_string().as_str()), None);
                        let node_stmt = NodeStatement::new(target_id, Vec::new());
                        subgraph_stmts.push(Statement::Node(node_stmt));
                    }
                    let subgraph = Subgraph::new(None, subgraph_stmts);
                    let item = EdgeStatementItem::Subgraph(subgraph);
                    let rhs = EdgeRHS::new(EdgeOp::Directed, item, None);
                    let stmt = EdgeStatement::new(source, rhs);
                    stmts.push(Statement::Edge(stmt));
                },
                None => {
                    if !seen.contains(&idx) {
                        stmts.push(Statement::Node(
                            NodeStatement::new(source_id, Vec::new())));
                    }
                },
            }
        }

        let id = Id::new(format!("dagex_{hash}").as_str());
        let graph = Graph::new(Some(id), true, true, stmts);
        let serializer = DotLangSerializer::new(&graph);
        let result = serializer.serialize();
        ser.stream.write_all(result.as_bytes())?;
        Ok(result.len())
    }
}
