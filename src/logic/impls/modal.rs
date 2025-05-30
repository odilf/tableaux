use std::{fmt, ops::Deref, str::FromStr};

use crate::{
    Logic, PartialTableau,
    logic::InferenceRule,
    tableau::{Branch, Tableau},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Modal {}

pub fn infer(input: &str) -> Tableau<Modal> {
    PartialTableau::from_str(input).unwrap().infer()
}

impl Logic for Modal {
    type Node = ModalNode;
    type Expr = Expr;

    fn infer(&self, branch: impl Branch<Self>) -> InferenceRule<Self::Node> {
        use InferenceRule as IR;

        let ModalNode::Expr { expr, world } = branch.leaf() else {
            // Relations don't do inferrence in basic modal logic.
            return IR::none();
        };

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

                return IR::chain(vec![
                    ModalNode::Relation {
                        from: world,
                        to: fresh_world,
                    },
                    ModalNode::Expr {
                        expr: *p.clone(),
                        world: fresh_world,
                    },
                ]);
            }
            Expr::Necessity(p) => {
                return IR::chain(
                    branch
                        .ancestors()
                        .filter_map(|ancestor| ancestor.accessible_world_from(world))
                        .map(|other_world| ModalNode::Expr {
                            expr: *p.clone(),
                            world: other_world,
                        })
                        .collect(),
                );
            }
        };

        classical_inference.map(|expr| ModalNode::Expr { expr, world })
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
        ModalNode::Expr {
            expr,
            world: World::ZERO,
        }
    }

    fn make_conclusion_node(&self, expr: Self::Expr) -> Self::Node {
        ModalNode::Expr {
            expr: Expr::Not(Box::new(expr)),
            world: World::ZERO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    // TODO: Use some kind of small string type
    Const(Box<str>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    MatImpl(Box<Expr>, Box<Expr>),
    MatEquiv(Box<Expr>, Box<Expr>),
    Possibility(Box<Expr>),
    Necessity(Box<Expr>),
}

impl Expr {
    pub fn not(self: &Box<Self>) -> Self {
        Self::Not(self.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalNode {
    Expr { expr: Expr, world: World },
    Relation { from: World, to: World },
}

impl ModalNode {
    pub fn world(&self) -> Option<World> {
        match self {
            Self::Expr { world, .. } => Some(*world),
            _ => None,
        }
    }

    pub fn accessible_world_from(&self, world: World) -> Option<World> {
        match self {
            Self::Relation { from, to } if *from == world => Some(*to),
            _ => None,
        }
    }

    pub fn interpretation(&self) -> Option<(&str, bool, World)> {
        match self {
            Self::Expr { expr, world } => match expr {
                Expr::Const(name) => Some((name, true, *world)),
                Expr::Not(p) => match p.deref() {
                    Expr::Const(name) => Some((name, false, *world)),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct World(u16);

impl World {
    pub const ZERO: Self = World(0);

    pub fn next(&self) -> Self {
        World(self.0 + 1)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(name) => f.write_str(&name),
            Self::Not(p) => write!(f, "¬{p}"),
            Self::And(x, y) => write!(f, "({x} ∧ {y})"),
            Self::Or(x, y) => write!(f, "({x} ∨ {y})"),
            Self::MatImpl(x, y) => write!(f, "({x} ⊃ {y})"),
            Self::MatEquiv(x, y) => write!(f, "({x} ≡ {y})"),
            Self::Possibility(p) => write!(f, "⋄{p}"),
            Self::Necessity(p) => write!(f, "□{p}"),
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ModalNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModalNode::Expr { expr, world } => write!(f, "{expr}, {world}"),
            ModalNode::Relation { from, to } => write!(f, "{from}r{to}"),
        }
    }
}

#[cfg(feature = "parse")]
impl FromStr for Expr {
    // TODO: It would be really nice to just return the proper error :/
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use winnow::{
            ModalResult, Parser,
            ascii::space0,
            combinator::alt,
            combinator::{delimited, preceded},
            seq,
            token::take_while,
        };

        fn ident<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
            let tokens = ['¬', '∨', '∧', '⊃', '≡', '□', '◇', '(', ')', ' '];
            let wrong_tokens = ['⬦', '⋄', '⬚', '◻', '⬜', '◽'];

            let is_ident = |char| {
                if wrong_tokens.contains(&char) {
                    panic!("Wrong token! {char}");
                }
                !tokens.contains(&char)
            };

            take_while(1.., is_ident).parse_next(input)
        }

        fn expr_single<'a>(input: &mut &'a str) -> ModalResult<Expr> {
            let main = alt((
                delimited('(', expr, ')'),
                ident.map(|name: &str| Expr::Const(name.to_string().into_boxed_str())),
                preceded('¬', expr).map(|expr| Expr::Not(Box::new(expr))),
                preceded('◇', expr).map(|p| Expr::Possibility(Box::new(p))),
                preceded('□', expr).map(|p| Expr::Necessity(Box::new(p))),
            ));

            delimited(space0, main, space0).parse_next(input)
        }

        fn expr<'a>(input: &mut &'a str) -> ModalResult<Expr> {
            let main = alt((
                seq!(expr_single, '∧', expr).map(|(a, _, b)| Expr::And(Box::new(a), Box::new(b))),
                seq!(expr_single, '∨', expr).map(|(a, _, b)| Expr::Or(Box::new(a), Box::new(b))),
                seq!(expr_single, '⊃', expr)
                    .map(|(a, _, b)| Expr::MatImpl(Box::new(a), Box::new(b))),
                seq!(expr_single, '≡', expr)
                    .map(|(a, _, b)| Expr::MatEquiv(Box::new(a), Box::new(b))),
                expr_single,
            ));

            delimited(space0, main, space0).parse_next(input)
        }

        // TODO: It would be really nice to just return the proper error :/
        expr.parse(s).map_err(|err| err.to_string())
    }
}
