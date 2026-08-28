use super::{
    ProjectNameRef, ReplacementsRef, ScaffoldError, ScaffoldPathRef, ScaffoldTextRef, ServicePort,
    naming_kebab_case, naming_upper_camel_case, naming_validate_project_name,
};

#[allow(
    clippy::single_call_fn,
    reason = "service command owns complete scaffold composition"
)]
pub(crate) fn scaffold_service(
    root: ScaffoldPathRef<'_>,
    service_name: ProjectNameRef<'_>,
    port: ServicePort,
) -> Result<(), ScaffoldError> {
    naming_validate_project_name::naming_validate_project_name(service_name)?;
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
    let kebab = naming_kebab_case::naming_kebab_case(service_name);
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
            naming_upper_camel_case::naming_upper_camel_case(service_name)
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
    crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
        ScaffoldPathRef::from(
            root.0
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE)
                .as_path(),
        ),
        ScaffoldPathRef::from(root.0.join(service).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
        ScaffoldPathRef::from(
            root.0
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                .as_path(),
        ),
        ScaffoldPathRef::from(root.0.join(config.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    crate::template_fs_copy_template_tree::template_fs_copy_template_tree(
        ScaffoldPathRef::from(
            root.0
                .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONTRACT)
                .as_path(),
        ),
        ScaffoldPathRef::from(root.0.join(contract.as_str()).as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;

    let manifest = root.0.join(constants_str::CARGO_TOML);
    crate::template_fs_insert_once::template_fs_insert_once(
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
    crate::template_fs_insert_once::template_fs_insert_once(
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
    crate::template_fs_replace_file::template_fs_replace_file(
        ScaffoldPathRef::from(k8s_destination.as_path()),
        ReplacementsRef::from(replacements.as_slice()),
    )?;
    let mut k8s_contents = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
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
    crate::template_fs_write_text::template_fs_write_text(
        ScaffoldPathRef::from(k8s_destination.as_path()),
        ScaffoldTextRef::from(k8s_contents.as_str()),
    )?;
    let kustomization = root
        .0
        .join(constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_PATH);
    crate::template_fs_insert_once::template_fs_insert_once(
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
    let config_example = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
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
    crate::template_fs_write_text::template_fs_write_text(
        ScaffoldPathRef::from(compose_path.as_path()),
        ScaffoldTextRef::from(compose.as_str()),
    )?;

    let service_catalog = root
        .0
        .join(constants_str::WORKSPACE_SCAFFOLD_SERVICE_CATALOG_PATH);
    let mut service_catalog_contents =
        crate::template_fs_read_bounded_text::template_fs_read_bounded_text(
            ScaffoldPathRef::from(service_catalog.as_path()),
        )?
        .as_ref()
        .to_owned();
    service_catalog_contents.push_str(
        format!(
            "\n[[service]]\ncrate = \"{service}\"\ncompose = \"{service}\"\ncompose_file = \"docker-compose.{service}.yml\"\ndockerfile = \"{service}/Dockerfile\"\nimage = \"{kebab}\"\nkubernetes = \"deploy/k8s/base/{k8s_file_name}\"\nport = {}\nrelease = false\nsocket_env = \"{upper_snake}_SERVICE_SOCKET_ADDRESS\"\n",
            port.0
        )
        .as_str(),
    );
    crate::template_fs_write_text::template_fs_write_text(
        ScaffoldPathRef::from(service_catalog.as_path()),
        ScaffoldTextRef::from(service_catalog_contents.as_str()),
    )?;
    Ok(())
}
