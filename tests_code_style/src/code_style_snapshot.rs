#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct RsSourceFile {
    ast: crate::types::SynFile,
    content: crate::types::SourceText,
    path: crate::types::OwnedPathBuf,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ProjectSourceFile {
    content: crate::types::SourceText,
    path: crate::types::OwnedPathBuf,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct CargoTomlSourceFile {
    content: crate::types::SourceText,
    parsed: crate::types::TomlTable,
    path: crate::types::OwnedPathBuf,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct CodebaseSnapshot {
    rs_files: Vec<RsSourceFile>,
    source: std::sync::Arc<CodebaseSourceSnapshot>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct CodebaseSourceSnapshot {
    cargo_toml_by_path:
        std::collections::BTreeMap<crate::types::OwnedPathBuf, crate::types::CargoTomlFileIdx>,
    cargo_toml_files: Vec<CargoTomlSourceFile>,
    project_source_files: Vec<ProjectSourceFile>,
    workspace_crate_names: crate::types::SourceTextBTreeSet,
    workspace_metadata: crate::types::CargoMetadata,
}
impl ProjectSourceFile {
    pub(super) fn content(&self) -> &crate::types::SourceText {
        &self.content
    }
    pub(super) fn path(&self) -> &crate::types::OwnedPathBuf {
        &self.path
    }
}
impl RsSourceFile {
    pub(super) fn ast(&self) -> &crate::types::SynFile {
        &self.ast
    }
    pub(super) fn content(&self) -> &crate::types::SourceText {
        &self.content
    }
    pub(super) fn path(&self) -> &crate::types::OwnedPathBuf {
        &self.path
    }
}
impl CodebaseSnapshot {
    pub(super) fn cargo_toml_content(
        &self,
        path: crate::types::PathRef<'_>,
    ) -> Option<crate::types::SourceText> {
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
        path: crate::types::PathRef<'_>,
    ) -> Option<crate::types::TomlTable> {
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
                        crate::types::TomlTable::from,
                    )
                })
            })
    }
    pub(super) fn rs_files(&self) -> &[RsSourceFile] {
        &self.rs_files
    }
    // The snapshot exposes this derived workspace-name set through one policy consumer.

    #[allow(clippy::single_call_fn)] // policy consumers share this snapshot accessor directly and through the root facade
    pub(super) fn workspace_crate_names(&self) -> crate::types::SourceTextBTreeSet {
        self.source.workspace_crate_names.clone()
    }
    pub(super) fn workspace_metadata(&self) -> crate::types::CargoMetadataRef<'_> {
        crate::types::CargoMetadataRef::from(self.source.workspace_metadata.as_ref())
    }
}
impl CodebaseSourceSnapshot {
    fn cargo_toml_file(&self, path: crate::types::PathRef<'_>) -> Option<&CargoTomlSourceFile> {
        self.cargo_toml_by_path
            .get(path.as_ref())
            .and_then(|idx| self.cargo_toml_files.get(idx.get()))
    }
}
pub(super) fn with_codebase_snapshot<R>(f: impl FnOnce(&CodebaseSnapshot) -> R) -> R {
    std::thread_local! {
        static SNAPSHOT: std::cell::OnceCell<CodebaseSnapshot> = const { std::cell::OnceCell::new() };
    }
    SNAPSHOT.with(|snapshot| {
        f(snapshot.get_or_init(|| {
            static SOURCE_SNAPSHOT: std::sync::OnceLock<std::sync::Arc<CodebaseSourceSnapshot>> =
                std::sync::OnceLock::new();
            let source_snapshot = std::sync::Arc::clone(SOURCE_SNAPSHOT.get_or_init(|| {
                std::sync::Arc::new({
                    let metadata = crate::types::CargoMetadata::from(
                        cargo_metadata::MetadataCommand::new()
                            .manifest_path(
                                constants_str::catalog::CODE_STYLE_WORKSPACE_MANIFEST_PATH,
                            )
                            .exec()
                            .expect("c84e9d1f workspace metadata invariant must hold"),
                    );
                    let workspace_members = crate::types::CargoPackageIdRefHashSet::from(
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
                                content: crate::types::SourceText::try_from(content)
                                    .expect("84f6a0d2 build invariant must hold"),
                                parsed: crate::types::TomlTable::from(parsed),
                                path: crate::types::OwnedPathBuf::from(path),
                            }
                        })
                        .collect();
                    let cargo_toml_by_path = cargo_toml_files
                        .iter()
                        .enumerate()
                        .map(|(idx, cargo_toml)| {
                            (
                                cargo_toml.path.clone(),
                                crate::types::CargoTomlFileIdx::from(idx),
                            )
                        })
                        .collect::<std::collections::BTreeMap<
                            crate::types::OwnedPathBuf,
                            crate::types::CargoTomlFileIdx,
                        >>();
                    let project_source_files = crate::types::WalkdirWalkDir::from(
                        walkdir::WalkDir::new(constants_str::catalog::TEXT_ALT_9),
                    )
                    .into_iter()
                    .filter_entry(|element| {
                        element.file_name() != constants_str::catalog::TARGET
                            && element.file_name() != constants_str::catalog::GIT
                            && element.file_name()
                                != constants_str::test_fixtures::WORKSPACE_SCAFFOLD_NODE_MODULES
                            && (element.file_type().is_dir()
                                || matches!(
                                    element.path().extension().and_then(std::ffi::OsStr::to_str),
                                    Some(
                                        constants_str::catalog::RS
                                            | constants_str::integration_fixtures::MD
                                            | constants_str::integration_fixtures::TOML
                                            | constants_str::catalog::TXT
                                            | constants_str::integration_fixtures::YML
                                            | constants_str::integration_fixtures::YAML
                                            | constants_str::integration_fixtures::JSON
                                    )
                                ))
                    })
                    .map(project_walk_entry)
                    .filter(|entry| !entry.file_type().is_dir())
                    .map(|entry| project_source_file(entry.into_path()))
                    .collect::<Vec<_>>();
                    CodebaseSourceSnapshot {
                        cargo_toml_by_path,
                        cargo_toml_files,
                        project_source_files,
                        workspace_metadata: metadata,
                        workspace_crate_names: crate::types::SourceTextBTreeSet::from(
                            workspace_crate_names,
                        ),
                    }
                })
            }));
            let rs_files = source_snapshot
                .project_source_files
                .iter()
                .filter(|source_file| {
                    source_file
                        .path
                        .as_ref()
                        .extension()
                        .and_then(std::ffi::OsStr::to_str)
                        == Some(constants_str::catalog::RS)
                })
                .map(|source_file| {
                    let ast =
                        syn::parse_file(source_file.content.as_ref()).unwrap_or_else(|error| {
                            panic!("5e7a83eb {}: {error}", source_file.path.as_ref().display())
                        });
                    RsSourceFile {
                        ast: crate::types::SynFile::from(ast),
                        content: source_file.content.clone(),
                        path: source_file.path.clone(),
                    }
                })
                .collect();
            CodebaseSnapshot {
                rs_files,
                source: source_snapshot,
            }
        }))
    })
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
        path: crate::types::OwnedPathBuf::from(path),
    }
}
fn project_source_content(path: &std::path::Path, raw_content: String) -> crate::types::SourceText {
    crate::types::SourceText::try_from(raw_content)
        .unwrap_or_else(|error| panic!("e27f9e15 invalid source {}: {error}", path.display()))
}
#[test]
fn invalid_project_source_content_fails_snapshot_loading() {
    let oversized = constants_str::catalog::X.repeat(16_777_217usize);
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
    let missing = std::path::PathBuf::from(constants_str::test_fixtures::VALUE_5E88EEB9);
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
