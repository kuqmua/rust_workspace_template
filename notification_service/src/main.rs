mod adapters;
mod domain_types;
mod run_main;

fn main() -> domain_types::NotificationExitCode {
    run_main::run_main()
}
