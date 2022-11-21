pub mod builder;
pub mod analyzer;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};
use crate::graph::analyzer::GraphAnalyzer;

type NodeId = usize;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct EmptyPayload;

impl Debug for EmptyPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")
    }
}

#[derive(Debug)]
pub struct Node<T> {
    id: NodeId,
    payload: T,
}


#[derive(Debug)]
pub struct DiGraph<NL, EL> {
    counter: usize,
    nodes: HashMap<NodeId, Node<NL>>,
    edges: HashMap<NodeId, HashMap<NodeId, EL>>,
    start: Option<NodeId>,
}


impl DiGraph<EmptyPayload, EmptyPayload> {
    pub fn empty() -> Self {
        Self::new()
    }
}

impl<NL, EL> DiGraph<NL, EL> {
    fn new_node(&mut self, payload: NL, id: NodeId) -> NodeId {
        self.nodes.insert(id, Node { id, payload });
        if self.start.is_none() {
            self.start = Some(id)
        }
        id
    }
    fn next_id(&mut self) -> NodeId {
        self.counter += 1;
        self.counter
    }

    pub fn new() -> Self {
        Self { counter: 0, nodes: HashMap::new(), edges: HashMap::new(), start: None }
    }
    pub fn gen_node(&mut self, payload: NL) -> Option<NodeId> {
        let id = self.next_id();
        Some(self.new_node(payload, id))
    }
    pub fn add_node(&mut self, id: NodeId, payload: NL) -> Option<NodeId> {
        Some(self.new_node(payload, id))
    }
    pub fn remove_node(&mut self, id: NodeId) -> Option<Node<NL>> {
        self.nodes.remove(&id)
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, payload: EL) -> Option<EL> {
        self.edges.entry(from).or_default().insert(to, payload)
    }
    pub fn remove_edge(&mut self, from: NodeId, to: NodeId) -> Option<EL> {
        self.edges.entry(from).or_default().remove(&to)
    }

    pub fn descendants(&self, from: NodeId) -> Option<&HashMap<NodeId, EL>> {
        self.edges.get(&from)
    }
    pub fn start(&self) -> Option<NodeId> {
        self.start
    }
    pub fn find(&self) -> GraphAnalyzer<NL, EL> {
        GraphAnalyzer{ graph: &self }
    }
}


impl<NL, EL> DiGraph<NL, EL>
    where NL: Default {
    pub fn gen_bare_node(&mut self) -> Option<NodeId> {
        self.gen_node(Default::default())
    }
    pub fn add_bare_node(&mut self, id: NodeId) -> Option<NodeId> {
        self.add_node(id, Default::default())
    }
}

impl<NL, EL> DiGraph<NL, EL>
    where EL: Default {
    pub fn add_bare_edge(&mut self, from: NodeId, to: NodeId) -> Option<EL> {
        self.add_edge(from, to, Default::default())
    }
}








