use crate::graph::min_weight::{MinWeight, Score};
use crate::graph::visualizer::{Processor, ToStringProcessor};
use crate::graph::DiGraph;
use graphviz_rust::attributes::*;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::Stmt;
use graphviz_rust::dot_structures::*;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::identity;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, Index};
#[derive(Debug)]
pub struct DijkstraPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> DijkstraPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
    EL: Ord + Add<Output = EL> + Clone,
{
    pub fn on_edge(&mut self, start: NId) -> MinPathMap<NId, EL> {
        self.on_edge_custom(start, identity)
    }
}

impl<'a, NId, NL, EL> DijkstraPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
    EL: Clone,
{
    pub fn on_edge_custom<ScoreV, F>(&mut self, start: NId, to_score: F) -> MinPathMap<NId, ScoreV>
    where
        F: Fn(EL) -> ScoreV,
        ScoreV: Ord + Add<Output = ScoreV> + Clone,
    {
        let mut dist = HashMap::from([(start.clone(), Score::Zero)]);
        let mut path = HashMap::new();
        let mut queue = BinaryHeap::new();

        for (id, _) in &self.graph.nodes {
            if id.ne(&start) {
                dist.insert(id.clone(), Score::Inf);
            }
            queue.push(MinWeight(&start, dist[&id].clone()))
        }

        while let Some(MinWeight(from, _)) = queue.pop() {
            if let Some(ss) = self.graph.edges.get(from) {
                let dist_from = dist[from].clone();
                for (to, ep) in ss {
                    let alt = dist_from.add(to_score(ep.clone()));
                    let dist_to = dist[to].clone();
                    if alt < dist_to {
                        dist.insert(to.clone(), alt.clone());
                        path.insert(to.clone(), from.clone());
                        queue.push(MinWeight(to, alt.clone()))
                    }
                }
            }
        }
        MinPathMap::new(start, dist, path)
    }
}

impl<'a, NId, NL, EL> DijkstraPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph }
    }
}

#[derive(Debug)]
pub struct MinPathMap<NId, ScoreV>
where
    NId: Eq + Hash + Clone,
    ScoreV: Clone,
{
    from: NId,
    distance: HashMap<NId, Score<ScoreV>>,
    path: HashMap<NId, NId>,
}

impl<NId, ScoreV> MinPathMap<NId, ScoreV>
where
    NId: Eq + Hash + Clone,
    ScoreV: Clone,
{
    pub fn new(from: NId, distance: HashMap<NId, Score<ScoreV>>, path: HashMap<NId, NId>) -> Self {
        Self {
            from,
            distance,
            path,
        }
    }

    pub fn score(&self, to: &NId) -> Score<ScoreV> {
        self.distance[to].clone()
    }
    pub fn trail(&self, to: &NId) -> Option<Vec<NId>> {
        let mut rhs = to;
        let mut trail = vec![];
        while let Some(start) = self.path.get(rhs) {
            trail.push(rhs.clone());
            rhs = start;
            if rhs.eq(&self.from) {
                trail.push(rhs.clone());
                trail.reverse();
                return Some(trail);
            }
        }
        None
    }
}

struct MinScorePathProcessor<NId, ScoreV>
where
    NId: Eq + Hash + Clone,
    ScoreV: Clone,
{
    from: NId,
    distance: HashMap<NId, Score<ScoreV>>,
    delegate: ToStringProcessor,
}

impl<NId, ScoreV> MinScorePathProcessor<NId, ScoreV>
where
    NId: Eq + Hash + Clone,
    ScoreV: Clone,
{
    pub fn new(from: NId, distance: HashMap<NId, Score<ScoreV>>) -> Self {
        Self {
            from,
            distance,
            delegate: ToStringProcessor {},
        }
    }
}

