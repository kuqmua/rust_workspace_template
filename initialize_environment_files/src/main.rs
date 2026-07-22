const ENV_FILE_MAX_BYTES: usize = 1_048_576usize;
const WORKSPACE_MANIFEST_MAX_BYTES: usize = 1_048_576usize;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    Apply,
    DryRun,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InitializationStatus {
    Created,
    SkippedExisting,
    Updated,
    WouldCreate,
    WouldUpdate,
}
#[derive(Debug, Eq, PartialEq)]
struct InitializationEntry {
    keys: EnvKeys,
    member: WorkspaceMember,
    status: InitializationStatus,
}
#[derive(Debug, Eq, PartialEq)]
struct EnvContent(String);
impl TryFrom<String> for EnvContent {
    type Error = InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > usize::try_from(isize::MAX).unwrap_or(usize::MAX) {
            Err(InitStringError)
        } else {
            Ok(Self(value))
        }
    }
}
impl From<server_runtime::BoundedText> for EnvContent {
    fn from(value: server_runtime::BoundedText) -> Self {
        Self(value.into_inner())
    }
}
impl AsRef<str> for EnvContent {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(Clone, Copy)]
struct EnvContentRef<'content_lt>(&'content_lt str);
impl<'content_lt> From<&'content_lt str> for EnvContentRef<'content_lt> {
    fn from(value: &'content_lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for EnvContentRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EnvKey(String);
impl TryFrom<String> for EnvKey {
    type Error = InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 1_024usize {
            Err(InitStringError)
        } else {
            Ok(Self(value))
        }
    }
}
impl AsRef<str> for EnvKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl std::borrow::Borrow<str> for EnvKey {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(Debug, Eq, PartialEq)]
struct EnvKeys(Vec<EnvKey>);
impl From<Vec<EnvKey>> for EnvKeys {
    fn from(value: Vec<EnvKey>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy)]
