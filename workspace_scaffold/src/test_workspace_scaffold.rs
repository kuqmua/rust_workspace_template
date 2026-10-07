fn assert_scaffold_file_content(path: &std::path::Path, str: &str) {
    let actual = std::fs::read_to_string(path).expect(constants_str::DIAGNOSTIC_371DBE92);
    assert_eq!(actual, str, "239c17b0: {}", path.display());
}

fn write(path: &std::path::Path, str: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect(constants_str::DIAGNOSTIC_2F0AD03A);
    }
    std::fs::write(path, str).expect(constants_str::DIAGNOSTIC_79AF6DC8);
}

#[test]
fn test_validates_and_converts_project_names() {
    let valid = crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
    crate::naming_validate_project_name::naming_validate_project_name(valid)
        .expect(constants_str::DIAGNOSTIC_96DE3A80);
    assert!(matches!(
        crate::naming_kebab_case::naming_kebab_case(valid),
        Ok(value) if value.as_ref() == constants_str::VALUE_77A8A329
    ));
    assert!(matches!(
        crate::naming_title_case::naming_title_case(valid),
        Ok(value) if value.as_ref() == constants_str::VALUE_3EEF5CDE
    ));
    assert!(matches!(
        crate::naming_upper_camel_case::naming_upper_camel_case(valid),
        Ok(value) if value.as_ref() == constants_str::VALUE_6B0B0F05
    ));
    assert!(
        crate::naming_validate_project_name::naming_validate_project_name(
            crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_4F059BD8)
        )
        .is_err()
    );
}

#[test]
fn test_requires_https_repository_url() {
    crate::naming_validate_repository_url::naming_validate_repository_url(
        crate::repository_url_ref::RepositoryUrlRef::from(constants_str::VALUE_A680FDEF),
    )
    .expect(constants_str::DIAGNOSTIC_28C1E7A4);
    assert!(
        crate::naming_validate_repository_url::naming_validate_repository_url(
            crate::repository_url_ref::RepositoryUrlRef::from(constants_str::VALUE_861AC68D,)
        )
        .is_err()
    );
}

#[test]
fn test_deployment_projection_check_rejects_stale_generated_content() {
    let path = std::env::temp_dir().join(format!(
        "workspace-scaffold-generated-test-{}",
        std::process::id()
    ));
    let begin = constants_str::VALUE_0BAD8889;
    let end = constants_str::VALUE_79B72852;
    write(path.as_path(), constants_str::VALUE_0889759C);
    let check = crate::synchronize_generated_file::synchronize_generated_file(
        crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
        crate::scaffold_text_ref::ScaffoldTextRef::from(begin),
        crate::scaffold_text_ref::ScaffoldTextRef::from(end),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        crate::should_write::ShouldWrite::from(false),
    );
    assert!(matches!(
        check,
        Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment)
    ));
    crate::synchronize_generated_file::synchronize_generated_file(
        crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
        crate::scaffold_text_ref::ScaffoldTextRef::from(begin),
        crate::scaffold_text_ref::ScaffoldTextRef::from(end),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        crate::should_write::ShouldWrite::from(true),
    )
    .expect(constants_str::DIAGNOSTIC_5A7E3C91);
    crate::synchronize_generated_file::synchronize_generated_file(
        crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
        crate::scaffold_text_ref::ScaffoldTextRef::from(begin),
        crate::scaffold_text_ref::ScaffoldTextRef::from(end),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        crate::should_write::ShouldWrite::from(false),
    )
    .expect(constants_str::DIAGNOSTIC_D2F8B4A6);
    std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_9C1E6A3F);
}

#[test]
fn test_service_catalog_owns_ci_and_release_projection_values() {
    let entries = crate::service_catalog_parse::service_catalog_parse(
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_D4291B4A),
    )
    .expect(constants_str::DIAGNOSTIC_4E8B2D7A);
    let entries_ref = crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(
        entries.get_inner().as_slice(),
    );
    assert_eq!(
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            entries_ref,
        )
        .as_ref(),
        constants_str::VALUE_CF9A8E24
    );
    assert_eq!(
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            entries_ref
        )
        .as_ref(),
        constants_str::VALUE_CF9A8E24
    );
}

#[test]
fn test_rejects_scaffold_text_over_size_limit() {
    let path = std::env::temp_dir().join(format!(
        "workspace-scaffold-oversize-test-{}",
        std::process::id()
    ));
    std::fs::write(
        path.as_path(),
        vec![b'x'; constants_usize::VALUE_16_777_216.saturating_add(constants_usize::ONE)],
    )
    .expect(constants_str::DIAGNOSTIC_D97E30AC);
    let result = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
        crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
    );
    assert!(
        matches!(
            result,
            Err(server_runtime_http::bounded_read_error::BoundedReadError::ExceedsMaximum { .. })
        ),
        "8f32bc16"
    );
    std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_51CD7B2E);
}

