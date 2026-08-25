mod adapters;
mod application;
mod domain_types;

fn main() -> domain_types::ServerExitCode {
    application::run_main()
}
