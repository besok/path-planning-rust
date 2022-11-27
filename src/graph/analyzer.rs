use std::hash::Hash;
use graphviz_rust::dot_structures::Node;
use crate::graph::{DiGraph};
use crate::graph::dijkstra::DijkstraPath;

#[derive(Debug)]
pub struct GraphAnalyzer<'a, NodeId, NL, EL>
where NodeId: Eq+ Hash
{
    pub(crate) graph: &'a DiGraph<NodeId, NL, EL>,
}

impl<'a, NodeId, NL, EL> GraphAnalyzer<'a, NodeId, NL, EL>
where
    NodeId: Eq+ Hash,
    NL: PartialEq,
{
    pub fn node_by_id(&self, id: &NodeId) -> Option<&NL> {
        self.graph.nodes.get(id)
    }
    pub fn first_node_by_payload(&self, payload: &NL) -> Option<&NL> {
        self.graph.nodes.values().find(|v| *v == payload)
    }
    pub fn node(&self, id: &NodeId, payload: &NL) -> Option<&NL> {
        self.graph.nodes.get(id).filter(|v| *v == payload)
    }


}

impl<'a,NodeId, NL, EL> GraphAnalyzer<'a,NodeId, NL, EL>
where NodeId: Eq+ Hash
{
    pub fn edge(&self, from: &NodeId, to: &NodeId) -> Option<&EL> {
        self.graph.edges.get(from).and_then(|tos| tos.get(to))
    }
}
