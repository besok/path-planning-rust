use graphviz_rust::dot_generator::{graph, id, stmt, node, edge, node_id};
use graphviz_rust::dot_structures::*;
use graphviz_rust::{exec, exec_dot};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use crate::graph::{DiGraph, EmptyPayload, NId};

pub trait ToDotNode {
    fn stmt(&self, id: &NId) -> Stmt;
}

pub trait ToDotEdge {
    fn stmt(&self, from: &NId, to: &NId) -> Stmt;
}

impl<NL: ToDotNode, EL: ToDotEdge> From<&DiGraph<NL, EL>> for Graph {
    fn from(g: &DiGraph<NL, EL>) -> Self {
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


pub fn visualize<NL: ToDotNode, EL: ToDotEdge>(graph: &DiGraph<NL, EL>) -> String {
    let dot_graph: Graph = graph.into();
    dot_graph.print(&mut PrinterContext::default())
}

pub fn visualize_to_file<NL: ToDotNode, EL: ToDotEdge>(graph: &DiGraph<NL, EL>, path: String) -> std::io::Result<String> {
    let dot_graph: Graph = graph.into();
    exec(dot_graph, &mut PrinterContext::default(), vec![
        CommandArg::Output(path),
        CommandArg::Format(Format::Svg),
    ])
}

impl ToDotNode for EmptyPayload {
    fn stmt(&self, n_id: &NId) -> Stmt {
        let label = format!("{}", n_id);
        stmt!(node!(label.as_str()))
    }
}

impl ToDotEdge for EmptyPayload {
    fn stmt(&self, from: &NId, to: &NId) -> Stmt {
        let from = format!("{}", from);
        let to = format!("{}", to);
        stmt!(edge!(node_id!(from.as_str()) => node_id!(to.as_str())))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::graph::DiGraph;
    use crate::graph::viz::{visualize, visualize_to_file};


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
        let dot = visualize_to_file(&digraph!(
            => [1,2,3,4,5,6,7,8,9,10] => {
             1 => [2,3,4];
             [2,3,4] => 5;
             [2,3,4] => 6;
             5 => 6;
             6 => [7,8];
             [7,8] => 9;
             9 => 10
            }
        ), "dots/output.svg".to_string());
        println!("{:?}", dot)
    }
}