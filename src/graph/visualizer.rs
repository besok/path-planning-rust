pub mod dot;
pub mod generator2d;

use crate::graph::{DiGraph, EmptyPayload};
use graphviz_rust::attributes::{EdgeAttributes, NodeAttributes};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use graphviz_rust::{exec, exec_dot};
use std::hash::Hash;

use self::dot::{DotProcessor, ToStringProcessor};
pub struct DotGraphVisualizer<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> DotGraphVisualizer<'a, NId, NL, EL>
where
    NId: Eq + Hash + ToString,
    NL: ToString,
    EL: ToString,
{
    pub fn str_to_dot_file(&self, path: &str) -> std::io::Result<String> {
        self.to_dot_file(path, ToStringProcessor {})
    }
}

impl<'a, NId, NL, EL> DotGraphVisualizer<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph }
    }
    pub fn to_dot<P>(&self, processor: P) -> Graph
    where
        P: DotProcessor<'a, NId, NL, EL>,
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

    pub fn to_dot_file<P>(&'a self, path: &str, processor: P) -> std::io::Result<String>
    where
        P: DotProcessor<'a, NId, NL, EL>,
    {
        vis_to_file(self.to_dot(processor), path.to_string())
    }
}

pub fn vis(dot_graph: Graph) -> String {
    dot_graph.print(&mut PrinterContext::default())
}

pub fn vis_to_file(dot_graph: Graph, path: String) -> std::io::Result<String> {
    let ext = path
        .split(".")
        .last()
        .map(|x| x.to_lowercase())
        .unwrap_or("svg".to_string());
    let format = match ext.as_str() {
        "svg" => Format::Svg,
        "dot" => Format::Dot,
        _ => panic!("dot or svg"),
    };
    exec(
        dot_graph,
        &mut PrinterContext::default(),
        vec![CommandArg::Output(path), CommandArg::Format(format)],
    )
}
