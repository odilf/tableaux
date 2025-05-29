use crate::tableau::Branch;

pub trait Logic {
    type Node: Clone;
    type Expr;

    fn infer(branch: Branch<Self>) -> InferenceRule<Self::Node>
    where
        Self: Sized;

    // TODO: Document that it is guaranteed that the ancestors won't contain a contradiction.
    fn has_contradiction(branch: Branch<Self>) -> bool
    where
        Self: Sized;

    fn make_premise_node(expr: Self::Expr) -> Self::Node;
    fn make_conclusion_node(expr: Self::Expr) -> Self::Node;
}

pub enum InferenceRule<E> {
    None,
    Single(E),
    Split([E; 2]),
    Chain(Vec<E>),
    SplitAndChain([[E; 2]; 2]),
}

impl<E> InferenceRule<E> {
    pub const fn none() -> Self {
        Self::None
    }

    pub const fn single(v: E) -> Self {
        Self::Single(v)
    }

    pub const fn split(left: E, right: E) -> Self {
        Self::Split([left, right])
    }

    pub const fn chain(exprs: Vec<E>) -> Self {
        Self::Chain(exprs)
    }

    pub fn split_and_chain(left_chain: [E; 2], right_chain: [E; 2]) -> Self {
        Self::SplitAndChain([left_chain, right_chain])
    }

    pub fn map<F>(self, f: impl Fn(E) -> F) -> InferenceRule<F> {
        match self {
            Self::None => InferenceRule::None,
            Self::Single(a) => InferenceRule::Single(f(a)),
            Self::Split([a, b]) => InferenceRule::Split([f(a), f(b)]),
            Self::Chain(exprs) => {
                InferenceRule::Chain(exprs.into_iter().map(|expr| f(expr)).collect())
            }
            Self::SplitAndChain([[la, lb], [ra, rb]]) => {
                InferenceRule::SplitAndChain([[f(la), f(lb)], [f(ra), f(rb)]])
            }
        }
    }
}

// Re-exports
pub mod classical {
    use std::str::FromStr;

    pub use crate::classical::Classical;
    use crate::{PartialTableau, tableau::Tableau};

    pub fn infer(statement: &str) -> Tableau<Classical> {
        PartialTableau::from_str(statement).unwrap().infer()
    }
}

pub mod modal {
    use std::str::FromStr;

    pub use crate::modal::Modal;
    use crate::{PartialTableau, tableau::Tableau};

    pub fn infer(statement: &str) -> Tableau<Modal> {
        PartialTableau::from_str(statement).unwrap().infer()
    }
}
