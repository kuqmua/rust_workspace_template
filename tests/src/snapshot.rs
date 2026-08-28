#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct RsSourceFile {
    ast: super::types::SynFile,
    content: super::types::SourceText,
    path: super::types::OwnedPathBuf,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ProjectSourceFile {
    content: super::types::SourceText,
    path: super::types::OwnedPathBuf,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct CargoTomlSourceFile {
    content: super::types::SourceText,
    parsed: super::types::TomlTable,
    path: super::types::OwnedPathBuf,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct CodebaseSnapshot {
    rs_files: Vec<RsSourceFile>,
    source: std::sync::Arc<CodebaseSourceSnapshot>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct CodebaseSourceSnapshot {
    cargo_toml_by_path:
        std::collections::BTreeMap<super::types::OwnedPathBuf, super::types::CargoTomlFileIdx>,
    cargo_toml_files: Vec<CargoTomlSourceFile>,
    project_source_files: Vec<ProjectSourceFile>,
    workspace_crate_names: super::types::SourceTextBTreeSet,
    workspace_metadata: super::types::CargoMetadata,
}
impl ProjectSourceFile {
    pub(super) fn content(&self) -> &super::types::SourceText {
        &self.content
    }
    pub(super) fn path(&self) -> &super::types::OwnedPathBuf {
        &self.path
    }
}
impl RsSourceFile {
    pub(super) fn ast(&self) -> &super::types::SynFile {
        &self.ast
    }
    pub(super) fn content(&self) -> &super::types::SourceText {
        &self.content
    }
    pub(super) fn path(&self) -> &super::types::OwnedPathBuf {
        &self.path
    }
}
impl CodebaseSnapshot {
    // The owner module retains lint-sensitive semantics from the original implementation.

    #[allow(clippy::single_call_fn)] // named constructor keeps snapshot initialization readable at the thread-local OnceCell call site
    fn build() -> Self {
        static SOURCE_SNAPSHOT: std::sync::OnceLock<std::sync::Arc<CodebaseSourceSnapshot>> =
            std::sync::OnceLock::new();
        let source_snapshot = std::sync::Arc::clone(
            SOURCE_SNAPSHOT.get_or_init(|| std::sync::Arc::new(CodebaseSourceSnapshot::build())),
        );
        let rs_files = source_snapshot
            .project_source_files
            .iter()
            .filter(|source_file| {
                source_file
                    .path
                    .as_ref()
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    == Some(constants_str::RS)
            })
            .map(|source_file| {
                let ast = syn::parse_file(source_file.content.as_ref()).unwrap_or_else(|error| {
                    panic!("5e7a83eb {}: {error}", source_file.path.as_ref().display())
                });
                RsSourceFile {
                    ast: super::types::SynFile::from(ast),
                    content: source_file.content.clone(),
                    path: source_file.path.clone(),
                }
            })
            .collect();
        Self {
            rs_files,
            source: source_snapshot,
        }
    }
    pub(super) fn cargo_toml_content(
        &self,
        path: super::types::PathRef<'_>,
    ) -> Option<super::types::SourceText> {
        self.source
            .cargo_toml_file(path)
            .map(|cargo_toml| cargo_toml.content.clone())
    }
    pub(super) fn crate_manifest_paths(&self) -> impl Iterator<Item = &std::path::Path> {
        self.source
            .cargo_toml_files
            .iter()
            .map(|cargo_toml| cargo_toml.path.as_ref())
    }
    pub(super) fn project_source_files(&self) -> &[ProjectSourceFile] {
        self.source.project_source_files.as_slice()
    }
    pub(super) fn read_toml_table(
        &self,
        path: super::types::PathRef<'_>,
    ) -> Option<super::types::TomlTable> {
        self.source
            .cargo_toml_file(path)
            .map(|cargo_toml| cargo_toml.parsed.clone())
            .or_else(|| {
                path.as_ref().exists().then(|| {
                    let value = std::fs::read_to_string(path.as_ref()).unwrap_or_else(|error| {
                        panic!(
                            "e12179c5 failed to read {}: {error}",
                            path.as_ref().display()
                        )
                    });
                    value.parse::<toml::Table>().map_or_else(
                        |error| {
                            panic!(
                                "77b2d82b failed to parse {}: {error}",
                                path.as_ref().display()
                            )
                        },
                        super::types::TomlTable::from,
                    )
                })
            })
    }
    pub(super) fn rs_files(&self) -> &[RsSourceFile] {
        &self.rs_files
    }
    // The snapshot exposes this derived workspace-name set through one policy consumer.

    #[allow(clippy::single_call_fn)] // the snapshot exposes this derived workspace-name set through one policy consumer
    pub(super) fn workspace_crate_names(&self) -> super::types::SourceTextBTreeSet {
        self.source.workspace_crate_names.clone()
    }
    pub(super) fn workspace_metadata(&self) -> super::types::CargoMetadataRef<'_> {
        super::types::CargoMetadataRef::from(self.source.workspace_metadata.as_ref())
    }
}
impl CodebaseSourceSnapshot {
    #[allow(clippy::single_call_fn)] // named constructor keeps process-wide immutable source initialization readable
    fn build() -> Self {
        let metadata = workspace_metadata_uncached();
        let workspace_members = super::types::CargoPackageIdRefHashSet::from(
            metadata
                .as_ref()
                .workspace_members
                .iter()
                .collect::<std::collections::HashSet<&cargo_metadata::PackageId>>(),
        );
        let workspace_crate_names: std::collections::BTreeSet<String> = metadata
            .as_ref()
            .packages
            .iter()
            .filter(|package| workspace_members.as_ref().contains(&package.id))
            .map(|package| package.name.to_string())
            .collect();
        let cargo_toml_files: Vec<CargoTomlSourceFile> = metadata
            .as_ref()
            .packages
            .iter()
            .filter(|package| workspace_members.as_ref().contains(&package.id))
            .map(|package| {
                let path = package.manifest_path.as_std_path().to_path_buf();
                let content = std::fs::read_to_string(&path).unwrap_or_else(|error| {
                    panic!("50da433e failed to read {}: {error}", path.display())
                });
                let parsed = content.parse::<toml::Table>().unwrap_or_else(|error| {
                    panic!("96f2c78a failed to parse {}: {error}", path.display())
                });
                CargoTomlSourceFile {
                    content: super::types::SourceText::try_from(content)
                        .expect("84f6a0d2 build invariant must hold"),
                    parsed: super::types::TomlTable::from(parsed),
                    path: super::types::OwnedPathBuf::from(path),
                }
            })
            .collect();
        let cargo_toml_by_path =
            cargo_toml_files
                .iter()
                .enumerate()
                .map(|(idx, cargo_toml)| {
                    (
                        cargo_toml.path.clone(),
                        super::types::CargoTomlFileIdx::from(idx),
                    )
                })
                .collect::<std::collections::BTreeMap<
                    super::types::OwnedPathBuf,
                    super::types::CargoTomlFileIdx,
                >>();
        let project_source_files =
            super::types::WalkdirWalkDir::from(walkdir::WalkDir::new(constants_str::TEXT_ALT_9))
                .into_iter()
                .filter_entry(|element| {
                    element.file_name() != constants_str::TARGET
                        && element.file_name() != constants_str::GIT
                        && element.file_name() != constants_str::WORKSPACE_SCAFFOLD_NODE_MODULES
                        && (element.file_type().is_dir()
                            || matches!(
                                element.path().extension().and_then(std::ffi::OsStr::to_str),
                                Some(
                                    constants_str::RS
                                        | constants_str::MD
                                        | constants_str::TOML
                                        | constants_str::TXT
                                        | constants_str::YML
                                        | constants_str::YAML
                                        | constants_str::JSON
                                )
                            ))
                })
                .map(project_walk_entry)
                .filter(|entry| !entry.file_type().is_dir())
                .map(|entry| project_source_file(entry.into_path()))
                .collect::<Vec<_>>();
        Self {
            cargo_toml_by_path,
            cargo_toml_files,
            project_source_files,
            workspace_metadata: metadata,
            workspace_crate_names: super::types::SourceTextBTreeSet::from(workspace_crate_names),
        }
    }
    fn cargo_toml_file(&self, path: super::types::PathRef<'_>) -> Option<&CargoTomlSourceFile> {
        self.cargo_toml_by_path
            .get(path.as_ref())
            .and_then(|idx| self.cargo_toml_files.get(idx.get()))
    }
}
pub(super) fn with_codebase_snapshot<R>(f: impl FnOnce(&CodebaseSnapshot) -> R) -> R {
    std::thread_local! {
        static SNAPSHOT: std::cell::OnceCell<CodebaseSnapshot> = const { std::cell::OnceCell::new() };
    }
    SNAPSHOT.with(|snapshot| f(snapshot.get_or_init(CodebaseSnapshot::build)))
}

#[allow(clippy::single_call_fn)] // isolates cargo_metadata command setup from snapshot construction
fn workspace_metadata_uncached() -> super::types::CargoMetadata {
    super::types::CargoMetadata::from(
        cargo_metadata::MetadataCommand::new()
            .manifest_path(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
            .exec()
            .expect("c84e9d1f workspace_metadata_uncached invariant must hold"),
    )
}
fn project_walk_entry(entry: walkdir::Result<walkdir::DirEntry>) -> walkdir::DirEntry {
    entry.unwrap_or_else(|error| panic!("1e4b17b0 walk failed: {error}"))
}
fn project_source_file(path: std::path::PathBuf) -> ProjectSourceFile {
    let raw_content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("68a041c3 failed to read {}: {error}", path.display()));
    let content = project_source_content(path.as_path(), raw_content);
    ProjectSourceFile {
        content,
        path: super::types::OwnedPathBuf::from(path),
    }
}
fn project_source_content(path: &std::path::Path, raw_content: String) -> super::types::SourceText {
    super::types::SourceText::try_from(raw_content)
        .unwrap_or_else(|error| panic!("e27f9e15 invalid source {}: {error}", path.display()))
}
#[test]
fn invalid_project_source_content_fails_snapshot_loading() {
    let oversized = constants_str::X.repeat(16_777_217usize);
    assert!(
        std::panic::catch_unwind(|| {
            project_source_content(std::path::Path::new("oversized.rs"), oversized)
        })
        .is_err(),
        "28fb322e"
    );
}
#[test]
fn missing_project_source_file_fails_snapshot_loading() {
    let missing = std::path::PathBuf::from(constants_str::VALUE_5E88EEB9);
    assert!(
        std::panic::catch_unwind(|| project_source_file(missing)).is_err(),
        "46045b88"
    );
}
#[test]
fn walk_error_fails_snapshot_loading() {
    assert!(
        std::panic::catch_unwind(|| {
            let missing = walkdir::WalkDir::new("code_style_snapshot_missing_directory")
                .into_iter()
                .next()
                .expect("1da2f4ed walk_error_fails_snapshot_loading invariant must hold");
            project_walk_entry(missing)
        })
        .is_err(),
        "6a6e2aac"
    );
}
