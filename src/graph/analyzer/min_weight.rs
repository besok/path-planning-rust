use std::cmp::Ordering;
use std::ops::Add;
use Score::{Inf, Value, Zero};

#[derive(Debug, Clone)]
pub enum Score<ScoreValue> {
    Inf,
    Zero,
    Value(ScoreValue),
}

impl<ScoreValue> Add for Score<ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Clone,
{
    type Output = Score<ScoreValue>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Inf,_) | (_,Inf) => Inf,
            (Zero,res)|(res,Zero) => res,
            (Value(lhs),Value(rhs)) => Value(lhs + rhs),
        }
    }

}

impl<ScoreValue:ToString> ToString for Score<ScoreValue> {
    fn to_string(&self) -> String {
        match self{
            Inf => "inf".to_string(),
            Zero => "zero".to_string(),
            Value(v) => v.to_string(),
        }
    }
}





impl<ScoreValue> Score<ScoreValue>
where
    ScoreValue: Add<Output = ScoreValue> + Clone,
{
    pub fn add_score_v(&self, score: ScoreValue) -> Score<ScoreValue> {
        match &self {
            Inf => Inf,
            Zero => Value(score),
            Value(left) => Value(left.clone() + score),
        }
    }
}

impl<'a, ScoreValue: Eq> Eq for Score<ScoreValue> {}
impl<'a, ScoreValue: Eq> PartialEq for Score<ScoreValue> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Inf, Inf) | (Zero, Zero) => true,
            (Value(lhs), Value(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}
impl<ScoreValue: Ord> PartialOrd for Score<ScoreValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match (self, other) {
                (Inf, _) => Some(Ordering::Greater),
                (Value(_), Inf) => Some(Ordering::Less),
                (Zero, _) => Some(Ordering::Less),
                (Value(lhs), Value(rhs)) => Some(lhs.cmp(rhs)),
                _ => None,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MinWeight<'a, Id, ScoreValue>(pub &'a Id, pub Score<ScoreValue>);

impl<'a, Id, ScoreValue: Ord> Eq for MinWeight<'a, Id, ScoreValue> {}
impl<'a, Id, ScoreValue: Ord> PartialEq<Self> for MinWeight<'a, Id, ScoreValue> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.1, &other.1) {
            (Inf, Inf) | (Zero, Zero) => true,
            (Value(l), Value(r)) => l.cmp(r) == Ordering::Equal,
            _ => false,
        }
    }
}

impl<'a, Id, ScoreValue: Ord> PartialOrd<Self> for MinWeight<'a, Id, ScoreValue> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, Id, ScoreValue: Ord> Ord for MinWeight<'a, Id, ScoreValue> {
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
