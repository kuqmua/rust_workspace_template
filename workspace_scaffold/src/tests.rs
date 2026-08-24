fn assert_file_content(path: &std::path::Path, expected: &str) {
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
    let valid = super::ProjectNameRef::from("order_platform");
    super::naming::validate_project_name(valid)
        .expect("96de3a80 validates_and_converts_project_names invariant must hold");
    assert_eq!(super::naming::kebab_case(valid).as_ref(), "order-platform");
    assert_eq!(super::naming::title_case(valid).as_ref(), "Order Platform");
    assert_eq!(
        super::naming::upper_camel_case(valid).as_ref(),
        "OrderPlatform"
    );
    assert!(super::naming::validate_project_name(super::ProjectNameRef("Order-Platform")).is_err());
}

#[test]
fn requires_https_repository_url() {
    super::naming::validate_repository_url(super::RepositoryUrlRef::from(
        "https://example.com/team/order_platform",
    ))
    .expect("28c1e7a4 requires_https_repository_url invariant must hold");
    assert!(
        super::naming::validate_repository_url(super::RepositoryUrlRef(
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
    .expect(
        "5a7e3c91 deployment_projection_check_rejects_stale_generated_content invariant must hold",
    );
    super::synchronize_generated_file(
        super::StdScaffoldPathRef::from(path.as_path()),
        super::ScaffoldTextRef::from(begin),
        super::ScaffoldTextRef::from(end),
        super::ScaffoldTextRef::from("current\n"),
        super::ShouldWrite::from(false),
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
    let entries = super::service_catalog::parse(super::ScaffoldTextRef::from(
            "[[service]]\ncrate = \"server\"\ncompose = \"server\"\ncompose_file = \"docker-compose.yml\"\ndockerfile = \"Dockerfile\"\nimage = \"application\"\nkubernetes = \"deploy/k8s/base/application.yaml\"\nport = 8080\nrelease = true\nsocket_env = \"SERVICE_SOCKET_ADDRESS\"\n\n[[service]]\ncrate = \"worker\"\ncompose = \"worker\"\ncompose_file = \"docker-compose.worker.yml\"\ndockerfile = \"worker/Dockerfile\"\nimage = \"worker\"\nkubernetes = \"deploy/k8s/base/worker.yaml\"\nport = 8082\nrelease = false\nsocket_env = \"WORKER_SERVICE_SOCKET_ADDRESS\"\n",
        ))
        .expect("4e8b2d7a service_catalog_owns_ci_and_release_projection_values invariant must hold");
    let entries_ref = super::ServiceCatalogEntriesRef::from(entries.0.as_slice());
    assert_eq!(
        super::service_catalog::render_ci_matrix(entries_ref).as_ref(),
        "          - name: application\n            dockerfile: Dockerfile\n"
    );
    assert_eq!(
        super::service_catalog::render_release_matrix(entries_ref).as_ref(),
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
    let result =
        super::template_fs::read_bounded_text(super::StdScaffoldPathRef::from(path.as_path()));
    assert!(
        matches!(
            result,
            Err(super::ServerRuntimeBoundedReadError(
                server_runtime_http::BoundedReadError::ExceedsMaximum { .. }
            ))
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
    .expect("4bff1d79 insert_sql invariant must hold");
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
    std::fs::remove_dir_all(root).expect("6f608418 insert_sql invariant must hold");
}
