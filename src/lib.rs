pub mod logic;
use logic::Logic;

mod tableau;
pub use tableau::PartialTableau;

pub mod classical;
pub mod modal;
// pub mod normal_modal;