#[test]
fn test_service_scaffold_registers_all_artifacts() {
    let root = std::env::temp_dir().join(format!("workspace-scaffold-test-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(root.as_path()).expect(constants_str::DIAGNOSTIC_1449608D);
    }
    write(
        root.join(constants_str::CARGO_TOML).as_path(),
        constants_str::VALUE_9A836A5B,
    );
    write(
        root.join(constants_str::VALUE_8E41EC63).as_path(),
        constants_str::VALUE_45AD55F9,
    );
    write(
        root.join(constants_str::VALUE_F7C1AF06).as_path(),
        constants_str::VALUE_244072F2,
    );
    write(
        root.join(constants_str::VALUE_0A7A2313).as_path(),
        constants_str::VALUE_B3508161,
    );
    write(
        root.join(constants_str::VALUE_4F50C4FE).as_path(),
        constants_str::VALUE_A64251C2,
    );
    write(
        root.join(constants_str::VALUE_09101A6F).as_path(),
        constants_str::VALUE_04354311,
    );
    write(
        root.join(constants_str::VALUE_13A8EB94).as_path(),
        constants_str::VALUE_D0FC32F7,
    );
    write(
        root.join(constants_str::VALUE_C1590960).as_path(),
        constants_str::VALUE_D4E98611,
    );
    (|| -> Result<(), crate::scaffold_error::ScaffoldError> {
        let root_ref = crate::scaffold_path_ref::ScaffoldPathRef::from(root.as_path());
        let service_name = crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_E896B9AF);
        let port = crate::service_port::ServicePort::from(8082u16);
        crate::naming_validate_project_name::naming_validate_project_name(service_name)?;
        if port.get() == constants_u16::ZERO {
            return Err(crate::scaffold_error::ScaffoldError::ServicePort);
        }
        let service = service_name.get();
        let config = format!("{service}_config");
        let contract = format!("{service}_contract");
        if [service, config.as_str(), contract.as_str()]
            .iter()
            .any(|path| root_ref.get().join(path).exists())
        {
            return Err(crate::scaffold_error::ScaffoldError::ServiceExists);
        }
        let kebab = crate::naming_kebab_case::naming_kebab_case(service_name)?;
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
                crate::naming_upper_camel_case::naming_upper_camel_case(service_name)?
                    .as_ref()
                    .to_owned(),
            ),
            (
                constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_LOWER,
                service.to_owned(),
            ),
            (
                constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_PORT,
                port.get().to_string(),
            ),
        ];
        crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(
                root_ref.get()
                    .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE)
                    .as_path(),
            ),
            crate::scaffold_path_ref::ScaffoldPathRef::from(root_ref.get().join(service).as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        )?;
        crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(
                root_ref.get()
                    .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                    .as_path(),
            ),
            crate::scaffold_path_ref::ScaffoldPathRef::from(root_ref.get().join(config.as_str()).as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        )?;
        crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(
                root_ref.get()
                    .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONTRACT)
                    .as_path(),
            ),
            crate::scaffold_path_ref::ScaffoldPathRef::from(root_ref.get().join(contract.as_str()).as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        )?;

        let manifest = root_ref.get().join(constants_str::CARGO_TOML);
        crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(manifest.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::WORKSPACE_SCAFFOLD_MANIFEST_MEMBER_MARKER),
            crate::scaffold_text_ref::ScaffoldTextRef::from(
                format!(
                    "  \"notification_service_contract\",\n  \"{service}\",\n  \"{config}\",\n  \"{contract}\","
                )
                .as_str(),
            ),
        )?;
        let dependency_marker = constants_str::WORKSPACE_SCAFFOLD_MANIFEST_DEPENDENCY_MARKER;
        crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(manifest.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(dependency_marker),
            crate::scaffold_text_ref::ScaffoldTextRef::from(
                format!(
                    "{dependency_marker}\n{service} = {{ path = \"./{service}\" }}\n{config} = {{ path = \"./{config}\" }}\n{contract} = {{ path = \"./{contract}\" }}"
                )
                .as_str(),
            ),
        )?;

        let k8s_source = root_ref
            .get()
            .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_K8S_PATH);
        let k8s_file_name = format!("{kebab}.yaml");
        let k8s_destination = root_ref
            .get()
            .join(constants_str::WORKSPACE_SCAFFOLD_K8S_BASE_PATH)
            .join(k8s_file_name.as_str());
        let _copied_bytes = std::fs::copy(k8s_source.as_path(), k8s_destination.as_path())?;
        crate::template_fs_replace_file::template_fs_replace_file(
            crate::scaffold_path_ref::ScaffoldPathRef::from(k8s_destination.as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        )?;
        let mut k8s_contents = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(k8s_destination.as_path()),
        )?
        .as_ref()
        .to_owned();
        k8s_contents.push_str(
            format!(
                "\n---\napiVersion: networking.k8s.io/v1\nkind: NetworkPolicy\nmetadata:\n  name: {kebab}-access\n  namespace: rust-workspace-template\nspec:\n  podSelector:\n    matchLabels:\n      app.kubernetes.io/name: {kebab}\n  ingress:\n    - from:\n        - podSelector:\n            matchLabels:\n              app.kubernetes.io/name: application\n      ports:\n        - protocol: TCP\n          port: {port}\n  egress:\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: database\n          podSelector:\n            matchLabels:\n              app.kubernetes.io/name: {kebab}-postgresql\n      ports:\n        - protocol: TCP\n          port: 5432\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: kube-system\n          podSelector:\n            matchLabels:\n              k8s-app: kube-dns\n      ports:\n        - protocol: UDP\n          port: 53\n        - protocol: TCP\n          port: 53\n  policyTypes: [\"Ingress\", \"Egress\"]\n",
                port = port.get(),
            )
            .as_str(),
        );
        crate::template_fs_write_text::template_fs_write_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(k8s_destination.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(k8s_contents.as_str()),
        )?;
        let kustomization = root_ref
            .get()
            .join(constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_PATH);
        crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(kustomization.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_MARKER),
            crate::scaffold_text_ref::ScaffoldTextRef::from(
                format!("  - notification-service.yaml\n  - {k8s_file_name}").as_str(),
            ),
        )?;

        let config_example_path = root_ref
            .get()
            .join(config.as_str())
            .join(constants_str::ENV_EXAMPLE);
        let config_example = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(config_example_path.as_path()),
        )?;
        let database_key = format!("{upper_snake}_DATABASE_URL");
        let socket_key = format!("{upper_snake}_SERVICE_SOCKET_ADDRESS");
        let compose_environment = config_example
            .as_ref()
            .lines()
            .map(|line| {
                let (key, example) = line.split_once('=').ok_or(crate::scaffold_error::ScaffoldError::Catalog)?;
                let value = if key == database_key {
                    format!(
                        "postgres://{service}:${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}@{service}_database:5432/{service}"
                    )
                } else if key == socket_key {
                    format!("0.0.0.0:{}", port.get())
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
            .collect::<Result<String, crate::scaffold_error::ScaffoldError>>()?;
        let ready_path =
            <common_routes::health_ready_route::HealthReadyRoute as frontend_contract::typed_route::TypedRoute>::metadata().path();
        let compose = format!(
            "services:\n  {service}_database:\n    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n    environment:\n      POSTGRES_DB: {service}\n      POSTGRES_USER: {service}\n      POSTGRES_PASSWORD: ${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}\n    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U {service} -d {service}\"]\n      interval: 5s\n      timeout: 3s\n      retries: 20\n    networks: [application]\n    volumes: [{service}_database_data:/var/lib/postgresql/data]\n  # BEGIN GENERATED COMPOSE IDENTITY {service}\n  {service}:\n    build:\n      context: .\n      dockerfile: {service}/Dockerfile\n  # END GENERATED COMPOSE IDENTITY {service}\n    depends_on:\n      {service}_database:\n        condition: service_healthy\n    environment:\n{environment}    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH {service}\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:{port}{ready_path}\"]\n      # END GENERATED COMPOSE HEALTH {service}\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n    networks: [application]\n    # BEGIN GENERATED COMPOSE PORT {service}\n    ports:\n      - \"127.0.0.1:{port}:{port}\"\n    # END GENERATED COMPOSE PORT {service}\n    read_only: true\n    restart: unless-stopped\n    tmpfs: [/tmp:size=16m,mode=1777]\nvolumes:\n  {service}_database_data:\n",
            port = port.get(),
            environment = compose_environment,
            ready_path = ready_path.as_ref(),
        );
        let compose_path = root_ref.get().join(format!("docker-compose.{service}.yml"));
        crate::template_fs_write_text::template_fs_write_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(compose.as_str()),
        )?;

        let service_catalog = root_ref
            .get()
            .join(constants_str::WORKSPACE_SCAFFOLD_SERVICE_CATALOG_PATH);
        let mut service_catalog_contents =
            crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
                crate::scaffold_path_ref::ScaffoldPathRef::from(service_catalog.as_path()),
            )?
            .as_ref()
            .to_owned();
        service_catalog_contents.push_str(
            format!(
                "\n[[service]]\ncrate = \"{service}\"\ncompose = \"{service}\"\ncompose_file = \"docker-compose.{service}.yml\"\ndockerfile = \"{service}/Dockerfile\"\nimage = \"{kebab}\"\nkubernetes = \"deploy/k8s/base/{k8s_file_name}\"\nport = {}\nrelease = false\nsocket_env = \"{upper_snake}_SERVICE_SOCKET_ADDRESS\"\n",
                port.get()
            )
            .as_str(),
        );
        crate::template_fs_write_text::template_fs_write_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(service_catalog.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(service_catalog_contents.as_str()),
        )?;
        Ok(())
    })()
    .expect(constants_str::DIAGNOSTIC_4BFF1D79);
    assert_scaffold_file_content(
        root.join(constants_str::CARGO_TOML).as_path(),
        constants_str::VALUE_ADF1A200,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_7654C453).as_path(),
        constants_str::VALUE_2120BC93,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_D3EA3646).as_path(),
        constants_str::VALUE_77C620D8,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_0626DBBE).as_path(),
        constants_str::VALUE_6DC62C71,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_83CBEECD).as_path(),
        constants_str::VALUE_7602E17D,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_13A8EB94).as_path(),
        constants_str::VALUE_9A2A3063,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_7D4D7140).as_path(),
        constants_str::VALUE_499A1FF6,
    );
    assert_scaffold_file_content(
        root.join(constants_str::VALUE_C1590960).as_path(),
        constants_str::VALUE_142D5AD3,
    );
    std::fs::remove_dir_all(root).expect(constants_str::DIAGNOSTIC_6F608418);
}

