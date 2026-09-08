#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct RsSourceFile {
    ast: crate::syn_file::SynFile,
    content: crate::source_text::SourceText,
    path: crate::owned_path_buf::OwnedPathBuf,
}
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ProjectSourceFile {
    content: crate::source_text::SourceText,
    path: crate::owned_path_buf::OwnedPathBuf,
}
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct CargoTomlSourceFile {
    content: crate::source_text::SourceText,
    parsed: crate::toml_table::TomlTable,
    path: crate::owned_path_buf::OwnedPathBuf,
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
    cargo_toml_by_path: std::collections::BTreeMap<
        crate::owned_path_buf::OwnedPathBuf,
        crate::cargo_toml_file_index::CargoTomlFileIndex,
    >,
    cargo_toml_files: Vec<CargoTomlSourceFile>,
    project_source_files: Vec<ProjectSourceFile>,
    workspace_crate_names: crate::source_text_b_tree_set::SourceTextBTreeSet,
    workspace_metadata: crate::cargo_metadata::CargoMetadata,
}
impl RsSourceFile {
    pub(super) fn external_module_declaration(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> Option<crate::syn_item_ref::SynItemRef<'_>> {
        if self.path.as_ref().parent() != path_ref.as_ref().parent()
            || !self
                .path
                .as_ref()
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|stem| matches!(stem, constants_str::LIB | constants_str::MAIN))
        {
            return None;
        }
        self.ast
            .as_ref()
            .items
            .iter()
            .find(|item| {
                matches!(item, syn::Item::Mod(item_mod) if item_mod.content.is_none()
                && path_ref.as_ref().file_stem().and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|stem| item_mod.ident == stem))
            })
            .map(crate::syn_item_ref::SynItemRef::from)
    }

    fn parse(project_source_file: &ProjectSourceFile) -> Self {
        let ast = syn::parse_file(project_source_file.content.as_ref()).unwrap_or_else(|error| {
            std::panic::panic_any(
                constants_str::PANIC_5E7A83EB
                    .replacen(
                        constants_str::PANIC_POSITIONAL_PLACEHOLDER,
                        project_source_file
                            .path
                            .as_ref()
                            .display()
                            .to_string()
                            .as_str(),
                        1usize,
                    )
                    .replacen(
                        constants_str::PANIC_PLACEHOLDER_81240055,
                        error.to_string().as_str(),
                        1usize,
                    ),
            )
        });
        Self {
            ast: crate::syn_file::SynFile::from(ast),
            content: project_source_file.content.clone(),
            path: project_source_file.path.clone(),
        }
    }
}
impl CodebaseSnapshot {
    pub(super) fn cargo_toml_content(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> Option<crate::source_text::SourceText> {
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
    pub(super) fn is_test_module_path(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> crate::analyzer_bool::AnalyzerBool {
        crate::rs_source_files_ref::RsSourceFilesRef::from(self.rs_files.as_slice())
            .is_test_module_path(path_ref)
    }

    pub(super) fn project_source_files(&self) -> &[ProjectSourceFile] {
        self.source.project_source_files.as_slice()
    }
    pub(super) fn read_toml_table(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> Option<crate::toml_table::TomlTable> {
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
                        crate::toml_table::TomlTable::from,
                    )
                })
            })
    }

    #[allow(
        clippy::single_call_fn,
        reason = "test code style snapshot remains a named owner because its boundary role is clearer and directly testable"
    )]
    pub(super) fn workspace_crate_names(
        &self,
    ) -> crate::source_text_b_tree_set::SourceTextBTreeSet {
        self.source.workspace_crate_names.clone()
    }
    pub(super) fn workspace_metadata(&self) -> crate::cargo_metadata_ref::CargoMetadataRef<'_> {
        crate::cargo_metadata_ref::CargoMetadataRef::from(self.source.workspace_metadata.as_ref())
    }
}
impl CodebaseSourceSnapshot {
    fn cargo_toml_file(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> Option<&CargoTomlSourceFile> {
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
                    let metadata = crate::cargo_metadata::CargoMetadata::from(
                        cargo_metadata::MetadataCommand::new()
                            .manifest_path(constants_str::CODE_STYLE_WORKSPACE_MANIFEST_PATH)
                            .exec()
                            .expect(constants_str::DIAGNOSTIC_C84E9D1F),
                    );
                    let workspace_members =
                        crate::cargo_package_id_ref_hash_set::CargoPackageIdRefHashSet::from(
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
                                content: crate::source_text::SourceText::try_from(content)
                                    .expect(constants_str::DIAGNOSTIC_84F6A0D2),
                                parsed: crate::toml_table::TomlTable::from(parsed),
                                path: crate::owned_path_buf::OwnedPathBuf::from(path),
                            }
                        })
                        .collect();
                    let cargo_toml_by_path = cargo_toml_files
                        .iter()
                        .enumerate()
                        .map(|(index, cargo_toml)| {
                            (
                                cargo_toml.path.clone(),
                                crate::cargo_toml_file_index::CargoTomlFileIndex::from(index),
                            )
                        })
                        .collect::<std::collections::BTreeMap<
                            crate::owned_path_buf::OwnedPathBuf,
                            crate::cargo_toml_file_index::CargoTomlFileIndex,
                        >>();
                    let project_source_files = crate::walkdir_walk_dir::WalkdirWalkDir::from(
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
                        workspace_crate_names:
                            crate::source_text_b_tree_set::SourceTextBTreeSet::from(
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
                .map(RsSourceFile::parse)
                .collect::<Vec<RsSourceFile>>();
            crate::rs_source_files_ref::RsSourceFilesRef::from(rs_files.as_slice())
                .assert_external_modules_resolve();
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
        path: crate::owned_path_buf::OwnedPathBuf::from(path_buf),
    }
}
fn project_source_content(
    path: &std::path::Path,
    string: String,
) -> crate::source_text::SourceText {
    crate::source_text::SourceText::try_from(string).unwrap_or_else(|error| {
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

fn test_snapshot_source(
    source_text_ref: crate::source_text_ref::SourceTextRef<'_>,
    syn_file: crate::syn_file::SynFile,
) -> RsSourceFile {
    RsSourceFile {
        ast: syn_file,
        content: crate::source_text::SourceText::try_from(String::new())
            .expect(constants_str::DIAGNOSTIC_E47E70D9),
        path: crate::owned_path_buf::OwnedPathBuf::from(std::path::PathBuf::from(format!(
            "{}.{}",
            source_text_ref.as_ref(),
            constants_str::RS
        ))),
    }
}

fn test_external_module_naming_fixture(
    source_text_ref: crate::source_text_ref::SourceTextRef<'_>,
    syn_file: crate::syn_file::SynFile,
    analyzer_count: crate::analyzer_count::AnalyzerCount,
) {
    let module_ident = syn::Ident::new(source_text_ref.as_ref(), proc_macro2::Span::call_site());
    let source_files = [
        test_snapshot_source(
            crate::source_text_ref::SourceTextRef::from(constants_str::LIB),
            <crate::syn_file::SynFile as From<syn::File>>::from(syn::parse_quote! {
                #[cfg(test)]
                mod #module_ident;
            }),
        ),
        test_snapshot_source(source_text_ref, syn_file),
    ];
    let rs_source_files_ref =
        crate::rs_source_files_ref::RsSourceFilesRef::from(source_files.as_slice());
    rs_source_files_ref.assert_external_modules_resolve();
    let module_source = &source_files[constants_usize::ONE];
    let module_names = rs_source_files_ref.module_names(crate::path_ref::PathRef::from(
        module_source.path().as_ref(),
    ));
    assert_eq!(module_names.as_slice(), [source_text_ref.as_ref()]);
    let mut visitor = crate::code_style::visit_syn_file(
        crate::syn_file_ref::SynFileRef::from(module_source.ast().as_ref()),
        crate::source_analysis::TestNameVisitor::new(
            crate::diagnostic_messages::DiagnosticMessages::default(),
            module_names,
            crate::analyzer_bool::AnalyzerBool::default(),
        ),
    );
    visitor.check_file_name(crate::path_ref::PathRef::from(
        module_source.path().as_ref(),
    ));
    assert_eq!(visitor.get_errors().len(), analyzer_count.get());
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "the naming fixture matrix uses iterator traversal to comply with the workspace no-for-loop policy"
)]
fn test_external_module_names_and_root_filenames_are_checked() {
    [
        (
            constants_str::CODE_STYLE_EXTERNAL_TEST_MODULE,
            constants_usize::ZERO,
        ),
        (
            constants_str::CODE_STYLE_UNPREFIXED_TEST_MODULE,
            constants_usize::TWO,
        ),
        (constants_str::TEST_TESTS, constants_usize::ONE),
        (constants_str::TESTS_ALT, constants_usize::ONE),
    ]
    .into_iter()
    .for_each(|(module_name, expected_errors)| {
        test_external_module_naming_fixture(
            crate::source_text_ref::SourceTextRef::from(module_name),
            <crate::syn_file::SynFile as From<syn::File>>::from(syn::parse_quote! {
                #[test]
                fn test_works() {}
            }),
            crate::analyzer_count::AnalyzerCount::from(expected_errors),
        );
    });
}

#[test]
fn test_external_helper_owner_can_contain_canonical_inline_tests() {
    test_external_module_naming_fixture(
        crate::source_text_ref::SourceTextRef::from(
            constants_str::CODE_STYLE_UNPREFIXED_TEST_MODULE,
        ),
        <crate::syn_file::SynFile as From<syn::File>>::from(syn::parse_quote! {
            mod tests {
                #[test]
                fn test_works() {}
            }
            mod test_widget {
                #[test]
                fn test_other() {}
            }
        }),
        crate::analyzer_count::AnalyzerCount::from(constants_usize::ZERO),
    );
}

#[test]
fn test_missing_external_module_source_fails_snapshot_validation() {
    let source_files = [test_snapshot_source(
        crate::source_text_ref::SourceTextRef::from(constants_str::LIB),
        <crate::syn_file::SynFile as From<syn::File>>::from(syn::parse_quote! {
            mod missing;
        }),
    )];
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            crate::rs_source_files_ref::RsSourceFilesRef::from(source_files.as_slice())
                .assert_external_modules_resolve();
        }))
        .is_err()
    );
}

