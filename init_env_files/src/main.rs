mod domain_types;
mod env_content;
mod env_content_ref;
mod env_key;
mod env_keys;
mod environment_keys;
mod init_entries;
mod init_io_error;
mod init_max_bytes;
mod init_path_exists;
mod init_path_ref;
mod init_string_error;
mod initialization_entry;
mod initialization_status;
mod initialize;
mod initialize_error;
mod path_exists;
mod read_bounded_content;
mod run;
mod run_mode;
mod server_runtime_bounded_read_error;
mod toml_init_error;
mod workspace_member;
mod workspace_root_path_ref;
mod write_content;

fn main() -> Result<(), domain_types::InitializeError> {
    run::run_env_file_initialization()
}
