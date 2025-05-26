use crate::core::SplitResult;
use std::{fmt, ops::Deref};

#[derive(Debug, Clone)]
pub enum Predicate {
    Atom(String),
    Not(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    MaterialImplication(Box<Self>, Box<Self>),
}

impl Predicate {
    fn terminal_value(&self) -> Option<(bool, &str)> {
        match self {
            Self::Atom(name) => Some((true, name)),
            Self::Not(atom) => match atom.deref() {
                Self::Atom(name) => Some((false, name)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(name) => f.write_str(&name),
            Self::Not(p) => write!(f, "(¬{p})"),
            Self::And(x, y) => write!(f, "({x} ∧ {y})"),
            Self::Or(x, y) => write!(f, "({x} ∨ {y})"),
            Self::MaterialImplication(x, y) => write!(f, "({x} ⊃ {y})"),
        }
    }
}

impl crate::core::Predicate for Predicate {
    fn negated(self) -> Self {
        Self::Not(Box::new(self))
    }

    fn split(&self) -> SplitResult<Self> {
        match self {
            Self::MaterialImplication(a, b) => {
                SplitResult::Disjunction([Self::Not(a.clone()), *b.clone()])
            }
            Self::Not(p) => match p.deref() {
                Self::MaterialImplication(a, b) => {
                    SplitResult::Conjunction([*a.clone(), Self::Not(b.clone())])
                }
                Self::Atom(_) => SplitResult::Terminal,
                _ => todo!("TODO: {self}"),
            },
            Self::Atom(_) => SplitResult::Terminal,
            _ => todo!(),
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Not(p) if matches!(p.deref(), Self::Atom(_)))
            || matches!(self, Self::Atom(_))
    }

    fn contradicts<'a>(&self, mut ancestors: impl Iterator<Item = &'a Self>) -> Option<String>
    where
        Self: 'a,
    {
        let (boolean, name) = self.terminal_value().unwrap();
        ancestors.find_map(|other| {
            let (other_bool, other_name) = other.terminal_value()?;
            (other_name == name && other_bool != boolean).then(|| name.to_string())
        })
    }
}

#[macro_export]
macro_rules! p {
    (($x:tt then $y:tt)) => {
        Predicate::MaterialImplication(Box::new(p!($x)), Box::new(p!($y)))
    };

    (($atom:literal)) => {
        Predicate::Atom($atom.into())
    };

    ($($expr:tt)*) => {
        p!(($($expr)*))
    };
}

pub use p;

// #[cfg(test)]
// mod tests {
//     use crate::core::Tableux;

//     use super::Predicate;

//     fn material_implication_transitivity() {
//         let premises = [p!("A" then "B"), p!("B" then "C")];
//         let conclusion = p!("A" then "C");

//         // let a = Predicate::Atom("A".into());
//         // let b = Predicate::Atom("B".into());
//         // let c = Predicate::Atom("C".into());
//         // let premises = [
//         //     Predicate::MaterialImplication(Box::new(a), Box::new(b)),
//         //     Predicate::MaterialImplication(Box::new(b), Box::new(c)),
//         // ];

//         // let conclusion = Predicate::MaterialImplication(Box::new(a), Box::new(c));

//         let tableux = Tableux::new(premises, conclusion);
//         println!("{tableux}");
//         let tableux = tableux.infer();
//         println!("{tableux}")
//     }
// }
