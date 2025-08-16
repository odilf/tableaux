use std::{error::Error, str::FromStr};

use tableaux::logic::{self, Logic, modal};

fn main() -> Result<(), Box<dyn Error>> {
    let tableau = logic::modal::Modal {}
        .tableau(
            [
                modal::Expr::from_str("□(A ⊃ B)").unwrap(),
                modal::Expr::from_str("□(B ⊃ C)").unwrap(),
            ],
            modal::Expr::from_str("□(A ⊃ C)").unwrap(),
        )
        .infer();

    println!("{tableau}");
    assert!(tableau.holds());

    Ok(())
}
