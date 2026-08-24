mod naming;
mod service_catalog;
mod template_fs;

const SCAFFOLD_TEXT_MAX_BYTES: usize = 16_777_216usize;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct ProjectNameRef<'value>(&'value str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct RepositoryUrlRef<'value>(&'value str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct ServicePort(u16);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceCrate(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceComposeName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceComposeFile(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceDockerfile(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceImage(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceKubernetesManifest(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ServiceSocketEnv(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct ServiceCatalogEntries(bounded_types::BoundedVec<ServiceCatalogEntry, 0, { usize::MAX }>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct ServiceCatalogEntriesRef<'entries_lt>(&'entries_lt [ServiceCatalogEntry]);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
struct ServiceCatalogEntry {
    compose_file: ServiceComposeFile,
    compose_name: ServiceComposeName,
    crate_name: ServiceCrate,
    dockerfile: ServiceDockerfile,
    image: ServiceImage,
    kubernetes_manifest: ServiceKubernetesManifest,
    socket_env: ServiceSocketEnv,
    port: ServicePort,
    release: ShouldRelease,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
struct ShouldRelease(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
struct IsCatalogPathSafe(bool);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
struct ServiceCatalogDraft {
    compose_file: Option<ServiceComposeFile>,
    compose_name: Option<ServiceComposeName>,
    crate_name: Option<ServiceCrate>,
    dockerfile: Option<ServiceDockerfile>,
    image: Option<ServiceImage>,
    kubernetes_manifest: Option<ServiceKubernetesManifest>,
    socket_env: Option<ServiceSocketEnv>,
    port: Option<ServicePort>,
    release: Option<ShouldRelease>,
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
struct ShouldWrite(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = SCAFFOLD_TEXT_MAX_BYTES)]
struct ScaffoldText(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct ScaffoldTextRef<'text_lt>(&'text_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct StdScaffoldPathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct ReplacementsRef<'replacements_lt>(&'replacements_lt [(&'replacements_lt str, String)]);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct CargoArgsRef<'args_lt>(&'args_lt [&'args_lt str]);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct UpdateEnvName(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
enum GeneratedProjection {
    CodeStyle,
    Config,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
struct ShouldSkip(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct StdScaffoldIoError(std::io::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ServerRuntimeBoundedReadError(server_runtime_http::BoundedReadError);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
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
        let compose =
            template_fs::read_bounded_text(StdScaffoldPathRef::from(compose_path.as_path()))?;
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
        let kubernetes =
            template_fs::read_bounded_text(StdScaffoldPathRef::from(kubernetes_path.as_path()))?;
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
        "metadata:\n  name: {0}\n  namespace: rust-workspace-template\n  labels:\n    app.kubernetes.io/name: {0}\nspec:\n  selector:\n    app.kubernetes.io/name: {0}\n",
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
    let source = template_fs::read_bounded_text(path)?;
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
    let catalog = template_fs::read_bounded_text(StdScaffoldPathRef::from(catalog_path.as_path()))?;
    let entries = service_catalog::parse(ScaffoldTextRef::from(catalog.as_ref()))?;
    let entries_ref = ServiceCatalogEntriesRef::from(entries.0.as_slice());
    let ci = service_catalog::render_ci_matrix(entries_ref);
    let release = service_catalog::render_release_matrix(entries_ref);
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
    naming::validate_project_name(service_name)?;
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
    let kebab = naming::kebab_case(service_name);
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
            naming::upper_camel_case(service_name).as_ref().to_owned(),
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
    template_fs::copy_template_tree(
        StdScaffoldPathRef::from(
            root.0
                .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE)
                .as_path(),
        ),
        StdScaffoldPathRef::from(root.0.join(service).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    template_fs::copy_template_tree(
        StdScaffoldPathRef::from(
            root.0
                .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                .as_path(),
        ),
        StdScaffoldPathRef::from(root.0.join(config.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    template_fs::copy_template_tree(
        StdScaffoldPathRef::from(
            root.0
                .join(str_constants::WORKSPACE_SCAFFOLD_NOTIFICATION_CONTRACT)
                .as_path(),
        ),
        StdScaffoldPathRef::from(root.0.join(contract.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;

    let manifest = root.0.join(str_constants::CARGO_TOML);
    template_fs::insert_once(
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
    template_fs::insert_once(
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
    template_fs::replace_file(
        StdScaffoldPathRef::from(k8s_destination.as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    let mut k8s_contents =
        template_fs::read_bounded_text(StdScaffoldPathRef::from(k8s_destination.as_path()))?
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
    template_fs::insert_once(
        StdScaffoldPathRef::from(kustomization.as_path()),
        ScaffoldTextRef::from(str_constants::WORKSPACE_SCAFFOLD_KUSTOMIZATION_MARKER),
        ScaffoldTextRef::from(
            format!("  - notification-service.yaml\n  - {k8s_file_name}").as_str(),
        ),
    )?;

    let config_example_path = root.0.join(config.as_str()).join(".env.example");
    let config_example =
        template_fs::read_bounded_text(StdScaffoldPathRef::from(config_example_path.as_path()))?;
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
        template_fs::read_bounded_text(StdScaffoldPathRef::from(service_catalog.as_path()))?
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
            naming::validate_project_name(name_ref)?;
            naming::validate_repository_url(repository_url_ref)?;
            template_fs::rename_identity(workspace_root()?, name_ref, repository_url_ref)
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
mod tests;
