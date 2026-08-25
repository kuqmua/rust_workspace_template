#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RunMode {
    Apply,
    DryRun,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitializationStatus {
    Created,
    SkippedExisting,
    Updated,
    WouldCreate,
    WouldUpdate,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Eq, PartialEq)]
pub(crate) struct InitializationEntry {
    keys: EnvKeys,
    member: WorkspaceMember,
    status: InitializationStatus,
}
impl InitializationEntry {
    pub(crate) const fn keys(&self) -> &EnvKeys {
        &self.keys
    }
    pub(crate) const fn member(&self) -> &WorkspaceMember {
        &self.member
    }
    pub(crate) const fn status(&self) -> InitializationStatus {
        self.status
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = EnvContent::validate)]
pub(crate) struct EnvContent(String);
impl EnvContent {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    fn validate(value: &str) -> Result<(), InitStringError> {
        if value.len() > usize::try_from(isize::MAX).unwrap_or(usize::MAX) {
            Err(InitStringError)
        } else {
            Ok(())
        }
    }
}
impl From<server_runtime_http::domain_types::BoundedText> for EnvContent {
    fn from(value: server_runtime_http::domain_types::BoundedText) -> Self {
        Self(value.into_inner())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::AsRefStr, newtype::FromInner,
)]
pub(crate) struct EnvContentRef<'content_lt>(&'content_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::BorrowStr,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = EnvKey::validate)]
pub(crate) struct EnvKey(String);
impl EnvKey {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), InitStringError> {
        if value.is_empty() || value.len() > 1_024usize {
            Err(InitStringError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub(crate) struct EnvKeys(
    bounded_types::domain_types::vector::BoundedVec<EnvKey, 0, { usize::MAX }>,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::Display,
    newtype::TryFrom,
)]
#[try_from(error = InitStringError, validator = WorkspaceMember::validate)]
pub(crate) struct WorkspaceMember(String);
impl WorkspaceMember {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), InitStringError> {
        if value.is_empty() || value.len() > 4_096usize {
            Err(InitStringError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub(crate) struct WorkspaceRootPathRef<'root_lt>(&'root_lt std::path::Path);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct InitPathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct InitMaxBytes(usize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct InitPathExists(bool);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoIterator)]
pub(crate) struct InitEntries(
    bounded_types::domain_types::vector::BoundedVec<InitializationEntry, 0, { usize::MAX }>,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct InitIoError(std::io::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerRuntimeBoundedReadError(
    server_runtime_http::domain_types::BoundedReadError,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct TomlInitError(toml::de::Error);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("environment initializer string value is invalid")]
pub(crate) struct InitStringError;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum InitializeError {
    #[error("workspace member path is invalid: {member}")]
    InvalidMember { member: WorkspaceMember },
    #[error("failed to parse workspace manifest")]
    ManifestParse {
        #[source]
        source: TomlInitError,
    },
    #[error("workspace manifest does not contain a members array")]
    MembersMissing,
    #[error("failed to read environment example")]
    ReadExample {
        #[source]
        source: ServerRuntimeBoundedReadError,
    },
    #[error("failed to read workspace manifest")]
    ReadManifest {
        #[source]
        source: ServerRuntimeBoundedReadError,
    },
    #[error(transparent)]
    String(#[from] InitStringError),
    #[error("failed to write environment file")]
    WriteEnvironment {
        #[source]
        source: InitIoError,
    },
}
fn environment_keys(content: EnvContentRef<'_>) -> Result<EnvKeys, InitStringError> {
    content
        .as_ref()
        .lines()
        .filter_map(|source_line| {
            let trimmed_line = source_line.trim();
            (!trimmed_line.is_empty() && !trimmed_line.starts_with('#'))
                .then(|| {
                    trimmed_line
                        .split_once('=')
                        .map(|(key, _value)| EnvKey::try_from(key.trim().to_owned()))
                })
                .flatten()
        })
        .collect::<Result<Vec<EnvKey>, InitStringError>>()
        .map(bounded_types::domain_types::vector::BoundedVec::from_max_iter)
        .map(EnvKeys::from)
}
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
    let manifest = crate::adapters::read_bounded_content(
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
            if !bool::from(crate::adapters::path_exists(InitPathRef::from(
                example_path.as_path(),
            ))) {
                return Ok(entries);
            }
            let content = crate::adapters::read_bounded_content(
                InitPathRef::from(example_path.as_path()),
                InitMaxBytes::from(constants_usize::VALUE_1_048_576),
            )
            .map_err(|source| InitializeError::ReadExample { source })?;
            let environment_path = root.as_ref().join(member.as_ref()).join(constants_str::ENV);
            let status = if bool::from(crate::adapters::path_exists(InitPathRef::from(
                environment_path.as_path(),
            ))) {
                let current = crate::adapters::read_bounded_content(
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
                        crate::adapters::write_content(
                            InitPathRef::from(environment_path.as_path()),
                            EnvContentRef::from(merged_content.as_ref()),
                        )?;
                        InitializationStatus::Updated
                    }
                }
            } else if mode == RunMode::DryRun {
                InitializationStatus::WouldCreate
            } else {
                crate::adapters::write_content(
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
        .map(bounded_types::domain_types::vector::BoundedVec::from_max_iter)
        .map(InitEntries::from)
}
#[cfg(test)]
mod tests {
    fn fixture() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rust-workspace-template-environment-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(root.join(constants_str::SERVICE))
            .expect("fdbf7411 fixture invariant must hold");
        std::fs::write(
            root.join(constants_str::CARGO_TOML),
            constants_str::WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE,
        )
        .expect("8e781c83 fixture invariant must hold");
        std::fs::write(
            root.join(constants_str::SERVICE_ENV_EXAMPLE),
            constants_str::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE,
        )
        .expect("f24fca72 fixture invariant must hold");
        root
    }
    #[test]
    fn dry_run_apply_and_repeat_are_safe_and_idempotent() {
        let root = fixture();
        let dry = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::DryRun,
        )
        .expect("93ce4136 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            dry.0
                .first()
                .expect(
                    "14b080ca dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::WouldCreate
        );
        assert!(!root.join("service/.env").exists());
        let applied = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("d58ed6a5 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            applied
                .0
                .first()
                .expect(
                    "c366cc59 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::Created
        );
        std::fs::write(
            root.join(constants_str::SERVICE_ENV),
            constants_str::SECRET_CUSTOM_NEWLINE,
        )
        .expect("2d67b058 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        let updated = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("546af7b6 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            updated
                .0
                .first()
                .expect(
                    "195600ec dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::Updated
        );
        let updated_content = std::fs::read_to_string(root.join(constants_str::SERVICE_ENV))
            .expect(
                "bd9f5208 dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold",
            );
        assert!(updated_content.contains("SECRET=custom"));
        assert!(updated_content.contains("PUBLIC=value"));
        let repeated = super::initialize(
            super::WorkspaceRootPathRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("a452843a dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold");
        assert_eq!(
            repeated
                .0
                .first()
                .expect(
                    "37a0752c dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold"
                )
                .status,
            super::InitializationStatus::SkippedExisting
        );
        std::fs::remove_dir_all(root).expect(
            "bd9180ca dry_run_apply_and_repeat_are_safe_and_idempotent invariant must hold",
        );
    }
    #[test]
    fn escaping_member_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(constants_str::CARGO_TOML),
            constants_str::WORKSPACE_NEWLINE_MEMBERS_OUTSIDE_NEWLINE,
        )
        .expect("350646f2 escaping_member_is_rejected invariant must hold");
        assert!(matches!(
            super::initialize(
                super::WorkspaceRootPathRef::from(root.as_path()),
                super::RunMode::DryRun
            ),
            Err(super::InitializeError::InvalidMember { .. })
        ));
        std::fs::remove_dir_all(root)
            .expect("d9154402 escaping_member_is_rejected invariant must hold");
    }
    #[test]
    fn oversized_environment_example_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(constants_str::SERVICE_ENV_EXAMPLE),
            constants_str::A_ALT
                .repeat(constants_usize::VALUE_1_048_576.saturating_add(constants_usize::ONE)),
        )
        .expect("f6290e85 oversized_environment_example_is_rejected invariant must hold");
        assert!(matches!(
            super::initialize(
                super::WorkspaceRootPathRef::from(root.as_path()),
                super::RunMode::DryRun
            ),
            Err(super::InitializeError::ReadExample {
                source: super::ServerRuntimeBoundedReadError(
                    server_runtime_http::domain_types::BoundedReadError::ExceedsMaximum { .. }
                )
            })
        ));
        std::fs::remove_dir_all(root)
            .expect("7d83384c oversized_environment_example_is_rejected invariant must hold");
    }
}
