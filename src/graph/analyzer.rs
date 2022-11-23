use crate::graph::{DiGraph, NId};

#[derive(Debug)]
pub struct GraphAnalyzer<'a, NL, EL> {
    pub(crate) graph: &'a DiGraph<NL, EL>,
}

impl<'a, NL, EL> GraphAnalyzer<'a, NL, EL>
where
    NL: PartialEq,
{
    pub fn node_by_id(&self, id: &NId) -> Option<&NL> {
        self.graph.nodes.get(id)
    }
    pub fn first_node_by_payload(&self, payload: &NL) -> Option<&NL> {
        self.graph.nodes.values().find(|v| *v == payload)
    }
    pub fn node(&self, id: &NId, payload: &NL) -> Option<&NL> {
        self.graph.nodes.get(id).filter(|v| *v == payload)
    }
}

impl<'a, NL, EL> GraphAnalyzer<'a, NL, EL> {
    pub fn edge(&self, from: &NId, to: &NId) -> Option<&EL> {
        self.graph.edges.get(from).and_then(|tos| tos.get(to))
    }
}
