use std::{error::Error, str::FromStr};

use tableaux::logic::{self, Logic, modal};

fn main() -> Result<(), Box<dyn Error>> {
    // let tableau = logic::modal::infer("◻p ⊢ p");
    // let tableau = logic::modal::infer("p ⊢ ◻⋄p");
    // let tableau = logic::modal::infer("◻p ⊢ ⋄p");
    // let tableau = logic::modal::infer("p ⊃ q, q ≡ p ⊢ q ⊃ p");

    let tableau = logic::normal_modal::NormalModal::new(true, false, false, true)
        .unwrap()
        .tableu(None, modal::Expr::from_str("□p ⊃ p").unwrap())
        .infer();

    println!("{tableau}");
    assert!(tableau.holds());

    Ok(())
}
