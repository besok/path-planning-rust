use graphviz_rust::{
    attributes::{color_name, NodeAttributes},
    dot_structures::Stmt,
};

use crate::graph::{
    analyzer::SearchRes,
    visualizer::dot::{DotProcessor, ToStringProcessor},
};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;

use super::visit::{Visited, VisitedSet};
use crate::graph::DiGraph;

struct DFS<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> DFS<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph }
    }
    pub fn search_by_eq(&self, start: &'a NId, target: &'a NId) -> Option<&'a NId> {
        self.search(start, |n| {
            if target == n {
                SearchRes::Find
            } else {
                SearchRes::Next
            }
        })
    }
    fn search<S>(&self, start: &'a NId, target: S) -> Option<&'a NId>
    where
        S: Fn(&'a NId) -> SearchRes,
    {
        let mut visited = VisitedSet::default();
        let mut q = vec![];

        q.push(start);

        while let Some(node) = q.pop() {
            visited.visit(node);
            match target(node) {
                SearchRes::Next => {
                    for nexts in self.graph.descendants(node.clone()) {
                        for s in nexts.keys() {
                            if !visited.already_visited(s) {
                                q.push(s)
                            }
                        }
                    }
                }
                SearchRes::Skip => (),
                SearchRes::Find => return Some(node),
                SearchRes::Stop => return None,
            }
        }

        None
    }
}

struct BFS<'a, NId, NL, EL>
where
    NId: Eq + Hash,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> BFS<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        BFS { graph }
    }

    pub fn search_by_eq(&self, start: &'a NId, target: &'a NId) -> Option<&'a NId> {
        self.search(start, |n| {
            if target == n {
                SearchRes::Find
            } else {
                SearchRes::Next
            }
        })
    }
    fn search<S>(&self, start: &'a NId, target: S) -> Option<&'a NId>
    where
        S: Fn(&'a NId) -> SearchRes,
    {
        let mut visited = VisitedSet::default();
        let mut q = VecDeque::new();

        q.push_back(start);

        while let Some(node) = q.pop_front() {
            visited.visit(node);
            match target(node) {
                SearchRes::Next => {
                    for nexts in self.graph.descendants(node.clone()) {
                        for s in nexts.keys() {
                            if !visited.already_visited(s) {
                                q.push_back(s)
                            }
                        }
                    }
                }
                SearchRes::Skip => (),
                SearchRes::Find => return Some(node),
                SearchRes::Stop => return None,
            }
        }

        None
    }
}

struct SrcTrgHighlighter<'a, NId>
where
    NId: Eq + Hash + Clone + ToString,
{
    src: &'a NId,
    trg: &'a NId,
    delegate: ToStringProcessor,
}

impl<'a, NId> SrcTrgHighlighter<'a, NId>
where
    NId: Eq + Hash + Clone + ToString,
{
    fn new(src: &'a NId, trg: &'a NId) -> Self {
        Self {
            src,
            trg,
            delegate: ToStringProcessor {},
        }
    }
}

impl<'a, NId, NL, EL> DotProcessor<'a, NId, NL, EL> for SrcTrgHighlighter<'a, NId>
where
    NId: Eq + Hash + Clone + ToString,
    EL: ToString,
    NL: ToString,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        if id != self.src && id != self.trg {
            (&self.delegate as &dyn DotProcessor<NId, NL, EL>).node(id, nl)
        } else {
            let green = NodeAttributes::color(color_name::green);
            let bold = NodeAttributes::style("bold".to_string());
            self.delegate.node_with_attrs(id, nl, vec![green, bold])
        }
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        (&self.delegate as &dyn DotProcessor<NId, NL, EL>).edge(from, to, el)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::analyzer::fs::{SearchRes, SrcTrgHighlighter, BFS, DFS};
    use crate::graph::DiGraph;
    use crate::graph::EmptyPayload;
    use crate::{digraph, extend_edges, extend_nodes};

    #[test]
    fn bfs_simple_test() {
        let graph = digraph!((usize,_,_) => [1,2,3,4,5,6,7,8,9,10] => {
            1 => 2;
            2 => [3,4,5];
            3 => 6;
            6 => [1,7,8];
            8 => 9;
            9 => [1,10];
        });

        let res = graph.visualize().str_to_dot_file("dots/bfs.svg");
        assert!(res.is_ok());

        let bfs = BFS::new(&graph);

        let res = bfs.search_by_eq(&1, &10);
        assert_eq!(res, Some(&10))
    }

    #[test]
    fn simple_test2() {
        let graph = digraph!((usize,_,_) => [1,2,3,4,5,6,7,8,9,10] => {
            1 => 2;
            2 => [3,4,5];
            3 => 6;
            6 => [1,7,8];
            8 => 9;
            9 => [1,10];
        });

        let res = graph.visualize().str_to_dot_file("dots/bfs.svg");
        assert!(res.is_ok());

        let dfs = DFS::new(&graph);

        let res = dfs.search_by_eq(&1, &10);
        assert_eq!(res, Some(&10))
    }
    #[test]
    fn cap_test() {
        let graph = digraph!((usize,_,_) => [1,2,3,4,5,6,7,8,9,10] => {
            1 => 2;
            2 => [3,4,5];
            3 => 6;
            6 => [1,7,8];
            8 => 9;
            9 => [1,10];
        });

        let res = graph.visualize().str_to_dot_file("dots/bfs.svg");
        assert!(res.is_ok());

        let dfs = DFS::new(&graph);
        let bfs = BFS::new(&graph);

        let res = dfs.search(&1, move |n| {
            if &10 == n {
                SearchRes::Find
            } else {
                println!("n = {}", n);
                SearchRes::Next
            }
        });
        assert_eq!(res, Some(&10));

        println!("bfs");

        let res = bfs.search(&1, move |n| {
            if &10 == n {
                SearchRes::Find
            } else {
                println!("n = {}", n);
                SearchRes::Next
            }
        });
        assert_eq!(res, Some(&10));
        graph
            .visualize()
            .to_dot_file("dots/test.svg", SrcTrgHighlighter::new(&1, &10));
    }
}