#[test]
fn test_deployment_sync_rejects_unsafe_catalog_paths_before_projection_writes() {
    let root = std::env::temp_dir().join(format!(
        "{}-{}",
        stringify!(test_deployment_sync_rejects_unsafe_catalog_paths_before_projection_writes),
        std::process::id(),
    ));
    let catalog_path = root.join(constants_str::VALUE_C1590960);
    assert!([
        constants_str::CRATE,
        constants_str::VALUE_739ED940,
        constants_str::VALUE_254DB0FB,
        constants_str::VALUE_94ABCB2D,
    ].into_iter().all(|key| {
        [
            '/'.to_string(),
            ['.', '.'].into_iter().collect::<String>(),
            '.'.to_string(),
        ].into_iter().all(|invalid_path| {
            let prefix = format!("{key} =");
            let catalog = constants_str::VALUE_D4291B4A.lines().map(|line| {
                if line.starts_with(prefix.as_str()) {
                    format!("{key} = \"{invalid_path}\"")
                } else {
                    line.to_owned()
                }
            }).collect::<Vec<_>>().join(constants_str::NEWLINE);
            assert!(crate::service_catalog_parse::service_catalog_parse(
                crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
            ).is_ok_and(|entries| entries.get_inner().as_slice().len() == 2usize));
            write(catalog_path.as_path(), catalog.as_str());
            [false, true].into_iter().all(|write_enabled| {
                let result = crate::synchronize_deployment_projections::synchronize_deployment_projections(
                    crate::scaffold_path_ref::ScaffoldPathRef::from(root.as_path()),
                    crate::should_write::ShouldWrite::from(write_enabled),
                );
                assert_scaffold_file_content(catalog_path.as_path(), catalog.as_str());
                matches!(result, Err(crate::scaffold_error::ScaffoldError::Catalog))
            })
        })
    }));
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_deployment_sync_repairs_stale_projection_and_preserves_all_other_files() {
    let result = (|| -> Result<(), crate::scaffold_error::ScaffoldError> {
        let source_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace_directory = source_directory
            .parent()
            .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?;
        let root = std::env::temp_dir().join(format!(
            "{}-{}",
            stringify!(test_deployment_sync_repairs_stale_projection_and_preserves_all_other_files),
            std::process::id(),
        ));
        let catalog_path = workspace_directory.join(constants_str::VALUE_C1590960);
        let catalog = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(catalog_path.as_path()),
        )?;
        let entries = crate::service_catalog_parse::service_catalog_parse(
            crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_ref()),
        )?;
        let paths = [
            constants_str::VALUE_C1590960.to_owned(),
            constants_str::CODE_STYLE_CI_WORKFLOW_PATH.to_owned(),
            constants_str::VALUE_87DB21A9.to_owned(),
        ]
        .into_iter()
        .chain(entries.get_inner().as_slice().iter().flat_map(|entry| {
            [
                entry.get_compose_file().as_ref().to_owned(),
                entry.get_dockerfile().as_ref().to_owned(),
                entry.get_kubernetes_manifest().as_ref().to_owned(),
                format!(
                    "{}/{}",
                    entry.get_crate_name().as_ref(),
                    constants_str::CARGO_TOML
                ),
            ]
        }))
        .collect::<std::collections::BTreeSet<_>>();
        let originals = paths
            .into_iter()
            .map(|path| {
                let source_path = workspace_directory.join(path.as_str());
                let text = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
                    crate::scaffold_path_ref::ScaffoldPathRef::from(source_path.as_path()),
                )?;
                let fixture_path = root.join(path);
                write(fixture_path.as_path(), text.as_ref());
                Ok::<_, crate::scaffold_error::ScaffoldError>((fixture_path, text))
            })
            .collect::<Result<Vec<_>, crate::scaffold_error::ScaffoldError>>()?;
        let synchronize = |write_enabled| {
            crate::synchronize_deployment_projections::synchronize_deployment_projections(
                crate::scaffold_path_ref::ScaffoldPathRef::from(root.as_path()),
                crate::should_write::ShouldWrite::from(write_enabled),
            )
        };
        synchronize(false)?;
        let projection_paths = [
            root.join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH),
            root.join(constants_str::VALUE_87DB21A9),
        ]
        .into_iter()
        .chain(entries.get_inner().as_slice().iter().flat_map(|entry| {
            [
                root.join(entry.get_compose_file().as_ref()),
                root.join(entry.get_kubernetes_manifest().as_ref()),
            ]
        }))
        .collect::<std::collections::BTreeSet<_>>();
        let begin_prefix = constants_str::VALUE_48916059
            .split_whitespace()
            .take(3usize)
            .collect::<Vec<_>>()
            .join(constants_str::SPACE);
        let end_prefix = constants_str::VALUE_37E65562
            .split_whitespace()
            .take(3usize)
            .collect::<Vec<_>>()
            .join(constants_str::SPACE);
        let assert_contents = |changed_path: &std::path::Path, changed_text: &str| {
            originals.iter().fold((), |(), (path, text)| {
                assert_scaffold_file_content(
                    path.as_path(),
                    if path.as_path() == changed_path {
                        changed_text
                    } else {
                        text.as_ref()
                    },
                );
            });
        };
        let block_count = originals
            .iter()
            .filter(|(path, _text)| projection_paths.contains(path))
            .try_fold(constants_usize::ZERO, |count, (path, text)| {
                let markers = text
                    .as_ref()
                    .lines()
                    .filter(|line| line.trim_start().starts_with(begin_prefix.as_str()))
                    .map(|line| {
                        (
                            format!("{line}{}", constants_str::NEWLINE),
                            format!(
                                "{}{}",
                                line.replacen(
                                    begin_prefix.as_str(),
                                    end_prefix.as_str(),
                                    constants_usize::ONE
                                ),
                                constants_str::NEWLINE
                            ),
                        )
                    })
                    .collect::<Vec<_>>();
                assert!(!markers.is_empty());
                markers.iter().try_for_each(|(begin, end)| {
                    let (prefix, generated_suffix) = text
                        .as_ref()
                        .split_once(begin.as_str())
                        .ok_or(crate::scaffold_error::ScaffoldError::Marker)?;
                    let (_generated, suffix) = generated_suffix
                        .split_once(end.as_str())
                        .ok_or(crate::scaffold_error::ScaffoldError::Marker)?;
                    let stale = format!("{prefix}{begin}{}{end}{suffix}", constants_str::X);
                    write(path.as_path(), stale.as_str());
                    assert!(matches!(
                        synchronize(false),
                        Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment)
                    ));
                    assert_contents(path.as_path(), stale.as_str());
                    synchronize(true)?;
                    synchronize(false)?;
                    assert_contents(path.as_path(), text.as_ref());
                    assert!([begin.as_str(), end.as_str()].into_iter().all(|marker| {
                        [false, true].into_iter().all(|duplicate| {
                            let replacement = if duplicate {
                                format!("{marker}{marker}")
                            } else {
                                constants_str::X.to_owned()
                            };
                            let malformed = text.as_ref().replace(marker, replacement.as_str());
                            write(path.as_path(), malformed.as_str());
                            let rejected = [false, true].into_iter().all(|write_enabled| {
                                let result = synchronize(write_enabled);
                                assert_contents(path.as_path(), malformed.as_str());
                                matches!(result, Err(crate::scaffold_error::ScaffoldError::Marker))
                            });
                            write(path.as_path(), text.as_ref());
                            rejected
                        })
                    }));
                    Ok::<(), crate::scaffold_error::ScaffoldError>(())
                })?;
                std::fs::remove_file(path.as_path())?;
                assert!([false, true].into_iter().all(|write_enabled| {
                    let result = synchronize(write_enabled);
                    assert!(!path.exists());
                    originals
                        .iter()
                        .filter(|(other_path, _text)| other_path != path)
                        .fold((), |(), (other_path, original)| {
                            assert_scaffold_file_content(other_path.as_path(), original.as_ref());
                        });
                    matches!(result, Err(crate::scaffold_error::ScaffoldError::Read(
                        server_runtime_http::bounded_read_error::BoundedReadError::Io { source },
                    )) if source.kind() == std::io::ErrorKind::NotFound)
                }));
                write(path.as_path(), text.as_ref());
                synchronize(false)?;
                Ok::<_, crate::scaffold_error::ScaffoldError>(count.saturating_add(markers.len()))
            })?;
        assert_eq!(
            block_count,
            entries
                .get_inner()
                .as_slice()
                .len()
                .saturating_mul(10usize)
                .saturating_add(2usize)
        );
        let fixture_catalog_path = root.join(constants_str::VALUE_C1590960);
        std::fs::remove_file(fixture_catalog_path.as_path())?;
        assert!([false, true].into_iter().all(|write_enabled| matches!(
            synchronize(write_enabled),
            Err(crate::scaffold_error::ScaffoldError::Read(
                server_runtime_http::bounded_read_error::BoundedReadError::Io { source },
            )) if source.kind() == std::io::ErrorKind::NotFound
        )));
        write(fixture_catalog_path.as_path(), catalog.as_ref());
        synchronize(false)?;
        entries
            .get_inner()
            .as_slice()
            .iter()
            .flat_map(|entry| {
                [
                    root.join(format!(
                        "{}/{}",
                        entry.get_crate_name().as_ref(),
                        constants_str::CARGO_TOML
                    )),
                    root.join(entry.get_dockerfile().as_ref()),
                ]
            })
            .try_for_each(|required_file| {
                let (_, original) = originals
                    .iter()
                    .find(|(original_path, _)| original_path == &required_file)
                    .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?;
                std::fs::remove_file(required_file.as_path())?;
                assert!([false, true].into_iter().all(|write_enabled| matches!(
                    synchronize(write_enabled),
                    Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment),
                )));
                write(required_file.as_path(), original.as_ref());
                synchronize(false)?;
                Ok::<(), crate::scaffold_error::ScaffoldError>(())
            })?;
        assert!(originals.iter().all(|(path, text)| {
            crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
                crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            )
            .is_ok_and(|observed| observed.as_ref() == text.as_ref())
        }));
        std::fs::remove_dir_all(root)?;

        Ok(())
    })();
    assert!(matches!(result, Ok(())));
}

