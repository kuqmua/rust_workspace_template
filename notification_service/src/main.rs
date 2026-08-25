mod adapters;
mod application;
mod domain_types;

fn main() -> domain_types::NotificationExitCode {
    application::run_main()
}
