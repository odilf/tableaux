use std::{cmp, collections::HashSet};

use crate::{
    Logic, PartialTableau,
    logic::{
        InferenceRule,
        lexer::Symbol,
        modal::{Expr, Modal, Node, World},
    },
    tableau::Branch,
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NormalModal {
    /// ρ, for every world w, `w R w`
    pub reflexive: bool,
    /// σ, if `w1 R w2` then `w2 R w1`
    pub symmetric: bool,
    /// τ, if `w1 R w2` and `w2 R w3,` then `w1 R w3`
    pub transitive: bool,
    /// η, if `w1`
    pub extendable: bool,
}

impl Logic for NormalModal {
    type Node = Node;
    type Expr = Expr;

    fn infer(&self, node: &Self::Node, branch: impl Branch<Self>) -> InferenceRule<Self::Node> {
        use InferenceRule as IR;

        let inferrence = match node {
            Node::Expr { expr, world } => {
                let world = *world;

                let classical_inference: IR<Expr> = match expr {
                    Expr::Const(_) => IR::none(),
                    Expr::Not(p) => match p.as_ref() {
                        Expr::Const(_) => IR::none(),
                        Expr::Not(_) => IR::single(*p.clone()),
                        Expr::And(p, q) => IR::split(p.not(), q.not()),
                        Expr::Or(p, q) => IR::chain(vec![p.not(), q.not()]),
                        Expr::MatImpl(p, q) => IR::chain(vec![*p.clone(), q.not()]),
                        Expr::MatEquiv(p, q) => {
                            IR::split_and_chain([p.not(), *q.clone()], [*p.clone(), q.not()])
                        }
                        Expr::Possibility(p) => IR::Single(Expr::Necessity(Box::new(p.not()))),
                        Expr::Necessity(p) => IR::Single(Expr::Possibility(Box::new(p.not()))),
                    },
                    Expr::And(p, q) => IR::chain(vec![*p.clone(), *q.clone()]),
                    Expr::Or(p, q) => IR::split(*p.clone(), *q.clone()),
                    Expr::MatImpl(p, q) => IR::split(p.not(), *q.clone()),
                    Expr::MatEquiv(p, q) => {
                        IR::split_and_chain([*p.clone(), *q.clone()], [p.not(), q.not()])
                    }
                    Expr::Possibility(p) => {
                        let max_so_far = branch
                            .ancestors()
                            .filter_map(|ancestor| ancestor.world())
                            .max();
                        let fresh_world = max_so_far.map_or(World::ZERO, |i| i.next());

                        // These nodes always get added
                        let basic = [
                            Node::Relation {
                                from: world,
                                to: fresh_world,
                            },
                            Node::Expr {
                                expr: *p.clone(),
                                world: fresh_world,
                            },
                        ]
                        .into_iter();

                        // Reflexive relation, create `i r i` for every new world `i`
                        let r = self.reflexive.then(|| Node::Relation {
                            from: fresh_world,
                            to: fresh_world,
                        });

                        return IR::chain(basic.chain(r).collect());
                    }
                    Expr::Necessity(p) => {
                        return IR::chain(
                            branch
                                .ancestors()
                                .filter_map(|ancestor| ancestor.accessible_world_from(world))
                                .map(|other_world| Node::Expr {
                                    expr: *p.clone(),
                                    world: other_world,
                                })
                                .collect(),
                        );
                    }
                };

                classical_inference.map(|expr| Node::Expr { expr, world })
            }
            Node::Relation { from, to } => {
                let s = self
                    .symmetric
                    .then(|| Node::Relation {
                        from: *to,
                        to: *from,
                    })
                    // Only add symmetric node if not on branch
                    .and_then(|sym| (!branch.contains(&sym)).then_some(sym));

                let t = self
                    .transitive
                    .then(|| {
                        // We have j->k, we get the i->j and add i->k
                        let j = from;
                        let k = to;
                        branch
                            .ancestors()
                            .filter_map(move |other| match other {
                                Node::Relation {
                                    from: i,
                                    to: j_other,
                                } if j == j_other => Some(i),
                                _ => None,
                            })
                            .map(|i| Node::Relation { from: *i, to: *k })
                            // Don't add if already on branch
                            // TODO: Is this redundant?
                            .filter(|t| !branch.contains(t))
                    })
                    .into_iter()
                    .flatten();

                IR::chain(t.chain(s).collect())
            }
        };

        // Add a new relation i->j to a fresh j from an existing i
        if matches!(inferrence, IR::None) && self.extendable {
            // Worlds that don't access any new worlds
            let mut maybe_leaf_worlds = HashSet::new();
            let mut non_leaf_worlds = HashSet::new();
            for node in branch.ancestors() {
                match node {
                    Node::Expr { world, .. } => {
                        maybe_leaf_worlds.insert(*world);
                    }
                    Node::Relation { from: i, to: j } => {
                        non_leaf_worlds.insert(*i);
                        maybe_leaf_worlds.insert(*j);
                    }
                }
            }

            let mut leaf_worlds = maybe_leaf_worlds.difference(&non_leaf_worlds);
            let Some(leaf_world) = leaf_worlds.next() else {
                return IR::none();
            };

            let max_so_far = branch
                .ancestors()
                .filter_map(|ancestor| ancestor.world())
                .max();
            let fresh_world = max_so_far.map_or(World::ZERO, |i| i.next());

            return IR::single(Node::Relation {
                from: *leaf_world,
                to: fresh_world,
            });
        }

        inferrence
    }

    fn has_contradiction(&self, branch: impl Branch<Self>) -> bool {
        let Some((name, value, world)) = branch.leaf().interpretation() else {
            return false;
        };

        branch
            .ancestors()
            .filter_map(|ancestor| ancestor.interpretation())
            .any(|(other_name, other_value, other_world)| {
                other_name == name && other_world == world && other_value != value
            })
    }

    fn make_premise_node(&self, expr: Self::Expr) -> Self::Node {
        Node::Expr {
            expr,
            world: World::ZERO,
        }
    }

    fn make_conclusion_node(&self, expr: Self::Expr) -> Self::Node {
        Node::Expr {
            expr: Expr::Not(Box::new(expr)),
            world: World::ZERO,
        }
    }

    fn initialize(tableau: &mut PartialTableau<Self>) {
        // NOTE: This is a bit overcomplicated for basically just adding
        // the rule `0 R 0`. But it is more "correct" in that it adds the
        // reflexive relation to each unique world in the tableau, it's just
        // that currently the only unique world is `World::ZERO` in one branch.
        if tableau.logic.reflexive {
            for leaf in tableau.live_leaves() {
                let branch = tableau.branch(leaf);
                let unique_worlds = branch
                    .ancestors()
                    .filter_map(|node| node.world())
                    .collect::<HashSet<_>>();
                drop(branch);

                for unique_world in unique_worlds {
                    let node = Node::Relation {
                        from: unique_world,
                        to: unique_world,
                    };

                    tableau.add_child(leaf, node);
                }
            }
        }
    }
}

impl NormalModal {
    /// Creates K, the basic modal logic.
    pub const fn new() -> Self {
        Self {
            reflexive: false,
            symmetric: false,
            transitive: false,
            extendable: false,
        }
    }

    pub const fn reflexive(self) -> Self {
        Self {
            reflexive: true,
            // Reflexivity implies extendability, so we can remove the explicit
            // extendability which tends to me more compute heavy and get the
            // same result.
            extendable: false,
            ..self
        }
    }

    pub const fn symmetric(self) -> Self {
        Self {
            symmetric: true,
            ..self
        }
    }

    pub const fn transitive(self) -> Self {
        Self {
            transitive: true,
            ..self
        }
    }

    pub const fn extendable(self) -> Self {
        Self {
            extendable: true,
            ..self
        }
    }

    const fn normalized(self) -> Self {
        Self {
            // sigma tau eta imply rho
            reflexive: self.reflexive || (self.symmetric && self.transitive && self.extendable),
            symmetric: self.symmetric,
            transitive: self.transitive,
            // rho implies eta
            extendable: self.extendable || self.reflexive,
        }
    }
}

/// Kρ
pub const T: NormalModal = NormalModal::new().reflexive();

/// Kη
pub const D: NormalModal = NormalModal {
    reflexive: false,
    symmetric: false,
    transitive: false,
    extendable: true,
};

/// Kρσ
pub const B: NormalModal = NormalModal {
    reflexive: false,
    symmetric: false,
    transitive: true,
    extendable: true,
};

/// Kρτ
pub const S4: NormalModal = NormalModal {
    reflexive: false,
    symmetric: false,
    transitive: true,
    extendable: true,
};

/// Kρστ
pub const S5: NormalModal = NormalModal {
    reflexive: false,
    symmetric: false,
    transitive: true,
    extendable: true,
};

impl cmp::PartialEq for NormalModal {
    fn eq(&self, other: &Self) -> bool {
        let a = self.normalized();
        let b = other.normalized();
        a.reflexive == b.reflexive
            && a.symmetric == b.symmetric
            && a.transitive == b.transitive
            && a.extendable == b.extendable
    }
}

impl cmp::Eq for NormalModal {}

impl NormalModal {
    /// Symbols used in classical logic.
    pub const fn symbols() -> &'static [Symbol] {
        Modal::symbols()
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl NormalModal {
    /// Symbols used in classical logic.
    #[wasm_bindgen(js_name = symbols)]
    pub fn symbols_wasm() -> Vec<Symbol> {
        Self::symbols().to_vec()
    }
}
