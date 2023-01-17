use mylib::{lemonde, nytimes};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{NAME} {VERSION}");

    let lemonde = lemonde::articles();
    println!("\nLe Monde");
    for article in lemonde {
        println!("* {}", article);
    }

    let nytimes = nytimes::articles();
    println!("\nNY Times");
    for article in nytimes {
        println!("* {}", article);
    }
}
