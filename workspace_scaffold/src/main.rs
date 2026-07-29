const SCAFFOLD_TEXT_MAX_BYTES: usize = 16_777_216usize;
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct ProjectNameRef<'value>(&'value str);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct RepositoryUrlRef<'value>(&'value str);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct ServicePort(u16);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceCrate(String);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceComposeName(String);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceComposeFile(String);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceDockerfile(String);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceImage(String);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceKubernetesManifest(String);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceSocketEnv(String);
#[derive(Debug, newtype::FromInner)]
struct ServiceCatalogEntries(bounded_types::BoundedVec<ServiceCatalogEntry, 0, { usize::MAX }>);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct ServiceCatalogEntriesRef<'entries_lt>(&'entries_lt [ServiceCatalogEntry]);
#[derive(Debug)]
struct ServiceCatalogEntry {
    compose_file: ServiceComposeFile,
    compose_name: ServiceComposeName,
    crate_name: ServiceCrate,
    dockerfile: ServiceDockerfile,
    image: ServiceImage,
    kubernetes_manifest: ServiceKubernetesManifest,
    port: ServicePort,
    release: ShouldRelease,
    socket_env: ServiceSocketEnv,
}
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct ShouldRelease(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct IsCatalogPathSafe(bool);
#[derive(Default)]
struct ServiceCatalogDraft {
    compose_file: Option<ServiceComposeFile>,
    compose_name: Option<ServiceComposeName>,
    crate_name: Option<ServiceCrate>,
    dockerfile: Option<ServiceDockerfile>,
    image: Option<ServiceImage>,
    kubernetes_manifest: Option<ServiceKubernetesManifest>,
    port: Option<ServicePort>,
    release: Option<ShouldRelease>,
    socket_env: Option<ServiceSocketEnv>,
}
impl ServiceCatalogDraft {
    fn finish(self) -> Result<ServiceCatalogEntry, ScaffoldError> {
        Ok(ServiceCatalogEntry {
            compose_file: self.compose_file.ok_or(ScaffoldError::Catalog)?,
            compose_name: self.compose_name.ok_or(ScaffoldError::Catalog)?,
            crate_name: self.crate_name.ok_or(ScaffoldError::Catalog)?,
            dockerfile: self.dockerfile.ok_or(ScaffoldError::Catalog)?,
            image: self.image.ok_or(ScaffoldError::Catalog)?,
            kubernetes_manifest: self.kubernetes_manifest.ok_or(ScaffoldError::Catalog)?,
            port: self.port.ok_or(ScaffoldError::Catalog)?,
            release: self.release.ok_or(ScaffoldError::Catalog)?,
            socket_env: self.socket_env.ok_or(ScaffoldError::Catalog)?,
        })
    }
}
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct ShouldWrite(bool);
#[derive(Clone, Debug, newtype::AsRefStr, newtype::BoundedString, newtype::Display)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ScaffoldText(String);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct ScaffoldTextRef<'text_lt>(&'text_lt str);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct StdScaffoldPathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct ReplacementsRef<'replacements_lt>(&'replacements_lt [(&'replacements_lt str, String)]);
#[derive(Clone, Copy, newtype::FromInner)]
struct CargoArgsRef<'args_lt>(&'args_lt [&'args_lt str]);
#[derive(Clone, Copy, newtype::FromInner)]
struct UpdateEnvName(&'static str);
#[derive(Clone, Copy)]
enum GeneratedProjection {
    CodeStyle,
    Config,
}
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct ShouldSkip(bool);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct StdScaffoldIoError(std::io::Error);
#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct ServerRuntimeBoundedReadError(server_runtime_http::BoundedReadError);

#[derive(Debug, thiserror::Error)]
enum ScaffoldError {
    #[error(
        "usage: workspace-scaffold project <snake_case_name> <repository_url> | service <snake_case_name> <port> | generate <sync|check> | deployment <sync|check>"
    )]
    Arguments,
    #[error("deployment service catalog is invalid")]
    Catalog,
    #[error("generated code-style snapshots are not synchronized")]
    GeneratedCodeStyle,
    #[error("generated configuration projections are not synchronized")]
    GeneratedConfig,
    #[error("generated deployment projections are not synchronized")]
    GeneratedDeployment,
    #[error("workspace operation failed: {0}")]
    Io(#[from] StdScaffoldIoError),
    #[error("workspace file does not contain the expected template marker")]
    Marker,
    #[error("project or service name must be non-empty lowercase snake_case ASCII")]
    ProjectName,
    #[error("workspace content read failed: {0}")]
    Read(#[from] ServerRuntimeBoundedReadError),
    #[error("repository URL must use https:// and must not end with /")]
    RepositoryUrl,
    #[error("service destination already exists")]
    ServiceExists,
    #[error("service port must be greater than zero")]
    ServicePort,
}
impl From<std::io::Error> for ScaffoldError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(StdScaffoldIoError::from(value))
    }
}

fn validate_project_name(value: ProjectNameRef<'_>) -> Result<(), ScaffoldError> {
    let text = value.0;
    if text.is_empty()
        || text.starts_with('_')
        || text.ends_with('_')
        || text.contains(str_constants::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE)
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(ScaffoldError::ProjectName);
    }
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "project command owns repository URL validation"
)]
fn validate_repository_url(value: RepositoryUrlRef<'_>) -> Result<(), ScaffoldError> {
    if !value.0.starts_with(str_constants::HTTPS_SCHEME_PREFIX) || value.0.ends_with('/') {
        return Err(ScaffoldError::RepositoryUrl);
    }
    Ok(())
}

fn kebab_case(value: ProjectNameRef<'_>) -> ScaffoldText {
    ScaffoldText::try_from(value.0.replace('_', str_constants::HYPHEN))
        .unwrap_or_else(ScaffoldText::from)
}

fn title_case(value: ProjectNameRef<'_>) -> ScaffoldText {
    let output = value
        .0
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect::<String>()
            })
        })
        .collect::<Vec<String>>()
        .join(str_constants::SPACE);
    ScaffoldText::try_from(output).unwrap_or_else(ScaffoldText::from)
}

#[allow(
    clippy::single_call_fn,
    reason = "service scaffold owns identifier case conversion"
)]
fn upper_camel_case(value: ProjectNameRef<'_>) -> ScaffoldText {
    ScaffoldText::try_from(
        title_case(value)
            .as_ref()
            .replace(' ', str_constants::EMPTY),
    )
    .unwrap_or_else(ScaffoldText::from)
}

#[allow(
    clippy::single_call_fn,
    reason = "identity traversal owns ignored directory policy"
)]
fn should_skip(path: StdScaffoldPathRef<'_>) -> ShouldSkip {
    ShouldSkip::from(path.0.components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(
                str_constants::GIT
                    | str_constants::TARGET
                    | str_constants::WORKSPACE_SCAFFOLD_NODE_MODULES
            )
        )
    }))
}

