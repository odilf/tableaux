use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use crate::logic::InferenceRule;
use crate::logic::lexer::Symbol;
use crate::tableau::{Branch, Tableau};
use crate::{Logic, PartialTableau};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Classical {}

pub fn infer(input: &str) -> Tableau<Classical> {
    PartialTableau::from_str(input).unwrap().infer()
}

impl Logic for Classical {
    type Node = Expr;
    type Expr = Expr;

    fn symbol(&self) -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    fn infer(&self, node: &Expr, _branch: impl Branch<Self>) -> InferenceRule<Self::Node> {
        use InferenceRule as IR;
        match node {
            Expr::Const(_) => IR::None,
            Expr::Not(p) => match p.as_ref() {
                Expr::Const(_) => IR::none(),
                Expr::Not(_) => IR::single(*p.clone()),
                Expr::And(p, q) => IR::split(p.not(), q.not()),
                Expr::Or(p, q) => IR::chain(vec![p.not(), q.not()]),
                Expr::MatImpl(p, q) => IR::chain(vec![*p.clone(), q.not()]),
                Expr::MatEquiv(p, q) => {
                    IR::split_and_chain([p.not(), *q.clone()], [*p.clone(), q.not()])
                }
            },
            Expr::And(p, q) => IR::chain(vec![*p.clone(), *q.clone()]),
            Expr::Or(p, q) => IR::split(*p.clone(), *q.clone()),
            Expr::MatImpl(p, q) => IR::split(p.not(), *q.clone()),
            Expr::MatEquiv(p, q) => {
                IR::split_and_chain([*p.clone(), *q.clone()], [p.not(), q.not()])
            }
        }
    }

    fn has_contradiction(&self, branch: impl Branch<Self>) -> bool {
        let Some((name, value)) = branch.leaf().interpretation() else {
            return false;
        };

        branch
            .ancestors()
            .filter_map(|ancestor| ancestor.interpretation())
            .any(|(other_name, other_value)| other_name == name && other_value != value)
    }

    fn make_premise_node(&self, expr: Self::Expr) -> Self::Node {
        expr
    }

    fn make_conclusion_node(&self, expr: Self::Expr) -> Self::Node {
        Expr::Not(Box::new(expr))
    }

    fn priority(&self, expr: &Self::Node) -> u16 {
        match expr {
            Expr::Const(_) => 10,
            Expr::Not(p) => match p.as_ref() {
                Expr::Const(_) => 10,
                Expr::Not(_) => 9,
                Expr::And(_, _) => 7,
                Expr::Or(_, _) => 8,
                Expr::MatImpl(_, _) => 8,
                Expr::MatEquiv(_, _) => 6,
            },
            Expr::And(_, _) => 8,
            Expr::Or(_, _) => 7,
            Expr::MatImpl(_, _) => 7,
            Expr::MatEquiv(_, _) => 6,
        }
    }
}

impl Classical {
    /// Symbols used in classical logic.
    pub const fn symbols() -> &'static [Symbol] {
        &[
            Symbol::Not,
            Symbol::And,
            Symbol::Or,
            Symbol::MatImpl,
            Symbol::MatEquiv,
        ]
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl Classical {
    /// Symbols used in classical logic.
    #[wasm_bindgen(js_name = symbols)]
    pub fn symbols_wasm() -> Vec<Symbol> {
        Self::symbols().to_vec()
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    // TODO: Use some kind of small string type
    Const(Box<str>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    MatImpl(Box<Expr>, Box<Expr>),
    MatEquiv(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn not(self: &Box<Expr>) -> Expr {
        Expr::Not(self.clone())
    }

    fn interpretation(&self) -> Option<(&str, bool)> {
        match self {
            Self::Const(name) => Some((name, true)),
            Self::Not(p) => match p.as_ref() {
                Self::Const(name) => Some((name, false)),
                _ => None,
            },
            _ => None,
        }
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
        }
    }
}

#[cfg(feature = "parse")]
impl FromStr for Expr {
    // TODO: It would be really nice to just return the proper error :/
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::logic::lexer::{Symbol, ident};
        use winnow::{
            ModalResult, Parser,
            ascii::space0,
            combinator::alt,
            combinator::{delimited, preceded},
            seq,
        };

        fn expr_single<'a>(input: &mut &'a str) -> ModalResult<Expr> {
            let main = alt((
                delimited('(', expr, ')'),
                ident.map(|name: &str| Expr::Const(name.to_string().into_boxed_str())),
                preceded(Symbol::Not.parser(), expr_single).map(|expr| Expr::Not(Box::new(expr))),
            ));

            delimited(space0, main, space0).parse_next(input)
        }

        fn expr<'a>(input: &mut &'a str) -> ModalResult<Expr> {
            let main = alt((
                seq!(expr_single, Symbol::And.parser(), expr)
                    .map(|(a, _, b)| Expr::And(Box::new(a), Box::new(b))),
                seq!(expr_single, Symbol::Or.parser(), expr)
                    .map(|(a, _, b)| Expr::Or(Box::new(a), Box::new(b))),
                seq!(expr_single, Symbol::MatImpl.parser(), expr)
                    .map(|(a, _, b)| Expr::MatImpl(Box::new(a), Box::new(b))),
                seq!(expr_single, Symbol::MatEquiv.parser(), expr)
                    .map(|(a, _, b)| Expr::MatEquiv(Box::new(a), Box::new(b))),
                expr_single,
            ));

            delimited(space0, main, space0).parse_next(input)
        }

        // TODO: It would be really nice to just return the proper error :/
        expr.parse(s).map_err(|err| err.to_string())
    }
}
