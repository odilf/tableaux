use crate::{
    logic::{Logic, classical, modal, normal_modal},
    tableau::Branch,
};

use super::InferenceRule;

use std::fmt;

make_dyn_logic![
    Classical, classical::Classical, "Classical";
    Modal, modal::Modal, "Modal";
    NormalModal, normal_modal::NormalModal, "Normal Modal";
];

macro_rules! make_dyn_logic {
    ($($name:tt, $logic:path, $display_name:literal);* $(;)?) => {
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

        impl fmt::Display for DynExpr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(DynExpr::$name(expr) => expr.fmt(f)),*
                }
            }
        }

        impl fmt::Display for DynNode {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(DynNode::$name(node) => node.fmt(f)),*
                }
            }
        }

    };
}

// To be able to use it at the top.
pub(crate) use make_dyn_logic;

#[cfg(feature = "wasm")]
mod wasm {
    #[wasm_bindgen(start)]
    fn start() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    use crate::{
        PartialTableau,
        logic::{
            DynExpr, DynLogic, DynNode, classical::Classical, modal::Modal,
            normal_modal::NormalModal,
        },
        tableau::NodeId,
    };
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = Logic)]
    pub struct DynLogicWasm {
        logic: DynLogic,
    }

    #[wasm_bindgen(js_name = Node)]
    #[repr(transparent)]
    pub struct DynNodeWasm {
        node: DynNode,
    }

    #[wasm_bindgen(js_name = Tableau)]
    pub struct DynPartialTableau {
        tableau: PartialTableau<DynLogic>,
    }

    #[wasm_bindgen(js_class = Logic)]
    impl DynLogicWasm {
        fn parse_expr(&self, expr: &str) -> Result<DynExpr, String> {
            Ok(match self.logic {
                DynLogic::Classical(_) => DynExpr::Classical(expr.parse()?),
                DynLogic::Modal(_) => DynExpr::Modal(expr.parse()?),
                DynLogic::NormalModal(_) => DynExpr::NormalModal(expr.parse()?),
            })
        }

        pub fn tableau(
            self,
            premises: Vec<String>,
            conclusion: &str,
        ) -> Result<DynPartialTableau, String>
        where
            Self: Sized,
        {
            let premises = premises
                .into_iter()
                .map(|s| self.parse_expr(&s))
                .collect::<Result<Vec<_>, _>>()?;
            let conclusion = self.parse_expr(conclusion)?;

            Ok(DynPartialTableau {
                tableau: PartialTableau::new(self.logic, premises, conclusion),
            })
        }

        pub fn classical(logic: Classical) -> Self {
            DynLogicWasm {
                logic: DynLogic::Classical(logic),
            }
        }

        pub fn modal(logic: Modal) -> Self {
            DynLogicWasm {
                logic: DynLogic::Modal(logic),
            }
        }

        pub fn normal_modal(logic: NormalModal) -> Self {
            DynLogicWasm {
                logic: DynLogic::NormalModal(logic),
            }
        }
    }

    #[wasm_bindgen(js_class = Tableau)]
    impl DynPartialTableau {
        pub fn infer_once(&mut self) -> bool {
            self.tableau.infer_once().is_some()
        }

        pub fn infer(&mut self) {
            while self.infer_once() {}
        }

        pub fn root(&self) -> u16 {
            self.tableau.root.index
        }

        pub fn get(&self, id: u16) -> DynNodeWasm {
            DynNodeWasm {
                node: self.tableau.get(NodeId { index: id }).value.clone(),
            }
        }

        pub fn children(&self, id: u16) -> Box<[u16]> {
            self.tableau
                .get(NodeId { index: id })
                .children
                .iter()
                .map(|child| child.index)
                .collect()
        }

        #[wasm_bindgen(js_name = toString)]
        pub fn to_string(&self) -> String {
            self.tableau.to_string()
        }
    }

    #[wasm_bindgen(js_class = Node)]
    impl DynNodeWasm {
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string(&self) -> String {
            self.node.to_string()
        }
    }

    #[wasm_bindgen]
    impl Classical {
        #[wasm_bindgen(constructor)]
        pub fn new_js() -> Self {
            Self {}
        }
    }

    #[wasm_bindgen]
    impl Modal {
        #[wasm_bindgen(constructor)]
        pub fn new_js() -> Self {
            Self {}
        }
    }

    #[wasm_bindgen]
    impl NormalModal {
        #[wasm_bindgen(constructor)]
        pub fn new_js(
            reflexive: bool,
            symmetric: bool,
            transitive: bool,
            extendable: bool,
        ) -> Self {
            Self {
                reflexive,
                symmetric,
                transitive,
                extendable,
            }
        }
    }
}
