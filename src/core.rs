use std::{
    collections::VecDeque,
    fmt::{self, Display},
};

use anyhow::Context;

pub type Arena = bumpalo::Bump;

pub trait Predicate: Sized {
    fn negated(self) -> Self;

    // TODO: Can this be an iterator?
    fn split(&self) -> SplitResult<Self>;

    fn is_terminal(&self) -> bool;

    /// This function will only be called on terminal values (but ancenstors might not be terminals).
    fn contradicts<'a>(&self, ancenstors: impl Iterator<Item = &'a Self>) -> Option<String>
    where
        Self: 'a;
}

pub enum SplitResult<P> {
    Disjunction([P; 2]),
    Conjunction([P; 2]),
    Terminal,
}

#[derive(Debug, Clone)]
pub struct Tableux<P> {
    nodes: Vec<Node<P>>,
    root: NodeId,
    unexpanded_nodes: VecDeque<NodeId>,
    // Also known as leaves, but book calls them tips
    tips: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub struct Node<P> {
    value: P,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
    dead_reason: Option<String>,
}

impl<P> Node<P> {
    fn dead(&self) -> bool {
        self.dead_reason.is_some()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId {
    index: u16,
}

impl<P> Tableux<P> {
    // TODO: Find better name
    pub fn holds(&self) -> Option<bool> {
        self.unexpanded_nodes
            .is_empty()
            .then(|| self.tips().is_empty())
    }

    fn get(&self, node: NodeId) -> &Node<P> {
        &self.nodes[node.index as usize]
    }

    fn get_mut(&mut self, node: NodeId) -> &mut Node<P> {
        &mut self.nodes[node.index as usize]
    }

    fn tips(&self) -> Vec<NodeId> {
        let mut queue: Vec<NodeId> = vec![self.root];
        let mut output = Vec::new();
        while let Some(node_id) = queue.pop() {
            // Skip dead branches
            if self.get(node_id).dead_reason.is_some() {
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

    fn new_chain(values: impl IntoIterator<Item = P>) -> Option<Self> {
        let mut nodes = Vec::<Node<P>>::new();
        let mut node_ids = VecDeque::<NodeId>::new();

        for (i, value) in values.into_iter().enumerate() {
            // Get and update the parent if exists
            let parent = nodes.get_mut(i.wrapping_sub(1)).map(|parent| {
                parent.children.push(NodeId { index: i as u16 });
                node_ids[i - 1]
            });

            nodes.push(Node {
                value,
                parent,
                children: Vec::new(),
                dead_reason: None,
            });

            node_ids.push_back(NodeId { index: i as u16 });
        }

        let root = *node_ids.get(0)?;
        let tip = node_ids[node_ids.len() - 1];
        Some(Self {
            nodes,
            unexpanded_nodes: node_ids,
            root,
            tips: vec![tip],
        })
    }

    fn add_new_orphan(&mut self, value: P) -> NodeId {
        self.nodes.push(Node {
            value,
            parent: None,
            children: Vec::new(),
            dead_reason: None,
        });

        let id = NodeId {
            index: self.nodes.len() as u16 - 1,
        };

        self.unexpanded_nodes.push_back(id);
        id
    }

    /// Precondition: `child` must be orphan.
    fn make_child(&mut self, parent: NodeId, child: NodeId)
    where
        P: Clone,
    {
        let old_parent = self.get_mut(child).parent.replace(parent);
        assert!(old_parent.is_none(), "Node is not orphan.");

        self.get_mut(parent).children.push(child);
    }

    fn ancestors<'a, 'b>(&'a self, node_id: NodeId) -> impl Iterator<Item = &'a Node<P>> {
        AncenstorIter {
            tableux: self,
            node: Some(self.get(node_id)),
        }
    }
}

impl<P: Predicate> Tableux<P> {
    pub fn new(premises: impl IntoIterator<Item = P>, conclusion: P) -> Self {
        Self::new_chain(
            premises
                .into_iter()
                .chain(std::iter::once(conclusion.negated())),
        )
        .expect("There is at least the conclusion")
    }

    pub fn complete(&self) -> bool {
        self.unexpanded_nodes.is_empty()
    }

    /// Returns whether the branch is dead.
    fn check_dead_branch(&mut self, tip: NodeId) -> bool {
        if self.get(tip).value.is_terminal() {
            let dead_reason = self
                .get(tip)
                .value
                .contradicts(self.ancestors(tip).map(|node| &node.value));
            let died = dead_reason.is_some();
            self.get_mut(tip).dead_reason = dead_reason;
            died
        } else {
            false
        }
    }

    // TODO: Remove dependency on `anyhow`
    pub fn expand_first(&mut self) -> anyhow::Result<()>
    where
        P: Clone,
    {
        let node_id = self
            .unexpanded_nodes
            .pop_front()
            .context("No nodes left to expand.")?;

        let node = self.get_mut(node_id);
        match node.value.split() {
            SplitResult::Terminal => return self.expand_first(),
            SplitResult::Disjunction(values) => {
                for tip in self.tips() {
                    let value_ids = values.clone().map(|value| self.add_new_orphan(value));
                    for &value_id in &value_ids {
                        self.make_child(tip, value_id);
                        self.check_dead_branch(value_id);
                    }
                }
            }
            SplitResult::Conjunction([a, b]) => {
                for tip in self.tips() {
                    let root = self.add_new_orphan(a.clone());
                    self.make_child(tip, root);
                    let died = self.check_dead_branch(root);
                    if died {
                        continue;
                    }

                    let leaf = self.add_new_orphan(b.clone());
                    self.make_child(root, leaf);
                    self.check_dead_branch(leaf);
                }
            }
        }

        Ok(())
    }
}

impl<P: Display + fmt::Debug> fmt::Display for Tableux<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut queue = vec![(self.root, 0)];
        while let Some((current_id, depth)) = queue.pop() {
            let node = self.get(current_id);
            for _ in 0..depth {
                f.write_str("  ")?;
            }
            write!(f, "{}", node.value,)?;

            if node.dead() {
                write!(f, "  x")?;
            }

            writeln!(f)?;

            if node.children.is_empty() {
                continue;
            }

            for child_id in node.children.iter().rev() {
                queue.push((*child_id, depth + 1))
            }
        }

        Ok(())
    }
}

pub struct AncenstorIter<'a, P> {
    tableux: &'a Tableux<P>,
    node: Option<&'a Node<P>>,
}

impl<'a, P> Iterator for AncenstorIter<'a, P> {
    type Item = &'a Node<P>;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node?;
        self.node = current.parent.map(|parent| self.tableux.get(parent));
        Some(current)
    }
}
