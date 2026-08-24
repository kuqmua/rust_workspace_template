#![allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]

#[test]
fn service_deployment_probes_use_registered_health_routes() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(
            "416090ad service_deployment_probes_use_registered_health_routes invariant must hold",
        );
    let catalog_source = std::fs::read_to_string(repository_root.join("deploy/services.toml"))
        .expect(
            "0c6173a3 service_deployment_probes_use_registered_health_routes invariant must hold",
        );
    let catalog = catalog_source.parse::<toml::Table>().expect(
        "7ce7751f service_deployment_probes_use_registered_health_routes invariant must hold",
    );
    let services = catalog
        .get("service")
        .and_then(toml::Value::as_array)
        .expect(
            "4d41c98f service_deployment_probes_use_registered_health_routes invariant must hold",
        );
    let live_path = common_routes::CommonRoute::HealthLive.path();
    let ready_path = common_routes::CommonRoute::HealthReady.path();
    services.iter().for_each(|service_value| {
        let table = service_value.as_table().expect("c04fc517 service_deployment_probes_use_registered_health_routes invariant must hold");
        let get_text = |field| {
            table
                .get(field)
                .and_then(toml::Value::as_str)
                .expect("9971e2bf service_deployment_probes_use_registered_health_routes invariant must hold")
        };
        let service_name = get_text("compose");
        let compose_source =
            std::fs::read_to_string(repository_root.join(get_text("compose_file")))
                .expect("1928801b service_deployment_probes_use_registered_health_routes invariant must hold");
        let service_marker = format!("  {service_name}:\n");
        let compose_service = compose_source
            .split_once(service_marker.as_str())
            .map(|(_before, source)| source)
            .expect("def93cb8 service_deployment_probes_use_registered_health_routes invariant must hold");
        assert!(compose_service.contains(ready_path.as_ref()), "4933eff6");
        let deployment_source =
            std::fs::read_to_string(repository_root.join(get_text("kubernetes")))
                .expect("631594d9 service_deployment_probes_use_registered_health_routes invariant must hold");
        assert_eq!(
            deployment_source
                .lines()
                .filter(|line| line.trim() == format!("path: {}", ready_path.as_ref()))
                .count(),
            2usize,
            "886062ce"
        );
        assert_eq!(
            deployment_source
                .lines()
                .filter(|line| line.trim() == format!("path: {}", live_path.as_ref()))
                .count(),
            1usize,
            "4173ba47"
        );
    });
}

