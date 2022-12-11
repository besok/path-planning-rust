use crate::graph::DiGraph;
use std::hash::Hash;

#[derive(Debug)]
pub struct AStarPath<'a, NId, NL, EL>
where
    NId: Eq + Hash + Clone,
{
    graph: &'a DiGraph<NId, NL, EL>,
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_test() {
     
    }
}
