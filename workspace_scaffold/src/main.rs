mod adapters;
mod application;
mod domain_types;

fn main() {
    if !application::run_ok().get() {
        std::process::exit(2i32);
    }
}