impl<'a, NId, NL, EL, ScoreV> Processor<'a, NId, NL, EL> for MinScorePathProcessor<NId, ScoreV>
where
    NId: Eq + Hash + Clone + ToString,
    NL: ToString,
    EL: ToString,
    ScoreV: ToString + Clone,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        if let Some(score) = self.distance.get(id) {
            let mut attrs = vec![NodeAttributes::xlabel(score.to_string())];
            if &self.from == id {
                attrs.push(NodeAttributes::color(color_name::red));
            }
            self.delegate.node_with_attrs(id, nl, attrs)
        } else {
            self.delegate.node_with_attrs(id, nl, vec![])
        }
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        (&self.delegate as &dyn Processor<NId, NL, EL>).edge(from, to, el)
    }
}

struct MinPathProcessor<NId>
where
    NId: Clone,
{
    path: Vec<NId>,
    delegate: ToStringProcessor,
}

impl<NId> MinPathProcessor<NId>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(path: Vec<NId>) -> Self {
        Self {
            path,
            delegate: ToStringProcessor {},
        }
    }
}

impl<'a, NId, NL, EL> Processor<'a, NId, NL, EL> for MinPathProcessor<NId>
where
    NId: Eq + Hash + Clone + ToString,
    NL: ToString,
    EL: ToString,
{
    fn node(&self, id: &'a NId, nl: &'a NL) -> Stmt {
        if self.path.is_empty() {
            (&self.delegate as &dyn Processor<NId, NL, EL>).node(id, nl)
        } else {
            let f = self.path.get(0).unwrap();
            let l = self.path.last().unwrap();
            if f == id || l == id {
                self.delegate.node_with_attrs(
                    id,
                    nl,
                    vec![
                        NodeAttributes::color(color_name::green),
                        NodeAttributes::style("bold".to_string()),
                    ],
                )
            } else if self.path.contains(id) {
                self.delegate.node_with_attrs(
                    id,
                    nl,
                    vec![NodeAttributes::color(color_name::green)],
                )
            } else {
                (&self.delegate as &dyn Processor<NId, NL, EL>).node(id, nl)
            }
        }
    }

    fn edge(&self, from: &'a NId, to: &'a NId, el: &'a EL) -> Stmt {
        let mut f = None;
        let mut t = None;

        for (idx, id) in self.path.iter().enumerate() {
            if id == from {
                f = Some(idx)
            };
            if id == to {
                t = Some(idx)
            };
        }

        match (f, t) {
            (Some(f), Some(t)) if f < t => {
                (&self.delegate as &dyn Processor<NId, NL, EL>).edge(from, to, el)
            }
            e => self.delegate.edge_with_attrs(
                from,
                to,
                el,
                vec![EdgeAttributes::style("dotted".to_string())],
            ),
        }
    }
}

// pub struct UniformCostSearch<T: Ord> {}

#[cfg(test)]
mod tests {
    use crate::graph::dijkstra::{
        DijkstraPath, MinPathProcessor, MinScorePathProcessor, MinWeight,
    };
    use crate::graph::min_weight::Score;
    use crate::graph::min_weight::Score::*;
    use crate::graph::DiGraph;
    use crate::graph::EmptyPayload;
    use crate::{digraph, extend_edges, extend_nodes};
    use std::collections::BinaryHeap;

