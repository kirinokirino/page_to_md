#[macro_use]
extern crate html5ever;

mod common;
mod wiki;
use wiki::wiki;
mod links;
use links::collect_links;

fn main() {
    collect_links();
}
