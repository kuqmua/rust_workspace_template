#[derive(Clone, Copy, Debug)]
struct ProjectNameRef<'value>(&'value str);

#[derive(Clone, Copy, Debug)]
struct RepositoryUrlRef<'value>(&'value str);

#[derive(Clone, Copy, Debug)]
struct ServicePort(u16);

#[derive(Debug, thiserror::Error)]
enum ScaffoldError {
    #[error(
        "usage: workspace-scaffold project <snake_case_name> <repository_url> | service <snake_case_name> <port>"
    )]
    Arguments,
    #[error("workspace operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("workspace file does not contain the expected template marker")]
    Marker,
    #[error("project or service name must be non-empty lowercase snake_case ASCII")]
    ProjectName,
    #[error("repository URL must use https:// and must not end with /")]
    RepositoryUrl,
    #[error("service destination already exists")]
    ServiceExists,
    #[error("service port must be greater than zero")]
    ServicePort,
}

fn validate_project_name(value: ProjectNameRef<'_>) -> Result<(), ScaffoldError> {
    let text = value.0;
    if text.is_empty()
        || text.starts_with('_')
        || text.ends_with('_')
        || text.contains("__")
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
    if !value.0.starts_with("https://") || value.0.ends_with('/') {
        return Err(ScaffoldError::RepositoryUrl);
    }
    Ok(())
}

fn kebab_case(value: ProjectNameRef<'_>) -> String {
    value.0.replace('_', "-")
}

fn title_case(value: ProjectNameRef<'_>) -> String {
    value
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
        .join(" ")
}

#[allow(
    clippy::single_call_fn,
    reason = "service scaffold owns identifier case conversion"
)]
fn upper_camel_case(value: ProjectNameRef<'_>) -> String {
    title_case(value).replace(' ', "")
}

#[allow(
    clippy::single_call_fn,
    reason = "identity traversal owns ignored directory policy"
)]
fn should_skip(path: &std::path::Path) -> bool {
    path.components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(".git" | "target" | "node_modules")
        )
    })
}

fn replace_file(
    path: &std::path::Path,
    replacements: &[(&str, String)],
) -> Result<(), ScaffoldError> {
    let Ok(mut contents) = std::fs::read_to_string(path) else {
        return Ok(());
    };
    contents = replacements.iter().fold(contents, |value, (from, to)| {
        value.replace(from, to.as_str())
    });
    std::fs::write(path, contents)?;
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "project command owns identity traversal"
)]
fn rename_identity(
    root: &std::path::Path,
    project_name: ProjectNameRef<'_>,
    repository_url: RepositoryUrlRef<'_>,
) -> Result<(), ScaffoldError> {
    let replacements = [
        (
            "https://github.com/kuqmua/rust_workspace_template",
            repository_url.0.to_owned(),
        ),
        ("rust_workspace_template", project_name.0.to_owned()),
        ("rust-workspace-template", kebab_case(project_name)),
        (
            "Rust microservice workspace template",
            title_case(project_name),
        ),
    ];
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        if should_skip(path.as_path()) {
            continue;
        }
        if path.is_dir() {
            std::fs::read_dir(path)?.try_for_each(|entry| {
                pending.push(entry?.path());
                Ok::<(), std::io::Error>(())
            })?;
        } else {
            replace_file(path.as_path(), &replacements)?;
        }
    }
    Ok(())
}

fn copy_template_tree(
    source: &std::path::Path,
    destination: &std::path::Path,
    replacements: &[(&str, String)],
) -> Result<(), ScaffoldError> {
    std::fs::create_dir_all(destination)?;
    std::fs::read_dir(source)?.try_for_each(|entry_result| {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_template_tree(
                source_path.as_path(),
                destination_path.as_path(),
                replacements,
            )
        } else {
            let _copied_bytes = std::fs::copy(source_path, destination_path.as_path())?;
            replace_file(destination_path.as_path(), replacements)
        }
    })?;
    Ok(())
}

fn insert_once(
    path: &std::path::Path,
    marker: &str,
    replacement: &str,
) -> Result<(), ScaffoldError> {
    let contents = std::fs::read_to_string(path)?;
    if contents.contains(replacement) {
        return Ok(());
    }
    let updated = contents.replacen(marker, replacement, 1usize);
    if updated == contents {
        return Err(ScaffoldError::Marker);
    }
    std::fs::write(path, updated)?;
    Ok(())
}

