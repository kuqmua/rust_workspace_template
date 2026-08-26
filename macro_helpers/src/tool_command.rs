#[path = "tool_command_os_string_value.rs"]
mod os_string_value;
#[path = "tool_command_path_ref.rs"]
mod path_ref;
#[path = "tool_command_process_command.rs"]
mod process_command;
#[path = "tool_command_process_exit_status.rs"]
mod process_exit_status;
#[path = "tool_command_process_output.rs"]
mod process_output;
#[path = "tool_command_tool_arg_ref.rs"]
mod tool_arg_ref;
#[path = "tool_command_tool_args_ref.rs"]
mod tool_args_ref;
#[path = "tool_command_tool_command.rs"]
#[allow(
    clippy::module_inception,
    reason = "the compatibility facade retains its public path while the same-named owner receives a dedicated module"
)]
mod tool_command;
#[path = "tool_command_tool_env_key_ref.rs"]
mod tool_env_key_ref;
#[path = "tool_command_tool_env_value_ref.rs"]
mod tool_env_value_ref;
#[path = "tool_command_tool_program_ref.rs"]
mod tool_program_ref;

pub use path_ref::PathRef;
pub use process_exit_status::ProcessExitStatus;
pub use process_output::ProcessOutput;
pub use tool_arg_ref::ToolArgRef;
pub use tool_args_ref::ToolArgsRef;
pub use tool_command::ToolCommand;
pub use tool_env_key_ref::ToolEnvKeyRef;
pub use tool_env_value_ref::ToolEnvValueRef;
pub use tool_program_ref::ToolProgramRef;
