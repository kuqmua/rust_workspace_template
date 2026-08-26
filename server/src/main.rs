mod adapters;
mod application;
mod domain_types;

fn main() -> domain_types::ServerExitCode {
    application::startup::run_main()
}
