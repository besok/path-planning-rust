use crate::graph::{DiGraph, Node, NodeId};

#[derive(Debug)]
pub struct GraphAnalyzer<'a, NL, EL> {
    pub(crate) graph: &'a DiGraph<NL, EL>,
}

impl<'a, NL, EL> GraphAnalyzer<'a, NL, EL>
    where NL: PartialEq
{
    pub fn node_by_id(&self, id: &NodeId) -> Option<&Node<NL>> {
        self.graph.nodes.get(id)
    }
    pub fn first_node_by_payload(&self, payload: &NL) -> Option<&Node<NL>> {
        self.graph.nodes.values().find(|v| &v.payload == payload)
    }
    pub fn node(&self, vertex: &Node<NL>) -> Option<&Node<NL>> {
        self.graph.nodes.get(&vertex.id).filter(|v| v.payload == v.payload)
    }
}

impl<'a, NL, EL> GraphAnalyzer<'a, NL, EL> {

    pub fn edge(&self, from: NodeId, to: NodeId) -> Option<&EL> {
        self.graph.edges.get(&from).and_then(|tos| tos.get(&to))
    }
}