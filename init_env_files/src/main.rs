#[path = "domain_types.rs"]
mod domain_types;
#[path = "path_exists.rs"]
mod path_exists;
#[path = "read_bounded_content.rs"]
mod read_bounded_content;
#[path = "run.rs"]
mod run;
#[path = "write_content.rs"]
mod write_content;

fn main() -> Result<(), domain_types::InitializeError> {
    run::run()
}