fn read_bounded_text(
    path: StdScaffoldPathRef<'_>,
) -> Result<ScaffoldText, ServerRuntimeBoundedReadError> {
    let bytes = server_runtime_http::read_bounded_file(
        server_runtime_http::StdPathRef::from(path.0),
        server_runtime_http::BoundedReadMaximumBytes::from(SCAFFOLD_TEXT_MAX_BYTES),
    )
    .map_err(ServerRuntimeBoundedReadError::from)?;
    let text = server_runtime_http::BoundedText::try_from(bytes)
        .map_err(ServerRuntimeBoundedReadError::from)?
        .into_inner();
    Ok(ScaffoldText::try_from(text).unwrap_or_else(ScaffoldText::from))
}

fn replace_file(
    path: StdScaffoldPathRef<'_>,
    replacements: ReplacementsRef<'_>,
) -> Result<(), ScaffoldError> {
    let Ok(contents) = read_bounded_text(path) else {
        return Ok(());
    };
    let updated_contents = replacements
        .0
        .iter()
        .fold(contents.as_ref().to_owned(), |value, (from, to)| {
            value.replace(from, to.as_str())
        });
    std::fs::write(path.0, updated_contents)?;
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "project command owns identity traversal"
)]
fn rename_identity(
    root: StdScaffoldPathRef<'_>,
    project_name: ProjectNameRef<'_>,
    repository_url: RepositoryUrlRef<'_>,
) -> Result<(), ScaffoldError> {
    let replacements = [
        (
            str_constants::WORKSPACE_SCAFFOLD_TEMPLATE_REPOSITORY_URL,
            repository_url.0.to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE,
            project_name.0.to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_KEBAB,
            kebab_case(project_name).as_ref().to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_TITLE,
            title_case(project_name).as_ref().to_owned(),
        ),
    ];
    let mut pending = vec![root.0.to_path_buf()];
    while let Some(path) = pending.pop() {
        if bool::from(should_skip(StdScaffoldPathRef::from(path.as_path()))) {
            continue;
        }
        if path.is_dir() {
            std::fs::read_dir(path)?.try_for_each(|entry| {
                pending.push(entry?.path());
                Ok::<(), std::io::Error>(())
            })?;
        } else {
            replace_file(
                StdScaffoldPathRef::from(path.as_path()),
                ReplacementsRef::from(replacements.as_slice()),
            )?;
        }
    }
    Ok(())
}

