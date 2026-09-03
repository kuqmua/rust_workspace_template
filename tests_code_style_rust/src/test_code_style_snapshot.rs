#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct RsSourceFile {
    ast: crate::types::SynFile,
    content: crate::types::SourceText,
    path: crate::types::OwnedPathBuf,
}
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ProjectSourceFile {
    content: crate::types::SourceText,
    path: crate::types::OwnedPathBuf,
}
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct CargoTomlSourceFile {
    content: crate::types::SourceText,
    parsed: crate::types::TomlTable,
    path: crate::types::OwnedPathBuf,
}
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct CodebaseSnapshot {
    rs_files: Vec<RsSourceFile>,
    source: std::sync::Arc<CodebaseSourceSnapshot>,
}
#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct CodebaseSourceSnapshot {
    cargo_toml_by_path:
        std::collections::BTreeMap<crate::types::OwnedPathBuf, crate::types::CargoTomlFileIndex>,
    cargo_toml_files: Vec<CargoTomlSourceFile>,
    project_source_files: Vec<ProjectSourceFile>,
    workspace_crate_names: crate::types::SourceTextBTreeSet,
    workspace_metadata: crate::types::CargoMetadata,
}
impl CodebaseSnapshot {
    pub(super) fn cargo_toml_content(
        &self,
        path_ref: crate::types::PathRef<'_>,
    ) -> Option<crate::types::SourceText> {
        self.source
            .cargo_toml_file(path_ref)
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
        path_ref: crate::types::PathRef<'_>,
    ) -> Option<crate::types::TomlTable> {
        self.source
            .cargo_toml_file(path_ref)
            .map(|cargo_toml| cargo_toml.parsed.clone())
            .or_else(|| {
                path_ref.as_ref().exists().then(|| {
                    let value =
                        std::fs::read_to_string(path_ref.as_ref()).unwrap_or_else(|error| {
                            std::panic::panic_any(
                                constants_str::PANIC_E12179C5
                                    .replacen(
                                        constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                                        path_ref.as_ref().display().to_string().as_str(),
                                        1usize,
                                    )
                                    .replacen(
                                        constants_str::PANIC_PLACEHOLDER_81240055,
                                        error.to_string().as_str(),
                                        1usize,
                                    ),
                            )
                        });
                    value.parse::<toml::Table>().map_or_else(
                        |error| {
                            std::panic::panic_any(
                                constants_str::PANIC_77B2D82B
                                    .replacen(
                                        constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                                        path_ref.as_ref().display().to_string().as_str(),
                                        1usize,
                                    )
                                    .replacen(
                                        constants_str::PANIC_PLACEHOLDER_81240055,
                                        error.to_string().as_str(),
                                        1usize,
                                    ),
                            )
                        },
                        crate::types::TomlTable::from,
                    )
                })
            })
    }