#[test]
fn test_project_name_validation_rejects_each_lexical_violation() {
    [
        String::from(constants_str::EMPTY),
        String::from(constants_str::SPACE),
        constants_str::X.to_ascii_uppercase(),
        format!("{}{}", constants_str::UNDERSCORE, constants_str::X),
        format!("{}{}", constants_str::X, constants_str::UNDERSCORE),
        format!(
            "{}{}{}",
            constants_str::X,
            constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE,
            constants_str::X
        ),
        format!(
            "{}{}{}",
            constants_str::X,
            constants_str::HYPHEN,
            constants_str::X
        ),
        format!("{}{}", constants_str::X, char::from(233u8)),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(matches!(
            crate::naming_validate_project_name::naming_validate_project_name(
                crate::project_name_ref::ProjectNameRef::from(text.as_str()),
            ),
            Err(crate::scaffold_error::ScaffoldError::ProjectName)
        ));
    });
}

#[test]
fn test_project_name_validation_accepts_exact_text_limit_and_digit_suffix() {
    [
        constants_str::X.repeat(constants_usize::VALUE_16_777_216),
        format!(
            "{}{}{}",
            constants_str::X,
            constants_str::UNDERSCORE,
            constants_usize::ONE
        ),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(matches!(
            crate::naming_validate_project_name::naming_validate_project_name(
                crate::project_name_ref::ProjectNameRef::from(text.as_str()),
            ),
            Ok(())
        ));
    });
}

#[test]
fn test_repository_url_validation_preserves_delimiter_and_scheme_rules() {
    [char::from(63u8), char::from(35u8)]
        .into_iter()
        .fold((), |(), delimiter| {
            let hostless = format!(
                "{}{delimiter}{}",
                constants_str::HTTPS_SCHEME_PREFIX,
                constants_str::X
            );
            let with_host = format!(
                "{}{}{delimiter}{}",
                constants_str::HTTPS_SCHEME_PREFIX,
                constants_str::X,
                constants_str::X
            );
            assert!(matches!(
                crate::naming_validate_repository_url::naming_validate_repository_url(
                    crate::repository_url_ref::RepositoryUrlRef::from(hostless.as_str()),
                ),
                Err(crate::scaffold_error::ScaffoldError::RepositoryUrl)
            ));
            assert!(matches!(
                crate::naming_validate_repository_url::naming_validate_repository_url(
                    crate::repository_url_ref::RepositoryUrlRef::from(with_host.as_str()),
                ),
                Ok(())
            ));
        });
    [
        format!(
            "{}{}{}",
            constants_str::HTTPS_SCHEME_PREFIX,
            constants_str::X,
            constants_str::SLASH
        ),
        format!(
            "{}{}",
            constants_str::HTTPS_SCHEME_PREFIX.to_ascii_uppercase(),
            constants_str::X
        ),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(matches!(
            crate::naming_validate_repository_url::naming_validate_repository_url(
                crate::repository_url_ref::RepositoryUrlRef::from(text.as_str()),
            ),
            Err(crate::scaffold_error::ScaffoldError::RepositoryUrl)
        ));
    });
    let shallow_host = format!(
        "{}{}",
        constants_str::HTTPS_SCHEME_PREFIX,
        constants_str::SPACE
    );
    assert!(matches!(
        crate::naming_validate_repository_url::naming_validate_repository_url(
            crate::repository_url_ref::RepositoryUrlRef::from(shallow_host.as_str()),
        ),
        Ok(())
    ));
}

#[test]
fn test_template_skip_matches_all_registered_directory_components_exactly() {
    [
        constants_str::GIT,
        constants_str::TARGET,
        constants_str::WORKSPACE_SCAFFOLD_NODE_MODULES,
    ]
    .into_iter()
    .fold((), |(), directory| {
        let skipped = std::path::PathBuf::from(constants_str::X)
            .join(directory)
            .join(constants_str::JSON);
        assert!(bool::from(
            crate::template_fs_should_skip::template_fs_should_skip(
                crate::scaffold_path_ref::ScaffoldPathRef::from(skipped.as_path()),
            )
        ));
        [
            format!("{}{directory}", constants_str::X),
            format!("{directory}{}", constants_str::X),
        ]
        .into_iter()
        .fold((), |(), near_name| {
            let retained = std::path::PathBuf::from(constants_str::X).join(near_name);
            assert!(!bool::from(
                crate::template_fs_should_skip::template_fs_should_skip(
                    crate::scaffold_path_ref::ScaffoldPathRef::from(retained.as_path()),
                )
            ));
        });
    });
}

#[test]
fn test_template_replacements_choose_earliest_position_then_catalog_order() {
    let path = std::env::temp_dir().join(stringify!(
        test_template_replacements_choose_earliest_position_then_catalog_order
    ));
    let overlapping = format!("{}{}", constants_str::X, constants_str::X);
    [
        (
            overlapping.clone(),
            vec![
                (constants_str::X, constants_str::JSON.to_owned()),
                (overlapping.as_str(), constants_str::TRUE.to_owned()),
            ],
            format!("{}{}", constants_str::JSON, constants_str::JSON),
        ),
        (
            overlapping.clone(),
            vec![
                (overlapping.as_str(), constants_str::TRUE.to_owned()),
                (constants_str::X, constants_str::JSON.to_owned()),
            ],
            constants_str::TRUE.to_owned(),
        ),
        (
            format!("{}{}", constants_str::X, constants_str::JSON),
            vec![
                (constants_str::JSON, constants_str::TRUE.to_owned()),
                (constants_str::X, constants_str::SPACE.to_owned()),
            ],
            format!("{}{}", constants_str::SPACE, constants_str::TRUE),
        ),
        (
            overlapping.clone(),
            vec![(constants_str::X, constants_str::EMPTY.to_owned())],
            constants_str::EMPTY.to_owned(),
        ),
    ]
    .into_iter()
    .fold((), |(), (source, replacements, expected)| {
        write(&path, &source);
        assert!(matches!(
            crate::template_fs_replace_file::template_fs_replace_file(
                crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
                crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
            ),
            Ok(())
        ));
        assert_scaffold_file_content(&path, &expected);
    });
    assert!(matches!(std::fs::remove_file(path), Ok(())));
}

