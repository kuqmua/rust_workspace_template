pub(super) struct RsSourceFile {
    ast: syn::File,
    content: String,
    path: std::path::PathBuf,
}
struct CargoTomlSourceFile {
    content: String,
    parsed: toml::Table,
    path: std::path::PathBuf,
}
pub(super) struct CodebaseSnapshot {
    cargo_toml_files: Vec<CargoTomlSourceFile>,
    metadata: cargo_metadata::Metadata,
    rs_files: Vec<RsSourceFile>,
    workspace_crate_names: std::collections::BTreeSet<String>,
}
impl RsSourceFile {
    pub(super) fn ast(&self) -> &syn::File {
        &self.ast
    }
    pub(super) fn content(&self) -> &str {
        &self.content
    }
    pub(super) fn path(&self) -> &std::path::Path {
        &self.path
    }
}
impl CodebaseSnapshot {
    #[allow(clippy::single_call_fn)] // named constructor keeps snapshot initialization readable at the thread-local OnceCell call site
    fn build() -> Self {
        let metadata = workspace_metadata_uncached();
        let workspace_members = workspace_member_ids(&metadata);
        let workspace_crate_names = metadata
            .packages
            .iter()
            .filter(|package| workspace_members.contains(&package.id))
            .map(|package| package.name.to_string())
            .collect();
        let cargo_toml_files = metadata
            .packages
            .iter()
            .filter(|package| workspace_members.contains(&package.id))
            .filter_map(|package| {
                let path = package.manifest_path.as_std_path().to_path_buf();
                let content = std::fs::read_to_string(&path).ok()?;
                let parsed = content.parse::<toml::Table>().ok()?;
                Some(CargoTomlSourceFile {
                    content,
                    parsed,
                    path,
                })
            })
            .collect();
        let rs_files = rs_project_files_uncached()
            .filter(|entry| {
                !super::is_exception(
                    entry.path(),
                    &super::GENERATED_TEST_FIXTURE_SOURCE_EXCEPTIONS,
                )
            })
            .filter_map(|entry| {
                let path = entry.into_path();
                let content = std::fs::read_to_string(&path).ok()?;
                let ast = syn::parse_file(&content).expect("5e7a83eb");
                Some(RsSourceFile { ast, content, path })
            })
            .collect();
        Self {
            cargo_toml_files,
            metadata,
            rs_files,
            workspace_crate_names,
        }
    }
    pub(super) fn cargo_toml_content(&self, path: &std::path::Path) -> Option<String> {
        self.cargo_toml_file(path)
            .map(|cargo_toml| cargo_toml.content.clone())
    }
    fn cargo_toml_file(&self, path: &std::path::Path) -> Option<&CargoTomlSourceFile> {
        self.cargo_toml_files
            .iter()
            .find(|cargo_toml| cargo_toml.path == path)
    }
    pub(super) fn package_manifest_paths(&self) -> impl Iterator<Item = &std::path::Path> {
        let workspace_members = workspace_member_ids(&self.metadata);
        self.metadata
            .packages
            .iter()
            .filter(move |package| workspace_members.contains(&package.id))
            .map(|package| package.manifest_path.as_std_path())
    }
    pub(super) fn read_toml_table(&self, path: &std::path::Path) -> Option<toml::Table> {
        self.cargo_toml_file(path)
            .map(|cargo_toml| cargo_toml.parsed.clone())
            .or_else(|| {
                let v = std::fs::read_to_string(path).ok()?;
                v.parse::<toml::Table>().ok()
            })
    }
    pub(super) fn rs_files(&self) -> &[RsSourceFile] {
        &self.rs_files
    }
    #[allow(clippy::single_call_fn)]
    pub(super) fn workspace_crate_names(&self) -> std::collections::BTreeSet<String> {
        self.workspace_crate_names.clone()
    }
}
pub(super) fn project_dir() -> walkdir::WalkDir {
    walkdir::WalkDir::new("../")
}
pub(super) fn with_codebase_snapshot<R>(f: impl FnOnce(&CodebaseSnapshot) -> R) -> R {
    std::thread_local! {
        static SNAPSHOT: std::cell::OnceCell<CodebaseSnapshot> = const { std::cell::OnceCell::new() };
    }
    SNAPSHOT.with(|snapshot| f(snapshot.get_or_init(CodebaseSnapshot::build)))
}
pub(super) fn is_ignored_dir_entry_name(name: &std::ffi::OsStr) -> bool {
    name == "target" || name == ".git"
}
#[allow(clippy::single_call_fn)] // isolates cargo_metadata command setup from snapshot construction
fn workspace_metadata_uncached() -> cargo_metadata::Metadata {
    cargo_metadata::MetadataCommand::new()
        .manifest_path("../Cargo.toml")
        .exec()
        .expect("c84e9d1f")
}
fn workspace_member_ids(
    metadata: &cargo_metadata::Metadata,
) -> std::collections::HashSet<&cargo_metadata::PackageId> {
    metadata.workspace_members.iter().collect()
}
#[allow(clippy::single_call_fn)] // keeps filesystem walker rules separate from snapshot materialization
fn rs_project_files_uncached() -> impl Iterator<Item = walkdir::DirEntry> {
    project_dir()
        .into_iter()
        .filter_entry(|el| {
            !is_ignored_dir_entry_name(el.file_name())
                && (el.file_type().is_dir() || is_rs_file_path(el.path()))
        })
        .filter_map(Result::ok)
        .filter(|el| is_rs_file_path(el.path()))
}
fn is_rs_file_path(path: &std::path::Path) -> bool {
    path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs")
}
