pub mod builder;
pub mod analyzer;
mod viz;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};
use graphviz_rust::dot_generator::{graph, node,id};
use graphviz_rust::dot_structures::{Graph, Stmt,Id};
use crate::graph::analyzer::GraphAnalyzer;

type NId = usize;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct EmptyPayload;

impl Debug for EmptyPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")
    }
}

#[derive(Debug)]
pub struct DiGraph<NL, EL> {
    counter: usize,
    nodes: HashMap<NId, NL>,
    edges: HashMap<NId, HashMap<NId, EL>>,
    start: Option<NId>,
}

impl DiGraph<EmptyPayload, EmptyPayload> {
    pub fn empty() -> Self {
        Self::new()
    }
}

impl<NL, EL> DiGraph<NL, EL> {
    fn new_node(&mut self, payload: NL, id: NId) -> NId {
        self.nodes.insert(id, payload);
        if self.start.is_none() {
            self.start = Some(id)
        }
        id
    }
    fn next_id(&mut self) -> NId {
        self.counter += 1;
        self.counter
    }

    pub fn new() -> Self {
        Self { counter: 0, nodes: HashMap::new(), edges: HashMap::new(), start: None }
    }
    pub fn gen_node(&mut self, payload: NL) -> Option<NId> {
        let id = self.next_id();
        Some(self.new_node(payload, id))
    }
    pub fn add_node(&mut self, id: NId, payload: NL) -> Option<NId> {
        Some(self.new_node(payload, id))
    }
    pub fn remove_node(&mut self, id: NId) -> Option<NL> {
        self.nodes.remove(&id)
    }

    pub fn add_edge(&mut self, from: NId, to: NId, payload: EL) -> Option<EL> {
        self.edges.entry(from).or_default().insert(to, payload)
    }
    pub fn remove_edge(&mut self, from: NId, to: NId) -> Option<EL> {
        self.edges.entry(from).or_default().remove(&to)
    }

    pub fn descendants(&self, from: NId) -> Option<&HashMap<NId, EL>> {
        self.edges.get(&from)
    }

    pub fn start(&self) -> Option<NId> {
        self.start
    }
    pub fn find(&self) -> GraphAnalyzer<NL, EL> {
        GraphAnalyzer { graph: &self }
    }
}


impl<NL, EL> DiGraph<NL, EL>
    where NL: Default {
    pub fn gen_bare_node(&mut self) -> Option<NId> {
        self.gen_node(Default::default())
    }
    pub fn add_bare_node(&mut self, id: NId) -> Option<NId> {
        self.add_node(id, Default::default())
    }
}

impl<NL, EL> DiGraph<NL, EL>
    where EL: Default {
    pub fn add_bare_edge(&mut self, from: NId, to: NId) -> Option<EL> {
        self.add_edge(from, to, Default::default())
    }
}








