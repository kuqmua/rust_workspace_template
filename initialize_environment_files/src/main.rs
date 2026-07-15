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
    keys: Vec<String>,
    member: String,
    status: InitializationStatus,
}
#[derive(Debug, thiserror::Error)]
enum InitializeError {
    #[error("workspace member path is invalid: {member}")]
    InvalidMember { member: String },
    #[error("failed to parse workspace manifest")]
    ManifestParse {
        #[source]
        source: toml::de::Error,
    },
    #[error("workspace manifest does not contain a members array")]
    MembersMissing,
    #[error("failed to read environment example")]
    ReadExample {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read workspace manifest")]
    ReadManifest {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write environment file")]
    WriteEnvironment {
        #[source]
        source: std::io::Error,
    },
}
#[allow(
    clippy::single_call_fn,
    reason = "keeps lexical path validation independently testable and reviewable"
)]
fn member_is_safe(member: &str) -> bool {
    !member.is_empty()
        && std::path::Path::new(member).is_relative()
        && std::path::Path::new(member)
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}
fn environment_keys(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|source_line| {
            let trimmed_line = source_line.trim();
            (!trimmed_line.is_empty() && !trimmed_line.starts_with('#'))
                .then(|| {
                    trimmed_line
                        .split_once('=')
                        .map(|(key, _value)| key.trim().to_owned())
                })
                .flatten()
        })
        .collect()
}
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "isolates the testable merge algorithm and repository policy forbids for loops"
)]
fn merge_missing_assignments(current: &str, example: &str) -> Option<String> {
    let current_keys = environment_keys(current)
        .into_iter()
        .collect::<std::collections::BTreeSet<String>>();
    let missing = example
        .lines()
        .filter(|line| {
            line.split_once('=')
                .is_some_and(|(key, _value)| !current_keys.contains(key.trim()))
        })
        .collect::<Vec<&str>>();
    if missing.is_empty() {
        return None;
    }
    let mut merged = current.to_owned();
    if !merged.is_empty() && !merged.ends_with('\n') {
        merged.push('\n');
    }
    missing.into_iter().for_each(|line| {
        merged.push_str(line);
        merged.push('\n');
    });
    Some(merged)
}
#[allow(
    clippy::single_call_fn,
    reason = "separates manifest validation from filesystem mutation"
)]
fn workspace_members(root: &std::path::Path) -> Result<Vec<String>, InitializeError> {
    let manifest = std::fs::read_to_string(root.join(str_constants::expr::S_0640))
        .map_err(|source| InitializeError::ReadManifest { source })?;
    let value = toml::from_str::<toml::Value>(&manifest)
        .map_err(|source| InitializeError::ManifestParse { source })?;
    let members = value
        .get(str_constants::expr::S_1913)
        .and_then(|workspace| workspace.get(str_constants::expr::S_1514))
        .and_then(toml::Value::as_array)
        .ok_or(InitializeError::MembersMissing)?;
    members
        .iter()
        .filter_map(toml::Value::as_str)
        .map(|member| {
            if member_is_safe(member) {
                Ok(member.to_owned())
            } else {
                Err(InitializeError::InvalidMember {
                    member: member.to_owned(),
                })
            }
        })
        .collect()
}
#[allow(
    clippy::single_call_fn,
    reason = "provides one testable dry-run and apply entry point"
)]
fn initialize(
    root: &std::path::Path,
    mode: RunMode,
) -> Result<Vec<InitializationEntry>, InitializeError> {
    workspace_members(root)?.into_iter().try_fold(
        Vec::new(),
        |mut entries, member| -> Result<Vec<InitializationEntry>, InitializeError> {
            let example_path = root.join(member.as_str()).join(str_constants::expr::S_0076);
            if !example_path.exists() {
                return Ok(entries);
            }
            let content = std::fs::read_to_string(example_path)
                .map_err(|source| InitializeError::ReadExample { source })?;
            let environment_path = root.join(member.as_str()).join(str_constants::expr::S_0075);
            let status = if environment_path.exists() {
                let current = std::fs::read_to_string(environment_path.as_path())
                    .map_err(|source| InitializeError::ReadExample { source })?;
                match merge_missing_assignments(current.as_str(), content.as_str()) {
                    None => InitializationStatus::SkippedExisting,
                    Some(_merged) if mode == RunMode::DryRun => InitializationStatus::WouldUpdate,
                    Some(merged) => {
                        std::fs::write(environment_path, merged.as_bytes())
                            .map_err(|source| InitializeError::WriteEnvironment { source })?;
                        InitializationStatus::Updated
                    }
                }
            } else if mode == RunMode::DryRun {
                InitializationStatus::WouldCreate
            } else {
                std::fs::write(environment_path, content.as_bytes())
                    .map_err(|source| InitializeError::WriteEnvironment { source })?;
                InitializationStatus::Created
            };
            entries.push(InitializationEntry {
                keys: environment_keys(content.as_str()),
                member,
                status,
            });
            Ok(entries)
        },
    )
}
fn main() -> Result<(), InitializeError> {
    let mode = if std::env::args().any(|argument| argument == str_constants::expr::S_0049) {
        RunMode::DryRun
    } else {
        RunMode::Apply
    };
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(InitializeError::MembersMissing)?;
    initialize(root, mode)?.into_iter().for_each(|entry| {
        println!(
            "member={} status={:?} keys={}",
            entry.member,
            entry.status,
            entry.keys.join(",")
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
        std::fs::create_dir_all(root.join(str_constants::expr::S_1732)).expect("fdbf7411");
        std::fs::write(
            root.join(str_constants::expr::S_0640),
            str_constants::expr::S_0847,
        )
        .expect("8e781c83");
        std::fs::write(
            root.join(str_constants::expr::S_1734),
            str_constants::expr::S_0723,
        )
        .expect("f24fca72");
        root
    }
    #[test]
    fn dry_run_apply_and_repeat_are_safe_and_idempotent() {
        let root = fixture();
        let dry = super::initialize(root.as_path(), super::RunMode::DryRun).expect("93ce4136");
        assert_eq!(
            dry.first().expect("14b080ca").status,
            super::InitializationStatus::WouldCreate
        );
        assert!(!root.join("service/.env").exists());
        let applied = super::initialize(root.as_path(), super::RunMode::Apply).expect("d58ed6a5");
        assert_eq!(
            applied.first().expect("c366cc59").status,
            super::InitializationStatus::Created
        );
        std::fs::write(
            root.join(str_constants::expr::S_1733),
            str_constants::expr::S_0741,
        )
        .expect("2d67b058");
        let updated = super::initialize(root.as_path(), super::RunMode::Apply).expect("546af7b6");
        assert_eq!(
            updated.first().expect("195600ec").status,
            super::InitializationStatus::Updated
        );
        let updated_content =
            std::fs::read_to_string(root.join(str_constants::expr::S_1733)).expect("bd9f5208");
        assert!(updated_content.contains("SECRET=custom"));
        assert!(updated_content.contains("PUBLIC=value"));
        let repeated = super::initialize(root.as_path(), super::RunMode::Apply).expect("a452843a");
        assert_eq!(
            repeated.first().expect("37a0752c").status,
            super::InitializationStatus::SkippedExisting
        );
        std::fs::remove_dir_all(root).expect("bd9180ca");
    }
    #[test]
    fn escaping_member_is_rejected() {
        let root = fixture();
        std::fs::write(
            root.join(str_constants::expr::S_0640),
            str_constants::expr::S_0846,
        )
        .expect("350646f2");
        assert!(matches!(
            super::initialize(root.as_path(), super::RunMode::DryRun),
            Err(super::InitializeError::InvalidMember { .. })
        ));
        std::fs::remove_dir_all(root).expect("d9154402");
    }
}
