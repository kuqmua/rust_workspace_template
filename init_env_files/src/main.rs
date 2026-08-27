mod domain_types;
mod path_exists;
mod read_bounded_content;
mod run;
mod write_content;

fn main() -> Result<(), domain_types::InitializeError> {
    run::run()
}
