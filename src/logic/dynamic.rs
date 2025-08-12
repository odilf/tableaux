use crate::{
    logic::{Logic, classical, modal, normal_modal},
    tableau::Branch,
};

use super::InferenceRule;

make_dyn_logic![
    Classical, classical::Classical;
    Modal, modal::Modal;
    NormalModal, normal_modal::NormalModal;
];

macro_rules! make_dyn_logic {
    ($($name:tt, $logic:path);* $(;)?) => {
        #[derive(Debug, Clone)]
        pub enum DynExpr {
            $(
                $name(<$logic as Logic>::Expr)
            ),*
        }

        #[derive(Debug, Clone)]
        pub enum DynNode {
            $(
                $name(<$logic as Logic>::Node)
            ),*
        }

        /// A logic that can be dynamically selected at runtime.
        ///
        /// Internally, this does dynamic dispatch to the underlying logic.
        ///
        /// It supports:
        /// - [Classical logic](classical::Classical)
        /// - [Modal logic](modal::Modal)
        /// - [Normal modal logic](normal_modal::NormalModal)
        #[derive(Debug, Clone)]
        pub enum DynLogic {
            $(
                $name($logic)
            ),*
        }

        impl Logic for DynLogic {
            type Expr = DynExpr;
            type Node = DynNode;

            fn infer(&self, branch: impl Branch<Self>) -> InferenceRule<Self::Node> {
                match self {
                    $(
                        DynLogic::$name(logic) => logic
                            .infer(branch.map(|node| match node {
                                DynNode::$name(node) => node,
                                _ => unreachable!(),
                            }))
                            .map(DynNode::$name),
                    )*
                }
            }

            fn has_contradiction(&self, branch: impl Branch<Self>) -> bool
            where
                Self: Sized,
            {
                match self {
                    $(
                        DynLogic::$name(logic) => logic.has_contradiction(branch.map(|node| match node {
                            DynNode::$name(node) => node,
                            _ => unreachable!(),
                        })),
                    )*
                }
            }

            fn make_premise_node(&self, expr: Self::Expr) -> Self::Node {
                match self {
                    $(
                        DynLogic::$name(logic) => DynNode::$name(logic.make_premise_node(match expr {
                            DynExpr::$name(expr) => expr,
                            _ => unreachable!(),
                        })),
                    )*
                }
            }

            fn make_conclusion_node(&self, expr: Self::Expr) -> Self::Node {
                match self {
                    $(
                        DynLogic::$name(logic) => DynNode::$name(logic.make_conclusion_node(match expr {
                            DynExpr::$name(expr) => expr,
                            _ => unreachable!(),
                        })),
                    )*
                }
            }
        }
    };
}

// To be able to use it at the top.
pub(crate) use make_dyn_logic;