#[test]
fn test_template_replacement_preserves_binary_bytes_before_pattern_validation() {
    let path = std::env::temp_dir().join(stringify!(
        test_template_replacement_preserves_binary_bytes_before_pattern_validation
    ));
    let source = [0xffu8, 120u8, 0u8, 0xfeu8];
    assert!(matches!(std::fs::write(&path, source), Ok(())));
    [
        Vec::new(),
        vec![(constants_str::X, constants_str::JSON.to_owned())],
        vec![(constants_str::EMPTY, constants_str::JSON.to_owned())],
    ]
    .into_iter()
    .fold((), |(), replacements| {
        assert!(matches!(
            crate::template_fs_replace_file::template_fs_replace_file(
                crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
                crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
            ),
            Ok(())
        ));
        assert!(std::fs::read(&path).is_ok_and(|content| content == source));
    });
    assert!(matches!(std::fs::remove_file(path), Ok(())));
}

#[test]
fn test_marker_insertion_changes_only_first_unmatched_occurrence() {
    let path = std::env::temp_dir().join(stringify!(
        test_marker_insertion_changes_only_first_unmatched_occurrence
    ));
    let source = format!(
        "{}{}{}",
        constants_str::X,
        constants_str::SPACE,
        constants_str::X
    );
    write(&path, &source);
    assert!(matches!(
        crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::X),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::JSON),
        ),
        Ok(())
    ));
    assert_scaffold_file_content(
        &path,
        &format!(
            "{}{}{}",
            constants_str::JSON,
            constants_str::SPACE,
            constants_str::X
        ),
    );
    assert!(matches!(std::fs::remove_file(path), Ok(())));
}

#[test]
fn test_marker_insertion_errors_preserve_original_file() {
    let path = std::env::temp_dir().join(stringify!(
        test_marker_insertion_errors_preserve_original_file
    ));
    [
        (constants_str::EMPTY, constants_str::JSON),
        (constants_str::X, constants_str::EMPTY),
        (constants_str::JSON, constants_str::TRUE),
    ]
    .into_iter()
    .fold((), |(), (marker, replacement)| {
        write(&path, constants_str::X);
        assert!(matches!(
            crate::template_fs_insert_once::template_fs_insert_once(
                crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
                crate::scaffold_text_ref::ScaffoldTextRef::from(marker),
                crate::scaffold_text_ref::ScaffoldTextRef::from(replacement),
            ),
            Err(crate::scaffold_error::ScaffoldError::Marker)
        ));
        assert_scaffold_file_content(&path, constants_str::X);
    });
    assert!(matches!(std::fs::remove_file(path), Ok(())));
}

#[test]
fn test_generated_marker_errors_preserve_file_in_check_and_write_modes() {
    let path = std::env::temp_dir().join(stringify!(
        test_generated_marker_errors_preserve_file_in_check_and_write_modes
    ));
    [
        (
            constants_str::EMPTY,
            constants_str::TRUE,
            format!("{}{}", constants_str::JSON, constants_str::TRUE),
        ),
        (
            constants_str::JSON,
            constants_str::EMPTY,
            format!("{}{}", constants_str::JSON, constants_str::TRUE),
        ),
        (
            constants_str::JSON,
            constants_str::TRUE,
            constants_str::X.to_owned(),
        ),
        (
            constants_str::JSON,
            constants_str::TRUE,
            format!("{}{}", constants_str::TRUE, constants_str::JSON),
        ),
        (
            constants_str::JSON,
            constants_str::TRUE,
            format!(
                "{}{}{}",
                constants_str::JSON,
                constants_str::TRUE,
                constants_str::TRUE
            ),
        ),
    ]
    .into_iter()
    .fold((), |(), (begin, end, source)| {
        [false, true].into_iter().fold((), |(), should_write| {
            write(&path, &source);
            assert!(matches!(
                crate::synchronize_generated_file::synchronize_generated_file(
                    crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
                    crate::scaffold_text_ref::ScaffoldTextRef::from(begin),
                    crate::scaffold_text_ref::ScaffoldTextRef::from(end),
                    crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::SPACE),
                    crate::should_write::ShouldWrite::from(should_write),
                ),
                Err(crate::scaffold_error::ScaffoldError::Marker)
            ));
            assert_scaffold_file_content(&path, &source);
        });
    });
    assert!(matches!(std::fs::remove_file(path), Ok(())));
}

#[test]
fn test_scaffold_file_reader_enforces_exact_byte_limit_and_utf8() {
    let path = std::env::temp_dir().join(stringify!(
        test_scaffold_file_reader_enforces_exact_byte_limit_and_utf8
    ));
    let maximum = constants_usize::VALUE_16_777_216;
    [constants_usize::ZERO, maximum, maximum.saturating_add(constants_usize::ONE)]
        .into_iter()
        .fold((), |(), length| {
            let source = constants_str::X.repeat(length);
            write(&path, &source);
            let result = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
                crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            );
            if length <= maximum {
                assert!(result.is_ok_and(|text| text.as_ref() == source));
            } else {
                assert!(matches!(result,
                    Err(server_runtime_http::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })
                        if maximum_bytes == server_runtime_http::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(maximum)
                ));
            }
        });
    assert!(matches!(std::fs::write(&path, [0xffu8]), Ok(())));
    assert!(matches!(
        crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
        ),
        Err(server_runtime_http::bounded_read_error::BoundedReadError::Utf8 { .. })
    ));
    assert!(std::fs::read(&path).is_ok_and(|content| content == [0xffu8]));
    assert!(matches!(std::fs::remove_file(path), Ok(())));
}

#[test]
fn test_catalog_requires_each_field_in_intermediate_and_final_service_blocks() {
    let fixture = constants_str::VALUE_D4291B4A;
    [
        constants_str::CRATE,
        constants_str::VALUE_DB669AF6,
        constants_str::VALUE_739ED940,
        constants_str::VALUE_254DB0FB,
        constants_str::VALUE_6105D6CC,
        constants_str::VALUE_94ABCB2D,
        constants_str::VALUE_F8D397A3,
        constants_str::VALUE_20E49707,
        constants_str::RELEASE,
    ]
    .into_iter()
    .fold((), |(), key| {
        [false, true].into_iter().fold((), |(), final_block| {
            let matches_key = |line: &&str| {
                line.split_once('=')
                    .is_some_and(|(name, _value)| name.trim() == key)
            };
            let omitted = if final_block {
                fixture.lines().rfind(matches_key)
            } else {
                fixture.lines().find(matches_key)
            };
            assert!(omitted.is_some());
            let Some(original_line) = omitted else {
                return;
            };
            let catalog =
                fixture.replacen(original_line, constants_str::EMPTY, constants_usize::ONE);
            assert!(matches!(
                crate::service_catalog_parse::service_catalog_parse(
                    crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
                ),
                Err(crate::scaffold_error::ScaffoldError::Catalog)
            ));
        });
    });
}

#[test]
fn test_catalog_port_bounds_preserve_native_values_and_reject_overflow() {
    [1u16, u16::MAX].into_iter().fold((), |(), port| {
        let replacement = format!(
            "{} = {port}{}",
            constants_str::VALUE_F8D397A3,
            constants_str::NEWLINE
        );
        let catalog = constants_str::VALUE_D4291B4A.replacen(
            constants_str::WORKSPACE_SCAFFOLD_PORT_8080_LINE,
            &replacement,
            constants_usize::ONE,
        );
        assert!(
            crate::service_catalog_parse::service_catalog_parse(
                crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
            )
            .is_ok_and(|entries| {
                entries
                    .get_inner()
                    .as_slice()
                    .first()
                    .is_some_and(|entry| entry.get_port().get() == port)
            })
        );
    });
    let overflow = format!(
        "{} = {}{}{}",
        constants_str::VALUE_F8D397A3,
        u16::MAX,
        constants_usize::ZERO,
        constants_str::NEWLINE
    );
    let catalog = constants_str::VALUE_D4291B4A.replacen(
        constants_str::WORKSPACE_SCAFFOLD_PORT_8080_LINE,
        &overflow,
        constants_usize::ONE,
    );
    assert!(matches!(
        crate::service_catalog_parse::service_catalog_parse(
            crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
        ),
        Err(crate::scaffold_error::ScaffoldError::Catalog)
    ));
}