#[test]
fn service_catalog_matches_build_and_deployment_representations() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("518d973f service_catalog_matches_build_and_deployment_representations invariant must hold");
    let catalog_source =
        std::fs::read_to_string(repository_root.join("deploy/services.toml")).expect("2683c1a0 service_catalog_matches_build_and_deployment_representations invariant must hold");
    let catalog = catalog_source.parse::<toml::Table>().expect(
        "8f1bea25 service_catalog_matches_build_and_deployment_representations invariant must hold",
    );
    let services = catalog
        .get("service")
        .and_then(toml::Value::as_array)
        .expect("c6269736 service_catalog_matches_build_and_deployment_representations invariant must hold");
    let ci = std::fs::read_to_string(repository_root.join(".github/workflows/ci.yml")).expect(
        "f21736f4 service_catalog_matches_build_and_deployment_representations invariant must hold",
    );
    let release = std::fs::read_to_string(repository_root.join(".github/workflows/release.yml"))
        .expect("a2bfc899 service_catalog_matches_build_and_deployment_representations invariant must hold");
    services.iter().for_each(|service| {
        let table = service.as_table().expect("24c7af1a service_catalog_matches_build_and_deployment_representations invariant must hold");
        let get_text = |field| {
            table
                .get(field)
                .and_then(toml::Value::as_str)
                .expect("704fa6dd service_catalog_matches_build_and_deployment_representations invariant must hold")
        };
        let crate_name = get_text("crate");
        let compose_name = get_text("compose");
        let compose_file = get_text("compose_file");
        let dockerfile = get_text("dockerfile");
        let image = get_text("image");
        let kubernetes = get_text("kubernetes");
        let is_released = table
            .get("release")
            .and_then(toml::Value::as_bool)
            .expect("e69fbcf1 service_catalog_matches_build_and_deployment_representations invariant must hold");
        let port = table
            .get("port")
            .and_then(toml::Value::as_integer)
            .expect("8cc73f18 service_catalog_matches_build_and_deployment_representations invariant must hold");
        let compose =
            std::fs::read_to_string(repository_root.join(compose_file)).expect("37124d48 service_catalog_matches_build_and_deployment_representations invariant must hold");
        assert!(
            repository_root
                .join(crate_name)
                .join("Cargo.toml")
                .is_file()
        );
        assert!(repository_root.join(dockerfile).is_file());
        assert!(compose.contains(format!("  {compose_name}:\n").as_str()));
        assert!(compose.contains(format!("dockerfile: {dockerfile}").as_str()));
        assert!(compose.contains(format!("127.0.0.1:{port}:{port}").as_str()));
        let deployment =
            std::fs::read_to_string(repository_root.join(kubernetes)).expect("f20be8a5 service_catalog_matches_build_and_deployment_representations invariant must hold");
        assert!(deployment.contains(format!("image: {image}:").as_str()));
        assert!(deployment.contains(format!("containerPort: {port}").as_str()));
        assert!(deployment.contains(format!("port: {port}").as_str()));
        if is_released {
            assert!(release.contains(format!("- name: {image}").as_str()));
            assert!(release.contains(format!("dockerfile: {dockerfile}").as_str()));
            assert!(ci.contains(format!("- name: {image}").as_str()));
            assert!(ci.contains(format!("dockerfile: {dockerfile}").as_str()));
        }
    });
}

#[test]
fn continuous_integration_uses_the_pinned_application_database_image() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("36869b03 continuous_integration_uses_the_pinned_application_database_image invariant must hold");
    let compose =
        std::fs::read_to_string(repository_root.join("docker-compose.yml")).expect("b9e6dd80 continuous_integration_uses_the_pinned_application_database_image invariant must hold");
    let database_image = compose
        .split_once("  database:\n")
        .and_then(|(_prefix, database)| {
            database
                .lines()
                .find(|line| line.trim().starts_with("image:"))
        })
        .map(str::trim)
        .expect("033beb54 continuous_integration_uses_the_pinned_application_database_image invariant must hold");
    let ci = std::fs::read_to_string(repository_root.join(".github/workflows/ci.yml"))
        .expect("346c695a continuous_integration_uses_the_pinned_application_database_image invariant must hold");
    assert!(ci.lines().any(|line| line.trim() == database_image));
    assert!(!ci.contains("postgresql_16_with_pg_jsonschema:latest"));
}

