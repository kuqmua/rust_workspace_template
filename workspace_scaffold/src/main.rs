#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "root-owned scaffold stages preserve the former owner-module grouping and shared facade vocabulary"
)]

pub mod cargo_args_ref;
pub mod domain_types;
pub mod generated_projection;
pub mod naming_capitalized_parts;
pub mod naming_kebab_case;
#[cfg(test)]
pub mod naming_title_case;
#[cfg(test)]
pub mod naming_upper_camel_case;
pub mod naming_validate_project_name;
#[cfg(test)]
pub mod naming_validate_repository_url;
pub mod project_name_ref;
pub mod replacements_ref;
pub mod repository_url_ref;
pub mod scaffold_error;
pub mod scaffold_io_error;
pub mod scaffold_path_ref;
pub mod scaffold_run_ok;
pub mod scaffold_text;
pub mod scaffold_text_ref;
pub mod service_catalog_draft;
pub mod service_catalog_entries;
pub mod service_catalog_entries_ref;
pub mod service_catalog_entry;
#[cfg(test)]
pub mod service_catalog_parse;
pub mod service_catalog_render_release_entries;
pub mod service_catalog_string_value;
pub mod service_compose_file;
pub mod service_compose_name;
pub mod service_crate;
pub mod service_dockerfile;
pub mod service_image;
pub mod service_kubernetes_manifest;
pub mod service_port;
pub mod service_socket_env;
pub mod should_release;
#[cfg(test)]
pub mod should_skip;
pub mod should_write;
pub mod synchronize_cargo_owned_projection;
pub mod synchronize_deployment_projections;
pub mod synchronize_generated_file;
pub mod template_fs_copy_template_tree;
pub mod template_fs_insert_once;
pub mod template_fs_read_bounded_text;
pub mod template_fs_replace_file;
#[cfg(test)]
pub mod template_fs_should_skip;
pub mod template_fs_write_text;
#[cfg(test)]
pub mod tests;
pub mod update_env_name;

fn workspace_root()
-> Result<scaffold_path_ref::ScaffoldPathRef<'static>, scaffold_error::ScaffoldError> {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(scaffold_path_ref::ScaffoldPathRef::from)
        .ok_or(scaffold_error::ScaffoldError::Arguments)
}

