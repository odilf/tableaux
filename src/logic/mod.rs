mod impls;
pub use impls::*;

mod dynamic;

use crate::{PartialTableau, tableau::Branch};

pub use dynamic::*;

pub trait Logic {
    type Node: Clone;
    type Expr;

    fn infer(&self, branch: impl Branch<Self>) -> InferenceRule<Self::Node>
    where
        Self: Sized;

    // TODO: Document that it is guaranteed that the ancestors won't contain a contradiction.
    fn has_contradiction(&self, branch: impl Branch<Self>) -> bool
    where
        Self: Sized;

    fn make_premise_node(&self, expr: Self::Expr) -> Self::Node;
    fn make_conclusion_node(&self, expr: Self::Expr) -> Self::Node;
    fn initialize(_tableau: &mut PartialTableau<Self>)
    where
        Self: Sized,
    {
    }

    /// The priority in terms of node expansion.
    ///
    /// If higher, it means to expand earlier.
    fn priority(&self, _node: &Self::Node) -> u16 {
        0
    }

    /// Contructs a new [`PartialTableau`] with the given premises and conclusion.
    ///
    /// See also [`PartialTableau::new`].
    fn tableau(
        self,
        premises: impl IntoIterator<Item = Self::Expr>,
        conclusion: Self::Expr,
    ) -> PartialTableau<Self>
    where
        Self: Sized,
    {
        PartialTableau::new(self, premises, conclusion)
    }
}

#[derive(Debug, Clone)]
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