#[test]
fn test_malformed_rust_source_fails_snapshot_parsing() {
    let project_source_file = ProjectSourceFile {
        content: crate::source_text::SourceText::try_from(String::from(
            constants_str::TEST_NAME_PREFIX,
        ))
        .expect(constants_str::DIAGNOSTIC_57E87204),
        path: crate::owned_path_buf::OwnedPathBuf::from(std::path::PathBuf::from(
            constants_str::VALUE_0544FC95,
        )),
    };
    assert!(std::panic::catch_unwind(|| RsSourceFile::parse(&project_source_file)).is_err());
}

#[test]
fn test_module_classification_preserves_all_crate_root_declarations() {
    let source_files = [
        test_snapshot_source(
            crate::source_text_ref::SourceTextRef::from(constants_str::LIB),
            <crate::syn_file::SynFile as From<syn::File>>::from(syn::parse_quote! {
                mod test_widget;
            }),
        ),
        test_snapshot_source(
            crate::source_text_ref::SourceTextRef::from(constants_str::MAIN),
            <crate::syn_file::SynFile as From<syn::File>>::from(syn::parse_quote! {
                #[cfg(test)]
                mod test_widget;
            }),
        ),
    ];
    let module_path = std::path::Path::new(constants_str::CODE_STYLE_EXTERNAL_TEST_FILE);
    assert!(
        crate::rs_source_files_ref::RsSourceFilesRef::from(source_files.as_slice())
            .is_test_module_path(crate::path_ref::PathRef::from(module_path))
            .get()
    );
}
