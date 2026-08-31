fn assert_scaffold_file_content(path: &std::path::Path, expected: &str) {
    let actual =
        std::fs::read_to_string(path).expect("371dbe92 assert_file_content invariant must hold");
    assert_eq!(actual, expected, "239c17b0: {}", path.display());
}

fn write(path: &std::path::Path, value: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("2f0ad03a write invariant must hold");
    }
    std::fs::write(path, value).expect("79af6dc8 write invariant must hold");
}

#[test]
fn validates_and_converts_project_names() {
    let valid = crate::project_name_ref::ProjectNameRef::from(constants_str::VALUE_F9EA74B8);
    crate::naming_validate_project_name::naming_validate_project_name(valid)
        .expect("96de3a80 validates_and_converts_project_names invariant must hold");
    assert_eq!(
        crate::naming_kebab_case::naming_kebab_case(valid).as_ref(),
        "order-platform"
    );
    assert_eq!(
        crate::naming_title_case::naming_title_case(valid).as_ref(),
        "Order Platform"
    );
    assert_eq!(
        crate::naming_upper_camel_case::naming_upper_camel_case(valid).as_ref(),
        "OrderPlatform"
    );
    assert!(
        crate::naming_validate_project_name::naming_validate_project_name(
            crate::project_name_ref::ProjectNameRef::from("Order-Platform")
        )
        .is_err()
    );
}

#[test]
fn requires_https_repository_url() {
    crate::naming_validate_repository_url::naming_validate_repository_url(
        crate::repository_url_ref::RepositoryUrlRef::from(constants_str::VALUE_A680FDEF),
    )
    .expect("28c1e7a4 requires_https_repository_url invariant must hold");
    assert!(
        crate::naming_validate_repository_url::naming_validate_repository_url(
            crate::repository_url_ref::RepositoryUrlRef::from(
                "http://example.com/team/order_platform",
            )
        )
        .is_err()
    );
}

#[test]
fn deployment_projection_check_rejects_stale_generated_content() {
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
    .expect(
        "5a7e3c91 deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
    crate::synchronize_generated_file::synchronize_generated_file(
        crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
        crate::scaffold_text_ref::ScaffoldTextRef::from(begin),
        crate::scaffold_text_ref::ScaffoldTextRef::from(end),
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_48AA6CAE),
        crate::should_write::ShouldWrite::from(false),
    )
    .expect(
        "d2f8b4a6 deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
    std::fs::remove_file(path).expect(
        "9c1e6a3f deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
}

#[test]
fn service_catalog_owns_ci_and_release_projection_values() {
    let entries = crate::service_catalog_parse::service_catalog_parse(
        crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::VALUE_D4291B4A),
    )
    .expect("4e8b2d7a service_catalog_owns_ci_and_release_projection_values invariant must hold");
    let entries_ref = crate::service_catalog_entries_ref::ServiceCatalogEntriesRef::from(
        entries.get_inner().as_slice(),
    );
    assert_eq!(
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            entries_ref,
        )
        .as_ref(),
        "          - name: application\n            dockerfile: Dockerfile\n"
    );
    assert_eq!(
        crate::service_catalog_render_release_entries::service_catalog_render_release_entries(
            entries_ref
        )
        .as_ref(),
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
        vec![b'x'; constants_usize::VALUE_16_777_216.saturating_add(constants_usize::ONE)],
    )
    .expect("d97e30ac rejects_scaffold_text_over_size_limit invariant must hold");
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
    std::fs::remove_file(path)
        .expect("51cd7b2e rejects_scaffold_text_over_size_limit invariant must hold");
}

#[test]
fn service_scaffold_registers_all_artifacts() {
    let root = std::env::temp_dir().join(format!("workspace-scaffold-test-{}", std::process::id()));
    if root.exists() {
        std::fs::remove_dir_all(root.as_path())
            .expect("1449608d service_scaffold_registers_all_artifacts invariant must hold");
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
        let kebab = crate::naming_kebab_case::naming_kebab_case(service_name);
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
                crate::naming_upper_camel_case::naming_upper_camel_case(service_name)
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
    .expect("4bff1d79 insert_sql invariant must hold");
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
    std::fs::remove_dir_all(root).expect("6f608418 insert_sql invariant must hold");
}
