pub mod eval;

use std::env;
use eval::tokenize;

fn main() {
    println!(tokenize(env::args()));
}
