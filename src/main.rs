use std::error::Error;

use tableaux::logic;

fn main() -> Result<(), Box<dyn Error>> {
    // let tableau = logic::modal::infer("◻p ⊢ p");
    // let tableau = logic::modal::infer("p ⊢ ◻⋄p");
    // let tableau = logic::modal::infer("◻p ⊢ ⋄p");
    let tableau = logic::modal::infer("p ⊃ q, q ≡ p ⊢ q ⊃ p");

    // let tableau = logic::classical::infer("p ⊃ q, q ⊃ r ⊢ p ⊃ r");
    // let tableau = logic::classical::infer("a ≡ b ⊢ b ≡ a");

    println!("{tableau}");
    assert!(tableau.holds());

    Ok(())
}