    #[test]
    fn simple_test() {
        let mut q = BinaryHeap::new();
        q.push(MinWeight(&EmptyPayload, Zero));
        q.push(MinWeight(&EmptyPayload, Inf));
        q.push(MinWeight(&EmptyPayload, Value(1)));
        q.push(MinWeight(&EmptyPayload, Value(5)));
        q.push(MinWeight(&EmptyPayload, Value(3)));
        q.push(MinWeight(&EmptyPayload, Zero));

        assert_eq!(q.pop(), Some(MinWeight(&EmptyPayload, Zero)));
        assert_eq!(q.pop(), Some(MinWeight(&EmptyPayload, Zero)));
        assert_eq!(q.pop(), Some(MinWeight(&EmptyPayload, Value(1))));
        assert_eq!(q.pop(), Some(MinWeight(&EmptyPayload, Value(3))));
        assert_eq!(q.pop(), Some(MinWeight(&EmptyPayload, Value(5))));
        assert_eq!(q.pop(), Some(MinWeight(&EmptyPayload, Inf)));
        assert_eq!(q.pop(), None);
    }
    #[test]
    fn simple_dijkstra_test() {
        let graph = digraph!((usize,_,usize) => [1,2,3,4,5,6,7,8,9,10,11,] => {
           1 => [(2,1),(3,1)];
           2 => (4,2);
           3 => (5,3);
           [4,5] => (6,1);
           6 => (7,1);
           7 => [(8,1),(9,2),(10,3)];
           [8,9,10] => (11,1)

        });
        let mut d = DijkstraPath::new(&graph);
        let d = d.on_edge(1).score(&11);
        println!("{:?}", d)
    }
    #[test]
    fn cycled_dijkstra_test() {
        let graph = digraph!((_,_,usize) => [1,2,3,4,5,6,7,8,9,10,11,] => {
           1 => [(2,1),(3,1)];
           2 => (4,2);
           3 => (5,3);
           [4,5] => (6,1);
           5 => (11,4);
           6 => [(7,1),(1,1)];
           7 => [(8,1),(9,2),(10,3)];
           [8,9,10] => (11,1)

        });
        let _ = graph.to_file("dots/output.svg");

        let mut d = DijkstraPath::new(&graph);
        let to = d.on_edge(1).score(&11);
        assert_eq!(to, Value(7));

        let mut d = DijkstraPath::new(&graph);
        let to = d.on_edge(8).score(&1);
        assert_eq!(to, Inf);

        let mut d = DijkstraPath::new(&graph);
        let to = d.on_edge(1).trail(&11);

        assert_eq!(to, Some(vec![1, 2, 4, 6, 7, 8, 11]));

        let mut d = DijkstraPath::new(&graph);
        let to = d.on_edge(8).trail(&1);
        assert_eq!(to, None);
    }

    #[test]
    fn viz_cycled_dijkstra_test() {
        let graph = digraph!((_,_,usize) => [1,2,3,4,5,6,7,8,9,10,11,] => {
           1 => [(2,1),(3,1)];
           2 => (4,2);
           3 => (5,3);
           [4,5] => (6,1);
           5 => (11,4);
           6 => [(7,1),(1,1)];
           7 => [(8,1),(9,2),(10,3)];
           [8,9,10] => (11,1)

        });

        let mut d = DijkstraPath::new(&graph);
        let to = d.on_edge(1).trail(&11).unwrap();
        assert_eq!(to, vec![1, 2, 4, 6, 7, 8, 11]);
        let r = graph.to_file_with("dots/output1.svg", MinPathProcessor::new(to));
        println!("{:?}", r);
    }
    #[test]
    fn viz_l_cycled_dijkstra_test() {
        let graph = digraph!((_,&str,usize) => [
            (1,"a"),
            (2,"b"),
            (3,"c"),
            (4,"d"),
            (5,"e"),
            (6,"f"),
            (7,"g"),
            (8,"h"),
            (9,"y"),
            (10,"u"),
            (11,"i"),

        ] => {
           1 => [(2,1),(3,1)];
           2 => (4,2);
           3 => (5,3);
           [4,5] => (6,1);
           5 => (11,4);
           6 => [(7,1),(1,1)];
           7 => [(8,1),(9,2),(10,3)];
           [8,9,10] => (11,1)

        });
        graph.to_file("dots/output.svg");
        let mut d = DijkstraPath::new(&graph);
        let map = d.on_edge(1);
        let to = map.trail(&11).unwrap();
        assert_eq!(to, vec![1, 2, 4, 6, 7, 8, 11]);
        let r = graph.to_file_with("dots/output_path.svg", MinPathProcessor::new(to));
        println!("{:?}", r);
        let r = graph.to_file_with(
            "dots/output_sc.svg",
            MinScorePathProcessor::new(map.from, map.distance),
        );
        println!("{:?}", r);
    }
}
