#![allow(
    unused_crate_dependencies,
    reason = "the package binary delegates to its library, which owns these shared dependencies"
)]

fn main() -> Result<(), runtime_tests::RuntimeTestError> {
    let config = runtime_tests::local_config()?;
    let _report = runtime_tests::run(&config)?;
    Ok(())
}
