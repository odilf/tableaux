use crate::{Logic, classical::Expr};

pub struct NormalModal {
    /// ρ, for every world w, `w R w`
    reflexive: bool,
    /// σ, if `w1 R w2` then `w2 R w1`
    symmetric: bool,
    /// τ, if `w1 R w2` and `w2 R w3,` then `w1 R w3`
    transitive: bool,
    /// η, if `w1`
    extendable: bool,
}

impl Logic for NormalModal {
    type Node = (bool, u16);
    type Expr = Expr;

    fn infer(branch: crate::logic::Branch<Self>) -> crate::logic::InferenceRule<Self::Node>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl NormalModal {
    fn new(reflexive: bool, symmetric: bool, transitive: bool, extendable: bool) -> Option<Self> {
        if reflexive && !extendable {
            // Reflexivity implies extendability
            return None;
        }

        if reflexive && symmetric && transitive && !extendable {
            // ρ, σ, τ imply η
            return None;
        }

        Some(Self {
            reflexive,
            symmetric,
            transitive,
            extendable,
        })
    }
}
