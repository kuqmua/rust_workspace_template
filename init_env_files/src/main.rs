pub mod env_content;
pub mod env_content_ref;
pub mod env_key;
pub mod env_keys;
pub mod environment_keys;
pub mod init_entries;
pub mod init_io_error;
pub mod init_max_bytes;
pub mod init_path_exists;
pub mod init_path_ref;
pub mod init_string_error;
pub mod initialization_entry;
pub mod initialization_status;
pub mod initialize;
pub mod initialize_error;
#[cfg(test)]
pub mod initialize_tests;
pub mod path_exists;
pub mod read_bounded_content;
pub mod run_mode;
pub mod toml_init_error;
pub mod workspace_member;
pub mod workspace_root_path_ref;
pub mod write_content;

fn main() -> Result<(), initialize_error::InitializeError> {
    let mode = if std::env::args().any(|argument| argument == constants_str::DRY_RUN) {
        run_mode::RunMode::DryRun
    } else {
        run_mode::RunMode::Apply
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(initialize_error::InitializeError::MembersMissing)?;
    initialize::initialize(
        workspace_root_path_ref::WorkspaceRootPathRef::from(root),
        mode,
    )?
    .into_iter()
    .for_each(|entry| {
        let separator = constants_str::COMMA_SPACE.trim();
        let keys_capacity = entry
            .keys()
            .as_ref()
            .iter()
            .map(|key| key.as_ref().len())
            .sum::<usize>()
            .saturating_add(
                entry
                    .keys()
                    .as_ref()
                    .len()
                    .get()
                    .saturating_sub(constants_usize::ONE)
                    .saturating_mul(separator.len()),
            );
        let keys = entry.keys().as_ref().iter().enumerate().fold(
            String::with_capacity(keys_capacity),
            |mut keys, (index, key)| {
                if index > constants_usize::ZERO {
                    keys.push_str(separator);
                }
                keys.push_str(key.as_ref());
                keys
            },
        );
        tracing::info!(
            member = entry.member().as_ref(),
            status = ?entry.status(),
            keys,
            message = %constants_str::TRACING_ENV_FILE_INITIALIZATION_COMPLETED,
        );
    });
    Ok(())
}
