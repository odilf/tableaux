// use crate::{
//     logic::{Logic, classical},
//     tableau::Branch,
// };

// use super::InferenceRule;

// #[derive(Debug, Clone)]
// pub enum DynExpr {
//     Classical(classical::Expr),
// }

// #[derive(Debug, Clone)]
// pub enum DynNode {
//     Classical(classical::Node),
// }

// #[derive(Debug, Clone)]
// pub enum DynLogic {
//     Classical(classical::Classical),
// }

// impl Logic for DynLogic {
//     type Expr = DynExpr;
//     type Node = DynNode;

//     fn infer(&self, branch: impl Branch<Self>) -> InferenceRule<Self::Node> {
//         match self {
//             DynLogic::Classical(logic) => logic
//                 .infer(branch.map(|node| match node {
//                     DynNode::Classical(node) => node,
//                     _ => unreachable!(),
//                 }))
//                 .map(DynNode::Classical),
//         }
//     }
// }