fn copy_template_tree(
    source: StdScaffoldPathRef<'_>,
    destination: StdScaffoldPathRef<'_>,
    replacements: ReplacementsRef<'_>,
) -> Result<(), ScaffoldError> {
    std::fs::create_dir_all(destination.0)?;
    std::fs::read_dir(source.0)?.try_for_each(|entry_result| {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.0.join(entry.file_name());
        if source_path.is_dir() {
            copy_template_tree(
                StdScaffoldPathRef::from(source_path.as_path()),
                StdScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        } else {
            let _copied_bytes = std::fs::copy(source_path, destination_path.as_path())?;
            replace_file(
                StdScaffoldPathRef::from(destination_path.as_path()),
                replacements,
            )
        }
    })?;
    Ok(())
}

fn insert_once(
    path: StdScaffoldPathRef<'_>,
    marker: ScaffoldTextRef<'_>,
    replacement: ScaffoldTextRef<'_>,
) -> Result<(), ScaffoldError> {
    let contents = read_bounded_text(path)?;
    if contents.as_ref().contains(replacement.0) {
        return Ok(());
    }
    let updated = contents.as_ref().replacen(marker.0, replacement.0, 1usize);
    if updated == contents.as_ref() {
        return Err(ScaffoldError::Marker);
    }
    std::fs::write(path.0, updated)?;
    Ok(())
}

fn catalog_string_value(
    line: ScaffoldTextRef<'_>,
    key: ScaffoldTextRef<'_>,
) -> Result<Option<ScaffoldText>, ScaffoldError> {
    line.0
        .strip_prefix(key.0)
        .and_then(|value| value.trim().strip_prefix('='))
        .map(str::trim)
        .and_then(|value| value.strip_prefix('"'))
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_owned)
        .map(ScaffoldText::try_from)
        .transpose()
        .map_err(|_error| ScaffoldError::Catalog)
}
#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns catalog parsing"
)]
fn parse_service_catalog(
    source: ScaffoldTextRef<'_>,
) -> Result<ServiceCatalogEntries, ScaffoldError> {
    let mut entries = Vec::new();
    let mut current = None;
    source.0.lines().try_for_each(|raw_line| {
        let trimmed_line = raw_line.trim();
        if trimmed_line == "[[service]]" {
            if let Some(draft) = current.take() {
                entries.push(ServiceCatalogDraft::finish(draft)?);
            }
            current = Some(ServiceCatalogDraft::default());
            return Ok(());
        }
        let Some(draft) = current.as_mut() else {
            return Ok(());
        };
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("crate"),
        )? {
            draft.crate_name = Some(
                ServiceCrate::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("compose"),
        )? {
            draft.compose_name = Some(
                ServiceComposeName::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("compose_file"),
        )? {
            draft.compose_file = Some(
                ServiceComposeFile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("dockerfile"),
        )? {
            draft.dockerfile = Some(
                ServiceDockerfile::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("image"),
        )? {
            draft.image = Some(
                ServiceImage::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("kubernetes"),
        )? {
            draft.kubernetes_manifest = Some(
                ServiceKubernetesManifest::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(port) = trimmed_line
            .strip_prefix("port")
            .and_then(|port_text| port_text.trim().strip_prefix('='))
            .map(str::trim)
            .and_then(|port_text| port_text.parse::<u16>().ok())
        {
            draft.port = Some(ServicePort::from(port));
            return Ok(());
        }
        if let Some(value) = catalog_string_value(
            ScaffoldTextRef::from(trimmed_line),
            ScaffoldTextRef::from("socket_env"),
        )? {
            draft.socket_env = Some(
                ServiceSocketEnv::try_from(value.as_ref().to_owned())
                    .map_err(|_error| ScaffoldError::Catalog)?,
            );
            return Ok(());
        }
        if let Some(release) = trimmed_line
            .strip_prefix("release")
            .and_then(|release_text| {
                release_text
                    .trim()
                    .strip_prefix('=')
                    .map(str::trim)
                    .and_then(|parsed_text| parsed_text.parse::<bool>().ok())
            })
        {
            draft.release = Some(ShouldRelease::from(release));
        }
        Ok::<(), ScaffoldError>(())
    })?;
    if let Some(draft) = current {
        entries.push(ServiceCatalogDraft::finish(draft)?);
    }
    if entries.is_empty() {
        return Err(ScaffoldError::Catalog);
    }
    Ok(ServiceCatalogEntries::from(
        bounded_types::BoundedVec::from_max_iter(entries),
    ))
}
#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns the CI projection"
)]
fn render_ci_service_matrix(entries: ServiceCatalogEntriesRef<'_>) -> ScaffoldText {
    ScaffoldText::try_from(
        entries
            .0
            .iter()
            .filter(|entry| bool::from(entry.release))
            .fold(String::new(), |mut output, entry| {
                output.push_str(str_constants::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT);
                output.push_str(entry.image.as_ref());
                output.push_str(str_constants::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT);
                output.push_str(entry.dockerfile.as_ref());
                output.push('\n');
                output
            }),
    )
    .unwrap_or_else(ScaffoldText::from)
}
#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns the release projection"
)]
fn render_release_matrix(entries: ServiceCatalogEntriesRef<'_>) -> ScaffoldText {
    ScaffoldText::try_from(
        entries
            .0
            .iter()
            .filter(|entry| bool::from(entry.release))
            .fold(String::new(), |mut output, entry| {
                output.push_str(str_constants::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT);
                output.push_str(entry.image.as_ref());
                output.push_str(str_constants::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT);
                output.push_str(entry.dockerfile.as_ref());
                output.push('\n');
                output
            }),
    )
    .unwrap_or_else(ScaffoldText::from)
}
#[allow(
    clippy::single_call_fn,
    reason = "catalog validation keeps path traversal checks explicit and typed"
)]
fn catalog_path_is_safe(path: StdScaffoldPathRef<'_>) -> IsCatalogPathSafe {
    IsCatalogPathSafe::from(
        path.0.is_relative()
            && path
                .0
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_))),
    )
}
#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization validates every non-generated catalog consumer"
)]
fn validate_deployment_representations(
    root: StdScaffoldPathRef<'_>,
    entries: ServiceCatalogEntriesRef<'_>,
) -> Result<(), ScaffoldError> {
    entries.0.iter().try_for_each(|entry| {
        if ![
            entry.crate_name.as_ref(),
            entry.compose_file.as_ref(),
            entry.dockerfile.as_ref(),
            entry.kubernetes_manifest.as_ref(),
        ]
        .into_iter()
        .all(|path| {
            bool::from(catalog_path_is_safe(StdScaffoldPathRef::from(
                std::path::Path::new(path),
            )))
        }) {
            return Err(ScaffoldError::Catalog);
        }
        if !root
            .0
            .join(entry.crate_name.as_ref())
            .join(str_constants::CARGO_TOML)
            .is_file()
            || !root.0.join(entry.dockerfile.as_ref()).is_file()
        {
            return Err(ScaffoldError::GeneratedDeployment);
        }
        let compose_path = root.0.join(entry.compose_file.as_ref());
        let compose = read_bounded_text(StdScaffoldPathRef::from(compose_path.as_path()))?;
        let port = entry.port.0;
        if !compose
            .as_ref()
            .contains(format!("  {}:\n", entry.compose_name.as_ref()).as_str())
            || !compose
                .as_ref()
                .contains(format!("dockerfile: {}", entry.dockerfile.as_ref()).as_str())
            || !compose
                .as_ref()
                .contains(format!("127.0.0.1:{port}:{port}").as_str())
        {
            return Err(ScaffoldError::GeneratedDeployment);
        }
        let kubernetes_path = root.0.join(entry.kubernetes_manifest.as_ref());
        let kubernetes = read_bounded_text(StdScaffoldPathRef::from(kubernetes_path.as_path()))?;
        if !kubernetes
            .as_ref()
            .contains(format!("image: {}:", entry.image.as_ref()).as_str())
            || !kubernetes
                .as_ref()
                .contains(format!("containerPort: {port}").as_str())
            || !kubernetes
                .as_ref()
                .contains(format!("port: {port}").as_str())
        {
            return Err(ScaffoldError::GeneratedDeployment);
        }
        Ok(())
    })
}
#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns all per-service generated sections"
)]
fn synchronize_service_deployment_sections(
    root: StdScaffoldPathRef<'_>,
    entry: &ServiceCatalogEntry,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let compose_path = root.0.join(entry.compose_file.as_ref());
    let compose_identity_begin = format!(
        "  # BEGIN GENERATED COMPOSE IDENTITY {}\n",
        entry.compose_name.as_ref()
    );
    let compose_identity_end = format!(
        "  # END GENERATED COMPOSE IDENTITY {}\n",
        entry.compose_name.as_ref()
    );
    let compose_identity = format!(
        "  {}:\n    build:\n      context: .\n      dockerfile: {}\n",
        entry.compose_name.as_ref(),
        entry.dockerfile.as_ref()
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(compose_path.as_path()),
        ScaffoldTextRef::from(compose_identity_begin.as_str()),
        ScaffoldTextRef::from(compose_identity_end.as_str()),
        ScaffoldTextRef::from(compose_identity.as_str()),
        write_changes,
    )?;
    let compose_socket_begin = format!(
        "      # BEGIN GENERATED COMPOSE SOCKET {}\n",
        entry.compose_name.as_ref()
    );
    let compose_socket_end = format!(
        "      # END GENERATED COMPOSE SOCKET {}\n",
        entry.compose_name.as_ref()
    );
    let compose_socket = format!(
        "      {}: \"0.0.0.0:{}\"\n",
        entry.socket_env.as_ref(),
        entry.port.0
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(compose_path.as_path()),
        ScaffoldTextRef::from(compose_socket_begin.as_str()),
        ScaffoldTextRef::from(compose_socket_end.as_str()),
        ScaffoldTextRef::from(compose_socket.as_str()),
        write_changes,
    )?;
    let ready_path =
        <common_routes::HealthReadyRoute as frontend_contract::TypedRoute>::metadata().path();
    let compose_health_begin = format!(
        "      # BEGIN GENERATED COMPOSE HEALTH {}\n",
        entry.compose_name.as_ref()
    );
    let compose_health_end = format!(
        "      # END GENERATED COMPOSE HEALTH {}\n",
        entry.compose_name.as_ref()
    );
    let compose_health = format!(
        "      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:{}{}\"]\n",
        entry.port.0,
        ready_path.as_ref()
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(compose_path.as_path()),
        ScaffoldTextRef::from(compose_health_begin.as_str()),
        ScaffoldTextRef::from(compose_health_end.as_str()),
        ScaffoldTextRef::from(compose_health.as_str()),
        write_changes,
    )?;
    let compose_port_begin = format!(
        "    # BEGIN GENERATED COMPOSE PORT {}\n",
        entry.compose_name.as_ref()
    );
    let compose_port_end = format!(
        "    # END GENERATED COMPOSE PORT {}\n",
        entry.compose_name.as_ref()
    );
    let compose_port = format!("    ports:\n      - \"127.0.0.1:{0}:{0}\"\n", entry.port.0);
    synchronize_generated_file(
        StdScaffoldPathRef::from(compose_path.as_path()),
        ScaffoldTextRef::from(compose_port_begin.as_str()),
        ScaffoldTextRef::from(compose_port_end.as_str()),
        ScaffoldTextRef::from(compose_port.as_str()),
        write_changes,
    )?;

    let kubernetes_path = root.0.join(entry.kubernetes_manifest.as_ref());
    let kubernetes_metadata_begin = format!(
        "# BEGIN GENERATED KUBERNETES METADATA {}\n",
        entry.image.as_ref()
    );
    let kubernetes_metadata_end = format!(
        "# END GENERATED KUBERNETES METADATA {}\n",
        entry.image.as_ref()
    );
    let kubernetes_metadata = format!(
        "metadata:\n  name: {0}\n  namespace: rust-workspace-template\n",
        entry.image.as_ref()
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(kubernetes_path.as_path()),
        ScaffoldTextRef::from(kubernetes_metadata_begin.as_str()),
        ScaffoldTextRef::from(kubernetes_metadata_end.as_str()),
        ScaffoldTextRef::from(kubernetes_metadata.as_str()),
        write_changes,
    )?;
    let kubernetes_workload_identity_begin = format!(
        "  # BEGIN GENERATED KUBERNETES WORKLOAD IDENTITY {}\n",
        entry.image.as_ref()
    );
    let kubernetes_workload_identity_end = format!(
        "  # END GENERATED KUBERNETES WORKLOAD IDENTITY {}\n",
        entry.image.as_ref()
    );
    let kubernetes_workload_identity = format!(
        "  selector:\n    matchLabels:\n      app.kubernetes.io/name: {0}\n  template:\n    metadata:\n      labels:\n        app.kubernetes.io/name: {0}\n    spec:\n",
        entry.image.as_ref()
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(kubernetes_path.as_path()),
        ScaffoldTextRef::from(kubernetes_workload_identity_begin.as_str()),
        ScaffoldTextRef::from(kubernetes_workload_identity_end.as_str()),
        ScaffoldTextRef::from(kubernetes_workload_identity.as_str()),
        write_changes,
    )?;
    let kubernetes_container_begin = format!(
        "      # BEGIN GENERATED KUBERNETES CONTAINER {}\n",
        entry.image.as_ref()
    );
    let kubernetes_container_end = format!(
        "      # END GENERATED KUBERNETES CONTAINER {}\n",
        entry.image.as_ref()
    );
    let kubernetes_container = format!(
        "      containers:\n        - name: {0}\n          image: {0}:replace-with-immutable-tag\n          envFrom:\n            - configMapRef:\n                name: {0}-config\n            - secretRef:\n                name: {0}-secrets\n          ports:\n            - containerPort: {1}\n              name: http\n",
        entry.image.as_ref(),
        entry.port.0
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(kubernetes_path.as_path()),
        ScaffoldTextRef::from(kubernetes_container_begin.as_str()),
        ScaffoldTextRef::from(kubernetes_container_end.as_str()),
        ScaffoldTextRef::from(kubernetes_container.as_str()),
        write_changes,
    )?;
    let live_path =
        <common_routes::HealthLiveRoute as frontend_contract::TypedRoute>::metadata().path();
    let kubernetes_probe_begin = format!(
        "          # BEGIN GENERATED KUBERNETES PROBES {}\n",
        entry.image.as_ref()
    );
    let kubernetes_probe_end = format!(
        "          # END GENERATED KUBERNETES PROBES {}\n",
        entry.image.as_ref()
    );
    let kubernetes_probe = format!(
        "          startupProbe:\n            httpGet:\n              path: {ready}\n              port: http\n            failureThreshold: 30\n            periodSeconds: 2\n          readinessProbe:\n            httpGet:\n              path: {ready}\n              port: http\n            periodSeconds: 5\n          livenessProbe:\n            httpGet:\n              path: {live}\n              port: http\n            periodSeconds: 10\n",
        ready = ready_path.as_ref(),
        live = live_path.as_ref()
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(kubernetes_path.as_path()),
        ScaffoldTextRef::from(kubernetes_probe_begin.as_str()),
        ScaffoldTextRef::from(kubernetes_probe_end.as_str()),
        ScaffoldTextRef::from(kubernetes_probe.as_str()),
        write_changes,
    )?;
    let kubernetes_service_identity_begin = format!(
        "# BEGIN GENERATED KUBERNETES SERVICE IDENTITY {}\n",
        entry.image.as_ref()
    );
    let kubernetes_service_identity_end = format!(
        "# END GENERATED KUBERNETES SERVICE IDENTITY {}\n",
        entry.image.as_ref()
    );
    let kubernetes_service_identity = format!(
        "metadata:\n  name: {0}\n  namespace: rust-workspace-template\nspec:\n  selector:\n    app.kubernetes.io/name: {0}\n",
        entry.image.as_ref()
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(kubernetes_path.as_path()),
        ScaffoldTextRef::from(kubernetes_service_identity_begin.as_str()),
        ScaffoldTextRef::from(kubernetes_service_identity_end.as_str()),
        ScaffoldTextRef::from(kubernetes_service_identity.as_str()),
        write_changes,
    )?;
    let kubernetes_service_port_begin = format!(
        "  # BEGIN GENERATED KUBERNETES SERVICE PORT {}\n",
        entry.image.as_ref()
    );
    let kubernetes_service_port_end = format!(
        "  # END GENERATED KUBERNETES SERVICE PORT {}\n",
        entry.image.as_ref()
    );
    let kubernetes_service_port = format!(
        "  ports:\n    - name: http\n      port: {}\n      targetPort: http\n",
        entry.port.0
    );
    synchronize_generated_file(
        StdScaffoldPathRef::from(kubernetes_path.as_path()),
        ScaffoldTextRef::from(kubernetes_service_port_begin.as_str()),
        ScaffoldTextRef::from(kubernetes_service_port_end.as_str()),
        ScaffoldTextRef::from(kubernetes_service_port.as_str()),
        write_changes,
    )
}
#[allow(
    clippy::single_call_fn,
    reason = "generated file synchronization owns marker replacement"
)]
fn replace_generated_section(
    source: ScaffoldTextRef<'_>,
    begin: ScaffoldTextRef<'_>,
    end: ScaffoldTextRef<'_>,
    generated: ScaffoldTextRef<'_>,
) -> Result<ScaffoldText, ScaffoldError> {
    let (prefix, after_begin) = source.0.split_once(begin.0).ok_or(ScaffoldError::Marker)?;
    let (_previous, suffix) = after_begin.split_once(end.0).ok_or(ScaffoldError::Marker)?;
    ScaffoldText::try_from(format!(
        "{prefix}{}{generated}{}{suffix}",
        begin.0,
        end.0,
        generated = generated.0
    ))
    .map_err(|_error| ScaffoldError::Catalog)
}
fn synchronize_generated_file(
    path: StdScaffoldPathRef<'_>,
    begin: ScaffoldTextRef<'_>,
    end: ScaffoldTextRef<'_>,
    generated: ScaffoldTextRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let source = read_bounded_text(path)?;
    let expected = replace_generated_section(
        ScaffoldTextRef::from(source.as_ref()),
        begin,
        end,
        generated,
    )?;
    if expected.as_ref() == source.as_ref() {
        return Ok(());
    }
    if bool::from(write_changes) {
        std::fs::write(path.0, expected.as_ref())?;
        Ok(())
    } else {
        Err(ScaffoldError::GeneratedDeployment)
    }
}
#[allow(
    clippy::single_call_fn,
    reason = "the deployment command owns all generated projections"
)]
fn synchronize_deployment_projections(
    root: StdScaffoldPathRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let catalog_path = root.0.join("deploy/services.toml");
    let catalog = read_bounded_text(StdScaffoldPathRef::from(catalog_path.as_path()))?;
    let entries = parse_service_catalog(ScaffoldTextRef::from(catalog.as_ref()))?;
    let entries_ref = ServiceCatalogEntriesRef::from(entries.0.as_slice());
    let ci = render_ci_service_matrix(entries_ref);
    let release = render_release_matrix(entries_ref);
    let ci_path = root.0.join(".github/workflows/ci.yml");
    synchronize_generated_file(
        StdScaffoldPathRef::from(ci_path.as_path()),
        ScaffoldTextRef::from("          # BEGIN GENERATED SERVICE MATRIX\n"),
        ScaffoldTextRef::from("          # END GENERATED SERVICE MATRIX\n"),
        ScaffoldTextRef::from(ci.as_ref()),
        write_changes,
    )?;
    let release_path = root.0.join(".github/workflows/release.yml");
    synchronize_generated_file(
        StdScaffoldPathRef::from(release_path.as_path()),
        ScaffoldTextRef::from("          # BEGIN GENERATED RELEASE MATRIX\n"),
        ScaffoldTextRef::from("          # END GENERATED RELEASE MATRIX\n"),
        ScaffoldTextRef::from(release.as_ref()),
        write_changes,
    )?;
    entries_ref.0.iter().try_for_each(|entry| {
        synchronize_service_deployment_sections(root, entry, write_changes)
    })?;
    validate_deployment_representations(root, entries_ref)
}
#[allow(
    clippy::single_call_fn,
    reason = "the aggregate generation command delegates snapshot ownership to code-style tests"
)]
fn synchronize_code_style_snapshots(
    root: StdScaffoldPathRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    synchronize_cargo_owned_projection(
        root,
        CargoArgsRef::from(
            &[
                str_constants::TEST_ALT_3,
                str_constants::P,
                "tests",
                "code_style",
            ][..],
        ),
        UpdateEnvName::from(str_constants::UPDATE_CODE_STYLE_SNAPSHOTS),
        GeneratedProjection::CodeStyle,
        write_changes,
    )
}

