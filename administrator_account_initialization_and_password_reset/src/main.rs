#![allow(
    unused_crate_dependencies,
    reason = "constants_str is used by binary unit tests"
)]

mod application;
mod domain_types;

fn main() -> domain_types::AdministratorAccountCommandExitCode {
    application::run_main()
}
