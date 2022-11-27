use crate::graph::min_weight::{MinWeight, Score};
use crate::graph::DiGraph;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::identity;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug)]
pub struct DijkstraPath<'a, NId, NL, EL, ScoreV>
where
    NId: Eq + Hash + Clone,
    ScoreV: Ord + Add<Output = ScoreV> + Clone,
{
    graph: &'a DiGraph<NId, NL, EL>,
    from: NId,
    distance: HashMap<NId, Score<ScoreV>>,
    path: HashMap<NId, NId>,
}

#[derive(Debug)]
pub struct DijkstraPathCalculated<'a, NId, NL, EL, ScoreV>
    where
        NId: Eq + Hash + Clone,
        ScoreV: Ord + Add<Output = ScoreV> + Clone,
{
    graph: &'a DiGraph<NId, NL, EL>,
    from: NId,
    distance: HashMap<NId, Score<ScoreV>>,
    path: HashMap<NId, NId>,
}

impl<'a, NId, NL, EL> DijkstraPath<'a, NId, NL, EL, EL>
where
    NId: Eq + Hash + Clone,
    EL: Ord + Add<Output = EL> + Clone,
{
    pub fn on_edge(&mut self) -> DijkstraPath<'a, NId, NL, EL, EL> {
        self.on_edge_custom(identity)
    }
}

impl<'a, NId, NL, EL, ScoreV> DijkstraPath<'a, NId, NL, EL, ScoreV>
where
    NId: Eq + Hash + Clone,
    EL: Clone,
    ScoreV: Ord + Add<Output = ScoreV> + Clone,
{
    pub fn on_edge_custom<F>(&mut self, to_score: F) -> DijkstraPath<'a, NId, NL, EL, ScoreV>
    where
        F: Fn(EL) -> ScoreV,
    {
        let start = self.from.clone();
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
        DijkstraPath::new_after(self.graph, self.from.clone(), dist, path)
    }
}

impl<'a, NId, NL, EL, ScoreV> DijkstraPath<'a, NId, NL, EL, ScoreV>
where
    NId: Eq + Hash + Clone,
    ScoreV: Ord + Add<Output = ScoreV> + Clone,
{
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

    pub fn new(graph: &'a DiGraph<NId, NL, EL>, from: NId) -> Self {
        Self {
            graph,
            from,
            distance: Default::default(),
            path: Default::default(),
        }
    }
    fn new_after(
        graph: &'a DiGraph<NId, NL, EL>,
        from: NId,
        distance: HashMap<NId, Score<ScoreV>>,
        path: HashMap<NId, NId>,
    ) -> Self {
        Self {
            graph,
            from,
            distance,
            path,
        }
    }
}

// pub struct UniformCostSearch<T: Ord> {}

#[cfg(test)]
mod tests {
    use crate::graph::dijkstra::{DijkstraPath, MinWeight};
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
        let mut d = DijkstraPath::new(&graph, 1);
        let d = d.on_edge().score(&11);
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
        let _ = graph.to_file("dots/output.svg".to_string());

        let mut d = DijkstraPath::new(&graph, 1);
        let to = d.on_edge().score(&11);
        assert_eq!(to, Value(7));

        let mut d = DijkstraPath::new(&graph, 8);
        let to = d.on_edge().score(&1);
        assert_eq!(to, Inf);

        let mut d = DijkstraPath::new(&graph, 1);
        let to = d.on_edge().trail(&11);

        assert_eq!(to, Some(vec![1, 2, 4, 6, 7, 8, 11]));

        let mut d = DijkstraPath::new(&graph, 8);
        let to = d.on_edge().trail(&1);
        assert_eq!(to, None);
    }
}