fn synchronize_cargo_owned_projection(
    root: StdScaffoldPathRef<'_>,
    arguments: CargoArgsRef<'_>,
    update_environment: UpdateEnvName,
    projection: GeneratedProjection,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let mut command = macros_helpers::tool_command::ToolCommand::new(
        macros_helpers::tool_command::ToolProgramRef::from("cargo"),
    );
    let _arguments = command
        .current_dir(macros_helpers::tool_command::StdPathRef::from(root.0))
        .args(macros_helpers::tool_command::ToolArgsRef::from(arguments.0));
    if bool::from(write_changes) {
        let _environment = command.env(
            macros_helpers::tool_command::ToolEnvKeyRef::from(update_environment.0),
            macros_helpers::tool_command::ToolEnvValueRef::from("1"),
        );
    }
    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(match projection {
            GeneratedProjection::CodeStyle => ScaffoldError::GeneratedCodeStyle,
            GeneratedProjection::Config => ScaffoldError::GeneratedConfig,
        })
    }
}

#[allow(
    clippy::single_call_fn,
    reason = "the aggregate generation command delegates environment projection ownership to config crates"
)]
fn synchronize_config_projections(
    root: StdScaffoldPathRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    synchronize_cargo_owned_projection(
        root,
        CargoArgsRef::from(
            &[
                str_constants::TEST_ALT_3,
                str_constants::P,
                "server_config",
                str_constants::P,
                "notification_service_config",
                "--tests",
            ][..],
        ),
        UpdateEnvName::from(str_constants::UPDATE_CONFIG_PROJECTIONS),
        GeneratedProjection::Config,
        write_changes,
    )
}

