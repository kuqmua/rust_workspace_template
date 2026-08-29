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
#[cfg(test)]
mod initialize_tests;
mod path_exists;
mod read_bounded_content;
mod run_mode;
mod toml_init_error;
mod workspace_member;
mod workspace_root_path_ref;
mod write_content;

pub(crate) use env_content::EnvContent;
pub(crate) use env_content_ref::EnvContentRef;
pub(crate) use env_key::EnvKey;
pub(crate) use env_keys::EnvKeys;
pub(crate) use environment_keys::environment_keys;
pub(crate) use init_entries::InitEntries;
pub(crate) use init_io_error::InitIoError;
pub(crate) use init_max_bytes::InitMaxBytes;
pub(crate) use init_path_exists::InitPathExists;
pub(crate) use init_path_ref::InitPathRef;
pub(crate) use init_string_error::InitStringError;
pub(crate) use initialization_entry::InitializationEntry;
pub(crate) use initialization_status::InitializationStatus;
pub(crate) use initialize::initialize;
pub(crate) use initialize_error::InitializeError;
pub(crate) use run_mode::RunMode;
pub(crate) use toml_init_error::TomlInitError;
pub(crate) use workspace_member::WorkspaceMember;
pub(crate) use workspace_root_path_ref::WorkspaceRootPathRef;

fn main() -> Result<(), InitializeError> {
    let mode = if std::env::args().any(|argument| argument == constants_str::DRY_RUN) {
        RunMode::DryRun
    } else {
        RunMode::Apply
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(InitializeError::MembersMissing)?;
    initialize(WorkspaceRootPathRef::from(root), mode)?
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