#[test]
fn test_catalog_ignores_comments_and_whitespace_and_preserves_release_flags() {
    let comment = format!(
        "{}{}{}{}",
        constants_str::SPACE,
        char::from(35u8),
        constants_str::X,
        constants_str::NEWLINE
    );
    assert!(matches!(
        crate::service_catalog_parse::service_catalog_parse(
            crate::scaffold_text_ref::ScaffoldTextRef::from(comment.as_str()),
        ),
        Err(crate::scaffold_error::ScaffoldError::Catalog)
    ));
    let catalog = constants_str::VALUE_D4291B4A
        .lines()
        .fold(comment, |mut output, line| {
            output.push_str(constants_str::SPACE);
            output.push_str(line);
            output.push_str(constants_str::SPACE);
            output.push_str(constants_str::NEWLINE);
            output.push_str(constants_str::NEWLINE);
            output
        });
    assert!(
        crate::service_catalog_parse::service_catalog_parse(
            crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
        )
        .is_ok_and(|entries| {
            let values = entries.get_inner().as_slice();
            values.len() == 2usize
                && values
                    .first()
                    .is_some_and(|entry| bool::from(*entry.get_release()))
                && values
                    .last()
                    .is_some_and(|entry| !bool::from(*entry.get_release()))
        })
    );
}

#[test]
fn test_catalog_string_decodes_all_short_and_unicode_escapes_exactly() {
    let decode = |encoded: &str| {
        crate::service_catalog_string_value::service_catalog_string_value(
            crate::scaffold_text_ref::ScaffoldTextRef::from(encoded),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
        )
    };
    [
        (34u8, 34u8),
        (92u8, 92u8),
        (98u8, 8u8),
        (116u8, 9u8),
        (110u8, 10u8),
        (102u8, 12u8),
        (114u8, 13u8),
    ]
    .into_iter()
    .fold((), |(), (escape, expected)| {
        let encoded = format!("{} = \"\\{}\"", constants_str::CRATE, char::from(escape));
        assert!(decode(&encoded).is_ok_and(|value| {
            value.is_some_and(|text| text.as_ref() == char::from(expected).to_string())
        }));
    });
    [
        (
            format!("{} = \"\\u{:04X}\"", constants_str::CRATE, 233u32),
            char::from(233u8).to_string(),
        ),
        (
            format!("{} = \"\\U{:08X}\"", constants_str::CRATE, 0x1f600u32),
            '\u{1f600}'.to_string(),
        ),
        (
            format!("{} = \"{}\"", constants_str::CRATE, char::from(9u8)),
            char::from(9u8).to_string(),
        ),
        (
            format!("{} = \"\"", constants_str::CRATE),
            constants_str::EMPTY.to_owned(),
        ),
    ]
    .into_iter()
    .fold((), |(), (encoded, expected)| {
        assert!(
            decode(&encoded)
                .is_ok_and(|value| { value.is_some_and(|text| text.as_ref() == expected) })
        );
    });
}

#[test]
fn test_catalog_string_rejects_malformed_escapes_quotes_and_controls() {
    [
        format!("{} = \"\\\"", constants_str::CRATE),
        format!(
            "{} = \"\\u{}\"",
            constants_str::CRATE,
            constants_str::X.repeat(4usize)
        ),
        format!("{} = \"\\u{:04X}\"", constants_str::CRATE, 0xd800u32),
        format!("{} = \"\\U{:08X}\"", constants_str::CRATE, 0x0011_0000u32),
        format!("{} = \"\\u{}\"", constants_str::CRATE, constants_usize::ONE),
        format!("{} = \"\\U{}\"", constants_str::CRATE, constants_usize::ONE),
        format!(
            "{} = \"{}\"{}\"",
            constants_str::CRATE,
            constants_str::X,
            constants_str::X
        ),
        format!("{} = \"{}\"", constants_str::CRATE, char::from(10u8)),
        format!("{} = {}", constants_str::CRATE, constants_str::X),
        format!("{} = \"{}", constants_str::CRATE, constants_str::X),
    ]
    .into_iter()
    .fold((), |(), encoded| {
        assert!(matches!(
            crate::service_catalog_string_value::service_catalog_string_value(
                crate::scaffold_text_ref::ScaffoldTextRef::from(encoded.as_str()),
                crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
            ),
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
    });
}

#[test]
fn test_catalog_string_nonmatching_keys_and_missing_assignments_return_none() {
    [
        constants_str::JSON.to_owned(),
        constants_str::CRATE.to_owned(),
        format!(
            "{}{} = \"{}\"",
            constants_str::CRATE,
            constants_str::X,
            constants_str::X
        ),
    ]
    .into_iter()
    .fold((), |(), encoded| {
        assert!(
            crate::service_catalog_string_value::service_catalog_string_value(
                crate::scaffold_text_ref::ScaffoldTextRef::from(encoded.as_str()),
                crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::CRATE),
            )
            .is_ok_and(|value| value.is_none())
        );
    });
}

#[test]
fn test_release_renderer_returns_empty_for_empty_and_disabled_catalogs() {
    let empty =
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(&[][..]),
        );
    assert_eq!(empty.as_ref(), constants_str::EMPTY);
    let catalog = constants_str::VALUE_D4291B4A.replace(
        &format!("{} = {}", constants_str::RELEASE, constants_str::TRUE),
        &format!("{} = {}", constants_str::RELEASE, constants_str::FALSE),
    );
    let parsed = crate::service_catalog_parse::service_catalog_parse(
        crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
    );
    assert!(parsed.is_ok());
    let Ok(entries) = parsed else {
        return;
    };
    assert_eq!(entries.get_inner().as_slice().len(), 2usize);
    let rendered =
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(
                entries.get_inner().as_slice(),
            ),
        );
    assert_eq!(rendered.as_ref(), constants_str::EMPTY);
}

#[test]
fn test_release_renderer_preserves_multiple_enabled_entry_order() {
    let catalog = constants_str::VALUE_D4291B4A.replace(
        &format!("{} = {}", constants_str::RELEASE, constants_str::FALSE),
        &format!("{} = {}", constants_str::RELEASE, constants_str::TRUE),
    );
    let parsed = crate::service_catalog_parse::service_catalog_parse(
        crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
    );
    assert!(parsed.is_ok());
    let Ok(entries) = parsed else {
        return;
    };
    let values = entries.get_inner().as_slice();
    assert_eq!(values.len(), 2usize);
    let Some(last) = values.last() else {
        return;
    };
    let expected = format!(
        "{}{}{}{}{}{}",
        constants_str::VALUE_CF9A8E24,
        constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT,
        last.get_image().as_ref(),
        constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT,
        last.get_dockerfile().as_ref(),
        constants_str::NEWLINE
    );
    let rendered =
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(values),
        );
    assert_eq!(rendered.as_ref(), expected.as_str());
    assert!(values.iter().all(|entry| bool::from(*entry.get_release())));
}

#[test]
fn test_release_renderer_preserves_error_text_fallback_for_oversized_output() {
    let maximum = constants_usize::VALUE_16_777_216;
    let original = constants_str::VALUE_D4291B4A.lines().find(|line| {
        line.split_once('=')
            .is_some_and(|(key, _value)| key.trim() == constants_str::VALUE_6105D6CC)
    });
    assert!(original.is_some());
    let Some(image_line) = original else {
        return;
    };
    let image = constants_str::X.repeat(maximum);
    let catalog = constants_str::VALUE_D4291B4A.replacen(
        image_line,
        &format!("{} = \"{image}\"", constants_str::VALUE_6105D6CC),
        constants_usize::ONE,
    );
    let parsed = crate::service_catalog_parse::service_catalog_parse(
        crate::scaffold_text_ref::ScaffoldTextRef::from(catalog.as_str()),
    );
    assert!(parsed.is_ok());
    let Ok(entries) = parsed else {
        return;
    };
    let output_length = maximum
        .saturating_add(constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT.len())
        .saturating_add(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT.len())
        .saturating_add(constants_str::VALUE_DD2C0EB6.len())
        .saturating_add(constants_str::NEWLINE.len());
    let rejected =
        crate::scaffold_text::ScaffoldText::try_from(constants_str::X.repeat(output_length));
    assert!(rejected.is_err());
    let Err(length_error) = rejected else {
        return;
    };
    let expected = crate::scaffold_text::ScaffoldText::from(length_error);
    let rendered =
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(
                entries.get_inner().as_slice(),
            ),
        );
    assert_eq!(rendered.as_ref(), expected.as_ref());
    assert!(!rendered.as_ref().is_empty());
    assert!(rendered.as_ref().len() < maximum);
}