#[allow(
    clippy::single_call_fn,
    reason = "the generate command exposes one aggregate synchronization boundary"
)]
fn synchronize_all_generated_artifacts(
    root: StdScaffoldPathRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    synchronize_deployment_projections(root, write_changes)?;
    synchronize_config_projections(root, write_changes)?;
    synchronize_code_style_snapshots(root, write_changes)
}

#[allow(
    clippy::single_call_fn,
    reason = "service command owns complete scaffold composition"
)]
fn scaffold_service(
    root: StdScaffoldPathRef<'_>,
    service_name: ProjectNameRef<'_>,
    port: ServicePort,
) -> Result<(), ScaffoldError> {
    validate_project_name(service_name)?;
    if port.0 == 0u16 {
        return Err(ScaffoldError::ServicePort);
    }
    let service = service_name.0;
    let config = format!("{service}_config");
    let contract = format!("{service}_contract");
    if [service, config.as_str(), contract.as_str()]
        .iter()
        .any(|path| root.0.join(path).exists())
    {
        return Err(ScaffoldError::ServiceExists);
    }
    let kebab = kebab_case(service_name);
    let upper_snake = service.to_ascii_uppercase();
    let replacements = [
        (
            str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE,
            service.to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE_KEBAB,
            kebab.as_ref().to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_UPPER,
            upper_snake.clone(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_TITLE,
            upper_camel_case(service_name).as_ref().to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_LOWER,
            service.to_owned(),
        ),
        (
            str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_PORT,
            port.0.to_string(),
        ),
    ];
    copy_template_tree(
        StdScaffoldPathRef::from(
            root.0
                .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE)
                .as_path(),
        ),
        StdScaffoldPathRef::from(root.0.join(service).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    copy_template_tree(
        StdScaffoldPathRef::from(
            root.0
                .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                .as_path(),
        ),
        StdScaffoldPathRef::from(root.0.join(config.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    copy_template_tree(
        StdScaffoldPathRef::from(
            root.0
                .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_CONTRACT)
                .as_path(),
        ),
        StdScaffoldPathRef::from(root.0.join(contract.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;

    let manifest = root.0.join(str_constants::CARGO_TOML);
    insert_once(
        StdScaffoldPathRef::from(manifest.as_path()),
        ScaffoldTextRef::from(str_constants::WORKSPACE_SCAFFOLD_MANIFEST_MEMBER_MARKER),
        ScaffoldTextRef::from(
            format!(
                "  \"notification_service_contract\",\n  \"{service}\",\n  \"{config}\",\n  \"{contract}\","
            )
            .as_str(),
        ),
    )?;
    let dependency_marker = str_constants::WORKSPACE_SCAFFOLD_MANIFEST_DEPENDENCY_MARKER;
    insert_once(
        StdScaffoldPathRef::from(manifest.as_path()),
        ScaffoldTextRef::from(dependency_marker),
        ScaffoldTextRef::from(
            format!(
                "{dependency_marker}\n{service} = {{ path = \"./{service}\" }}\n{config} = {{ path = \"./{config}\" }}\n{contract} = {{ path = \"./{contract}\" }}"
            )
            .as_str(),
        ),
    )?;

    let k8s_source = root
        .0
        .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_K8S_PATH);
    let k8s_file_name = format!("{kebab}.yaml");
    let k8s_destination = root
        .0
        .join(str_constants::WORKSPACE_SCAFFOLD_K8S_BASE_PATH)
        .join(k8s_file_name.as_str());
    let _copied_bytes = std::fs::copy(k8s_source, k8s_destination.as_path())?;
    replace_file(
        StdScaffoldPathRef::from(k8s_destination.as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    let mut k8s_contents = read_bounded_text(StdScaffoldPathRef::from(k8s_destination.as_path()))?
        .as_ref()
        .to_owned();
    k8s_contents.push_str(
        format!(
            "\n---\napiVersion: networking.k8s.io/v1\nkind: NetworkPolicy\nmetadata:\n  name: {kebab}-access\n  namespace: rust-workspace-template\nspec:\n  podSelector:\n    matchLabels:\n      app.kubernetes.io/name: {kebab}\n  ingress:\n    - from:\n        - podSelector:\n            matchLabels:\n              app.kubernetes.io/name: application\n      ports:\n        - protocol: TCP\n          port: {port}\n  egress:\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: database\n          podSelector:\n            matchLabels:\n              app.kubernetes.io/name: {kebab}-postgresql\n      ports:\n        - protocol: TCP\n          port: 5432\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: kube-system\n          podSelector:\n            matchLabels:\n              k8s-app: kube-dns\n      ports:\n        - protocol: UDP\n          port: 53\n        - protocol: TCP\n          port: 53\n  policyTypes: [\"Ingress\", \"Egress\"]\n",
            port = port.0,
        )
        .as_str(),
    );
    std::fs::write(k8s_destination.as_path(), k8s_contents)?;
    let kustomization = root
        .0
        .join(str_constants::WORKSPACE_SCAFFOLD_KUSTOMIZATION_PATH);
    insert_once(
        StdScaffoldPathRef::from(kustomization.as_path()),
        ScaffoldTextRef::from(str_constants::WORKSPACE_SCAFFOLD_KUSTOMIZATION_MARKER),
        ScaffoldTextRef::from(
            format!("  - notification-service.yaml\n  - {k8s_file_name}").as_str(),
        ),
    )?;

    let config_example_path = root.0.join(config.as_str()).join(".env.example");
    let config_example =
        read_bounded_text(StdScaffoldPathRef::from(config_example_path.as_path()))?;
    let database_key = format!("{upper_snake}_DATABASE_URL");
    let socket_key = format!("{upper_snake}_SERVICE_SOCKET_ADDRESS");
    let compose_environment = config_example
        .as_ref()
        .lines()
        .map(|line| {
            let (key, example) = line.split_once('=').ok_or(ScaffoldError::Catalog)?;
            let value = if key == database_key {
                format!(
                    "postgres://{service}:${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}@{service}_database:5432/{service}"
                )
            } else if key == socket_key {
                format!("0.0.0.0:{}", port.0)
            } else {
                example.to_owned()
            };
            if key == socket_key {
                Ok(format!(
                    "      # BEGIN GENERATED COMPOSE SOCKET {service}\n      {key}: \"{value}\"\n      # END GENERATED COMPOSE SOCKET {service}\n"
                ))
            } else {
                Ok(format!("      {key}: \"{value}\"\n"))
            }
        })
        .collect::<Result<String, ScaffoldError>>()?;
    let ready_path =
        <common_routes::HealthReadyRoute as frontend_contract::TypedRoute>::metadata().path();
    let compose = format!(
        "services:\n  {service}_database:\n    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n    environment:\n      POSTGRES_DB: {service}\n      POSTGRES_USER: {service}\n      POSTGRES_PASSWORD: ${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}\n    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U {service} -d {service}\"]\n      interval: 5s\n      timeout: 3s\n      retries: 20\n    networks: [application]\n    volumes: [{service}_database_data:/var/lib/postgresql/data]\n  # BEGIN GENERATED COMPOSE IDENTITY {service}\n  {service}:\n    build:\n      context: .\n      dockerfile: {service}/Dockerfile\n  # END GENERATED COMPOSE IDENTITY {service}\n    depends_on:\n      {service}_database:\n        condition: service_healthy\n    environment:\n{environment}    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH {service}\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:{port}{ready_path}\"]\n      # END GENERATED COMPOSE HEALTH {service}\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n    networks: [application]\n    # BEGIN GENERATED COMPOSE PORT {service}\n    ports:\n      - \"127.0.0.1:{port}:{port}\"\n    # END GENERATED COMPOSE PORT {service}\n    read_only: true\n    restart: unless-stopped\n    tmpfs: [/tmp:size=16m,mode=1777]\nvolumes:\n  {service}_database_data:\n",
        port = port.0,
        environment = compose_environment,
        ready_path = ready_path.as_ref(),
    );
    std::fs::write(
        root.0.join(format!("docker-compose.{service}.yml")),
        compose,
    )?;

    let service_catalog = root
        .0
        .join(str_constants::WORKSPACE_SCAFFOLD_SERVICE_CATALOG_PATH);
    let mut service_catalog_contents =
        read_bounded_text(StdScaffoldPathRef::from(service_catalog.as_path()))?
            .as_ref()
            .to_owned();
    service_catalog_contents.push_str(
        format!(
            "\n[[service]]\ncrate = \"{service}\"\ncompose = \"{service}\"\ncompose_file = \"docker-compose.{service}.yml\"\ndockerfile = \"{service}/Dockerfile\"\nimage = \"{kebab}\"\nkubernetes = \"deploy/k8s/base/{k8s_file_name}\"\nport = {}\nrelease = false\nsocket_env = \"{upper_snake}_SERVICE_SOCKET_ADDRESS\"\n",
            port.0
        )
        .as_str(),
    );
    std::fs::write(service_catalog, service_catalog_contents)?;
    Ok(())
}

fn workspace_root() -> Result<StdScaffoldPathRef<'static>, ScaffoldError> {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(StdScaffoldPathRef::from)
        .ok_or(ScaffoldError::Arguments)
}

#[allow(
    clippy::single_call_fn,
    reason = "binary entry point delegates fallible argument handling"
)]
fn run() -> Result<(), ScaffoldError> {
    let mut arguments = std::env::args().skip(1usize);
    match arguments.next().as_deref() {
        Some(str_constants::WORKSPACE_SCAFFOLD_PROJECT_COMMAND) => {
            let name = arguments.next().ok_or(ScaffoldError::Arguments)?;
            let repository_url = arguments.next().ok_or(ScaffoldError::Arguments)?;
            if arguments.next().is_some() {
                return Err(ScaffoldError::Arguments);
            }
            let name_ref = ProjectNameRef::from(name.as_str());
            let repository_url_ref = RepositoryUrlRef::from(repository_url.as_str());
            validate_project_name(name_ref)?;
            validate_repository_url(repository_url_ref)?;
            rename_identity(workspace_root()?, name_ref, repository_url_ref)
        }
        Some(str_constants::SERVICE) => {
            let name = arguments.next().ok_or(ScaffoldError::Arguments)?;
            let port = match arguments
                .next()
                .ok_or(ScaffoldError::Arguments)?
                .parse::<u16>()
            {
                Ok(value) => ServicePort::from(value),
                Err(_error) => return Err(ScaffoldError::ServicePort),
            };
            if arguments.next().is_some() {
                return Err(ScaffoldError::Arguments);
            }
            scaffold_service(workspace_root()?, ProjectNameRef::from(name.as_str()), port)
        }
        Some("generate") => {
            let write_changes = match arguments.next().as_deref() {
                Some(str_constants::SYNC) => ShouldWrite::from(true),
                Some(str_constants::CHECK) => ShouldWrite::from(false),
                Some(_) | None => return Err(ScaffoldError::Arguments),
            };
            if arguments.next().is_some() {
                return Err(ScaffoldError::Arguments);
            }
            synchronize_all_generated_artifacts(workspace_root()?, write_changes)
        }
        Some("deployment") => {
            let write_changes = match arguments.next().as_deref() {
                Some(str_constants::SYNC) => ShouldWrite::from(true),
                Some(str_constants::CHECK) => ShouldWrite::from(false),
                Some(_) | None => return Err(ScaffoldError::Arguments),
            };
            if arguments.next().is_some() {
                return Err(ScaffoldError::Arguments);
            }
            synchronize_deployment_projections(workspace_root()?, write_changes)
        }
        Some(_) | None => Err(ScaffoldError::Arguments),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2i32);
    }
}

#[cfg(test)]
mod tests {
    fn assert_file_content(path: &std::path::Path, expected: &str) {
        let actual = std::fs::read_to_string(path).expect("371dbe92");
        assert_eq!(actual, expected, "239c17b0: {}", path.display());
    }

    fn write(path: &std::path::Path, value: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("2f0ad03a");
        }
        std::fs::write(path, value).expect("79af6dc8");
    }

    #[test]
    fn validates_and_converts_project_names() {
        let valid = super::ProjectNameRef::from("order_platform");
        super::validate_project_name(valid).expect("96de3a80");
        assert_eq!(super::kebab_case(valid).as_ref(), "order-platform");
        assert_eq!(super::title_case(valid).as_ref(), "Order Platform");
        assert_eq!(super::upper_camel_case(valid).as_ref(), "OrderPlatform");
        assert!(super::validate_project_name(super::ProjectNameRef("Order-Platform")).is_err());
    }

    #[test]
    fn requires_https_repository_url() {
        super::validate_repository_url(super::RepositoryUrlRef::from(
            "https://example.com/team/order_platform",
        ))
        .expect("28c1e7a4");
        assert!(
            super::validate_repository_url(super::RepositoryUrlRef(
                "http://example.com/team/order_platform"
            ))
            .is_err()
        );
    }

    #[test]
    fn deployment_projection_check_rejects_stale_generated_content() {
        let path = std::env::temp_dir().join(format!(
            "workspace-scaffold-generated-test-{}",
            std::process::id()
        ));
        let begin = "BEGIN GENERATED\n";
        let end = "END GENERATED\n";
        write(
            path.as_path(),
            "header\nBEGIN GENERATED\nstale\nEND GENERATED\n",
        );
        let check = super::synchronize_generated_file(
            super::StdScaffoldPathRef::from(path.as_path()),
            super::ScaffoldTextRef::from(begin),
            super::ScaffoldTextRef::from(end),
            super::ScaffoldTextRef::from("current\n"),
            super::ShouldWrite::from(false),
        );
        assert!(matches!(
            check,
            Err(super::ScaffoldError::GeneratedDeployment)
        ));
        super::synchronize_generated_file(
            super::StdScaffoldPathRef::from(path.as_path()),
            super::ScaffoldTextRef::from(begin),
            super::ScaffoldTextRef::from(end),
            super::ScaffoldTextRef::from("current\n"),
            super::ShouldWrite::from(true),
        )
        .expect("5a7e3c91");
        super::synchronize_generated_file(
            super::StdScaffoldPathRef::from(path.as_path()),
            super::ScaffoldTextRef::from(begin),
            super::ScaffoldTextRef::from(end),
            super::ScaffoldTextRef::from("current\n"),
            super::ShouldWrite::from(false),
        )
        .expect("d2f8b4a6");
        std::fs::remove_file(path).expect("9c1e6a3f");
    }

    #[test]
    fn service_catalog_owns_ci_and_release_projection_values() {
        let entries = super::parse_service_catalog(super::ScaffoldTextRef::from(
            "[[service]]\ncrate = \"server\"\ncompose = \"server\"\ncompose_file = \"docker-compose.yml\"\ndockerfile = \"Dockerfile\"\nimage = \"application\"\nkubernetes = \"deploy/k8s/base/application.yaml\"\nport = 8080\nrelease = true\nsocket_env = \"SERVICE_SOCKET_ADDRESS\"\n\n[[service]]\ncrate = \"worker\"\ncompose = \"worker\"\ncompose_file = \"docker-compose.worker.yml\"\ndockerfile = \"worker/Dockerfile\"\nimage = \"worker\"\nkubernetes = \"deploy/k8s/base/worker.yaml\"\nport = 8082\nrelease = false\nsocket_env = \"WORKER_SERVICE_SOCKET_ADDRESS\"\n",
        ))
        .expect("4e8b2d7a");
        let entries_ref = super::ServiceCatalogEntriesRef::from(entries.0.as_slice());
        assert_eq!(
            super::render_ci_service_matrix(entries_ref).as_ref(),
            "          - name: application\n            dockerfile: Dockerfile\n"
        );
        assert_eq!(
            super::render_release_matrix(entries_ref).as_ref(),
            "          - name: application\n            dockerfile: Dockerfile\n"
        );
    }

    #[test]
    fn rejects_scaffold_text_over_size_limit() {
        let path = std::env::temp_dir().join(format!(
            "workspace-scaffold-oversize-test-{}",
            std::process::id()
        ));
        std::fs::write(
            path.as_path(),
            vec![b'x'; super::SCAFFOLD_TEXT_MAX_BYTES.saturating_add(1usize)],
        )
        .expect("d97e30ac");
        let result = super::read_bounded_text(super::StdScaffoldPathRef::from(path.as_path()));
        assert!(
            matches!(
                result,
                Err(super::ServerRuntimeBoundedReadError(
                    server_runtime_http::BoundedReadError::ExceedsMaximum { .. }
                ))
            ),
            "8f32bc16"
        );
        std::fs::remove_file(path).expect("51cd7b2e");
    }

    #[test]
    fn service_scaffold_registers_all_artifacts() {
        let root =
            std::env::temp_dir().join(format!("workspace-scaffold-test-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(root.as_path()).expect("1449608d");
        }
        write(
            root.join("Cargo.toml").as_path(),
            "[workspace]\nmembers = [\n  \"notification_service_contract\",\n]\n[workspace.dependencies]\nnotification_service_contract = { path = \"./notification_service_contract\" }\n",
        );
        write(
            root.join("notification_service/src/main.rs").as_path(),
            "struct Notification; const PORT: u16 = 8081; fn insert_sql() -> &'static str { \"INSERT INTO notifications (id, message) VALUES ($1, $2)\" }",
        );
        write(
            root.join("notification_service_config/src/lib.rs")
                .as_path(),
            "struct NotificationConfig;",
        );
        write(
            root.join("notification_service_config/.env.example")
                .as_path(),
            "NOTIFICATION_DATABASE_URL=postgres://notification_service:change-me@127.0.0.1:5432/notification_service\nNOTIFICATION_SERVICE_SOCKET_ADDRESS=127.0.0.1:8081\nPG_POOL_MAX_CONNECTIONS=10\nREQUEST_TIMEOUT_SECONDS=30\nTRACING_FORMAT=text\n",
        );
        write(
            root.join("notification_service_contract/src/lib.rs")
                .as_path(),
            "struct NotificationContract;",
        );
        write(
            root.join("deploy/k8s/base/notification-service.yaml")
                .as_path(),
            "metadata:\n  name: notification-service\ncontainerPort: 8081\n",
        );
        write(
            root.join("deploy/k8s/base/kustomization.yaml").as_path(),
            "resources:\n  - notification-service.yaml\n",
        );
        write(
            root.join("deploy/services.toml").as_path(),
            "[[service]]\ncrate = \"notification_service\"\n",
        );
        super::scaffold_service(
            super::StdScaffoldPathRef::from(root.as_path()),
            super::ProjectNameRef::from("order_service"),
            super::ServicePort::from(8082u16),
        )
        .expect("4bff1d79");
        assert_file_content(
            root.join("Cargo.toml").as_path(),
            "[workspace]\nmembers = [\n  \"notification_service_contract\",\n  \"order_service\",\n  \"order_service_config\",\n  \"order_service_contract\",\n]\n[workspace.dependencies]\nnotification_service_contract = { path = \"./notification_service_contract\" }\norder_service = { path = \"./order_service\" }\norder_service_config = { path = \"./order_service_config\" }\norder_service_contract = { path = \"./order_service_contract\" }\n",
        );
        assert_file_content(
            root.join("order_service/src/main.rs").as_path(),
            "struct OrderService; const PORT: u16 = 8082; fn insert_sql() -> &'static str { \"INSERT INTO order_services (id, message) VALUES ($1, $2)\" }",
        );
        assert_file_content(
            root.join("order_service_config/src/lib.rs").as_path(),
            "struct OrderServiceConfig;",
        );
        assert_file_content(
            root.join("order_service_contract/src/lib.rs").as_path(),
            "struct OrderServiceContract;",
        );
        assert_file_content(
            root.join("deploy/k8s/base/order-service.yaml").as_path(),
            "metadata:\n  name: order-service\ncontainerPort: 8082\n\n---\napiVersion: networking.k8s.io/v1\nkind: NetworkPolicy\nmetadata:\n  name: order-service-access\n  namespace: rust-workspace-template\nspec:\n  podSelector:\n    matchLabels:\n      app.kubernetes.io/name: order-service\n  ingress:\n    - from:\n        - podSelector:\n            matchLabels:\n              app.kubernetes.io/name: application\n      ports:\n        - protocol: TCP\n          port: 8082\n  egress:\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: database\n          podSelector:\n            matchLabels:\n              app.kubernetes.io/name: order-service-postgresql\n      ports:\n        - protocol: TCP\n          port: 5432\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: kube-system\n          podSelector:\n            matchLabels:\n              k8s-app: kube-dns\n      ports:\n        - protocol: UDP\n          port: 53\n        - protocol: TCP\n          port: 53\n  policyTypes: [\"Ingress\", \"Egress\"]\n",
        );
        assert_file_content(
            root.join("deploy/k8s/base/kustomization.yaml").as_path(),
            "resources:\n  - notification-service.yaml\n  - order-service.yaml\n",
        );
        assert_file_content(
            root.join("docker-compose.order_service.yml").as_path(),
            "services:\n  order_service_database:\n    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n    environment:\n      POSTGRES_DB: order_service\n      POSTGRES_USER: order_service\n      POSTGRES_PASSWORD: ${ORDER_SERVICE_POSTGRES_PASSWORD:?set ORDER_SERVICE_POSTGRES_PASSWORD}\n    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U order_service -d order_service\"]\n      interval: 5s\n      timeout: 3s\n      retries: 20\n    networks: [application]\n    volumes: [order_service_database_data:/var/lib/postgresql/data]\n  # BEGIN GENERATED COMPOSE IDENTITY order_service\n  order_service:\n    build:\n      context: .\n      dockerfile: order_service/Dockerfile\n  # END GENERATED COMPOSE IDENTITY order_service\n    depends_on:\n      order_service_database:\n        condition: service_healthy\n    environment:\n      ORDER_SERVICE_DATABASE_URL: \"postgres://order_service:${ORDER_SERVICE_POSTGRES_PASSWORD:?set ORDER_SERVICE_POSTGRES_PASSWORD}@order_service_database:5432/order_service\"\n      # BEGIN GENERATED COMPOSE SOCKET order_service\n      ORDER_SERVICE_SERVICE_SOCKET_ADDRESS: \"0.0.0.0:8082\"\n      # END GENERATED COMPOSE SOCKET order_service\n      PG_POOL_MAX_CONNECTIONS: \"10\"\n      REQUEST_TIMEOUT_SECONDS: \"30\"\n      TRACING_FORMAT: \"text\"\n    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH order_service\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:8082/health/ready\"]\n      # END GENERATED COMPOSE HEALTH order_service\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n    networks: [application]\n    # BEGIN GENERATED COMPOSE PORT order_service\n    ports:\n      - \"127.0.0.1:8082:8082\"\n    # END GENERATED COMPOSE PORT order_service\n    read_only: true\n    restart: unless-stopped\n    tmpfs: [/tmp:size=16m,mode=1777]\nvolumes:\n  order_service_database_data:\n",
        );
        assert_file_content(
            root.join("deploy/services.toml").as_path(),
            "[[service]]\ncrate = \"notification_service\"\n\n[[service]]\ncrate = \"order_service\"\ncompose = \"order_service\"\ncompose_file = \"docker-compose.order_service.yml\"\ndockerfile = \"order_service/Dockerfile\"\nimage = \"order-service\"\nkubernetes = \"deploy/k8s/base/order-service.yaml\"\nport = 8082\nrelease = false\nsocket_env = \"ORDER_SERVICE_SERVICE_SOCKET_ADDRESS\"\n",
        );
        std::fs::remove_dir_all(root).expect("6f608418");
    }
}
