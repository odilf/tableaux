use core::fmt;
use std::{error::Error, str::FromStr};

use crate::Logic;

#[derive(Debug, Clone)]
pub struct PartialTableau<L: Logic> {
    nodes: Vec<TableauNode<L::Node>>,
    root: NodeId,
    // TODO: Change into binary heap.
    uninferred_nodes: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub struct TableauNode<V> {
    value: V,
    parent: Option<NodeId>,
    // TODO: Use a smallvec type
    children: Vec<NodeId>,
    live_children: u8,
    // TODO: Add death reason.
    death_reason: Option<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    index: u16,
}

impl<L: Logic> PartialTableau<L> {
    /// Contructs a new [`PartialTableau`] with the given premises and conclusion.
    pub fn new(premises: impl IntoIterator<Item = L::Expr>, conclusion: L::Expr) -> Self {
        let premises = premises.into_iter();
        let mut tableau = PartialTableau {
            nodes: Vec::with_capacity(premises.size_hint().0 + 1),
            root: NodeId { index: 0 },
            uninferred_nodes: Vec::with_capacity(premises.size_hint().0 + 1),
        };

        for premise in premises {
            tableau.add_orphan(L::make_premise_node(premise));
        }

        tableau.add_orphan(L::make_conclusion_node(conclusion));

        for i in 0..tableau.nodes.len() - 1 {
            tableau.bind_child(
                NodeId { index: i as u16 },
                NodeId {
                    index: i as u16 + 1,
                },
            );
        }

        tableau
    }
}

impl<L: Logic> Tableau<L> {
    /// Same as [`PartialTableau::new`]
    pub fn new(
        premises: impl IntoIterator<Item = L::Expr>,
        conclusion: L::Expr,
    ) -> PartialTableau<L> {
        PartialTableau::new(premises, conclusion)
    }
}

// -- Inference --

impl<L: Logic> Tableau<L> {
    /// Whether the original statement used to create this tableu holds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tableaux::logic;
    ///
    /// let tableau = logic::modal::infer("p ⊃ q, q ≡ p ⊢ q ⊃ p");
    /// assert_eq!(tableau.holds(), true);
    /// ```
    ///
    /// ```rust
    /// use tableaux::logic;
    ///
    /// let tableau = logic::modal::infer("p ⊃ q ⊢ q ⊃ p");
    /// assert_eq!(tableau.holds(), false);
    /// ```
    pub fn holds(&self) -> bool {
        self.get(self.root).is_dead()
    }
}

impl<L: Logic> PartialTableau<L> {
    pub fn infer(mut self) -> Tableau<L> {
        while let Some(()) = self.infer_once() {}

        debug_assert!(self.uninferred_nodes.is_empty());
        Tableau {
            nodes: self.nodes,
            root: self.root,
        }
    }