#[test]
fn service_catalog_covers_every_build_and_runtime_projection() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("2f1a4b8c service_catalog_covers_every_build_and_runtime_projection invariant must hold");
    let catalog = std::fs::read_to_string(repository_root.join("deploy/services.toml"))
        .expect("7c3d9e10 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
        .parse::<toml::Table>()
        .expect("4a8f2c61 service_catalog_covers_every_build_and_runtime_projection invariant must hold");
    let services = catalog
        .get("service")
        .and_then(toml::Value::as_array)
        .expect("9b6e0d42 service_catalog_covers_every_build_and_runtime_projection invariant must hold");
    let field_values = |field: &str| {
        services
            .iter()
            .map(|service| {
                service
                    .as_table()
                    .and_then(|table| table.get(field))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
                    .expect("6e1c5a93 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
            })
            .collect::<std::collections::BTreeSet<_>>()
    };
    let catalog_compose = field_values("compose");
    let catalog_dockerfiles = field_values("dockerfile");
    let catalog_kubernetes = field_values("kubernetes");
    let released_images = services
        .iter()
        .filter(|service| {
            service
                .as_table()
                .expect("5b0e7c14 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
                .get("release")
                .and_then(toml::Value::as_bool)
                .expect("8d4a1f63 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
        })
        .map(|service| {
            service
                .as_table()
                .and_then(|table| table.get("image"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
                .expect("3f9c6a20 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
        })
        .collect::<std::collections::BTreeSet<_>>();

    let compose = std::fs::read_to_string(repository_root.join("docker-compose.yml")).expect(
        "1d7a3f85 service_catalog_covers_every_build_and_runtime_projection invariant must hold",
    );
    let mut current_service = None;
    let mut compose_build_services = std::collections::BTreeSet::new();
    compose.lines().for_each(|line| {
        if line.starts_with("  ") && !line.starts_with("    ") && line.ends_with(':') {
            current_service = Some(line.trim().trim_end_matches(':').to_owned());
        }
        if line.trim() == "build:"
            && let Some(service) = current_service.as_ref()
        {
            let _inserted = compose_build_services.insert(service.clone());
        }
    });
    assert_eq!(compose_build_services, catalog_compose);

    let dockerfiles = walkdir::WalkDir::new(repository_root)
        .into_iter()
        .filter_entry(|entry| {
            !entry
                .path()
                .components()
                .any(|component| component.as_os_str() == "target")
        })
        .map(|entry| entry.expect("7a2d5c91 service_catalog_covers_every_build_and_runtime_projection invariant must hold"))
        .filter(|entry| {
            !entry.file_type().is_dir()
                && entry
                    .file_name()
                    .to_string_lossy()
                    .eq_ignore_ascii_case("Dockerfile")
        })
        .map(|entry| {
            entry
                .path()
                .strip_prefix(repository_root)
                .expect("5f9b2d74 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
                .to_string_lossy()
                .into_owned()
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(dockerfiles, catalog_dockerfiles);

    let kubernetes_deployments = walkdir::WalkDir::new(repository_root.join("deploy/k8s"))
        .into_iter()
        .map(|entry| entry.expect("1c8f4b60 service_catalog_covers_every_build_and_runtime_projection invariant must hold"))
        .filter(|entry| !entry.file_type().is_dir())
        .filter_map(|entry| {
            let source = std::fs::read_to_string(entry.path()).expect("9e3a6d27 service_catalog_covers_every_build_and_runtime_projection invariant must hold");
            source
                .lines()
                .any(|line| line.trim() == "kind: Deployment")
                .then(|| {
                    entry
                        .path()
                        .strip_prefix(repository_root)
                        .expect("8a0c4e16 service_catalog_covers_every_build_and_runtime_projection invariant must hold")
                        .to_string_lossy()
                        .into_owned()
                })
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(kubernetes_deployments, catalog_kubernetes);

    let release = std::fs::read_to_string(repository_root.join(".github/workflows/release.yml"))
        .expect("3e7b1a59 service_catalog_covers_every_build_and_runtime_projection invariant must hold");
    let release_matrix = release
        .split_once("      matrix:\n")
        .and_then(|(_prefix, matrix_and_steps)| matrix_and_steps.split_once("    steps:\n"))
        .map(|(matrix, _steps)| matrix)
        .expect("4c8e2a70 service_catalog_covers_every_build_and_runtime_projection invariant must hold");
    let release_images = release_matrix
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- name: ").map(str::to_owned))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(release_images, released_images);

    let ci = std::fs::read_to_string(repository_root.join(".github/workflows/ci.yml")).expect(
        "0d6f2c83 service_catalog_covers_every_build_and_runtime_projection invariant must hold",
    );
    let ci_matrix = ci
        .split_once("# BEGIN GENERATED SERVICE MATRIX\n")
        .and_then(|(_prefix, matrix)| matrix.split_once("# END GENERATED SERVICE MATRIX\n"))
        .map(|(matrix, _suffix)| matrix)
        .expect("d5ef2be8 service_catalog_covers_every_build_and_runtime_projection invariant must hold");
    let ci_images = ci_matrix
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- name: ").map(str::to_owned))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ci_images, released_images);
}

fn unpinned_dockerfile_base_images(
    source: super::types::SourceTextRef<'_>,
) -> super::types::SourceTextList {
    let from_parts = |line: &str| {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        if !words
            .first()
            .is_some_and(|directive| directive.eq_ignore_ascii_case("FROM"))
        {
            return None;
        }
        let image_index = words
            .iter()
            .enumerate()
            .skip(1usize)
            .find(|(_, word)| !word.starts_with("--"))
            .map(|(index, _)| index)?;
        let image = words.get(image_index)?.to_string();
        let stage = words
            .get(image_index.saturating_add(1usize))
            .filter(|keyword| keyword.eq_ignore_ascii_case("AS"))
            .and_then(|_| words.get(image_index.saturating_add(2usize)))
            .map(|stage| (*stage).to_ascii_lowercase());
        Some((image, stage))
    };
    let stage_names = source
        .as_ref()
        .lines()
        .filter_map(from_parts)
        .filter_map(|(_image, stage)| stage)
        .collect::<std::collections::BTreeSet<_>>();
    source
        .as_ref()
        .lines()
        .filter_map(from_parts)
        .map(|(image, _stage)| image)
        .filter(|image| {
            let is_stage = stage_names.contains(&image.to_ascii_lowercase());
            let is_scratch = image.eq_ignore_ascii_case("scratch");
            let has_valid_digest = image.rsplit_once("@sha256:").is_some_and(|(name, digest)| {
                !name.is_empty()
                    && digest.len() == 64usize
                    && digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            });
            !is_stage && !is_scratch && !has_valid_digest
        })
        .collect::<Vec<String>>()
        .into()
}

#[test]
fn catalog_dockerfiles_pin_every_external_base_image_by_digest() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("d31c857a catalog_dockerfiles_pin_every_external_base_image_by_digest invariant must hold");
    let catalog = std::fs::read_to_string(repository_root.join("deploy/services.toml"))
        .expect("a7641e3b catalog_dockerfiles_pin_every_external_base_image_by_digest invariant must hold")
        .parse::<toml::Table>()
        .expect("2b0c96d4 catalog_dockerfiles_pin_every_external_base_image_by_digest invariant must hold");
    let services = catalog
        .get("service")
        .and_then(toml::Value::as_array)
        .expect("74f02a1c catalog_dockerfiles_pin_every_external_base_image_by_digest invariant must hold");
    let mut ers = Vec::new();
    services.iter().for_each(|service| {
        let dockerfile = service
            .as_table()
            .and_then(|table| table.get("dockerfile"))
            .and_then(toml::Value::as_str)
            .expect("c1854d7f catalog_dockerfiles_pin_every_external_base_image_by_digest invariant must hold");
        let source = std::fs::read_to_string(repository_root.join(dockerfile)).expect("3fa21b68 catalog_dockerfiles_pin_every_external_base_image_by_digest invariant must hold");
        unpinned_dockerfile_base_images(super::types::SourceTextRef::from(source.as_str()))
            .into_iter()
            .for_each(|image| ers.push(format!("{dockerfile}: unpinned base image `{image}`")));
    });
    assert!(ers.is_empty(), "e40a7c16 {ers:#?}");
}

#[test]
fn dockerfile_base_image_policy_rejects_latest_and_allows_named_stages() {
    let violations = unpinned_dockerfile_base_images(super::types::SourceTextRef::from(
        "from --platform=$BUILDPLATFORM rust:latest as builder\nFROM builder AS packaged\nFROM alpine:3.22\nFROM busybox@sha256:abcd\nFROM scratch\n",
    ));
    assert_eq!(
        violations.as_slice(),
        [
            String::from("rust:latest"),
            String::from("alpine:3.22"),
            String::from("busybox@sha256:abcd"),
        ]
    );
    assert!(
        unpinned_dockerfile_base_images(super::types::SourceTextRef::from(
            "FROM rust:1.90@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef AS builder\nFROM BUILDER\n"
        ))
        .is_empty()
    );
}