    #[allow(
        clippy::single_call_fn,
        reason = "test code style snapshot remains a named owner because its boundary role is clearer and directly testable"
    )]
    pub(super) fn workspace_crate_names(&self) -> crate::types::SourceTextBTreeSet {
        self.source.workspace_crate_names.clone()
    }
    pub(super) fn workspace_metadata(&self) -> crate::types::CargoMetadataRef<'_> {
        crate::types::CargoMetadataRef::from(self.source.workspace_metadata.as_ref())
    }
}
impl CodebaseSourceSnapshot {
    fn cargo_toml_file(&self, path_ref: crate::types::PathRef<'_>) -> Option<&CargoTomlSourceFile> {
        self.cargo_toml_by_path
            .get(path_ref.as_ref())
            .and_then(|index| self.cargo_toml_files.get(index.get()))
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
                            .manifest_path(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
                            .exec()
                            .expect(constants_str::DIAGNOSTIC_C84E9D1F),
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
                                std::panic::panic_any(
                                    constants_str::PANIC_50DA433E
                                        .replacen(
                                            constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                                            path.display().to_string().as_str(),
                                            1usize,
                                        )
                                        .replacen(
                                            constants_str::PANIC_PLACEHOLDER_81240055,
                                            error.to_string().as_str(),
                                            1usize,
                                        ),
                                )
                            });
                            let parsed = content.parse::<toml::Table>().unwrap_or_else(|error| {
                                std::panic::panic_any(
                                    constants_str::PANIC_96F2C78A
                                        .replacen(
                                            constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                                            path.display().to_string().as_str(),
                                            1usize,
                                        )
                                        .replacen(
                                            constants_str::PANIC_PLACEHOLDER_81240055,
                                            error.to_string().as_str(),
                                            1usize,
                                        ),
                                )
                            });
                            CargoTomlSourceFile {
                                content: crate::types::SourceText::try_from(content)
                                    .expect(constants_str::DIAGNOSTIC_84F6A0D2),
                                parsed: crate::types::TomlTable::from(parsed),
                                path: crate::types::OwnedPathBuf::from(path),
                            }
                        })
                        .collect();
                    let cargo_toml_by_path = cargo_toml_files
                        .iter()
                        .enumerate()
                        .map(|(index, cargo_toml)| {
                            (
                                cargo_toml.path.clone(),
                                crate::types::CargoTomlFileIndex::from(index),
                            )
                        })
                        .collect::<std::collections::BTreeMap<
                            crate::types::OwnedPathBuf,
                            crate::types::CargoTomlFileIndex,
                        >>();
                    let project_source_files = crate::types::WalkdirWalkDir::from(
                        walkdir::WalkDir::new(constants_str::TEXT_ALT_9),
                    )
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
                        == Some(constants_str::RS)
                })
                .map(|source_file| {
                    let ast =
                        syn::parse_file(source_file.content.as_ref()).unwrap_or_else(|error| {
                            std::panic::panic_any(
                                constants_str::PANIC_5E7A83EB
                                    .replacen(
                                        constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                                        source_file.path.as_ref().display().to_string().as_str(),
                                        1usize,
                                    )
                                    .replacen(
                                        constants_str::PANIC_PLACEHOLDER_81240055,
                                        error.to_string().as_str(),
                                        1usize,
                                    ),
                            )
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

fn project_walk_entry(result: walkdir::Result<walkdir::DirEntry>) -> walkdir::DirEntry {
    result.unwrap_or_else(|error| {
        std::panic::panic_any(constants_str::PANIC_1E4B17B0.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ))
    })
}
fn project_source_file(path_buf: std::path::PathBuf) -> ProjectSourceFile {
    let raw_content = std::fs::read_to_string(&path_buf).unwrap_or_else(|error| {
        std::panic::panic_any(
            constants_str::PANIC_68A041C3
                .replacen(
                    constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                    path_buf.display().to_string().as_str(),
                    1usize,
                )
                .replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ),
        )
    });
    let content = project_source_content(path_buf.as_path(), raw_content);
    ProjectSourceFile {
        content,
        path: crate::types::OwnedPathBuf::from(path_buf),
    }
}
fn project_source_content(path: &std::path::Path, string: String) -> crate::types::SourceText {
    crate::types::SourceText::try_from(string).unwrap_or_else(|error| {
        std::panic::panic_any(
            constants_str::PANIC_E27F9E15
                .replacen(
                    constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                    path.display().to_string().as_str(),
                    1usize,
                )
                .replacen(
                    constants_str::PANIC_PLACEHOLDER_81240055,
                    error.to_string().as_str(),
                    1usize,
                ),
        )
    })
}
#[test]
fn test_invalid_project_source_content_fails_snapshot_loading() {
    let oversized = constants_str::X.repeat(16_777_217usize);
    assert!(
        std::panic::catch_unwind(|| {
            project_source_content(
                std::path::Path::new(constants_str::VALUE_AB1CDF0B),
                oversized,
            )
        })
        .is_err(),
        "28fb322e"
    );
}
#[test]
fn test_missing_project_source_file_fails_snapshot_loading() {
    let missing = std::path::PathBuf::from(constants_str::VALUE_5E88EEB9);
    assert!(
        std::panic::catch_unwind(|| project_source_file(missing)).is_err(),
        "46045b88"
    );
}
#[test]
fn test_walk_error_fails_snapshot_loading() {
    assert!(
        std::panic::catch_unwind(|| {
            let missing = walkdir::WalkDir::new(constants_str::VALUE_DE5C9E66)
                .into_iter()
                .next()
                .expect(constants_str::DIAGNOSTIC_1DA2F4ED);
            project_walk_entry(missing)
        })
        .is_err(),
        "6a6e2aac"
    );
}