    fn infer_once(&mut self) -> Option<()> {
        let node_id = self.uninferred_nodes.pop()?;
        let branch = self.branch(node_id);

        let initial_node_len = self.nodes.len();
        // NOTE: In all cases, we have to check branch liveness at the end in splits because we need to add both before killing other branches,
        // but we need to check on the loop in chains to make sure we don't expand extra nodes if it's dead.
        use crate::logic::InferenceRule as IR;
        match L::infer(branch) {
            IR::None => (),
            IR::Single(p) => {
                for leaf in self.live_leaves() {
                    let new_node = self.add_orphan(p.clone());
                    self.bind_child(leaf, new_node);
                    self.check_branch_liveness(new_node);
                }
            }
            IR::Split([left, right]) => {
                for leaf in self.live_leaves() {
                    let new_left = self.add_orphan(left.clone());
                    self.bind_child(leaf, new_left);

                    let new_right = self.add_orphan(right.clone());
                    self.bind_child(leaf, new_right);

                    self.check_branch_liveness(new_left);
                    self.check_branch_liveness(new_right);
                }
            }
            IR::Chain(ps) => {
                if ps.is_empty() {
                    return Some(());
                }
                for leaf in self.live_leaves() {
                    let new_nodes = ps
                        .iter()
                        .map(|p| self.add_orphan(p.clone()))
                        .collect::<Vec<_>>();

                    self.bind_child(leaf, new_nodes[0]);
                    self.check_branch_liveness(new_nodes[0]);

                    for i in 1..new_nodes.len() {
                        self.bind_child(new_nodes[i - 1], new_nodes[i]);
                        let died = self.check_branch_liveness(new_nodes[i]);
                        if died {
                            break;
                        }
                    }
                }
            }
            IR::SplitAndChain(chains) => {
                for leaf in self.live_leaves() {
                    for [a, b] in &chains {
                        let new_a = self.add_orphan(a.clone());
                        self.bind_child(leaf, new_a);
                        let died = self.check_branch_liveness(new_a);
                        if !died {
                            let new_b = self.add_orphan(b.clone());
                            self.bind_child(new_a, new_b);
                            self.check_branch_liveness(new_b);
                        }
                    }
                }
            }
        };

        // NOTE: This depends on the implementation of `Self::add_orphan`. Thankfully it's pretty logical but just watch out if
        // that tries to be optimized.
        for i in initial_node_len..self.nodes.len() {
            let node_id = NodeId { index: i as u16 };
            self.propagate_branch_liveness(node_id);
        }

        Some(())
    }

    fn check_branch_liveness(&mut self, leaf: NodeId) -> bool {
        if L::has_contradiction(self.branch(leaf)) {
            self.get_mut(leaf).death_reason = Some(());
            true
        } else {
            false
        }
    }

    fn propagate_branch_liveness(&mut self, node_id: NodeId) {
        if !self.get(node_id).is_dead() {
            return;
        }

        let Some(parent) = self.get(node_id).parent else {
            return;
        };

        self.get_mut(parent).live_children -= 1;
        if self.get(parent).live_children == 0 {
            self.get_mut(parent).death_reason = Some(());
            self.propagate_branch_liveness(parent);
        }
    }
}

pub struct Tableau<L: Logic> {
    nodes: Vec<TableauNode<L::Node>>,
    root: NodeId,
}

#[derive(Debug, Clone)]
pub struct Countermodel<N> {
    nodes: Vec<N>,
}

impl<N: fmt::Debug + fmt::Display> Error for Countermodel<N> {}

impl<N: fmt::Display> fmt::Display for Countermodel<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Countermodel: ")?;
        for node in &self.nodes {
            writeln!(f, "➡ {node}")?;
        }

        Ok(())
    }
}

// -- Tree operations --

impl<L: Logic> PartialTableau<L> {
    fn get(&self, node_id: NodeId) -> &TableauNode<L::Node> {
        &self.nodes[node_id.index as usize]
    }

    fn get_mut(&mut self, node_id: NodeId) -> &mut TableauNode<L::Node> {
        &mut self.nodes[node_id.index as usize]
    }

    fn add_orphan(&mut self, node_value: L::Node) -> NodeId {
        let node = TableauNode {
            value: node_value,
            parent: None,
            children: Vec::new(),
            live_children: 0,
            death_reason: None,
        };

        let node_id = NodeId {
            index: self.nodes.len() as u16,
        };
        self.nodes.push(node);
        self.uninferred_nodes.push(node_id);

        node_id
    }

    /// # Panics
    ///
    /// If the child is not orhpan.
    fn bind_child(&mut self, parent: NodeId, child: NodeId) {
        let old = self.get_mut(child).parent.replace(parent);
        assert!(old.is_none());
        let parent = self.get_mut(parent);
        parent.children.push(child);
        parent.live_children += 1;
    }

