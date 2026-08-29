mod domain_types;
#[cfg(test)]
mod domain_types_tests;
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
mod run_mode;
mod toml_init_error;
mod workspace_member;
mod workspace_root_path_ref;
mod write_content;

fn main() -> Result<(), domain_types::InitializeError> {
    let mode = if std::env::args().any(|argument| argument == constants_str::DRY_RUN) {
        domain_types::RunMode::DryRun
    } else {
        domain_types::RunMode::Apply
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(domain_types::InitializeError::MembersMissing)?;
    domain_types::initialize(domain_types::WorkspaceRootPathRef::from(root), mode)?
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
                "environment file initialization completed"
            );
        });
    Ok(())
}
