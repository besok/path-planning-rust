use crate::graph::{DiGraph, EmptyPayload};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::{attr, edge, graph, id, node, node_id, stmt};
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use graphviz_rust::{exec, exec_dot};
use std::hash::Hash;

pub trait ToDotNode<NId> {
    fn stmt(&self, id: &NId) -> Stmt;
}

pub trait ToDotEdge<NId> {
    fn stmt(&self, from: &NId, to: &NId) -> Stmt;
}

impl<NId, NL, EL> From<&DiGraph<NId, NL, EL>> for Graph
where
    NId: Eq + Hash,
    NL: ToDotNode<NId>,
    EL: ToDotEdge<NId>,
{
    fn from(g: &DiGraph<NId, NL, EL>) -> Self {
        let mut dot = graph!(strict di id!("di_graph"));
        for (id, pl) in g.nodes.iter() {
            dot.add_stmt(pl.stmt(id));
        }
        for (from, to_map) in g.edges.iter() {
            for (to, pl) in to_map.iter() {
                dot.add_stmt(pl.stmt(from, to))
            }
        }
        dot
    }
}

pub fn visualize<NId: Eq + Hash, NL: ToDotNode<NId>, EL: ToDotEdge<NId>>(
    graph: &DiGraph<NId, NL, EL>,
) -> String {
    let dot_graph: Graph = graph.into();
    dot_graph.print(&mut PrinterContext::default())
}

pub fn visualize_to_file<NId: Eq + Hash, NL: ToDotNode<NId>, EL: ToDotEdge<NId>>(
    graph: &DiGraph<NId, NL, EL>,
    path: String,
) -> std::io::Result<String> {
    let dot_graph: Graph = graph.into();
    exec(
        dot_graph,
        &mut PrinterContext::default(),
        vec![CommandArg::Output(path), CommandArg::Format(Format::Svg)],
    )
}

impl<NId: ToString> ToDotNode<NId> for EmptyPayload {
    fn stmt(&self, n_id: &NId) -> Stmt {
        let label = format!("{}", n_id.to_string());
        stmt!(node!(label.as_str()))
    }
}

impl<NId: ToString> ToDotEdge<NId> for EmptyPayload {
    fn stmt(&self, from: &NId, to: &NId) -> Stmt {
        let from = format!("{}", from.to_string());
        let to = format!("{}", to.to_string());
        stmt!(edge!(node_id!(from.as_str()) => node_id!(to.as_str())))
    }
}
impl<NId: ToString,T:ToString> ToDotEdge<NId> for T {
    fn stmt(&self, from: &NId, to: &NId) -> Stmt {
        let from = format!("{}", from.to_string());
        let to = format!("{}", to.to_string());
        let p = format!("{}", self.to_string());
        stmt!(edge!(node_id!(from.as_str()) => node_id!(to.as_str()); attr!("label",p.as_str())))
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::visualizer::{visualize, visualize_to_file};
    use crate::graph::DiGraph;
    use crate::graph::EmptyPayload;
    use crate::*;

    #[test]
    fn simple_viz_test() {
        let dot = visualize(&digraph!(
            => [1,2,3,4,5,6] => {
             1 => 2;
             2 => [3,4];
             [3,4] => 5;
             5 => 6;
             6 => 1;
            }
        ));
    }
    #[test]
    fn simple_viz_to_file_test() {
        let dot = visualize_to_file(
            &digraph!(
                => [1,2,3,4,5,6,7,8,9,10] => {
                 1 => [2,3,4];
                 [2,3,4] => 5;
                 [2,3,4] => 6;
                 5 => 6;
                 6 => [7,8];
                 [7,8] => 9;
                 9 => 10
                }
            ),
            "dots/output.svg".to_string(),
        );
        println!("{:?}", dot)
    }
    #[test]
    fn simple_viz_to_file_payload_edge_test() {
        let dot = visualize_to_file(
            &digraph!(
               (_,_,i32) => [1,2,3,4,5,6,7,8,9,10] => {
                 1 => [2,3,4];
                 [2,3,4] => (5,100);
                 [2,3,4] => (6,10);
                 5 => (6,1);
                 6 => [(7,14),(8,14)];
                 [7,8] => 9;
                 9 => 10
                }
            ),
            "dots/output.svg".to_string(),
        );
        println!("{:?}", dot)
    }
    #[test]
    fn simple_viz_to_file_str_edge_test() {
        let dot = visualize_to_file(
            &digraph!(
               (&str,_,_) => ["company","employer","employee"] => {
                    "employer" => "company";
                    "company" => "employee"
                }
            ),
            "dots/output.svg".to_string(),
        );
        println!("{:?}", dot)
    }
}
