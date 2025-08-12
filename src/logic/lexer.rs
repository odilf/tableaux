use winnow::{ModalResult, Parser as _, combinator::alt, token::take_while};

mod unicode {
    pub const NOT: char = '¬';
    pub const AND: &str = "∧";
    pub const OR: &str = "∨";
    pub const MAT_IMPL: char = '⊃';
    pub const MAT_EQUIV: char = '≡';
    pub const POSSIB: &str = "◇";
    pub const NECESS: &str = "□";
}

use unicode::*;

pub fn left_paren<'a>(input: &mut &'a str) -> ModalResult<char> {
    '('.parse_next(input)
}

pub fn right_paren<'a>(input: &mut &'a str) -> ModalResult<char> {
    ')'.parse_next(input)
}

pub fn not<'a>(input: &mut &'a str) -> ModalResult<char> {
    alt([NOT, '!']).parse_next(input)
}

pub fn and<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    alt([AND, "&&"]).parse_next(input)
}

pub fn or<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    alt([OR, "||"]).parse_next(input)
}

pub fn mat_impl<'a>(input: &mut &'a str) -> ModalResult<char> {
    alt([MAT_IMPL, '>']).parse_next(input)
}

pub fn mat_equiv<'a>(input: &mut &'a str) -> ModalResult<char> {
    alt([MAT_EQUIV, '=']).parse_next(input)
}

pub fn possib<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    alt([POSSIB, "<>"]).parse_next(input)
}

pub fn necess<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    alt([NECESS, "[]"]).parse_next(input)
}

pub fn ident<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |char| !PUNCT_START.contains(&char)).parse_next(input)
}

/// Characters with which a punctuation symbol can start.
pub const PUNCT_START: &'static [char] = &[
    '(', ')', ' ', '¬', '∨', '∧', '⊃', '≡', '□', '◇', '!', '|', '&', '>', '=', '[', '<', ' ',
];

// use winnow::{
//     ModalResult, Parser,
//     ascii::space1,
//     combinator::{ParserIterator, alt, delimited, iterator},
//     error::{ContextError, ErrMode, ParserError},
//     stream::Stream,
//     token::{literal, take_while},
// };

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct TokenStream<'a> {
//     input: &'a str,
// }

// pub fn tokenize<'a>(
//     input: &'a str,
// ) -> ParserIterator<
//     impl Parser<&'a str, Token<'a>, ErrMode<ContextError>>,
//     &'a str,
//     Token<'a>,
//     ErrMode<ContextError>,
// > {
//     iterator(input, token)
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Token<'a> {
//     Punctuation(Punctuation),
//     Ident(&'a str),
// }

// fn punctuation<'a>(input: &mut &'a str) -> ModalResult<Punctuation> {
//     alt((
//         '('.map(|_| Punctuation::LeftParen),
//         ')'.map(|_| Punctuation::RightParen),
//         alt(['¬', '!']).map(|_| Punctuation::Not),
//         alt(["∨", "|"]).map(|_| Punctuation::Or),
//         alt(["∧", "&"]).map(|_| Punctuation::And),
//         alt(["⊃", ">"]).map(|_| Punctuation::MaterialImplication),
//         alt(["≡", "="]).map(|_| Punctuation::MaterialEquivalence),
//         alt(["□", "[]"]).map(|_| Punctuation::Possibility),
//         alt(["◇", "<>"]).map(|_| Punctuation::Necessity),
//         space1.map(|_| Punctuation::Whitespace),
//     ))
//     .parse_next(input)
// }

// fn ident<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
//     let is_ident = |char| !Punctuation::PUNCT_START.contains(&char);
//     take_while(1.., is_ident).parse_next(input)
// }

// fn token<'a>(input: &mut &'a str) -> ModalResult<Token<'a>> {
//     alt((punctuation.map(Token::Punctuation), ident.map(Token::Ident))).parse_next(input)
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Punctuation {
//     LeftParen,
//     RightParen,
//     Whitespace,
//     Not,
//     Or,
//     And,
//     MaterialImplication,
//     MaterialEquivalence,
//     Possibility,
//     Necessity,
// }

// impl Punctuation {
//     pub fn symbol(self) -> char {
//         match self {
//             Punctuation::LeftParen => '(',
//             Punctuation::RightParen => ')',
//             Punctuation::Whitespace => ' ',
//             Punctuation::Not => '¬',
//             Punctuation::Or => '∨',
//             Punctuation::And => '∧',
//             Punctuation::MaterialImplication => '⊃',
//             Punctuation::MaterialEquivalence => '≡',
//             Punctuation::Possibility => '□',
//             Punctuation::Necessity => '◇',
//         }
//     }

//     pub fn simple_symbol(self) -> &'static str {
//         match self {
//             Punctuation::LeftParen => "(",
//             Punctuation::RightParen => ")",
//             Punctuation::Whitespace => " ",
//             Punctuation::Not => "!",
//             Punctuation::Or => "||",
//             Punctuation::And => "&&",
//             Punctuation::MaterialImplication => ">",
//             Punctuation::MaterialEquivalence => "==",
//             Punctuation::Possibility => "[]",
//             Punctuation::Necessity => "<>",
//         }
//     }

//     pub fn short_name(self) -> &'static str {
//         match self {
//             Punctuation::LeftParen => "(",
//             Punctuation::RightParen => ")",
//             Punctuation::Whitespace => " ",
//             Punctuation::Not => "not",
//             Punctuation::Or => "or",
//             Punctuation::And => "and",
//             Punctuation::MaterialImplication => "impl",
//             Punctuation::MaterialEquivalence => "equiv",
//             Punctuation::Possibility => "possib",
//             Punctuation::Necessity => "neces",
//         }
//     }

//
//

//     pub const MISLEADING_SYMBOLS: &'static [char] = &['⬦', '⋄', '⬚', '◻', '⬜', '◽'];
// }
