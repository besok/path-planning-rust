use crate::graph::{DiGraph};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use MinWeight::{Inf, Some, Zero};

type Score = usize;

#[derive(Debug)]
enum MinWeight<E> {
    Inf,
    Zero,
    Some(Score, E),
}

impl<E> Eq for MinWeight<E> {}

impl<E> PartialEq<Self> for MinWeight<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Inf, Inf) | (Zero, Zero) => true,
            (Some(l, _), Some(r, _)) => l == r,
            _ => false,
        }
    }
}

impl<E> PartialOrd<Self> for MinWeight<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))
    }
}

impl<E> Ord for MinWeight<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            match (self, other) {
                (Inf, _) | (_, Zero) => Ordering::Less,
                (Zero, _) | (_, Inf) => Ordering::Greater,
                (Some(lhs, _), Some(rhs, _)) => {
                    if lhs < rhs {
                        Ordering::Greater
                    } else if lhs > rhs {
                        Ordering::Less
                    } else if lhs.ne(lhs) && rhs.ne(rhs) {
                        Ordering::Equal
                    } else if lhs.ne(lhs) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
        }
    }
}

pub struct Dijkstra<'a,NId,NL, EL>
where NId: Eq+ Hash
{
    graph: &'a DiGraph<NId,NL, EL>,
}

impl<'a,NId, NL, EL> Dijkstra<'a,NId, NL, EL>
    where NId: Eq+ Hash
{
    pub fn new(graph: &'a DiGraph<NId,NL, EL>) -> Self {
        Self { graph }
    }

    pub fn calculate<F>(start: NId, score: F)
    where
        F: Fn(NL, NL, EL) -> Score,
    {

    }
}

// pub struct UniformCostSearch<T: Ord> {}

#[cfg(test)]
mod tests {
    use crate::graph::dijkstra::MinWeight;
    use crate::graph::EmptyPayload;
    use std::collections::BinaryHeap;

    #[test]
    fn simple_test() {
        let zero = MinWeight::Zero;
        let inf = MinWeight::Inf;
        let one = MinWeight::Some(1, EmptyPayload);
        let two = MinWeight::Some(2, EmptyPayload);
        let three = MinWeight::Some(3, EmptyPayload);

        let mut q = BinaryHeap::new();
        q.push(one);
        q.push(two);
        q.push(zero);
        q.push(three);
        q.push(inf);

        assert_eq!(q.pop(), Some(MinWeight::Zero));
        assert_eq!(q.pop(), Some(MinWeight::Some(1, EmptyPayload)));
        assert_eq!(q.pop(), Some(MinWeight::Some(2, EmptyPayload)));
        assert_eq!(q.pop(), Some(MinWeight::Some(3, EmptyPayload)));
        assert_eq!(q.pop(), Some(MinWeight::Inf));
        assert_eq!(q.pop(), None);
    }
}
