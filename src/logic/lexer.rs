use winnow::{ModalResult, Parser as _, combinator::alt, token::take_while};

mod unicode {
    pub const NOT: char = '¬';
    pub const AND: &str = "∧";
    pub const OR: &str = "∨";
    pub const MAT_IMPL: char = '⊃';
    pub const MAT_EQUIV: &str = "≡";
    pub const POSSIB: &str = "◇";
    pub const NECESS: &str = "□";
}

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use unicode::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    Not,
    And,
    Or,
    MatImpl,
    MatEquiv,
    Possib,
    Necess,
}

impl Symbol {
    pub fn symbol(self) -> char {
        match self {
            Symbol::Not => NOT,
            Symbol::And => AND.chars().next().unwrap(),
            Symbol::Or => OR.chars().next().unwrap(),
            Symbol::MatImpl => MAT_IMPL,
            Symbol::MatEquiv => MAT_EQUIV.chars().next().unwrap(),
            Symbol::Possib => POSSIB.chars().next().unwrap(),
            Symbol::Necess => NECESS.chars().next().unwrap(),
        }
    }

    pub fn ascii_str(&self) -> &str {
        match self {
            Symbol::Not => "!",
            Symbol::And => "&&",
            Symbol::Or => "||",
            Symbol::MatImpl => ">",
            Symbol::MatEquiv => "==",
            Symbol::Possib => "<>",
            Symbol::Necess => "[]",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Symbol::Not => "Not",
            Symbol::And => "And",
            Symbol::Or => "Or",
            Symbol::MatImpl => "Material implication",
            Symbol::MatEquiv => "Material equivalence",
            Symbol::Possib => "Possibility",
            Symbol::Necess => "Necessity",
        }
    }

    pub fn parser(self) -> impl Fn(&mut &str) -> ModalResult<()> {
        move |input| match self {
            Symbol::Not => alt([NOT, '!']).map(|_| ()).parse_next(input),
            Symbol::And => alt([AND, "&&"]).map(|_| ()).parse_next(input),
            Symbol::Or => alt([OR, "||"]).map(|_| ()).parse_next(input),
            Symbol::MatImpl => alt([MAT_IMPL, '>']).map(|_| ()).parse_next(input),
            Symbol::MatEquiv => alt([MAT_EQUIV, "=="]).map(|_| ()).parse_next(input),
            Symbol::Possib => alt([POSSIB, "<>"]).map(|_| ()).parse_next(input),
            Symbol::Necess => alt([NECESS, "[]"]).map(|_| ()).parse_next(input),
        }
    }

    pub fn iter() -> impl Iterator<Item = Symbol> {
        [
            Symbol::Not,
            Symbol::And,
            Symbol::Or,
            Symbol::MatEquiv,
            Symbol::Possib,
            Symbol::Necess,
            // NOTE: Material implication goes after <> because in ascii it's >
            // and we want to first replace the <> and after the > otherwise we
            // get wrong symbols.
            // FIXME: This does work but is very fragile...
            Symbol::MatImpl,
        ]
        .into_iter()
    }
}

pub fn ident<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |char| !SYMBOL_START.contains(&char)).parse_next(input)
}

/// Characters with which a symbol can start.
pub const SYMBOL_START: &'static [char] = &[
    '(', ')', ' ', '¬', '∨', '∧', '⊃', '≡', '□', '◇', '!', '|', '&', '>', '=', '[', '<', ' ',
];

// Standalone methods for wasm since javascript doesn't have enum methods
#[cfg(feature = "wasm")]
mod wasm {
    use super::Symbol;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_name = symbolChar)]
    pub fn symbol_char(p: Symbol) -> char {
        p.symbol()
    }

    #[wasm_bindgen(js_name = symbolAsciiStr)]
    pub fn symbol_ascii_str(p: Symbol) -> String {
        p.ascii_str().to_string()
    }

    #[wasm_bindgen(js_name = symbolName)]
    pub fn symbol_name(p: Symbol) -> String {
        p.name().to_string()
    }

    #[wasm_bindgen(js_name = symbolIter)]
    pub fn symbol_iter() -> Vec<Symbol> {
        Symbol::iter().collect()
    }
}
