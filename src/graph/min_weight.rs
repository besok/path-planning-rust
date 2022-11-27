use std::cmp::Ordering;
use std::ops::Add;
use Score::{Inf, Value, Zero};

#[derive(Debug, Clone)]
pub struct MinWeight<'a, Id, ScoreValue>(pub &'a Id, pub Score<ScoreValue>)
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone;

#[derive(Debug, Clone)]
pub enum Score<ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,
{
    Inf,
    Zero,
    Value(ScoreValue),
}

impl<ScoreValue> Score<ScoreValue> where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,
{
    pub fn add(&self, score: ScoreValue) -> Score<ScoreValue> {
        match &self {
            Inf => Inf,
            Zero => Value(score),
            Value(left) => Value(left.clone() + score),
        }
    }

}


impl<'a, Id, ScoreValue> Eq for MinWeight<'a, Id, ScoreValue>
    where
        ScoreValue: Add<Output = ScoreValue> + Ord + Clone {}

impl<'a, Id, ScoreValue> PartialEq<Self> for MinWeight<'a, Id, ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        match (&self.1, &other.1) {
            (Inf, Inf) | (Zero, Zero) => true,
            (Value(l), Value(r)) => l.cmp(r) == Ordering::Equal,
            _ => false,
        }
    }
}

impl<'a, Id, ScoreValue> PartialOrd<Self> for MinWeight<'a, Id, ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, Id, ScoreValue> Ord for MinWeight<'a, Id, ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            match (&self.1, &other.1) {
                (Inf, _) | (_, Zero) => Ordering::Less,
                (Zero, _) | (_, Inf) => Ordering::Greater,
                (Value(lhs), Value(rhs)) => {
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

impl<'a, ScoreValue> Eq for Score<ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,{}
impl<'a, ScoreValue> PartialEq for Score<ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,{
    fn eq(&self, other: &Self) -> bool {
        match (self,other){
            (Inf,Inf) | (Zero, Zero) => true,
            (Value(lhs),Value(rhs)) => lhs.eq(rhs),
            _ => false
        }
    }
}
impl<'a, ScoreValue> PartialOrd for Score<ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Ord + Clone,{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other{ Some(Ordering::Equal)}
        else {
            match (self,other) {
                (Inf,_) => Some(Ordering::Greater),
                (Value(_),Inf) => Some(Ordering::Less),
                (Zero,_) => Some(Ordering::Less),
                (Value(lhs),Value(rhs)) => Some(lhs.cmp(rhs)),
                _ => None
            }
        }
    }
}
