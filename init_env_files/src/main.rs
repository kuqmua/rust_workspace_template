mod adapters;
mod application;
mod domain_types;

fn main() -> Result<(), domain_types::InitializeError> {
    application::run()
}
