pub mod analyzer;
pub mod builder;
pub mod dijkstra;
pub mod min_weight;
pub mod visit;
pub mod visualizer;

use crate::graph::analyzer::GraphAnalyzer;
use crate::graph::visualizer::{visualize_to_file, VizGraph, ToStringProcessor, Processor, visualize};
use graphviz_rust::dot_generator::{graph, id, node};
use graphviz_rust::dot_structures::{Graph, Id, Stmt};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};
use std::hash::Hash;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct EmptyPayload;

impl ToString for EmptyPayload {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

impl Debug for EmptyPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(".")
    }
}

#[derive(Debug)]
pub struct DiGraph<NId, NL, EL>
where
    NId: Eq + Hash,
{
    nodes: HashMap<NId, NL>,
    edges: HashMap<NId, HashMap<NId, EL>>,
    start: Option<NId>,
}

impl DiGraph<usize, EmptyPayload, EmptyPayload> {
    pub fn empty() -> Self {
        Self::new()
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
where
    NId: Eq + Hash + ToString,
    NL: ToString,
    EL: ToString,
{

}

impl<NId: ToString, NL: ToString, EL: ToString> DiGraph<NId, NL, EL>
where
    NId: Eq + Hash + ToString,
    NL: ToString,
    EL: ToString,
{
    pub fn to_file(&self, path: &str) -> std::io::Result<String> {
        let cg = VizGraph::new(self);
        let graph = cg.to_dot(ToStringProcessor {});
        visualize_to_file(graph, path.to_string())
    }
    pub fn to_file_with<'a,P>(&'a self, path: &str, processor:P ) -> std::io::Result<String>
    where P:Processor<'a,NId,NL,EL>
    {
        let cg = VizGraph::new(self);
        let graph = cg.to_dot(processor);
        visualize_to_file(graph, path.to_string())
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
where
    NId: Clone + Eq + Hash,
{
    fn insert_new_node(&mut self, payload: NL, id: NId) -> NId {
        self.nodes.insert(id.clone(), payload);
        if self.start.is_none() {
            self.start = Some(id.clone())
        }

        id
    }

    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            start: None,
        }
    }

    fn add_node(&mut self, id: NId, payload: NL) -> Option<NId> {
        Some(self.insert_new_node(payload, id))
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

    pub fn start(&self) -> &Option<NId> {
        &self.start
    }
    pub fn find(&self) -> GraphAnalyzer<NId, NL, EL> {
        GraphAnalyzer { graph: &self }
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
where
    NId: Clone + Eq + Hash,
    NL: Default,
{
    fn add_bare_node(&mut self, id: NId) -> Option<NId> {
        self.add_node(id, Default::default())
    }
}

impl<NId, NL, EL> DiGraph<NId, NL, EL>
where
    NId: Clone + Eq + Hash,
    EL: Default,
{
    pub fn add_bare_edge(&mut self, from: NId, to: NId) -> Option<EL> {
        self.add_edge(from, to, Default::default())
    }
}
