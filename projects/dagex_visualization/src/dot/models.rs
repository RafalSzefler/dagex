#![allow(dead_code)]

pub struct Id {
    value: String,
}

impl Id {
    pub fn new(value: &str) -> Self { 
        let repl = format!("\"{}\"", value.replace('"', "\\\""));
        Self { value: repl }
    }
    pub fn as_string(&self) -> &String {
        &self.value
    }
}

pub enum Compass {
    N, NE, E, SE, S, SW, W, NW, C, UNDERSCORE,
}

pub struct IdPort {
    id: Id,
    compass: Option<Compass>,
}

impl IdPort {
    pub fn new(id: Id, compass: Option<Compass>) -> Self {
        Self { id, compass }
    }
    pub fn id(&self) -> &Id { &self.id }
    pub fn compass(&self) -> &Option<Compass> { &self.compass }
}

pub struct CompassPort {
    compass: Compass,
}

impl CompassPort {
    pub fn new(compass: Compass) -> Self {
        Self { compass }
    }
    pub fn compass(&self) -> &Compass { &self.compass }
}

pub enum Port {
    Id(IdPort),
    Compass(CompassPort),
}

pub struct NodeId {
    id: Id,
    port: Option<Port>,
}

impl NodeId {
    pub fn new(id: Id, port: Option<Port>) -> Self {
        Self { id, port }
    }
    pub fn id(&self) -> &Id { &self.id }
    pub fn port(&self) -> &Option<Port> { &self.port }
}

pub struct Graph {
    id: Option<Id>,
    strict: bool,
    directed: bool,
    statements: Vec<Statement>,
}

impl Graph {
    pub fn new(id: Option<Id>, strict: bool, directed: bool, statements: Vec<Statement>) -> Self {
        Self { id, strict, directed, statements }
    }
    pub fn id(&self) -> &Option<Id> { &self.id }
    pub fn strict(&self) -> bool { self.strict }
    pub fn directed(&self) -> bool { self.directed }
    pub fn statements(&self) -> &Vec<Statement> { &self.statements }
}

pub struct Subgraph {
    id: Option<Id>,
    statements: Vec<Statement>,
}

impl Subgraph {
    pub fn new(id: Option<Id>, statements: Vec<Statement>) -> Self {
        Self { id, statements }
    }
    pub fn id(&self) -> &Option<Id> { &self.id }
    pub fn statements(&self) -> &Vec<Statement> { &self.statements }
}

pub enum EdgeStatementItem {
    Node(NodeId),
    Subgraph(Subgraph),
}

pub enum EdgeOp {
    Directed,
    Undirected,
}

pub struct EdgeRHS {
    operator: EdgeOp,
    target: EdgeStatementItem,
    next: Option<Box<EdgeRHS>>,
}

impl EdgeRHS {
    pub fn new(operator: EdgeOp, target: EdgeStatementItem, next: Option<Box<EdgeRHS>>) -> Self {
        Self { operator, target, next }
    }
    pub fn operator(&self) -> &EdgeOp { &self.operator }
    pub fn target(&self) -> &EdgeStatementItem { &self.target }
    pub fn next(&self) -> &Option<Box<EdgeRHS>> { &self.next }
}

pub struct EdgeStatement {
    source: EdgeStatementItem,
    rhs: EdgeRHS,
}

impl EdgeStatement {
    pub fn new(source: EdgeStatementItem, rhs: EdgeRHS) -> Self {
        Self { source, rhs }
    }
    pub fn source(&self) -> &EdgeStatementItem { &self.source }
    pub fn rhs(&self) -> &EdgeRHS { &self.rhs }
}

pub struct Attr {
    key: Id,
    value: Id,
}

impl Attr {
    pub fn new(key: Id, value: Id) -> Self {
        Self { key, value }
    }
    pub fn key(&self) -> &Id { &self.key }
    pub fn value(&self) -> &Id { &self.value }
}

pub struct NodeStatement {
    node: NodeId,
    attrs: Vec<Attr>,
}

impl NodeStatement {
    pub fn new(node: NodeId, attrs: Vec<Attr>) -> Self {
        Self { node, attrs }
    }
    pub fn node(&self) -> &NodeId { &self.node }
    pub fn attrs(&self) -> &Vec<Attr> { &self.attrs }
}

pub enum Statement {
    Edge(EdgeStatement),
    Node(NodeStatement),
}
