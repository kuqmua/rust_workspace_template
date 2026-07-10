pub(super) struct RsSourceFile {
    ast: super::types::SynFile,
    content: super::types::SourceText,
    path: super::types::StdPathBuf,
}
struct CargoTomlSourceFile {
    content: super::types::SourceText,
    parsed: super::types::TomlTable,
    path: super::types::StdPathBuf,
}
pub(super) struct CodebaseSnapshot {
    cargo_toml_by_path:
        std::collections::BTreeMap<super::types::StdPathBuf, super::types::CargoTomlFileIdx>,
    cargo_toml_files: Vec<CargoTomlSourceFile>,
    rs_files: Vec<RsSourceFile>,
    workspace_crate_names: super::types::StdSourceTextSet,
}
impl RsSourceFile {
    pub(super) fn ast(&self) -> &super::types::SynFile {
        &self.ast
    }
    pub(super) fn content(&self) -> &super::types::SourceText {
        &self.content
    }
    pub(super) fn path(&self) -> &super::types::StdPathBuf {
        &self.path
    }
}
impl CodebaseSnapshot {
    #[allow(clippy::single_call_fn)] // named constructor keeps snapshot initialization readable at the thread-local OnceCell call site
    fn build() -> Self {
        let metadata = workspace_metadata_uncached();
        let workspace_members =
            workspace_member_ids(super::types::CargoMetadataRef::from(metadata.as_ref()));
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
            .filter_map(|package| {
                let path = package.manifest_path.as_std_path().to_path_buf();
                let content = std::fs::read_to_string(&path).ok()?;
                let parsed = content.parse::<toml::Table>().ok()?;
                Some(CargoTomlSourceFile {
                    content: super::types::SourceText::try_from(content).expect("84f6a0d2"),
                    parsed: super::types::TomlTable::from(parsed),
                    path: super::types::StdPathBuf::from(path),
                })
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
                    super::types::StdPathBuf,
                    super::types::CargoTomlFileIdx,
                >>();
        let rs_files = rs_project_files_uncached()
            .filter_map(|entry| {
                let path = entry.into_path();
                let content = std::fs::read_to_string(&path).ok()?;
                let ast = syn::parse_file(&content).expect("5e7a83eb");
                Some(RsSourceFile {
                    ast: super::types::SynFile::from(ast),
                    content: super::types::SourceText::try_from(content).expect("b0c39e18"),
                    path: super::types::StdPathBuf::from(path),
                })
            })
            .collect();
        Self {
            cargo_toml_by_path,
            cargo_toml_files,
            rs_files,
            workspace_crate_names: super::types::StdSourceTextSet::from(workspace_crate_names),
        }
    }
    pub(super) fn cargo_toml_content(
        &self,
        path: super::types::StdPathRef<'_>,
    ) -> Option<super::types::SourceText> {
        self.cargo_toml_file(path)
            .map(|cargo_toml| cargo_toml.content.clone())
    }
    fn cargo_toml_file(&self, path: super::types::StdPathRef<'_>) -> Option<&CargoTomlSourceFile> {
        self.cargo_toml_by_path
            .get(path.as_ref())
            .and_then(|idx| self.cargo_toml_files.get(idx.get()))
    }
    pub(super) fn crate_manifest_paths(&self) -> impl Iterator<Item = &std::path::Path> {
        self.cargo_toml_files
            .iter()
            .map(|cargo_toml| cargo_toml.path.as_ref())
    }
    pub(super) fn read_toml_table(
        &self,
        path: super::types::StdPathRef<'_>,
    ) -> Option<super::types::TomlTable> {
        self.cargo_toml_file(path)
            .map(|cargo_toml| cargo_toml.parsed.clone())
            .or_else(|| {
                let v = std::fs::read_to_string(path.as_ref()).ok()?;
                v.parse::<toml::Table>()
                    .ok()
                    .map(super::types::TomlTable::from)
            })
    }
    pub(super) fn rs_files(&self) -> &[RsSourceFile] {
        &self.rs_files
    }
    #[allow(clippy::single_call_fn)]
    pub(super) fn workspace_crate_names(&self) -> super::types::StdSourceTextSet {
        self.workspace_crate_names.clone()
    }
}
pub(super) fn project_dir() -> super::types::WalkdirWalkDir {
    super::types::WalkdirWalkDir::from(walkdir::WalkDir::new("../"))
}
pub(super) fn with_codebase_snapshot<R>(f: impl FnOnce(&CodebaseSnapshot) -> R) -> R {
    std::thread_local! {
        static SNAPSHOT: std::cell::OnceCell<CodebaseSnapshot> = const { std::cell::OnceCell::new() };
    }
    SNAPSHOT.with(|snapshot| f(snapshot.get_or_init(CodebaseSnapshot::build)))
}
pub(super) fn is_ignored_dir_entry_name(
    name: super::types::StdOsStrRef<'_>,
) -> super::types::AnalyzerBool {
    super::types::AnalyzerBool::from(name.as_ref() == "target" || name.as_ref() == ".git")
}
#[allow(clippy::single_call_fn)] // isolates cargo_metadata command setup from snapshot construction
fn workspace_metadata_uncached() -> super::types::CargoMetadata {
    super::types::CargoMetadata::from(
        cargo_metadata::MetadataCommand::new()
            .manifest_path(super::WORKSPACE_MANIFEST_PATH)
            .exec()
            .expect("c84e9d1f"),
    )
}
#[allow(clippy::single_call_fn)] // keeps workspace membership extraction named while snapshot construction reuses it twice
fn workspace_member_ids(
    metadata: super::types::CargoMetadataRef<'_>,
) -> super::types::StdCargoPackageIdRefSet<'_> {
    super::types::StdCargoPackageIdRefSet::from(
        metadata
            .get()
            .workspace_members
            .iter()
            .collect::<std::collections::HashSet<&cargo_metadata::PackageId>>(),
    )
}
#[allow(clippy::single_call_fn)] // keeps filesystem walker rules separate from snapshot materialization
fn rs_project_files_uncached() -> impl Iterator<Item = walkdir::DirEntry> {
    project_dir()
        .into_iter()
        .filter_entry(|el| {
            !is_ignored_dir_entry_name(super::types::StdOsStrRef::from(el.file_name())).get()
                && (el.file_type().is_dir()
                    || is_rs_file_path(super::types::StdPathRef::from(el.path())).get())
        })
        .filter_map(Result::ok)
        .filter(|el| is_rs_file_path(super::types::StdPathRef::from(el.path())).get())
}
fn is_rs_file_path(path: super::types::StdPathRef<'_>) -> super::types::AnalyzerBool {
    super::types::AnalyzerBool::from(
        path.as_ref().extension().and_then(std::ffi::OsStr::to_str) == Some("rs"),
    )
}
