mod adapters;
mod domain_types;
mod run;

fn main() -> Result<(), domain_types::InitializeError> {
    run::run()
}
