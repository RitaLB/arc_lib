mod datatypes;
use datatypes::datatypes as dt;
// outr opção, para não por dt antes (importa tudo)
//use datatypes::datatypes::*;
// source code: cargo expect all of the source code to be in this folder
fn main() {
    println!("Hello, world!");
    dt::variables();
    dt::constants();
}

// cargo build --> compile
// cargo run --> compile and run
// cargo check --> checks the code if its able to compile
