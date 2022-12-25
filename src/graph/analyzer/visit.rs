use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Visited<'a, T> {
    fn visit(&mut self, v: &'a T) -> bool;
    fn already_visited(&self, v: &'a T) -> bool;
}

#[derive(Debug)]
pub struct VisitedSet<'a, T>
where
    T: Hash + Eq,
{
    visited: HashSet<&'a T>,
}

impl<'a, T> Default for VisitedSet<'a, T>
where
    T: Hash + Eq,
{
    fn default() -> Self {
        Self { visited: Default::default() }
    }
}


impl<'a, T> Visited<'a, T> for VisitedSet<'a, T>
where
    T: Hash + Eq,
{
    fn visit(&mut self, v: &'a T) -> bool {
        self.visited.insert(v)
    }
    fn already_visited(&self, v: &'a T) -> bool {
        self.visited.contains(v)
    }
}