fn main() {
    let run_ok = {
        let result = (|| {
            let mut arguments = std::env::args().skip(constants_usize::ONE);
            match arguments.next().as_deref() {
                Some(constants_str::WORKSPACE_SCAFFOLD_PROJECT_COMMAND) => {
                    let name = arguments
                        .next()
                        .ok_or(scaffold_error::ScaffoldError::Arguments)?;
                    let repository_url = arguments
                        .next()
                        .ok_or(scaffold_error::ScaffoldError::Arguments)?;
                    if arguments.next().is_some() {
                        return Err(scaffold_error::ScaffoldError::Arguments);
                    }
                    let name_ref = project_name_ref::ProjectNameRef::from(name.as_str());
                    let repository_url_ref =
                        repository_url_ref::RepositoryUrlRef::from(repository_url.as_str());
                    naming_validate_project_name::naming_validate_project_name(name_ref)?;
                    if !repository_url_ref
                        .0
                        .starts_with(constants_str::HTTPS_SCHEME_PREFIX)
                        || repository_url_ref.0.ends_with('/')
                    {
                        return Err(scaffold_error::ScaffoldError::RepositoryUrl);
                    }
                    let root = workspace_root()?;
                    let replacements = [
                        (
                            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_REPOSITORY_URL,
                            repository_url_ref.get().to_owned(),
                        ),
                        (
                            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_SNAKE,
                            name_ref.get().to_owned(),
                        ),
                        (
                            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_KEBAB,
                            naming_kebab_case::naming_kebab_case(name_ref)
                                .as_ref()
                                .to_owned(),
                        ),
                        (
                            constants_str::WORKSPACE_SCAFFOLD_TEMPLATE_PROJECT_TITLE,
                            naming_capitalized_parts::naming_capitalized_parts(
                                name_ref,
                                scaffold_text_ref::ScaffoldTextRef::from(constants_str::SPACE),
                            )
                            .as_ref()
                            .to_owned(),
                        ),
                    ];
                    let mut pending = vec![root.get().to_path_buf()];
                    while let Some(path) = pending.pop() {
                        if path.components().any(|component| {
                            matches!(
                                component.as_os_str().to_str(),
                                Some(
                                    constants_str::GIT
                                        | constants_str::TARGET
                                        | constants_str::WORKSPACE_SCAFFOLD_NODE_MODULES
                                )
                            )
                        }) {
                            continue;
                        }
                        if path.is_dir() {
                            std::fs::read_dir(path)?.try_for_each(|entry| {
                                pending.push(entry?.path());
                                Ok::<(), std::io::Error>(())
                            })?;
                        } else {
                            template_fs_replace_file::template_fs_replace_file(
                                scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
                                replacements_ref::ReplacementsRef::from(replacements.as_slice()),
                            )?;
                        }
                    }
                    Ok(())
                }
                Some(constants_str::SERVICE) => {
                    let name = arguments
                        .next()
                        .ok_or(scaffold_error::ScaffoldError::Arguments)?;
                    let port = match arguments
                        .next()
                        .ok_or(scaffold_error::ScaffoldError::Arguments)?
                        .parse::<u16>()
                    {
                        Ok(value) => service_port::ServicePort::from(value),
                        Err(_error) => return Err(scaffold_error::ScaffoldError::ServicePort),
                    };
                    if arguments.next().is_some() {
                        return Err(scaffold_error::ScaffoldError::Arguments);
                    }
                    {
                        let root = workspace_root()?;
                        let service_name = project_name_ref::ProjectNameRef::from(name.as_str());
                        naming_validate_project_name::naming_validate_project_name(service_name)?;
                        if port.0 == constants_u16::ZERO {
                            return Err(scaffold_error::ScaffoldError::ServicePort);
                        }
                        let service = service_name.0;
                        let config = format!("{service}_config");
                        let contract = format!("{service}_contract");
                        if [service, config.as_str(), contract.as_str()]
                            .iter()
                            .any(|path| root.0.join(path).exists())
                        {
                            return Err(scaffold_error::ScaffoldError::ServiceExists);
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
                                naming_capitalized_parts::naming_capitalized_parts(
                                    service_name,
                                    scaffold_text_ref::ScaffoldTextRef::from(constants_str::EMPTY),
                                )
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
                        template_fs_copy_template_tree::template_fs_copy_template_tree(
                            scaffold_path_ref::ScaffoldPathRef::from(
                                root.0
                                    .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE)
                                    .as_path(),
                            ),
                            scaffold_path_ref::ScaffoldPathRef::from(
                                root.0.join(service).as_path(),
                            ),
                            replacements_ref::ReplacementsRef::from(replacements.as_slice()),
                        )?;
                        template_fs_copy_template_tree::template_fs_copy_template_tree(
                            scaffold_path_ref::ScaffoldPathRef::from(
                                root.0
                                    .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONFIG)
                                    .as_path(),
                            ),
                            scaffold_path_ref::ScaffoldPathRef::from(
                                root.0.join(config.as_str()).as_path(),
                            ),
                            replacements_ref::ReplacementsRef::from(replacements.as_slice()),
                        )?;
                        template_fs_copy_template_tree::template_fs_copy_template_tree(
                            scaffold_path_ref::ScaffoldPathRef::from(
                                root.0
                                    .join(constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_CONTRACT)
                                    .as_path(),
                            ),
                            scaffold_path_ref::ScaffoldPathRef::from(
                                root.0.join(contract.as_str()).as_path(),
                            ),
                            replacements_ref::ReplacementsRef::from(replacements.as_slice()),
                        )?;

                        let manifest = root.0.join(constants_str::CARGO_TOML);
                        template_fs_insert_once::template_fs_insert_once(
                                scaffold_path_ref::ScaffoldPathRef::from(manifest.as_path()),
                                scaffold_text_ref::ScaffoldTextRef::from(constants_str::WORKSPACE_SCAFFOLD_MANIFEST_MEMBER_MARKER),
                                scaffold_text_ref::ScaffoldTextRef::from(
                                    format!(
                                        "  \"notification_service_contract\",\n  \"{service}\",\n  \"{config}\",\n  \"{contract}\","
                                    )
                                    .as_str(),
                                ),
                            )?;
                        let dependency_marker =
                            constants_str::WORKSPACE_SCAFFOLD_MANIFEST_DEPENDENCY_MARKER;
                        template_fs_insert_once::template_fs_insert_once(
                                scaffold_path_ref::ScaffoldPathRef::from(manifest.as_path()),
                                scaffold_text_ref::ScaffoldTextRef::from(dependency_marker),
                                scaffold_text_ref::ScaffoldTextRef::from(
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
                        let _copied_bytes =
                            std::fs::copy(k8s_source.as_path(), k8s_destination.as_path())?;
                        template_fs_replace_file::template_fs_replace_file(
                            scaffold_path_ref::ScaffoldPathRef::from(k8s_destination.as_path()),
                            replacements_ref::ReplacementsRef::from(replacements.as_slice()),
                        )?;
                        let mut k8s_contents =
                            template_fs_read_bounded_text::template_fs_read_bounded_text(
                                scaffold_path_ref::ScaffoldPathRef::from(k8s_destination.as_path()),
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
                        template_fs_write_text::template_fs_write_text(
                            scaffold_path_ref::ScaffoldPathRef::from(k8s_destination.as_path()),
                            scaffold_text_ref::ScaffoldTextRef::from(k8s_contents.as_str()),
                        )?;
                        let kustomization = root
                            .0
                            .join(constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_PATH);
                        template_fs_insert_once::template_fs_insert_once(
                            scaffold_path_ref::ScaffoldPathRef::from(kustomization.as_path()),
                            scaffold_text_ref::ScaffoldTextRef::from(
                                constants_str::WORKSPACE_SCAFFOLD_KUSTOMIZATION_MARKER,
                            ),
                            scaffold_text_ref::ScaffoldTextRef::from(
                                format!("  - notification-service.yaml\n  - {k8s_file_name}")
                                    .as_str(),
                            ),
                        )?;

                        let config_example_path = root
                            .0
                            .join(config.as_str())
                            .join(constants_str::ENV_EXAMPLE);
                        let config_example =
                            template_fs_read_bounded_text::template_fs_read_bounded_text(
                                scaffold_path_ref::ScaffoldPathRef::from(
                                    config_example_path.as_path(),
                                ),
                            )?;
                        let database_key = format!("{upper_snake}_DATABASE_URL");
                        let socket_key = format!("{upper_snake}_SERVICE_SOCKET_ADDRESS");
                        let compose_environment = config_example
                                .as_ref()
                                .lines()
                                .map(|line| {
                                    let (key, example) = line.split_once('=').ok_or(scaffold_error::ScaffoldError::Catalog)?;
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
                                .collect::<Result<String, scaffold_error::ScaffoldError>>()?;
                        let ready_path =
                                <common_routes::health_ready_route::HealthReadyRoute as frontend_contract::typed_route::TypedRoute>::metadata().path();
                        let compose = format!(
                            "services:\n  {service}_database:\n    image: postgres:16-bookworm@sha256:92620daddcd947f8d5ab5ba66e848702fe443d87fed30c4cea8e389fd78dfc55\n    environment:\n      POSTGRES_DB: {service}\n      POSTGRES_USER: {service}\n      POSTGRES_PASSWORD: ${{{upper_snake}_POSTGRES_PASSWORD:?set {upper_snake}_POSTGRES_PASSWORD}}\n    healthcheck:\n      test: [\"CMD-SHELL\", \"pg_isready -U {service} -d {service}\"]\n      interval: 5s\n      timeout: 3s\n      retries: 20\n    networks: [application]\n    volumes: [{service}_database_data:/var/lib/postgresql/data]\n  # BEGIN GENERATED COMPOSE IDENTITY {service}\n  {service}:\n    build:\n      context: .\n      dockerfile: {service}/Dockerfile\n  # END GENERATED COMPOSE IDENTITY {service}\n    depends_on:\n      {service}_database:\n        condition: service_healthy\n    environment:\n{environment}    healthcheck:\n      # BEGIN GENERATED COMPOSE HEALTH {service}\n      test: [\"CMD\", \"curl\", \"--fail\", \"--silent\", \"http://127.0.0.1:{port}{ready_path}\"]\n      # END GENERATED COMPOSE HEALTH {service}\n      interval: 10s\n      timeout: 5s\n      retries: 12\n      start_period: 20s\n    networks: [application]\n    # BEGIN GENERATED COMPOSE PORT {service}\n    ports:\n      - \"127.0.0.1:{port}:{port}\"\n    # END GENERATED COMPOSE PORT {service}\n    read_only: true\n    restart: unless-stopped\n    tmpfs: [/tmp:size=16m,mode=1777]\nvolumes:\n  {service}_database_data:\n",
                            port = port.0,
                            environment = compose_environment,
                            ready_path = ready_path.as_ref(),
                        );
                        let compose_path = root.0.join(format!("docker-compose.{service}.yml"));
                        template_fs_write_text::template_fs_write_text(
                            scaffold_path_ref::ScaffoldPathRef::from(compose_path.as_path()),
                            scaffold_text_ref::ScaffoldTextRef::from(compose.as_str()),
                        )?;

                        let service_catalog = root
                            .0
                            .join(constants_str::WORKSPACE_SCAFFOLD_SERVICE_CATALOG_PATH);
                        let mut service_catalog_contents =
                            template_fs_read_bounded_text::template_fs_read_bounded_text(
                                scaffold_path_ref::ScaffoldPathRef::from(service_catalog.as_path()),
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
                        template_fs_write_text::template_fs_write_text(
                            scaffold_path_ref::ScaffoldPathRef::from(service_catalog.as_path()),
                            scaffold_text_ref::ScaffoldTextRef::from(
                                service_catalog_contents.as_str(),
                            ),
                        )?;
                        Ok(())
                    }
                }
                Some(constants_str::VALUE_24CACF50) => {
                    let write_changes = match arguments.next().as_deref() {
                        Some(constants_str::SYNC) => should_write::ShouldWrite::from(true),
                        Some(constants_str::CHECK) => should_write::ShouldWrite::from(false),
                        Some(_) | None => {
                            return Err(scaffold_error::ScaffoldError::Arguments);
                        }
                    };
                    if arguments.next().is_some() {
                        return Err(scaffold_error::ScaffoldError::Arguments);
                    }
                    let root = workspace_root()?;
                    synchronize_deployment_projections::synchronize_deployment_projections(
                        root,
                        write_changes,
                    )?;
                    synchronize_cargo_owned_projection::synchronize_cargo_owned_projection(
                        root,
                        cargo_args_ref::CargoArgsRef::from(
                            &[
                                constants_str::TEST_ALT_3,
                                constants_str::P,
                                constants_str::VALUE_B2F5A0ED,
                                constants_str::P,
                                constants_str::VALUE_8B9F9090,
                                constants_str::VALUE_B43DA2C2,
                            ][..],
                        ),
                        update_env_name::UpdateEnvName::from(
                            constants_str::UPDATE_CONFIG_PROJECTIONS,
                        ),
                        generated_projection::GeneratedProjection::Config,
                        write_changes,
                    )?;
                    synchronize_cargo_owned_projection::synchronize_cargo_owned_projection(
                        root,
                        cargo_args_ref::CargoArgsRef::from(
                            &[
                                constants_str::TEST_ALT_3,
                                constants_str::P,
                                constants_str::TESTS_ALT,
                                constants_str::CODE_STYLE,
                            ][..],
                        ),
                        update_env_name::UpdateEnvName::from(
                            constants_str::UPDATE_CODE_STYLE_SNAPSHOTS,
                        ),
                        generated_projection::GeneratedProjection::CodeStyle,
                        write_changes,
                    )
                }
                Some(constants_str::VALUE_AEE50B18) => {
                    let write_changes = match arguments.next().as_deref() {
                        Some(constants_str::SYNC) => should_write::ShouldWrite::from(true),
                        Some(constants_str::CHECK) => should_write::ShouldWrite::from(false),
                        Some(_) | None => {
                            return Err(scaffold_error::ScaffoldError::Arguments);
                        }
                    };
                    if arguments.next().is_some() {
                        return Err(scaffold_error::ScaffoldError::Arguments);
                    }
                    synchronize_deployment_projections::synchronize_deployment_projections(
                        workspace_root()?,
                        write_changes,
                    )
                }
                Some(_) | None => Err(scaffold_error::ScaffoldError::Arguments),
            }
        })();
        match result {
            Ok(()) => scaffold_run_ok::ScaffoldRunOk::from(true),
            Err(error) => {
                tracing::error!(
                    error = %error,
                    message = %constants_str::TRACING_WORKSPACE_SCAFFOLDING_FAILED,
                );
                scaffold_run_ok::ScaffoldRunOk::from(false)
            }
        }
    };
    if !run_ok.get() {
        std::process::exit(2i32);
    }
}