#[test]
fn test_service_text_wrappers_preserve_empty_unicode_and_exact_byte_bounds() {
    let maximum = constants_usize::VALUE_16_777_216;
    let unicode = char::from(233u8).to_string();
    [
        String::new(),
        format!(
            "{}{}{}{}",
            constants_str::SPACE,
            unicode,
            char::from(0u8),
            constants_str::SPACE
        ),
        constants_str::X.repeat(maximum),
        unicode.repeat(maximum.checked_div(unicode.len()).unwrap_or_default()),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(
            [
                crate::service_crate::ServiceCrate::try_from(text.clone())
                    .is_ok_and(|value| value.as_ref() == text.as_str()),
                crate::service_compose_name::ServiceComposeName::try_from(text.clone())
                    .is_ok_and(|value| value.as_ref() == text.as_str()),
                crate::service_compose_file::ServiceComposeFile::try_from(text.clone())
                    .is_ok_and(|value| value.as_ref() == text.as_str()),
                crate::service_dockerfile::ServiceDockerfile::try_from(text.clone())
                    .is_ok_and(|value| value.as_ref() == text.as_str()),
                crate::service_image::ServiceImage::try_from(text.clone())
                    .is_ok_and(|value| value.as_ref() == text.as_str()),
                crate::service_kubernetes_manifest::ServiceKubernetesManifest::try_from(
                    text.clone()
                )
                .is_ok_and(|value| value.as_ref() == text.as_str()),
                crate::service_socket_env::ServiceSocketEnv::try_from(text.clone())
                    .is_ok_and(|value| value.as_ref() == text.as_str()),
            ]
            .into_iter()
            .all(|preserved| preserved)
        );
    });
}

