use crate::{
    EnvContent, EnvContentRef, EnvKey, InitEntries, InitMaxBytes, InitPathRef, InitializationEntry,
    InitializationStatus, InitializeError, RunMode, WorkspaceMember, WorkspaceRootPathRef,
    environment_keys,
};

#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "provides one testable dry-run and apply entry point; repository policy forbids for loops"
)]
pub(crate) fn initialize(
    root: WorkspaceRootPathRef<'_>,
    mode: RunMode,
) -> Result<InitEntries, InitializeError> {
    let manifest_path = root.as_ref().join(constants_str::CARGO_TOML);
    let manifest = crate::read_bounded_content::read_bounded_content(
        InitPathRef::from(manifest_path.as_path()),
        InitMaxBytes::from(constants_usize::VALUE_1_048_576),
    )
    .map_err(|source| InitializeError::ReadManifest { source })?;
    let value = toml::from_str::<toml::Value>(manifest.as_ref()).map_err(|source| {
        InitializeError::ManifestParse {
            source: source.into(),
        }
    })?;
    let members = value
        .get(constants_str::WORKSPACE)
        .and_then(|workspace| workspace.get(constants_str::MEMBERS))
        .and_then(toml::Value::as_array)
        .ok_or(InitializeError::MembersMissing)?
        .iter()
        .filter_map(toml::Value::as_str)
        .map(|raw_member| {
            let member = WorkspaceMember::try_from(raw_member.to_owned())?;
            let member_path = std::path::Path::new(member.as_ref());
            if !member.as_ref().is_empty()
                && member_path.is_relative()
                && member_path
                    .components()
                    .all(|component| matches!(component, std::path::Component::Normal(_)))
            {
                Ok(member)
            } else {
                Err(InitializeError::InvalidMember { member })
            }
        })
        .collect::<Result<Vec<WorkspaceMember>, InitializeError>>()?;
    members
        .into_iter()
        .try_fold(Vec::new(), |mut entries, member| {
            let example_path = root
                .as_ref()
                .join(member.as_ref())
                .join(constants_str::ENV_EXAMPLE);
            if !bool::from(crate::path_exists::path_exists(InitPathRef::from(
                example_path.as_path(),
            ))) {
                return Ok(entries);
            }
            let content = crate::read_bounded_content::read_bounded_content(
                InitPathRef::from(example_path.as_path()),
                InitMaxBytes::from(constants_usize::VALUE_1_048_576),
            )
            .map_err(|source| InitializeError::ReadExample { source })?;
            let environment_path = root.as_ref().join(member.as_ref()).join(constants_str::ENV);
            let status = if bool::from(crate::path_exists::path_exists(InitPathRef::from(
                environment_path.as_path(),
            ))) {
                let current = crate::read_bounded_content::read_bounded_content(
                    InitPathRef::from(environment_path.as_path()),
                    InitMaxBytes::from(constants_usize::VALUE_1_048_576),
                )
                .map_err(|source| InitializeError::ReadExample { source })?;
                let current_keys = environment_keys(EnvContentRef::from(current.as_ref()))?
                    .0
                    .into_iter()
                    .collect::<std::collections::BTreeSet<EnvKey>>();
                let missing = content
                    .as_ref()
                    .lines()
                    .filter(|line| {
                        line.split_once('=')
                            .is_some_and(|(key, _value)| !current_keys.contains(key.trim()))
                    })
                    .collect::<Vec<&str>>();
                let merged = if missing.is_empty() {
                    None
                } else {
                    let mut merged_text = current.as_ref().to_owned();
                    if !merged_text.is_empty() && !merged_text.ends_with('\n') {
                        merged_text.push('\n');
                    }
                    missing.into_iter().for_each(|line| {
                        merged_text.push_str(line);
                        merged_text.push('\n');
                    });
                    Some(EnvContent::try_from(merged_text)?)
                };
                match merged {
                    None => InitializationStatus::SkippedExisting,
                    Some(_merged) if mode == RunMode::DryRun => InitializationStatus::WouldUpdate,
                    Some(merged_content) => {
                        crate::write_content::write_content(
                            InitPathRef::from(environment_path.as_path()),
                            EnvContentRef::from(merged_content.as_ref()),
                        )?;
                        InitializationStatus::Updated
                    }
                }
            } else if mode == RunMode::DryRun {
                InitializationStatus::WouldCreate
            } else {
                crate::write_content::write_content(
                    InitPathRef::from(environment_path.as_path()),
                    EnvContentRef::from(content.as_ref()),
                )?;
                InitializationStatus::Created
            };
            entries.push(InitializationEntry {
                keys: environment_keys(EnvContentRef::from(content.as_ref()))?,
                member,
                status,
            });
            Ok(entries)
        })
        .map(bounded_types::BoundedVec::from_max_iter)
        .map(InitEntries::from)
}