#[allow(
    clippy::single_call_fn,
    reason = "service command owns complete scaffold composition"
)]
fn scaffold_service(
    root: &std::path::Path,
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
        .any(|path| root.join(path).exists())
    {
        return Err(ScaffoldError::ServiceExists);
    }
    let kebab = kebab_case(service_name);
    let upper_snake = service.to_ascii_uppercase();
    let replacements = [
        ("notification_service", service.to_owned()),
        ("notification-service", kebab.clone()),
        ("NOTIFICATION", upper_snake.clone()),
        ("Notification", upper_camel_case(service_name)),
        ("notification", service.to_owned()),
        ("8081", port.0.to_string()),
    ];
    copy_template_tree(
        root.join("notification_service").as_path(),
        root.join(service).as_path(),
        &replacements,
    )?;
    copy_template_tree(
        root.join("notification_service_config").as_path(),
        root.join(config.as_str()).as_path(),
        &replacements,
    )?;
    copy_template_tree(
        root.join("notification_service_contract").as_path(),
        root.join(contract.as_str()).as_path(),
        &replacements,
    )?;

    let manifest = root.join("Cargo.toml");
    insert_once(
        manifest.as_path(),
        "  \"notification_service_contract\",",
        format!(
            "  \"notification_service_contract\",\n  \"{service}\",\n  \"{config}\",\n  \"{contract}\"," 
        )
        .as_str(),
    )?;
    let dependency_marker = "notification_service_contract = { path = \"./notification_service_contract\", version = \"0.1.0\" }";
    insert_once(
        manifest.as_path(),
        dependency_marker,
        format!(
            "{dependency_marker}\n{service} = {{ path = \"./{service}\", version = \"0.1.0\" }}\n{config} = {{ path = \"./{config}\", version = \"0.1.0\" }}\n{contract} = {{ path = \"./{contract}\", version = \"0.1.0\" }}"
        )
        .as_str(),
    )?;

    let k8s_source = root.join("deploy/k8s/base/notification-service.yaml");
    let k8s_file_name = format!("{kebab}.yaml");
    let k8s_destination = root.join("deploy/k8s/base").join(k8s_file_name.as_str());
    let _copied_bytes = std::fs::copy(k8s_source, k8s_destination.as_path())?;
    replace_file(k8s_destination.as_path(), &replacements)?;
    let mut k8s_contents = std::fs::read_to_string(k8s_destination.as_path())?;
    k8s_contents.push_str(
        format!(
            "\n---\napiVersion: networking.k8s.io/v1\nkind: NetworkPolicy\nmetadata:\n  name: {kebab}-access\n  namespace: rust-workspace-template\nspec:\n  podSelector:\n    matchLabels:\n      app.kubernetes.io/name: {kebab}\n  ingress:\n    - from:\n        - podSelector:\n            matchLabels:\n              app.kubernetes.io/name: application\n      ports:\n        - protocol: TCP\n          port: {port}\n  egress:\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: database\n          podSelector:\n            matchLabels:\n              app.kubernetes.io/name: {kebab}-postgresql\n      ports:\n        - protocol: TCP\n          port: 5432\n    - to:\n        - namespaceSelector:\n            matchLabels:\n              kubernetes.io/metadata.name: kube-system\n          podSelector:\n            matchLabels:\n              k8s-app: kube-dns\n      ports:\n        - protocol: UDP\n          port: 53\n        - protocol: TCP\n          port: 53\n  policyTypes: [\"Ingress\", \"Egress\"]\n",
            port = port.0,
        )
        .as_str(),
    );
    std::fs::write(k8s_destination.as_path(), k8s_contents)?;
    let kustomization = root.join("deploy/k8s/base/kustomization.yaml");
    insert_once(
        kustomization.as_path(),
        "  - notification-service.yaml",
        format!("  - notification-service.yaml\n  - {k8s_file_name}").as_str(),
    )?;

    let compose = format!(
        "services:\n  {service}_database:\n    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n    environment:\n      POSTGRES_DB: {service}\n      POSTGRES_USER: {service}\n      POSTGRES_PASSWORD: ${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}\n    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U {service} -d {service}\"]\n      interval: 5s\n      timeout: 3s\n      retries: 20\n    networks: [application]\n    volumes: [{service}_database_data:/var/lib/postgresql/data]\n  {service}:\n    build:\n      context: .\n      dockerfile: {service}/Dockerfile\n    depends_on:\n      {service}_database:\n        condition: service_healthy\n    environment:\n      {upper_snake}_DATABASE_URL: postgres://{service}:${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}@{service}_database:5432/{service}\n      {upper_snake}_SERVICE_SOCKET_ADDRESS: 0.0.0.0:{port}\n      MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES: \"8192\"\n      PG_POOL_MAX_CONNECTIONS: \"10\"\n      REQUEST_TIMEOUT_SECONDS: \"30\"\n      TRACING_FORMAT: text\n    networks: [application]\n    ports: [\"127.0.0.1:{port}:{port}\"]\n    read_only: true\n    restart: unless-stopped\n    tmpfs: [/tmp:size=16m,mode=1777]\nvolumes:\n  {service}_database_data:\n",
        port = port.0,
    );
    std::fs::write(root.join(format!("docker-compose.{service}.yml")), compose)?;

    let constants = root.join("str_constants/src/lib.rs");
    let sql_constant = format!(
        "\npub const {upper_snake}_INSERT_SQL: &str = \"INSERT INTO {service}s (id, message) VALUES ($1, $2)\";\n"
    );
    let mut constants_contents = std::fs::read_to_string(constants.as_path())?;
    constants_contents.push_str(sql_constant.as_str());
    std::fs::write(constants, constants_contents)?;
    Ok(())
}