    fn branch(&self, leaf: NodeId) -> Branch<'_, L> {
        Branch {
            leaf,
            tableau: self,
        }
    }

    /// Iter over every leaf node that is not dead.
    fn live_leaves(&self) -> Vec<NodeId> {
        let mut queue: Vec<NodeId> = vec![self.root];
        let mut output = Vec::new();
        while let Some(node_id) = queue.pop() {
            // Skip dead branches
            if self.get(node_id).is_dead() {
                continue;
            }

            let children = &self.get(node_id).children;
            if children.is_empty() {
                output.push(node_id);
            } else {
                queue.extend(children.into_iter())
            }
        }

        output
    }
}
impl<L: Logic> Tableau<L> {
    fn get(&self, node_id: NodeId) -> &TableauNode<L::Node> {
        &self.nodes[node_id.index as usize]
    }
}

impl<V> TableauNode<V> {
    fn is_dead(&self) -> bool {
        self.death_reason.is_some()
    }
}

pub struct Branch<'t, L: Logic> {
    leaf: NodeId,
    tableau: &'t PartialTableau<L>,
}

impl<'t, L: Logic> Branch<'t, L> {
    pub fn leaf(&self) -> &'t L::Node {
        &self.tableau.get(self.leaf).value
    }

    pub fn ancestors(&self) -> impl Iterator<Item = &L::Node> {
        AncestorIter {
            tableau: self.tableau,
            current: self.leaf,
        }
    }
}

struct AncestorIter<'t, L: Logic> {
    tableau: &'t PartialTableau<L>,
    current: NodeId,
}

impl<'t, L: Logic> Iterator for AncestorIter<'t, L> {
    type Item = &'t L::Node;
    fn next(&mut self) -> Option<Self::Item> {
        let parent = self.tableau.get(self.current).parent?;
        self.current = parent;
        Some(&self.tableau.get(parent).value)
    }
}

// -- Trait implementations --

#[cfg(feature = "parse")]
impl<L: Logic> FromStr for PartialTableau<L>
where
    L::Expr: FromStr,
{
    type Err = TableauParseError<<L::Expr as FromStr>::Err>;

    /// Parses an argument of the style of `Σ ⊢ A`, where `Σ` can be multiple expressions separated
    /// by commas.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((premises, conclusion)) = s.split_once('⊢') else {
            return Err(TableauParseError::MissingInferenceSymbol);
        };

        let premises = if premises.is_empty() {
            Vec::new()
        } else {
            premises
                .split(',')
                .map(|premise| {
                    L::Expr::from_str(premise.trim())
                        .map_err(|inner_err| TableauParseError::ExpressionError(inner_err))
                })
                .collect::<Result<_, _>>()?
        };

        let conclusion = L::Expr::from_str(conclusion)?;

        Ok(PartialTableau::new(premises, conclusion))
    }
}

#[cfg(feature = "parse")]
#[derive(Debug, Clone)]
pub enum TableauParseError<E> {
    ExpressionError(E),
    MissingInferenceSymbol,
    MissingConclusion,
}

impl<E> From<E> for TableauParseError<E> {
    fn from(err: E) -> Self {
        Self::ExpressionError(err)
    }
}

impl<L: Logic> fmt::Display for PartialTableau<L>
where
    L::Node: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut queue = vec![(self.get(self.root), 0)];

        while let Some((node, depth)) = queue.pop() {
            for _ in 0..depth {
                write!(f, "  ")?;
            }

            write!(f, "{}", node.value)?;
            if node.is_dead() {
                write!(f, " ✘")?;
            }
            write!(f, " (live_children={})", node.live_children)?;
            writeln!(f)?;

            for &child in node.children.iter().rev() {
                queue.push((self.get(child), depth + 1))
            }
        }

        Ok(())
    }
}

impl<L: Logic> fmt::Display for Tableau<L>
where
    L::Node: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut queue = vec![(self.get(self.root), 0)];

        while let Some((node, depth)) = queue.pop() {
            for _ in 0..depth {
                write!(f, "  ")?;
            }

            let style = if node.is_dead() {
                anstyle::Style::new()
            } else {
                anstyle::Style::new().bold()
            };

            writeln!(f, "{style}{}{style:#}", node.value)?;

            for &child in node.children.iter().rev() {
                queue.push((self.get(child), depth + 1))
            }
        }

        Ok(())
    }
}
