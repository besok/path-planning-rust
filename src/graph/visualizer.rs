pub mod dot;
pub mod kiss;
pub mod generator2d;


use crate::graph::{DiGraph, EmptyPayload};
use graphviz_rust::attributes::{EdgeAttributes, NodeAttributes};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use graphviz_rust::{exec, exec_dot};
use std::hash::Hash;

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
