#![allow(clippy::cast_sign_loss)]
use super::models::{Compass, EdgeOp, EdgeRHS, EdgeStatement, EdgeStatementItem, Graph, NodeId, Port, Statement, Subgraph};

pub struct DotLangSerializer<'a> {
    graph: &'a Graph,
    buffer: Vec<String>,
    current_indent: i32,
}

impl<'a> DotLangSerializer<'a> {
    pub fn new(graph: &'a Graph) -> DotLangSerializer<'_> {
        Self { graph, buffer: Vec::new(), current_indent: 0 }
    }

    pub fn serialize(mut self) -> String {
        let graph = self.graph;
        let buffer = &mut self.buffer;

        if graph.strict() {
            buffer.push("strict ".to_owned());
        }

        if graph.directed() {
            buffer.push("digraph ".to_owned());
        }
        else
        {
            buffer.push("graph ".to_owned());    
        }

        if let Some(id) = graph.id() {
            buffer.push(id.as_string().clone());
            buffer.push(" ".to_owned());
        }

        self.serialize_statements(graph.statements(), true);

        self.buffer.concat()
    }

    fn indent(&self) -> String {
        let indent = self.current_indent as usize;
        let mut result = String::with_capacity(indent);
        for _ in 0..indent {
            result.push(' ');
        }
        return result;
    }

    fn serialize_statements(&mut self, stmts: &Vec<Statement>, indent: bool) {
        if indent {
            self.buffer.push("{\n".to_owned());
            self.current_indent += 4;
            for stmt in stmts {
                self.buffer.push(self.indent());
                self.serialize_statement(stmt);
                self.buffer.push(";\n".to_owned());
            }
            self.current_indent -= 4;
            self.buffer.push("}\n".to_owned());
        }
        else
        {
            self.buffer.push("{".to_owned());
            let count = stmts.len();
            for stmt in stmts.iter().take(count-1) {
                self.serialize_statement(stmt);
                self.buffer.push("; ".to_owned());
            }
            let stmt = &stmts[count-1];
            self.serialize_statement(stmt);
            self.buffer.push("}".to_owned());
        }
    }
    
    fn serialize_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Edge(edge) => {
                self.serialize_edge(edge);
            },
            Statement::Node(node) => {
                self.serialize_node_id(node.node());
                let attrs = node.attrs();
                if !attrs.is_empty() {
                    self.buffer.push("[".to_owned());
                    let size = attrs.len();
                    for attr in attrs.iter().take(size-1) {
                        self.buffer.push(attr.key().as_string().clone());
                        self.buffer.push("=".to_owned());
                        self.buffer.push(attr.value().as_string().clone());
                        self.buffer.push("; ".to_owned());
                    }
                    let attr = &attrs[size-1];
                    self.buffer.push(attr.key().as_string().clone());
                    self.buffer.push("=".to_owned());
                    self.buffer.push(attr.value().as_string().clone());
                    self.buffer.push("]".to_owned());
                }
            }
        }
    }

    fn serialize_edge(&mut self, edge: &EdgeStatement) {
        match edge.source() {
            EdgeStatementItem::Node(node) => {
                self.serialize_node_id(node);
            }
            EdgeStatementItem::Subgraph(subgraph) => {
                self.serialize_subgraph(subgraph);
            }
        }
        
        self.buffer.push(" ".to_owned());
        self.serialize_rhs(edge.rhs());
    }

    fn serialize_subgraph(&mut self, subgraph: &Subgraph) {
        if let Some(id) = subgraph.id() {
            self.buffer.push(format!("subgraph {} ", id.as_string()));
        }
        self.serialize_statements(subgraph.statements(), false);
    }

    fn serialize_node_id(&mut self, node_id: &NodeId) {
        self.buffer.push(node_id.id().as_string().clone());
        if let Some(port) = node_id.port() {
            self.buffer.push(":".to_owned());
            match port {
                Port::Id(id) => {
                    self.buffer.push(id.id().as_string().clone());
                    if let Some(compass) = id.compass() {
                        self.buffer.push(":".to_owned());
                        self.serialize_compass(compass);
                    }
                },
                Port::Compass(compass) => {
                    self.buffer.push(":".to_owned());
                    self.serialize_compass(compass.compass());
                }
            }
        }
    }
    
    fn serialize_compass(&mut self, compass: &Compass) {
        match compass {
            Compass::N => self.buffer.push("n".to_owned()),
            Compass::NE => self.buffer.push("ne".to_owned()),
            Compass::E => self.buffer.push("e".to_owned()),
            Compass::SE => self.buffer.push("se".to_owned()),
            Compass::S => self.buffer.push("s".to_owned()),
            Compass::SW => self.buffer.push("sw".to_owned()),
            Compass::W => self.buffer.push("w".to_owned()),
            Compass::NW => self.buffer.push("nw".to_owned()),
            Compass::C => self.buffer.push("c".to_owned()),
            Compass::UNDERSCORE => self.buffer.push("_".to_owned()),
        }
    }
    
    fn serialize_rhs(&mut self, rhs: &EdgeRHS) {
        match rhs.operator() {
            EdgeOp::Directed => {
                self.buffer.push("-> ".to_owned());
            },
            EdgeOp::Undirected => {
                self.buffer.push("-- ".to_owned());
            },
        }
        
        match rhs.target() {
            EdgeStatementItem::Node(node) => {
                self.serialize_node_id(node);
            }
            EdgeStatementItem::Subgraph(subgraph) => {
                self.serialize_subgraph(subgraph);
            }
        }

        if let Some(next) = rhs.next() {
            self.serialize_rhs(next);
        }
    }
    
}
