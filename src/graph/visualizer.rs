use crate::graph::{DiGraph, EmptyPayload};
use graphviz_rust::attributes::{EdgeAttributes, NodeAttributes};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use graphviz_rust::{exec, exec_dot};
use std::hash::Hash;

pub trait Processor<'a, NId, NL, EL> {
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt;
    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt;
}

pub struct VizGraph<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> VizGraph<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph }
    }
    pub fn to_dot<C>(&self, processor: C) -> Graph
    where
        C: Processor<'a, NId, NL, EL>,
    {
        let mut dot = graph!(strict di id!("di_graph"));
        for (id, pl) in self.graph.nodes.iter() {
            dot.add_stmt(processor.node(id, pl));
        }
        for (from, to_map) in self.graph.edges.iter() {
            for (to, pl) in to_map.iter() {
                dot.add_stmt(processor.edge(from, to, pl))
            }
        }
        dot
    }
}

pub fn visualize(dot_graph: Graph) -> String {
    dot_graph.print(&mut PrinterContext::default())
}

pub fn visualize_to_file(dot_graph: Graph, path: String) -> std::io::Result<String> {
    exec(
        dot_graph,
        &mut PrinterContext::default(),
        vec![CommandArg::Output(path), CommandArg::Format(Format::Svg)],
    )
}

pub struct ToStringProcessor;

impl ToStringProcessor {
    pub fn node_with_attrs<'a, NId, NL>(
        &self,
        id: &'a NId,
        nl: &'a NL,
        attrs: Vec<Attribute>,
    ) -> Stmt
    where
        NId: ToString,
        NL: ToString,
    {
        let id = id.to_string();
        let label = format!("\"{} {}\"", id, nl.to_string());
        let mut attrs = attrs;
        attrs.push(NodeAttributes::label(label));
        stmt!(node!(id.as_str(), attrs))
    }
    pub fn edge_with_attrs<'a, NId, EL>(
        &self,
        from: &'a NId,
        to: &'a NId,
        el: &'a EL,
        attrs: Vec<Attribute>,
    ) -> Stmt
    where
        NId: ToString,
        EL: ToString,
    {
        let from = format!("{}", from.to_string());
        let to = format!("{}", to.to_string());
        let label = format!("{}", el.to_string());
        let mut attrs = attrs;
        attrs.push(EdgeAttributes::label(label));

        stmt!(edge!(node_id!(from.as_str()) => node_id!(to.as_str()), attrs))
    }
}

impl<'a, NId, NL, EL> Processor<'a, NId, NL, EL> for ToStringProcessor
where
    NId: ToString,
    NL: ToString,
    EL: ToString,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        self.node_with_attrs(id, nl, vec![])
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        self.edge_with_attrs(from, to, el, vec![])
    }
}
#[cfg(test)]
mod tests {
    use crate::graph::visualizer::{visualize, visualize_to_file, VizGraph};
    use crate::graph::DiGraph;
    use crate::graph::EmptyPayload;
    use crate::*;
    use graphviz_rust::dot_structures::Graph;

    #[test]
    fn simple_viz_to_file_test() {
        let dot = digraph!(
            => [1,2,3,4,5,6,7,8,9,10] => {
             1 => [2,3,4];
             [2,3,4] => 5;
             [2,3,4] => 6;
             5 => 6;
             6 => [7,8];
             [7,8] => 9;
             9 => 10
            }
        )
        .to_file("dots/output.svg");
        println!("{:?}", dot)
    }
    #[test]
    fn simple_viz_to_file_payload_edge_test() {
        let dot = digraph!(
           (_,_,i32) => [1,2,3,4,5,6,7,8,9,10] => {
             1 => [2,3,4];
             [2,3,4] => (5,100);
             [2,3,4] => (6,10);
             5 => (6,1);
             6 => [(7,14),(8,14)];
             [7,8] => 9;
             9 => 10
            }
        )
        .to_file("dots/output.svg");
        println!("{:?}", dot)
    }
    #[test]
    fn simple_viz_to_file_str_edge_test() {
        let dot = digraph!(
           (&str,_,_) => ["company","employer","employee"] => {
                "employer" => "company";
                "company" => "employee"
            }
        )
        .to_file("dots/output.svg");

        println!("{:?}", dot)
    }
}
