use tableux::{
    classical::{Predicate, p},
    core::Tableux,
};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let premises = [p!("A" then "B"), p!("B" then "C")];
    let conclusion = p!("A" then "C");

    let mut tableux = Tableux::new(premises, conclusion);
    println!("{tableux}");
    println!("START\n\n");
    while let Ok(()) = tableux.expand_first() {
        println!("{tableux}");
    }
    println!("{tableux}");

    println!("{:?}", tableux.holds());

    Ok(())
}
