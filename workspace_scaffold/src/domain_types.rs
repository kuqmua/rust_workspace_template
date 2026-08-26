#[path = "naming_capitalized_parts.rs"]
mod naming_capitalized_parts;
#[path = "naming_kebab_case.rs"]
pub(crate) mod naming_kebab_case;
#[path = "naming_title_case.rs"]
pub(crate) mod naming_title_case;
#[path = "naming_upper_camel_case.rs"]
pub(crate) mod naming_upper_camel_case;
#[path = "naming_validate_project_name.rs"]
pub(crate) mod naming_validate_project_name;
#[path = "naming_validate_repository_url.rs"]
pub(crate) mod naming_validate_repository_url;
#[path = "service_catalog_parse.rs"]
mod service_catalog_parse;
#[path = "service_catalog_render_ci_matrix.rs"]
mod service_catalog_render_ci_matrix;
#[path = "service_catalog_render_release_entries.rs"]
mod service_catalog_render_release_entries;
#[path = "service_catalog_render_release_matrix.rs"]
mod service_catalog_render_release_matrix;
#[path = "service_catalog_string_value.rs"]
mod service_catalog_string_value;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ProjectNameRef<'value>(&'value str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct RepositoryUrlRef<'value>(&'value str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct ServicePort(u16);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ScaffoldRunOk(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceCrate(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceComposeName(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceComposeFile(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceDockerfile(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceImage(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceKubernetesManifest(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct ServiceSocketEnv(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct ServiceCatalogEntries(
    bounded_types::domain_types::vector::BoundedVec<ServiceCatalogEntry, 0, { usize::MAX }>,
);
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
pub(crate) struct ShouldWrite(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
pub(crate) struct ScaffoldText(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ScaffoldTextRef<'text_lt>(&'text_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ScaffoldPathRef<'path_lt>(&'path_lt std::path::Path);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ReplacementsRef<'replacements_lt>(
    &'replacements_lt [(&'replacements_lt str, String)],
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct CargoArgsRef<'args_lt>(&'args_lt [&'args_lt str]);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct UpdateEnvName(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) enum GeneratedProjection {
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
pub(crate) struct ShouldSkip(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ScaffoldIoError(std::io::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerRuntimeBoundedReadError(
    server_runtime_http::domain_types::BoundedReadError,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum ScaffoldError {
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
    Io(#[from] ScaffoldIoError),
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
        Self::Io(ScaffoldIoError::from(value))
    }
}

fn synchronize_generated_file(
    path: ScaffoldPathRef<'_>,
    begin: ScaffoldTextRef<'_>,
    end: ScaffoldTextRef<'_>,
    generated: ScaffoldTextRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let source = crate::adapters::template_fs_read_bounded_text::read_bounded_text(path)?;
    let (prefix, after_begin) = source
        .as_ref()
        .split_once(begin.0)
        .ok_or(ScaffoldError::Marker)?;
    let (_previous, suffix) = after_begin.split_once(end.0).ok_or(ScaffoldError::Marker)?;
    let expected = ScaffoldText::try_from(format!(
        "{prefix}{}{generated}{}{suffix}",
        begin.0,
        end.0,
        generated = generated.0
    ))
    .map_err(|_error| ScaffoldError::Catalog)?;
    if expected.as_ref() == source.as_ref() {
        return Ok(());
    }
    if bool::from(write_changes) {
        crate::adapters::template_fs_write_text::write_text(
            path,
            ScaffoldTextRef::from(expected.as_ref()),
        )
    } else {
        Err(ScaffoldError::GeneratedDeployment)
    }
}
#[allow(
    clippy::single_call_fn,
    reason = "the deployment command owns all generated projections"
)]
pub(crate) fn synchronize_deployment_projections(
    root: ScaffoldPathRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let catalog_path = root.0.join(constants_str::VALUE_C1590960);
    let catalog = crate::adapters::template_fs_read_bounded_text::read_bounded_text(
        ScaffoldPathRef::from(catalog_path.as_path()),
    )?;
    let entries = service_catalog_parse::parse(ScaffoldTextRef::from(catalog.as_ref()))?;
    let entries_ref = ServiceCatalogEntriesRef::from(entries.0.as_slice());
    let ci = service_catalog_render_ci_matrix::render_ci_matrix(entries_ref);
    let release = service_catalog_render_release_matrix::render_release_matrix(entries_ref);
    let ci_path = root.0.join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH);
    synchronize_generated_file(
        ScaffoldPathRef::from(ci_path.as_path()),
        ScaffoldTextRef::from(constants_str::VALUE_48916059),
        ScaffoldTextRef::from(constants_str::VALUE_37E65562),
        ScaffoldTextRef::from(ci.as_ref()),
        write_changes,
    )?;
    let release_path = root.0.join(constants_str::VALUE_87DB21A9);
    synchronize_generated_file(
        ScaffoldPathRef::from(release_path.as_path()),
        ScaffoldTextRef::from(constants_str::VALUE_BF61857A),
        ScaffoldTextRef::from(constants_str::VALUE_1BC591D5),
        ScaffoldTextRef::from(release.as_ref()),
        write_changes,
    )?;
    entries_ref.0.iter().try_for_each(|entry| {
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
            ScaffoldPathRef::from(compose_path.as_path()),
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
            ScaffoldPathRef::from(compose_path.as_path()),
            ScaffoldTextRef::from(compose_socket_begin.as_str()),
            ScaffoldTextRef::from(compose_socket_end.as_str()),
            ScaffoldTextRef::from(compose_socket.as_str()),
            write_changes,
        )?;
        let ready_path =
            <common_routes::domain_types::HealthReadyRoute as frontend_contract::domain_types::TypedRoute>::metadata(
            )
            .path();
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
            ScaffoldPathRef::from(compose_path.as_path()),
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
            ScaffoldPathRef::from(compose_path.as_path()),
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
            ScaffoldPathRef::from(kubernetes_path.as_path()),
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
            ScaffoldPathRef::from(kubernetes_path.as_path()),
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
            ScaffoldPathRef::from(kubernetes_path.as_path()),
            ScaffoldTextRef::from(kubernetes_container_begin.as_str()),
            ScaffoldTextRef::from(kubernetes_container_end.as_str()),
            ScaffoldTextRef::from(kubernetes_container.as_str()),
            write_changes,
        )?;
        let live_path =
            <common_routes::domain_types::HealthLiveRoute as frontend_contract::domain_types::TypedRoute>::metadata()
                .path();
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
            ScaffoldPathRef::from(kubernetes_path.as_path()),
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
            ScaffoldPathRef::from(kubernetes_path.as_path()),
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
            ScaffoldPathRef::from(kubernetes_path.as_path()),
            ScaffoldTextRef::from(kubernetes_service_port_begin.as_str()),
            ScaffoldTextRef::from(kubernetes_service_port_end.as_str()),
            ScaffoldTextRef::from(kubernetes_service_port.as_str()),
            write_changes,
        )
    })?;
    entries_ref.0.iter().try_for_each(|entry| {
        if ![
            entry.crate_name.as_ref(),
            entry.compose_file.as_ref(),
            entry.dockerfile.as_ref(),
            entry.kubernetes_manifest.as_ref(),
        ]
        .into_iter()
        .all(|path| {
            let entry_path = std::path::Path::new(path);
            entry_path.is_relative()
                && entry_path
                    .components()
                    .all(|component| matches!(component, std::path::Component::Normal(_)))
        }) {
            return Err(ScaffoldError::Catalog);
        }
        if !root
            .0
            .join(entry.crate_name.as_ref())
            .join(constants_str::CARGO_TOML)
            .is_file()
            || !root.0.join(entry.dockerfile.as_ref()).is_file()
        {
            return Err(ScaffoldError::GeneratedDeployment);
        }
        let compose_path = root.0.join(entry.compose_file.as_ref());
        let compose = crate::adapters::template_fs_read_bounded_text::read_bounded_text(
            ScaffoldPathRef::from(compose_path.as_path()),
        )?;
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
        let kubernetes = crate::adapters::template_fs_read_bounded_text::read_bounded_text(
            ScaffoldPathRef::from(kubernetes_path.as_path()),
        )?;
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
pub(crate) fn synchronize_cargo_owned_projection(
    root: ScaffoldPathRef<'_>,
    arguments: CargoArgsRef<'_>,
    update_environment: UpdateEnvName,
    projection: GeneratedProjection,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let mut command = macro_helpers::domain_types::tool_command::ToolCommand::new(
        macro_helpers::domain_types::tool_command::ToolProgramRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ),
    );
    let _arguments = command
        .current_dir(macro_helpers::domain_types::tool_command::PathRef::from(
            root.get(),
        ))
        .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(arguments.get()));
    if bool::from(write_changes) {
        let _environment = command.env(
            macro_helpers::domain_types::tool_command::ToolEnvKeyRef::from(
                update_environment.get(),
            ),
            macro_helpers::domain_types::tool_command::ToolEnvValueRef::from(
                constants_str::VALUE_1,
            ),
        );
    }
    let run_ok = ScaffoldRunOk::from(command.status()?.success());
    if run_ok.get() {
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
    reason = "service command owns complete scaffold composition"
)]
pub(crate) fn scaffold_service(
    root: ScaffoldPathRef<'_>,
    service_name: ProjectNameRef<'_>,
    port: ServicePort,
) -> Result<(), ScaffoldError> {
    naming_validate_project_name::validate_project_name(service_name)?;
    if port.0 == constants_u16::ZERO {
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
    let kebab = naming_kebab_case::kebab_case(service_name);
    let upper_snake = service.to_ascii_uppercase();
    let replacements = [
        (
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE,
            service.to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE_KEBAB,
            kebab.as_ref().to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_UPPER,
            upper_snake.clone(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_TITLE,
            naming_upper_camel_case::upper_camel_case(service_name)
                .as_ref()
                .to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_LOWER,
            service.to_owned(),
        ),
        (
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_PORT,
            port.0.to_string(),
        ),
    ];
    crate::adapters::template_fs_copy_template_tree::copy_template_tree(
        ScaffoldPathRef::from(
            root.0
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE)
                .as_path(),
        ),
        ScaffoldPathRef::from(root.0.join(service).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    crate::adapters::template_fs_copy_template_tree::copy_template_tree(
        ScaffoldPathRef::from(
            root.0
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                .as_path(),
        ),
        ScaffoldPathRef::from(root.0.join(config.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    crate::adapters::template_fs_copy_template_tree::copy_template_tree(
        ScaffoldPathRef::from(
            root.0
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONTRACT)
                .as_path(),
        ),
        ScaffoldPathRef::from(root.0.join(contract.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;

    let manifest = root.0.join(constants_str::CARGO_TOML);
    crate::adapters::template_fs_insert_once::insert_once(
        ScaffoldPathRef::from(manifest.as_path()),
        ScaffoldTextRef::from(constants_str::WORKSPACE_SCAFFOLD_MANIFEST_MEMBER_MARKER),
        ScaffoldTextRef::from(
            format!(
                "  \"notification_service_contract\",\n  \"{service}\",\n  \"{config}\",\n  \"{contract}\","
            )
            .as_str(),
        ),
    )?;
    let dependency_marker = constants_str::WORKSPACE_SCAFFOLD_MANIFEST_DEPENDENCY_MARKER;
    crate::adapters::template_fs_insert_once::insert_once(
        ScaffoldPathRef::from(manifest.as_path()),
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
        .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_K8S_PATH);
    let k8s_file_name = format!("{kebab}.yaml");
    let k8s_destination = root
        .0
        .join(constants_str::WORKSPACE_SCAFFOLD_K8S_BASE_PATH)
        .join(k8s_file_name.as_str());
    let _copied_bytes = std::fs::copy(k8s_source.as_path(), k8s_destination.as_path())?;
    crate::adapters::template_fs_replace_file::replace_file(
        ScaffoldPathRef::from(k8s_destination.as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    let mut k8s_contents = crate::adapters::template_fs_read_bounded_text::read_bounded_text(
        ScaffoldPathRef::from(k8s_destination.as_path()),
    )?
    .as_ref()
    .to_owned();
    k8s_contents.push_str(
        format!(
            "\n---\napiVersion: networking.k8s.io/v1\nkind: NetworkPolicy\nmetadata:\n  name: {kebab}-access\n  namespace: rust-workspace-template\nspec:\n  podSelector:\n    matchLabels:\n      app.kubernetes.io/name: {kebab}\n  ingress:\n    - from:\n        - podSelector:\n            matchLabels:\n              app.kubernetes.io/name: application\n      ports:\n        - protocol: TCP\n          port: {port}\n  egress:\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: database\n          podSelector:\n            matchLabels:\n              app.kubernetes.io/name: {kebab}-postgresql\n      ports:\n        - protocol: TCP\n          port: 5432\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: kube-system\n          podSelector:\n            matchLabels:\n              k8s-app: kube-dns\n      ports:\n        - protocol: UDP\n          port: 53\n        - protocol: TCP\n          port: 53\n  policyTypes: [\"Ingress\", \"Egress\"]\n",
            port = port.0,
        )
        .as_str(),
    );
    crate::adapters::template_fs_write_text::write_text(
        ScaffoldPathRef::from(k8s_destination.as_path()),
        ScaffoldTextRef::from(k8s_contents.as_str()),
    )?;
    let kustomization = root
        .0
        .join(constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_PATH);
    crate::adapters::template_fs_insert_once::insert_once(
        ScaffoldPathRef::from(kustomization.as_path()),
        ScaffoldTextRef::from(constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_MARKER),
        ScaffoldTextRef::from(
            format!("  - notification-service.yaml\n  - {k8s_file_name}").as_str(),
        ),
    )?;

    let config_example_path = root
        .0
        .join(config.as_str())
        .join(constants_str::ENV_EXAMPLE);
    let config_example = crate::adapters::template_fs_read_bounded_text::read_bounded_text(
        ScaffoldPathRef::from(config_example_path.as_path()),
    )?;
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
        <common_routes::domain_types::HealthReadyRoute as frontend_contract::domain_types::TypedRoute>::metadata(
        )
        .path();
    let compose = format!(
        "services:\n  {service}_database:\n    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n    environment:\n      POSTGRES_DB: {service}\n      POSTGRES_USER: {service}\n      POSTGRES_PASSWORD: ${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}\n    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U {service} -d {service}\"]\n      interval: 5s\n      timeout: 3s\n      retries: 20\n    networks: [application]\n    volumes: [{service}_database_data:/var/lib/postgresql/data]\n  # BEGIN GENERATED COMPOSE IDENTITY {service}\n  {service}:\n    build:\n      context: .\n      dockerfile: {service}/Dockerfile\n  # END GENERATED COMPOSE IDENTITY {service}\n    depends_on:\n      {service}_database:\n        condition: service_healthy\n    environment:\n{environment}    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH {service}\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:{port}{ready_path}\"]\n      # END GENERATED COMPOSE HEALTH {service}\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n    networks: [application]\n    # BEGIN GENERATED COMPOSE PORT {service}\n    ports:\n      - \"127.0.0.1:{port}:{port}\"\n    # END GENERATED COMPOSE PORT {service}\n    read_only: true\n    restart: unless-stopped\n    tmpfs: [/tmp:size=16m,mode=1777]\nvolumes:\n  {service}_database_data:\n",
        port = port.0,
        environment = compose_environment,
        ready_path = ready_path.as_ref(),
    );
    let compose_path = root.0.join(format!("docker-compose.{service}.yml"));
    crate::adapters::template_fs_write_text::write_text(
        ScaffoldPathRef::from(compose_path.as_path()),
        ScaffoldTextRef::from(compose.as_str()),
    )?;

    let service_catalog = root
        .0
        .join(constants_str::WORKSPACE_SCAFFOLD_SERVICE_CATALOG_PATH);
    let mut service_catalog_contents =
        crate::adapters::template_fs_read_bounded_text::read_bounded_text(ScaffoldPathRef::from(
            service_catalog.as_path(),
        ))?
        .as_ref()
        .to_owned();
    service_catalog_contents.push_str(
        format!(
            "\n[[service]]\ncrate = \"{service}\"\ncompose = \"{service}\"\ncompose_file = \"docker-compose.{service}.yml\"\ndockerfile = \"{service}/Dockerfile\"\nimage = \"{kebab}\"\nkubernetes = \"deploy/k8s/base/{k8s_file_name}\"\nport = {}\nrelease = false\nsocket_env = \"{upper_snake}_SERVICE_SOCKET_ADDRESS\"\n",
            port.0
        )
        .as_str(),
    );
    crate::adapters::template_fs_write_text::write_text(
        ScaffoldPathRef::from(service_catalog.as_path()),
        ScaffoldTextRef::from(service_catalog_contents.as_str()),
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
