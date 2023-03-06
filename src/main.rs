#[macro_use]
extern crate html5ever;

mod common;
mod wiki;
use wiki::wiki;

fn main() {
    wiki();
}