struct MemberSafe(bool);
impl From<bool> for MemberSafe {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl From<MemberSafe> for bool {
    fn from(value: MemberSafe) -> Self {
        value.0
    }
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorkspaceMember(String);
impl TryFrom<String> for WorkspaceMember {
    type Error = InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 4_096usize {
            Err(InitStringError)
        } else {
            Ok(Self(value))
        }
    }
}
impl AsRef<str> for WorkspaceMember {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl std::fmt::Display for WorkspaceMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Copy)]
struct WorkspaceMemberRef<'member_lt>(&'member_lt str);
impl<'member_lt> From<&'member_lt str> for WorkspaceMemberRef<'member_lt> {
    fn from(value: &'member_lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for WorkspaceMemberRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
struct WorkspaceMembers(Vec<WorkspaceMember>);
impl From<Vec<WorkspaceMember>> for WorkspaceMembers {
    fn from(value: Vec<WorkspaceMember>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy)]
struct StdWorkspaceRootRef<'root_lt>(&'root_lt std::path::Path);
impl<'root_lt> From<&'root_lt std::path::Path> for StdWorkspaceRootRef<'root_lt> {
    fn from(value: &'root_lt std::path::Path) -> Self {
        Self(value)
    }
}
impl AsRef<std::path::Path> for StdWorkspaceRootRef<'_> {
    fn as_ref(&self) -> &std::path::Path {
        self.0
    }
}
#[derive(Clone, Copy)]
struct StdInitPathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> From<&'path_lt std::path::Path> for StdInitPathRef<'path_lt> {
    fn from(value: &'path_lt std::path::Path) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy)]
struct InitMaxBytes(usize);
impl From<usize> for InitMaxBytes {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
struct InitEntries(Vec<InitializationEntry>);
impl From<Vec<InitializationEntry>> for InitEntries {
    fn from(value: Vec<InitializationEntry>) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct StdInitIoError(std::io::Error);
impl From<std::io::Error> for StdInitIoError {
    fn from(value: std::io::Error) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for StdInitIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdInitIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
struct ServerRuntimeBoundedReadError(server_runtime::BoundedReadError);
impl From<server_runtime::BoundedReadError> for ServerRuntimeBoundedReadError {
    fn from(value: server_runtime::BoundedReadError) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for ServerRuntimeBoundedReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ServerRuntimeBoundedReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
struct TomlInitError(toml::de::Error);
impl From<toml::de::Error> for TomlInitError {
    fn from(value: toml::de::Error) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for TomlInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for TomlInitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug, thiserror::Error)]
#[error("environment initializer string value is invalid")]
struct InitStringError;
#[derive(Debug, thiserror::Error)]
enum InitializeError {
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
        source: StdInitIoError,
    },
}
fn read_bounded_content(
    path: StdInitPathRef<'_>,
    maximum_bytes: InitMaxBytes,
) -> Result<EnvContent, ServerRuntimeBoundedReadError> {
    let bytes = server_runtime::read_bounded_file(
        server_runtime::StdPathRef::from(path.0),
        server_runtime::BoundedReadMaximumBytes::from(maximum_bytes.0),
    )
    .map_err(ServerRuntimeBoundedReadError::from)?;
    server_runtime::BoundedText::try_from(bytes)
        .map(EnvContent::from)
        .map_err(ServerRuntimeBoundedReadError::from)
}
#[allow(
    clippy::single_call_fn,
    reason = "keeps lexical path validation independently testable and reviewable"
)]
fn member_is_safe(member: WorkspaceMemberRef<'_>) -> MemberSafe {
    MemberSafe::from(
        !member.as_ref().is_empty()
            && std::path::Path::new(member.as_ref()).is_relative()
            && std::path::Path::new(member.as_ref())
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_))),
    )
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
        .map(EnvKeys::from)
}
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "isolates the testable merge algorithm and repository policy forbids for loops"
)]
fn merge_missing_assignments(
    current: EnvContentRef<'_>,
    example: EnvContentRef<'_>,
) -> Result<Option<EnvContent>, InitStringError> {
    let current_keys = environment_keys(current)?
        .0
        .into_iter()
        .collect::<std::collections::BTreeSet<EnvKey>>();
    let missing = example
        .as_ref()
        .lines()
        .filter(|line| {
            line.split_once('=')
                .is_some_and(|(key, _value)| !current_keys.contains(key.trim()))
        })
        .collect::<Vec<&str>>();
    if missing.is_empty() {
        return Ok(None);
    }
    let mut merged = current.as_ref().to_owned();
    if !merged.is_empty() && !merged.ends_with('\n') {
        merged.push('\n');
    }
    missing.into_iter().for_each(|line| {
        merged.push_str(line);
        merged.push('\n');
    });
    EnvContent::try_from(merged).map(Some)
}
#[allow(
    clippy::single_call_fn,
    reason = "separates manifest validation from filesystem mutation"
)]
fn workspace_members(root: StdWorkspaceRootRef<'_>) -> Result<WorkspaceMembers, InitializeError> {
    let manifest_path = root.as_ref().join(str_constants::CARGO_TOML);
    let manifest = read_bounded_content(
        StdInitPathRef::from(manifest_path.as_path()),
        InitMaxBytes::from(WORKSPACE_MANIFEST_MAX_BYTES),
    )
    .map_err(|source| InitializeError::ReadManifest { source })?;
    let value = toml::from_str::<toml::Value>(manifest.as_ref()).map_err(|source| {
        InitializeError::ManifestParse {
            source: source.into(),
        }
    })?;
    let members = value
        .get(str_constants::WORKSPACE)
        .and_then(|workspace| workspace.get(str_constants::MEMBERS))
        .and_then(toml::Value::as_array)
        .ok_or(InitializeError::MembersMissing)?;
    members
        .iter()
        .filter_map(toml::Value::as_str)
        .map(|raw_member| {
            let member = WorkspaceMember::try_from(raw_member.to_owned())?;
            if bool::from(member_is_safe(WorkspaceMemberRef::from(member.as_ref()))) {
                Ok(member)
            } else {
                Err(InitializeError::InvalidMember { member })
            }
        })
        .collect::<Result<Vec<WorkspaceMember>, InitializeError>>()
        .map(WorkspaceMembers::from)
}
#[allow(
    clippy::single_call_fn,
    reason = "provides one testable dry-run and apply entry point"
)]
fn initialize(
    root: StdWorkspaceRootRef<'_>,
    mode: RunMode,
) -> Result<InitEntries, InitializeError> {
    workspace_members(root)?
        .0
        .into_iter()
        .try_fold(Vec::new(), |mut entries, member| {
            let example_path = root
                .as_ref()
                .join(member.as_ref())
                .join(str_constants::ENV_EXAMPLE);
            if !example_path.exists() {
                return Ok(entries);
            }
            let content = read_bounded_content(
                StdInitPathRef::from(example_path.as_path()),
                InitMaxBytes::from(ENV_FILE_MAX_BYTES),
            )
            .map_err(|source| InitializeError::ReadExample { source })?;
            let environment_path = root.as_ref().join(member.as_ref()).join(str_constants::ENV);
            let status = if environment_path.exists() {
                let current = read_bounded_content(
                    StdInitPathRef::from(environment_path.as_path()),
                    InitMaxBytes::from(ENV_FILE_MAX_BYTES),
                )
                .map_err(|source| InitializeError::ReadExample { source })?;
                match merge_missing_assignments(
                    EnvContentRef::from(current.as_ref()),
                    EnvContentRef::from(content.as_ref()),
                )? {
                    None => InitializationStatus::SkippedExisting,
                    Some(_merged) if mode == RunMode::DryRun => InitializationStatus::WouldUpdate,
                    Some(merged) => {
                        std::fs::write(environment_path, merged.as_ref().as_bytes()).map_err(
                            |source| InitializeError::WriteEnvironment {
                                source: source.into(),
                            },
                        )?;
                        InitializationStatus::Updated
                    }
                }
            } else if mode == RunMode::DryRun {
                InitializationStatus::WouldCreate
            } else {
                std::fs::write(environment_path, content.as_ref().as_bytes()).map_err(
                    |source| InitializeError::WriteEnvironment {
                        source: source.into(),
                    },
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
        .map(InitEntries::from)
}
fn main() -> Result<(), InitializeError> {
    let mode = if std::env::args().any(|argument| argument == str_constants::DRY_RUN) {
        RunMode::DryRun
    } else {
        RunMode::Apply
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(InitializeError::MembersMissing)?;
    initialize(StdWorkspaceRootRef::from(root), mode)?
        .0
        .into_iter()
        .for_each(|entry| {
            println!(
                "member={} status={:?} keys={}",
                entry.member.as_ref(),
                entry.status,
                entry
                    .keys
                    .0
                    .iter()
                    .map(EnvKey::as_ref)
                    .collect::<Vec<&str>>()
                    .join(",")
            );
        });
    Ok(())
}
#[cfg(test)]
mod tests {
    fn fixture() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rust-workspace-template-environment-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(root.join(str_constants::SERVICE)).expect("fdbf7411");
        std::fs::write(
            root.join(str_constants::CARGO_TOML),
            str_constants::WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE,
        )
        .expect("8e781c83");
        std::fs::write(
            root.join(str_constants::SERVICE_ENV_EXAMPLE),
            str_constants::PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE,
        )
        .expect("f24fca72");
        root
    }
    #[test]
    fn dry_run_apply_and_repeat_are_safe_and_idempotent() {
        let root = fixture();
        let dry = super::initialize(
            super::StdWorkspaceRootRef::from(root.as_path()),
            super::RunMode::DryRun,
        )
        .expect("93ce4136");
        assert_eq!(
            dry.0.first().expect("14b080ca").status,
            super::InitializationStatus::WouldCreate
        );
        assert!(!root.join("service/.env").exists());
        let applied = super::initialize(
            super::StdWorkspaceRootRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("d58ed6a5");
        assert_eq!(
            applied.0.first().expect("c366cc59").status,
            super::InitializationStatus::Created
        );
        std::fs::write(
            root.join(str_constants::SERVICE_ENV),
            str_constants::SECRET_CUSTOM_NEWLINE,
        )
        .expect("2d67b058");
        let updated = super::initialize(
            super::StdWorkspaceRootRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("546af7b6");
        assert_eq!(
            updated.0.first().expect("195600ec").status,
            super::InitializationStatus::Updated
        );
        let updated_content =
            std::fs::read_to_string(root.join(str_constants::SERVICE_ENV)).expect("bd9f5208");
        assert!(updated_content.contains("SECRET=custom"));
        assert!(updated_content.contains("PUBLIC=value"));
        let repeated = super::initialize(
            super::StdWorkspaceRootRef::from(root.as_path()),
            super::RunMode::Apply,
        )
        .expect("a452843a");
        assert_eq!(
            repeated.0.first().expect("37a0752c").status,
            super::InitializationStatus::SkippedExisting
        );
        std::fs::remove_dir_all(root).expect("bd9180ca");
    }
    #[test]
    fn escaping_member_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(str_constants::CARGO_TOML),
            str_constants::WORKSPACE_NEWLINE_MEMBERS_OUTSIDE_NEWLINE,
        )
        .expect("350646f2");
        assert!(matches!(
            super::initialize(
                super::StdWorkspaceRootRef::from(root.as_path()),
                super::RunMode::DryRun
            ),
            Err(super::InitializeError::InvalidMember { .. })
        ));
        std::fs::remove_dir_all(root).expect("d9154402");
    }
    #[test]
    fn oversized_environment_example_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(str_constants::SERVICE_ENV_EXAMPLE),
            str_constants::A_ALT.repeat(super::ENV_FILE_MAX_BYTES.saturating_add(1usize)),
        )
        .expect("f6290e85");
        assert!(matches!(
            super::initialize(
                super::StdWorkspaceRootRef::from(root.as_path()),
                super::RunMode::DryRun
            ),
            Err(super::InitializeError::ReadExample {
                source: super::ServerRuntimeBoundedReadError(
                    server_runtime::BoundedReadError::ExceedsMaximum { .. }
                )
            })
        ));
        std::fs::remove_dir_all(root).expect("7d83384c");
    }
}
