use super::min_weight::{MinWeight, Score};
use crate::graph::DiGraph;
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::convert::identity;
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

#[derive(Debug)]
pub struct MinPathStrict<NId>
where
    NId: Eq + Hash + Clone,
{
    path: HashMap<NId, NId>,
    start: NId,
    target: NId,
}

impl<NId> MinPathStrict<NId>
where
    NId: Eq + Hash + Clone,
{
    fn path(&self) -> Vec<NId> {
        if self.path.is_empty() {
            vec![]
        } else {
            let mut path = Vec::new();
            let mut step = Some(self.target.clone());

            while let Some(s) = step {
                path.push(s.clone());
                if s == self.start {
                    break;
                }
                step = self.path.get(&s).cloned();
            }

            path.reverse();
            path
        }
    }
}

#[derive(Debug)]
pub struct AStarPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

impl<'a, NId, NL, EL> AStarPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn on_edge_custom<H, E, ScoreV>(
        &self,
        start: NId,
        target: NId,
        heuristic: H,
        edge_w: E,
    ) -> MinPathStrict<NId>
    where
        H: Fn(&NId) -> ScoreV,
        E: Fn(EL) -> ScoreV,
        ScoreV: Ord + Add<Output = ScoreV> + Clone,
        EL: Clone,
    {
        let mut traverse: BinaryHeap<MinWeight<NId, ScoreV>> = BinaryHeap::new();
        let mut path: HashMap<NId, NId> = HashMap::new();
        let mut scores: HashMap<&NId, Score<ScoreV>> =
            HashMap::from_iter(self.graph.nodes.keys().map(|k| (k, Score::Inf)));
        let mut est_scores: HashMap<&NId, Score<ScoreV>> = HashMap::new();

        scores.insert(&start, Score::Zero);
        traverse.push(MinWeight(&start, Score::Value(heuristic(&start))));

        while let Some(MinWeight(current, curr_est_score)) = traverse.pop() {
            if current == &target {
                return MinPathStrict {
                    path,
                    start,
                    target,
                };
            }

            match est_scores.entry(current) {
                Occupied(mut entry) => {
                    // If the node has been visited with an equal or lower score, then skip.
                    if *entry.get() <= curr_est_score {
                        continue;
                    }
                    entry.insert(curr_est_score);
                }
                Vacant(entry) => {
                    entry.insert(curr_est_score);
                }
            }

            if let Some(ss) = self.graph.edges.get(current) {
                let current_score = scores.get(current).unwrap().clone();
                for (to, el) in ss {
                    let next_score = scores.get(to).unwrap().clone();
                    let tentative_score = current_score.clone() + Score::Value(edge_w(el.clone()));
                    if tentative_score < next_score {
                        path.insert(to.clone(), current.clone());
                        scores.insert(to, tentative_score.clone());
                        traverse.push(MinWeight(
                            to,
                            tentative_score + Score::Value(heuristic(&to)),
                        ))
                    }
                }
            }
        }

        MinPathStrict {
            path,
            start,
            target,
        }
    }
}

impl<'a, NId, NL, EL> AStarPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
    EL: Ord + Add<Output = EL> + Clone,
{

    pub fn on_edge<H>(
        &self,
        start: NId,
        target: NId,
        heuristic: H,
    ) -> MinPathStrict<NId>
    where
        H: Fn(&NId) -> EL,
    {
        self.on_edge_custom(start, target, heuristic, identity)
    }
}

impl<'a, NId, NL, EL> AStarPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    pub fn new(graph: &'a DiGraph<NId, NL, EL>) -> Self {
        Self { graph }
    }
}

#[cfg(test)]
mod tests {
    use super::AStarPath;
    use crate::graph::analyzer::dijkstra::DijkstraPath;
    use crate::graph::analyzer::dijkstra::MinPathProcessor;
    use crate::graph::DiGraph;
    use crate::graph::EmptyPayload;
    use crate::{digraph, extend_edges, extend_nodes};
    use std::convert::identity;

    #[test]
    fn simple_test() {
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

        let astar = AStarPath::new(&graph);

        let astar_res = astar.on_edge(1, 11, |from| 0).path();
        let dijkstra_res = DijkstraPath::new(&graph).on_edge(1).trail(&11).unwrap();

        assert_eq!(astar_res, dijkstra_res);
    }
}
