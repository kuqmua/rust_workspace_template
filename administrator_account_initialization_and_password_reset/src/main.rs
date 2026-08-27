#![allow(
    unused_crate_dependencies,
    reason = "constants_str is used by binary unit tests"
)]

mod domain_types;
mod error_status;
mod parse_args;
mod password_from_bytes;
mod password_from_file;
mod run;

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
    match runtime.block_on(run::run()) {
        Ok(user_id) => {
            tracing::info!(user_id = %user_id, "administrator operation completed");
            domain_types::AdministratorAccountCommandExitCode::from(std::process::ExitCode::SUCCESS)
        }
        Err(error) => {
            tracing::error!(error = %error, "administrator operation failed");
            domain_types::AdministratorAccountCommandExitCode::from(std::process::ExitCode::from(
                u8::from(error_status::error_status(&error)),
            ))
        }
    }
}
