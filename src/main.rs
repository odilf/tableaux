use std::{error::Error, str::FromStr};

use tableaux::logic::{self, Logic, modal};

fn main() -> Result<(), Box<dyn Error>> {
    let tableau = logic::normal_modal::NormalModal::new()
        .reflexive()
        .tableau(None, modal::Expr::from_str("□p ⊃ p").unwrap())
        .infer();

    println!("{tableau}");
    assert!(tableau.holds());

    Ok(())
}
