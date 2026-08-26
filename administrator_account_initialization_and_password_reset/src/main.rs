#![allow(
    unused_crate_dependencies,
    reason = "constants_str is used by binary unit tests"
)]

mod application;
mod domain_types;

fn main() -> domain_types::AdministratorAccountCommandExitCode {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            tracing::error!(error = %error, "failed to create initial administrator creation runtime");
            return domain_types::AdministratorAccountCommandExitCode::from(
                std::process::ExitCode::FAILURE,
            );
        }
    };
    match runtime.block_on(application::run()) {
        Ok(user_id) => {
            tracing::info!(user_id = %user_id, "administrator operation completed");
            domain_types::AdministratorAccountCommandExitCode::from(std::process::ExitCode::SUCCESS)
        }
        Err(error) => {
            tracing::error!(error = %error, "administrator operation failed");
            domain_types::AdministratorAccountCommandExitCode::from(std::process::ExitCode::from(
                u8::from(application::error_status(&error)),
            ))
        }
    }
}
