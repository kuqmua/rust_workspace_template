pub(crate) fn synchronize_deployment_projections(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    should_write: crate::should_write::ShouldWrite,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let catalog_path = scaffold_path_ref.get().join(constants_str::VALUE_C1590960);
    let catalog = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
        crate::scaffold_path_ref::ScaffoldPathRef::from(catalog_path.as_path()),
    )?;
    let entries = crate::service_catalog_parse::service_catalog_parse(
        crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_ref()),
    )?;
    let entries_ref = crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(
        entries.get_inner().as_slice(),
    );
    let synchronize_static_projection =
        |projection_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
         begin: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
         end: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
         generated: crate::scaffold_text_ref::ScaffoldTextRef<'_>| {
            crate::synchronize_generated_file::synchronize_generated_file(
                projection_path_ref,
                begin,
                end,
                generated,
                should_write,
            )
        };
    let synchronize_entry_projection =
        |projection_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
         begin: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
         end: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
         generated: crate::scaffold_text_ref::ScaffoldTextRef<'_>| {
            crate::synchronize_generated_file::synchronize_generated_file(
                projection_path_ref,
                begin,
                end,
                generated,
                should_write,
            )
        };
    let ci = crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
        entries_ref,
    );
    let release =
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            entries_ref,
        );
    let ci_path = scaffold_path_ref
        .get()
        .join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH);
    synchronize_static_projection(
        crate::scaffold_path_ref::ScaffoldPathRef::from(ci_path.as_path()),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_48916059),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_37E65562),
        crate::scaffold_text_ref::ScaffoldTextRef::from(ci.as_ref()),
    )?;
    let release_path = scaffold_path_ref.get().join(constants_str::VALUE_87DB21A9);
    synchronize_static_projection(
        crate::scaffold_path_ref::ScaffoldPathRef::from(release_path.as_path()),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_BF61857A),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_1BC591D5),
        crate::scaffold_text_ref::ScaffoldTextRef::from(release.as_ref()),
    )?;
    entries_ref.get().iter().try_for_each(|entry| {
        let compose_path = scaffold_path_ref.get().join(entry.get_compose_file().as_ref());
        let compose_identity_begin = format!(
            "  # BEGIN GENERATED COMPOSE IDENTITY {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_identity_end = format!(
            "  # END GENERATED COMPOSE IDENTITY {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_identity = format!(
            "  {}:\n    build:\n      context: .\n      dockerfile: {}\n",
            entry.get_compose_name().as_ref(),
            entry.get_dockerfile().as_ref()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_identity_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_identity_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_identity.as_str()),
        )?;
        let compose_socket_begin = format!(
            "      # BEGIN GENERATED COMPOSE SOCKET {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_socket_end = format!(
            "      # END GENERATED COMPOSE SOCKET {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_socket = format!(
            "      {}: \"0.0.0.0:{}\"\n",
            entry.get_socket_env().as_ref(),
            entry.get_port().get()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_socket_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_socket_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_socket.as_str()),
        )?;
        let ready_path =
            <common_routes::health_ready_route::HealthReadyRoute as frontend_contract::typed_route::TypedRoute>::metadata(
            )
            .path();
        let compose_health_begin = format!(
            "      # BEGIN GENERATED COMPOSE HEALTH {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_health_end = format!(
            "      # END GENERATED COMPOSE HEALTH {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_health = format!(
            "      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:{}{}\"]\n",
            entry.get_port().get(),
            ready_path.as_ref()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_health_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_health_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_health.as_str()),
        )?;
        let compose_port_begin = format!(
            "    # BEGIN GENERATED COMPOSE PORT {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_port_end = format!(
            "    # END GENERATED COMPOSE PORT {}\n",
            entry.get_compose_name().as_ref()
        );
        let compose_port = format!("    ports:\n      - \"127.0.0.1:{0}:{0}\"\n", entry.get_port().get());
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_port_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_port_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose_port.as_str()),
        )?;

        let kubernetes_path = scaffold_path_ref.get().join(entry.get_kubernetes_manifest().as_ref());
        let kubernetes_metadata_begin = format!(
            "# BEGIN GENERATED KUBERNETES METADATA {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_metadata_end = format!(
            "# END GENERATED KUBERNETES METADATA {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_metadata = format!(
            "metadata:\n  name: {0}\n  namespace: rust-workspace-template\n",
            entry.get_image().as_ref()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_metadata_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_metadata_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_metadata.as_str()),
        )?;
        let kubernetes_workload_identity_begin = format!(
            "  # BEGIN GENERATED KUBERNETES WORKLOAD IDENTITY {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_workload_identity_end = format!(
            "  # END GENERATED KUBERNETES WORKLOAD IDENTITY {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_workload_identity = format!(
            "  selector:\n    matchLabels:\n      app.kubernetes.io/name: {0}\n  template:\n    metadata:\n      labels:\n        app.kubernetes.io/name: {0}\n    spec:\n",
            entry.get_image().as_ref()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_workload_identity_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_workload_identity_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_workload_identity.as_str()),
        )?;
        let kubernetes_container_begin = format!(
            "      # BEGIN GENERATED KUBERNETES CONTAINER {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_container_end = format!(
            "      # END GENERATED KUBERNETES CONTAINER {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_container = format!(
            "      containers:\n        - name: {0}\n          image: {0}:replace-with-immutable-tag\n          envFrom:\n            - configMapRef:\n                name: {0}-config\n            - secretRef:\n                name: {0}-secrets\n          ports:\n            - containerPort: {1}\n              name: http\n",
            entry.get_image().as_ref(),
            entry.get_port().get()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_container_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_container_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_container.as_str()),
        )?;
        let live_path =
            <common_routes::health_live_route::HealthLiveRoute as frontend_contract::typed_route::TypedRoute>::metadata()
                .path();
        let kubernetes_probe_begin = format!(
            "          # BEGIN GENERATED KUBERNETES PROBES {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_probe_end = format!(
            "          # END GENERATED KUBERNETES PROBES {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_probe = format!(
            "          startupProbe:\n            httpGet:\n              path: {ready}\n              port: http\n            failureThreshold: 30\n            periodSeconds: 2\n          readinessProbe:\n            httpGet:\n              path: {ready}\n              port: http\n            periodSeconds: 5\n          livenessProbe:\n            httpGet:\n              path: {live}\n              port: http\n            periodSeconds: 10\n",
            ready = ready_path.as_ref(),
            live = live_path.as_ref()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_probe_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_probe_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_probe.as_str()),
        )?;
        let kubernetes_service_identity_begin = format!(
            "# BEGIN GENERATED KUBERNETES SERVICE IDENTITY {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_service_identity_end = format!(
            "# END GENERATED KUBERNETES SERVICE IDENTITY {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_service_identity = format!(
            "metadata:\n  name: {0}\n  namespace: rust-workspace-template\n  labels:\n    app.kubernetes.io/name: {0}\nspec:\n  selector:\n    app.kubernetes.io/name: {0}\n",
            entry.get_image().as_ref()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_service_identity_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_service_identity_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_service_identity.as_str()),
        )?;
        let kubernetes_service_port_begin = format!(
            "  # BEGIN GENERATED KUBERNETES SERVICE PORT {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_service_port_end = format!(
            "  # END GENERATED KUBERNETES SERVICE PORT {}\n",
            entry.get_image().as_ref()
        );
        let kubernetes_service_port = format!(
            "  ports:\n    - name: http\n      port: {}\n      targetPort: http\n",
            entry.get_port().get()
        );
        synchronize_entry_projection(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_service_port_begin.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_service_port_end.as_str()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(kubernetes_service_port.as_str()),
        )
    })?;
    entries_ref.get().iter().try_for_each(|entry| {
        if ![
            entry.get_crate_name().as_ref(),
            entry.get_compose_file().as_ref(),
            entry.get_dockerfile().as_ref(),
            entry.get_kubernetes_manifest().as_ref(),
        ]
        .into_iter()
        .all(|path| {
            let entry_path = std::path::Path::new(path);
            entry_path.is_relative()
                && entry_path
                    .components()
                    .all(|component| matches!(component, std::path::Component::Normal(_)))
        }) {
            return Err(crate::scaffold_error::ScaffoldError::Catalog);
        }
        if !scaffold_path_ref
            .get()
            .join(entry.get_crate_name().as_ref())
            .join(constants_str::CARGO_TOML)
            .is_file()
            || !scaffold_path_ref
                .get()
                .join(entry.get_dockerfile().as_ref())
                .is_file()
        {
            return Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment);
        }
        let compose_path = scaffold_path_ref
            .get()
            .join(entry.get_compose_file().as_ref());
        let compose = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
        )?;
        let port = entry.get_port().get();
        if !compose
            .as_ref()
            .contains(format!("  {}:\n", entry.get_compose_name().as_ref()).as_str())
            || !compose
                .as_ref()
                .contains(format!("dockerfile: {}", entry.get_dockerfile().as_ref()).as_str())
            || !compose
                .as_ref()
                .contains(format!("127.0.0.1:{port}:{port}").as_str())
        {
            return Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment);
        }
        let kubernetes_path = scaffold_path_ref
            .get()
            .join(entry.get_kubernetes_manifest().as_ref());
        let kubernetes = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kubernetes_path.as_path()),
        )?;
        if !kubernetes
            .as_ref()
            .contains(format!("image: {}:", entry.get_image().as_ref()).as_str())
            || !kubernetes
                .as_ref()
                .contains(format!("containerPort: {port}").as_str())
            || !kubernetes
                .as_ref()
                .contains(format!("port: {port}").as_str())
        {
            return Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment);
        }
        Ok(())
    })
}
