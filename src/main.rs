use quasar::pieces::*;
use quasar::geometry::*;
use quasar::parser::*;
use std::fs;

mod quasar;

fn main() {
    let contents = fs::read_to_string("resources/default_position.fen").expect("Error opening a file");
    let state = parse_fen(&contents).unwrap();
    println!("{}", state);
}
