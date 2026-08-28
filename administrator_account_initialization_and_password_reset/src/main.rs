#![allow(
    unused_crate_dependencies,
    reason = "constants_str is used by binary unit tests"
)]
#![allow(clippy::single_call_fn)] // root-owned command parsing and execution stages each have one binary composition owner

mod admin_command;
mod administrator_account_command_error;
mod administrator_account_command_exit_code;
mod administrator_account_command_status;
mod administrator_command_args_error;
mod administrator_password_file_path_buf;
mod domain_types;
mod error_status;
mod initial_administrator_creation_args;
mod parse_args;
mod password_from_bytes;
mod password_from_file;
mod password_reset_args;
mod run;
mod sqlx_administrator_database_connection_error;

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
    match runtime.block_on(run::run_admin_account_command()) {
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
