#![allow(
    clippy::exit,
    reason = "the workspace test runner owns immediate process termination for failed tool modes"
)]

mod adapters;
mod application;
mod domain_types;

fn main() {
    application::run_main();
}
