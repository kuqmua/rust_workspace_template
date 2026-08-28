use super::{
    ScaffoldError, ScaffoldPathRef, ScaffoldTextRef, ServiceCatalogEntriesRef, ShouldWrite,
    service_catalog_render_release_entries, synchronize_generated_file,
};

pub(crate) fn synchronize_deployment_projections(
    root: ScaffoldPathRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let catalog_path = root.0.join(constants_str::VALUE_C1590960);
    let catalog = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
        ScaffoldPathRef::from(catalog_path.as_path()),
    )?;
    let entries = {
        let source = ScaffoldTextRef::from(catalog.as_ref());
        let mut entries = Vec::new();
        let mut current = None;
        source.0.lines().try_for_each(|raw_line| {
            let trimmed_line = raw_line.trim();
            if trimmed_line == constants_str::VALUE_484ADD83 {
                if let Some(draft) = current.take() {
                    entries.push(super::ServiceCatalogDraft::finish(draft)?);
                }
                current = Some(super::ServiceCatalogDraft::default());
                return Ok(());
            }
            let Some(draft) = current.as_mut() else {
                return Ok(());
            };
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::CRATE),
            )? {
                draft.crate_name = Some(
                    super::ServiceCrate::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::VALUE_DB669AF6),
            )? {
                draft.compose_name = Some(
                    super::ServiceComposeName::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::VALUE_739ED940),
            )? {
                draft.compose_file = Some(
                    super::ServiceComposeFile::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::VALUE_254DB0FB),
            )? {
                draft.dockerfile = Some(
                    super::ServiceDockerfile::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::VALUE_6105D6CC),
            )? {
                draft.image = Some(
                    super::ServiceImage::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::VALUE_94ABCB2D),
            )? {
                draft.kubernetes_manifest = Some(
                    super::ServiceKubernetesManifest::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(port) = trimmed_line
                .strip_prefix(constants_str::VALUE_F8D397A3)
                .and_then(|port_text| port_text.trim().strip_prefix('='))
                .map(str::trim)
                .and_then(|port_text| port_text.parse::<u16>().ok())
            {
                draft.port = Some(super::ServicePort::from(port));
                return Ok(());
            }
            if let Some(value) = super::service_catalog_string_value::service_catalog_string_value(
                ScaffoldTextRef::from(trimmed_line),
                ScaffoldTextRef::from(constants_str::VALUE_20E49707),
            )? {
                draft.socket_env = Some(
                    super::ServiceSocketEnv::try_from(value.as_ref().to_owned())
                        .map_err(|_error| ScaffoldError::Catalog)?,
                );
                return Ok(());
            }
            if let Some(release) =
                trimmed_line
                    .strip_prefix(constants_str::RELEASE)
                    .and_then(|release_text| {
                        release_text
                            .trim()
                            .strip_prefix('=')
                            .map(str::trim)
                            .and_then(|parsed_text| parsed_text.parse::<bool>().ok())
                    })
            {
                draft.release = Some(super::ShouldRelease::from(release));
            }
            Ok::<(), ScaffoldError>(())
        })?;
        if let Some(draft) = current {
            entries.push(super::ServiceCatalogDraft::finish(draft)?);
        }
        if entries.is_empty() {
            return Err(ScaffoldError::Catalog);
        }
        super::ServiceCatalogEntries::from(bounded_types::BoundedVec::from_max_iter(entries))
    };
    let entries_ref = ServiceCatalogEntriesRef::from(entries.0.as_slice());
    let ci =
        service_catalog_render_release_entries::service_catalog_render_release_entries(entries_ref);
    let release =
        service_catalog_render_release_entries::service_catalog_render_release_entries(entries_ref);
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
            <common_routes::HealthReadyRoute as frontend_contract::TypedRoute>::metadata(
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
            <common_routes::HealthLiveRoute as frontend_contract::TypedRoute>::metadata()
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
        let compose = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
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
        let kubernetes = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
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