fn workspace_root() -> Result<&'static std::path::Path, ScaffoldError> {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(ScaffoldError::Arguments)
}

#[allow(
    clippy::single_call_fn,
    reason = "binary entry point delegates fallible argument handling"
)]
fn run() -> Result<(), ScaffoldError> {
    let mut arguments = std::env::args().skip(1usize);
    match arguments.next().as_deref() {
        Some("project") => {
            let name = arguments.next().ok_or(ScaffoldError::Arguments)?;
            let repository_url = arguments.next().ok_or(ScaffoldError::Arguments)?;
            if arguments.next().is_some() {
                return Err(ScaffoldError::Arguments);
            }
            let name_ref = ProjectNameRef(name.as_str());
            let repository_url_ref = RepositoryUrlRef(repository_url.as_str());
            validate_project_name(name_ref)?;
            validate_repository_url(repository_url_ref)?;
            rename_identity(workspace_root()?, name_ref, repository_url_ref)
        }
        Some("service") => {
            let name = arguments.next().ok_or(ScaffoldError::Arguments)?;
            let port = arguments
                .next()
                .ok_or(ScaffoldError::Arguments)?
                .parse::<u16>()
                .map(ServicePort)
                .map_err(|_error| ScaffoldError::ServicePort)?;
            if arguments.next().is_some() {
                return Err(ScaffoldError::Arguments);
            }
            scaffold_service(workspace_root()?, ProjectNameRef(name.as_str()), port)
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
    fn write(path: &std::path::Path, value: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("2f0ad03a");
        }
        std::fs::write(path, value).expect("79af6dc8");
    }

    #[test]
    fn validates_and_converts_project_names() {
        let valid = super::ProjectNameRef("order_platform");
        super::validate_project_name(valid).expect("96de3a80");
        assert_eq!(super::kebab_case(valid), "order-platform");
        assert_eq!(super::title_case(valid), "Order Platform");
        assert_eq!(super::upper_camel_case(valid), "OrderPlatform");
        assert!(super::validate_project_name(super::ProjectNameRef("Order-Platform")).is_err());
    }

    #[test]
    fn requires_https_repository_url() {
        super::validate_repository_url(super::RepositoryUrlRef(
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
    fn service_scaffold_registers_all_artifacts() {
        let root =
            std::env::temp_dir().join(format!("workspace-scaffold-test-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(root.as_path()).expect("1449608d");
        }
        write(
            root.join("Cargo.toml").as_path(),
            "[workspace]\nmembers = [\n  \"notification_service_contract\",\n]\n[workspace.dependencies]\nnotification_service_contract = { path = \"./notification_service_contract\", version = \"0.1.0\" }\n",
        );
        write(
            root.join("notification_service/src/main.rs").as_path(),
            "struct Notification; const PORT: u16 = 8081; const SQL: &str = str_constants::NOTIFICATION_INSERT_SQL;",
        );
        write(
            root.join("notification_service_config/src/lib.rs")
                .as_path(),
            "struct NotificationConfig;",
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
        write(root.join("str_constants/src/lib.rs").as_path(), "");
        super::scaffold_service(
            root.as_path(),
            super::ProjectNameRef("order_service"),
            super::ServicePort(8082u16),
        )
        .expect("4bff1d79");
        assert!(root.join("order_service/src/main.rs").is_file());
        assert!(root.join("docker-compose.order_service.yml").is_file());
        assert!(
            std::fs::read_to_string(root.join("Cargo.toml"))
                .expect("371dbe92")
                .contains("order_service_contract")
        );
        assert!(
            std::fs::read_to_string(root.join("deploy/k8s/base/order-service.yaml"))
                .expect("239c17b0")
                .contains("port: 8082")
        );
        std::fs::remove_dir_all(root).expect("6f608418");
    }
}