#[test]
fn test_service_text_wrappers_reject_oversized_ascii_and_unicode_with_exact_lengths() {
    let maximum = constants_usize::VALUE_16_777_216;
    let unicode = char::from(233u8).to_string();
    [
        constants_str::X.repeat(maximum.saturating_add(constants_usize::ONE)),
        format!("{}{}", unicode.repeat(maximum.checked_div(unicode.len()).unwrap_or_default()), unicode),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!([
            matches!(
                crate::service_crate::ServiceCrate::try_from(text.clone()),
                Err(crate::service_crate::ServiceCrateTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
            matches!(
                crate::service_compose_name::ServiceComposeName::try_from(text.clone()),
                Err(crate::service_compose_name::ServiceComposeNameTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
            matches!(
                crate::service_compose_file::ServiceComposeFile::try_from(text.clone()),
                Err(crate::service_compose_file::ServiceComposeFileTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
            matches!(
                crate::service_dockerfile::ServiceDockerfile::try_from(text.clone()),
                Err(crate::service_dockerfile::ServiceDockerfileTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
            matches!(
                crate::service_image::ServiceImage::try_from(text.clone()),
                Err(crate::service_image::ServiceImageTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
            matches!(
                crate::service_kubernetes_manifest::ServiceKubernetesManifest::try_from(text.clone()),
                Err(crate::service_kubernetes_manifest::ServiceKubernetesManifestTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
            matches!(
                crate::service_socket_env::ServiceSocketEnv::try_from(text.clone()),
                Err(crate::service_socket_env::ServiceSocketEnvTryFromStringError::TooLong { len, max })
                    if len == text.len() && max == maximum
            ),
        ].into_iter().all(|rejected| rejected));
    });
}

#[cfg(unix)]
#[test]
fn test_template_copy_rejects_regular_and_dangling_source_root_symlinks_before_writes() {
    let root = std::env::temp_dir().join(stringify!(
        test_template_copy_rejects_regular_and_dangling_source_root_symlinks_before_writes
    ));
    let directory = root.join(constants_str::X);
    let source_file = directory.join(constants_str::CARGO_TOML);
    write(source_file.as_path(), constants_str::X);
    let file = root.join(constants_str::TRUE);
    write(file.as_path(), constants_str::X);
    let missing = root.join(constants_str::FALSE);
    let link = root.join(constants_str::ENV_EXAMPLE);
    let absent_parent = root.join(constants_str::TARGET);
    let absent_destination = absent_parent.join(constants_str::X);
    let existing_destination = root.join(constants_str::JSON);
    let existing_file = existing_destination.join(constants_str::CARGO_TOML);
    write(existing_file.as_path(), constants_str::JSON);
    let replacements = [(constants_str::X, constants_str::TRUE.to_owned())];
    [directory.as_path(), file.as_path(), missing.as_path()]
        .into_iter()
        .fold((), |(), target| {
            assert!(matches!(
                std::os::unix::fs::symlink(target, link.as_path()),
                Ok(())
            ));
            [absent_destination.as_path(), existing_destination.as_path()]
                .into_iter()
                .fold((), |(), destination| {
                    assert!(matches!(
                        crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
                            crate::scaffold_path_ref::ScaffoldPathRef::from(link.as_path()),
                            crate::scaffold_path_ref::ScaffoldPathRef::from(destination),
                            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
                        ),
                        Err(crate::scaffold_error::ScaffoldError::Catalog)
                    ));
                    assert!(!absent_parent.exists());
                    assert_scaffold_file_content(existing_file.as_path(), constants_str::JSON);
                    assert_scaffold_file_content(source_file.as_path(), constants_str::X);
                    assert_scaffold_file_content(file.as_path(), constants_str::X);
                    assert!(!missing.exists());
                });
            assert!(matches!(std::fs::remove_file(link.as_path()), Ok(())));
        });
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_template_copy_io_failures_preserve_source_and_destination_creation_order() {
    let root = std::env::temp_dir().join(stringify!(
        test_template_copy_io_failures_preserve_source_and_destination_creation_order
    ));
    let directory = root.join(constants_str::X);
    let source_file = directory.join(constants_str::CARGO_TOML);
    write(source_file.as_path(), constants_str::X);
    let missing = root.join(constants_str::FALSE);
    let destination = root.join(constants_str::TARGET);
    let blocked = root.join(constants_str::JSON);
    write(blocked.as_path(), constants_str::JSON);
    [
        (missing.as_path(), destination.as_path()),
        (source_file.as_path(), destination.as_path()),
        (directory.as_path(), blocked.as_path()),
    ]
    .into_iter()
    .enumerate()
    .fold((), |(), (index, (source, output))| {
        let result = crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(source),
            crate::scaffold_path_ref::ScaffoldPathRef::from(output),
            crate::replacements_ref::ReplacementsRef::from(&[][..]),
        );
        assert!(matches!(
            result,
            Err(crate::scaffold_error::ScaffoldError::Io(_))
        ));
        if index == constants_usize::ZERO {
            assert!(!destination.exists());
        } else {
            assert!(destination.is_dir());
            assert!(
                std::fs::read_dir(destination.as_path())
                    .is_ok_and(|mut entries| entries.next().is_none())
            );
        }
        assert_scaffold_file_content(source_file.as_path(), constants_str::X);
        assert_scaffold_file_content(blocked.as_path(), constants_str::JSON);
        assert!(!missing.exists());
    });
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_template_copy_propagates_direct_and_nested_file_copy_errors_without_changes() {
    let root = std::env::temp_dir().join(stringify!(
        test_template_copy_propagates_direct_and_nested_file_copy_errors_without_changes
    ));
    [false, true].into_iter().fold((), |(), nested| {
        let source = root.join(nested.to_string()).join(constants_str::X);
        let destination = root.join(nested.to_string()).join(constants_str::TARGET);
        let relative = if nested {
            std::path::PathBuf::from(constants_str::JSON).join(constants_str::CARGO_TOML)
        } else {
            std::path::PathBuf::from(constants_str::CARGO_TOML)
        };
        let source_file = source.join(relative.as_path());
        let blocked_directory = destination.join(relative.as_path());
        let preserved_file = blocked_directory.join(constants_str::ENV_EXAMPLE);
        write(source_file.as_path(), constants_str::X);
        write(preserved_file.as_path(), constants_str::TRUE);
        let replacements = [(constants_str::X, constants_str::JSON.to_owned())];
        let result = crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(source.as_path()),
            crate::scaffold_path_ref::ScaffoldPathRef::from(destination.as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        );
        assert!(matches!(
            result,
            Err(crate::scaffold_error::ScaffoldError::Io(_))
        ));
        assert!(blocked_directory.is_dir());
        assert_scaffold_file_content(source_file.as_path(), constants_str::X);
        assert_scaffold_file_content(preserved_file.as_path(), constants_str::TRUE);
    });
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_template_copy_propagates_direct_and_nested_replacement_errors_after_copy() {
    let root = std::env::temp_dir().join(stringify!(
        test_template_copy_propagates_direct_and_nested_replacement_errors_after_copy
    ));
    [false, true].into_iter().fold((), |(), nested| {
        let source = root.join(nested.to_string()).join(constants_str::X);
        let destination = root.join(nested.to_string()).join(constants_str::TARGET);
        let relative = if nested {
            std::path::PathBuf::from(constants_str::JSON).join(constants_str::CARGO_TOML)
        } else {
            std::path::PathBuf::from(constants_str::CARGO_TOML)
        };
        let source_file = source.join(relative.as_path());
        let destination_file = destination.join(relative.as_path());
        write(source_file.as_path(), constants_str::X);
        write(destination_file.as_path(), constants_str::JSON);
        let replacements = [(constants_str::EMPTY, constants_str::TRUE.to_owned())];
        let result = crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(source.as_path()),
            crate::scaffold_path_ref::ScaffoldPathRef::from(destination.as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        );
        assert!(matches!(
            result,
            Err(crate::scaffold_error::ScaffoldError::Catalog)
        ));
        assert_scaffold_file_content(source_file.as_path(), constants_str::X);
        assert_scaffold_file_content(destination_file.as_path(), constants_str::X);
    });
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_template_copy_preserves_empty_directories_and_binary_bytes_without_pattern_validation() {
    let root = std::env::temp_dir().join(stringify!(
        test_template_copy_preserves_empty_directories_and_binary_bytes_without_pattern_validation
    ));
    let source = root.join(constants_str::X);
    let destination = root.join(constants_str::TARGET);
    let empty_source = source.join(constants_str::JSON);
    let empty_destination = destination.join(constants_str::JSON);
    assert!(matches!(
        std::fs::create_dir_all(empty_source.as_path()),
        Ok(())
    ));
    let replacements = [(constants_str::EMPTY, constants_str::TRUE.to_owned())];
    let copy = || {
        crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
            crate::scaffold_path_ref::ScaffoldPathRef::from(source.as_path()),
            crate::scaffold_path_ref::ScaffoldPathRef::from(destination.as_path()),
            crate::replacements_ref::ReplacementsRef::from(replacements.as_slice()),
        )
    };
    assert!(matches!(copy(), Ok(())));
    assert!(empty_destination.is_dir());
    assert!(
        std::fs::read_dir(empty_destination.as_path())
            .is_ok_and(|mut entries| entries.next().is_none())
    );
    let source_file = source.join(constants_str::CARGO_TOML);
    let destination_file = destination.join(constants_str::CARGO_TOML);
    let bytes = [0xffu8, b'x', 0u8, 0xfeu8];
    assert!(matches!(
        std::fs::write(source_file.as_path(), bytes),
        Ok(())
    ));
    assert!(matches!(copy(), Ok(())));
    assert!(std::fs::read(source_file.as_path()).is_ok_and(|actual| actual.as_slice() == bytes));
    assert!(
        std::fs::read(destination_file.as_path()).is_ok_and(|actual| actual.as_slice() == bytes)
    );
    assert!(
        std::fs::read_dir(empty_destination.as_path())
            .is_ok_and(|mut entries| entries.next().is_none())
    );
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_cargo_projection_maps_success_and_failed_exit_for_both_projection_modes() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(constants_str::TARGET)
        .join(stringify!(
            test_cargo_projection_maps_success_and_failed_exit_for_both_projection_modes
        ));
    let sentinel = root.join(constants_str::ENV_EXAMPLE);
    write(sentinel.as_path(), constants_str::JSON);
    [
        (crate::generated_projection::GeneratedProjection::CodeStyle, constants_str::UPDATE_CODE_STYLE_SNAPSHOTS),
        (crate::generated_projection::GeneratedProjection::Config, constants_str::UPDATE_CONFIG_PROJECTIONS),
    ]
    .into_iter()
    .fold((), |(), (projection, environment_name)| {
        [false, true].into_iter().fold((), |(), write_enabled| {
            let successful_arguments = [constants_str::VERSION];
            let failed_arguments = [constants_str::VERSION, constants_str::VERSION];
            [successful_arguments.as_slice(), failed_arguments.as_slice()]
                .into_iter()
                .enumerate()
                .fold((), |(), (index, arguments)| {
                    let result = crate::synchronize_cargo_owned_projection::synchronize_cargo_owned_projection(
                        crate::scaffold_path_ref::ScaffoldPathRef::from(root.as_path()),
                        crate::cargo_args_ref::CargoArgsRef::from(arguments),
                        crate::update_env_name::UpdateEnvName::from(environment_name),
                        projection,
                        crate::should_write::ShouldWrite::from(write_enabled),
                    );
                    if index == constants_usize::ZERO {
                        assert!(matches!(result, Ok(())));
                    } else {
                        assert!(matches!(
                            (projection, result),
                            (crate::generated_projection::GeneratedProjection::CodeStyle, Err(crate::scaffold_error::ScaffoldError::GeneratedCodeStyle))
                                | (crate::generated_projection::GeneratedProjection::Config, Err(crate::scaffold_error::ScaffoldError::GeneratedConfig))
                        ));
                    }
                    assert_scaffold_file_content(sentinel.as_path(), constants_str::JSON);
                    assert!(std::fs::read_dir(root.as_path()).is_ok_and(|mut entries| entries.next().is_some_and(|entry| entry.is_ok()) && entries.next().is_none()));
                });
        });
    });
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}

#[test]
fn test_cargo_projection_preserves_launch_errors_for_invalid_working_directories() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(constants_str::TARGET)
        .join(stringify!(
            test_cargo_projection_preserves_launch_errors_for_invalid_working_directories
        ));
    let file = root.join(constants_str::TRUE);
    write(file.as_path(), constants_str::JSON);
    let missing = root.join(constants_str::FALSE);
    let arguments = [constants_str::VERSION];
    [
        (crate::generated_projection::GeneratedProjection::CodeStyle, constants_str::UPDATE_CODE_STYLE_SNAPSHOTS),
        (crate::generated_projection::GeneratedProjection::Config, constants_str::UPDATE_CONFIG_PROJECTIONS),
    ]
    .into_iter()
    .fold((), |(), (projection, environment_name)| {
        [false, true].into_iter().fold((), |(), write_enabled| {
            [file.as_path(), missing.as_path()].into_iter().fold((), |(), directory| {
                let result = crate::synchronize_cargo_owned_projection::synchronize_cargo_owned_projection(
                    crate::scaffold_path_ref::ScaffoldPathRef::from(directory),
                    crate::cargo_args_ref::CargoArgsRef::from(arguments.as_slice()),
                    crate::update_env_name::UpdateEnvName::from(environment_name),
                    projection,
                    crate::should_write::ShouldWrite::from(write_enabled),
                );
                assert!(matches!(&result, Err(crate::scaffold_error::ScaffoldError::Io(_))));
                assert!(result.is_err_and(|error| std::error::Error::source(&error)
                    .is_some_and(|source| source.downcast_ref::<crate::scaffold_io_error::ScaffoldIoError>().is_some())));
                assert_scaffold_file_content(file.as_path(), constants_str::JSON);
                assert!(!missing.exists());
            });
        });
    });
    assert!(matches!(std::fs::remove_dir_all(root), Ok(())));
}
