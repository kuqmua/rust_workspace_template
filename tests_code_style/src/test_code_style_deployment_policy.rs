#![allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]

#[test]
fn test_service_deployment_probes_use_registered_health_routes() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_416090AD);
    let catalog_source =
        std::fs::read_to_string(repository_root.join(constants_str::VALUE_C1590960))
            .expect(constants_str::DIAGNOSTIC_0C6173A3);
    let catalog = catalog_source
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_7CE7751F);
    let services = catalog
        .get(constants_str::SERVICE)
        .and_then(toml::Value::as_array)
        .expect(constants_str::DIAGNOSTIC_4D41C98F);
    let live_path = common_routes::common_route::CommonRoute::HealthLive.path();
    let ready_path = common_routes::common_route::CommonRoute::HealthReady.path();
    services.iter().for_each(|service_value| {
        let table = service_value
            .as_table()
            .expect(constants_str::DIAGNOSTIC_C04FC517);
        let get_text = |field| {
            table
                .get(field)
                .and_then(toml::Value::as_str)
                .expect(constants_str::DIAGNOSTIC_9971E2BF)
        };
        let service_name = get_text(constants_str::VALUE_DB669AF6);
        let compose_source =
            std::fs::read_to_string(repository_root.join(get_text(constants_str::VALUE_739ED940)))
                .expect(constants_str::DIAGNOSTIC_1928801B);
        let service_marker = format!("  {service_name}:\n");
        let compose_service = compose_source
            .split_once(service_marker.as_str())
            .map(|(_before, source)| source)
            .expect(constants_str::DIAGNOSTIC_DEF93CB8);
        assert!(compose_service.contains(ready_path.as_ref()), "4933eff6");
        let deployment_source =
            std::fs::read_to_string(repository_root.join(get_text(constants_str::VALUE_94ABCB2D)))
                .expect(constants_str::DIAGNOSTIC_631594D9);
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
            constants_usize::ONE,
            "4173ba47"
        );
    });
}

#[test]
fn test_service_catalog_matches_build_and_deployment_representations() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_518D973F);
    let catalog_source =
        std::fs::read_to_string(repository_root.join(constants_str::VALUE_C1590960))
            .expect(constants_str::DIAGNOSTIC_2683C1A0);
    let catalog = catalog_source
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_8F1BEA25);
    let services = catalog
        .get(constants_str::SERVICE)
        .and_then(toml::Value::as_array)
        .expect(constants_str::DIAGNOSTIC_C6269736);
    let ci =
        std::fs::read_to_string(repository_root.join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH))
            .expect(constants_str::DIAGNOSTIC_F21736F4);
    let release = std::fs::read_to_string(repository_root.join(constants_str::VALUE_87DB21A9))
        .expect(constants_str::DIAGNOSTIC_A2BFC899);
    services.iter().for_each(|service| {
        let table = service
            .as_table()
            .expect(constants_str::DIAGNOSTIC_24C7AF1A);
        let get_text = |field| {
            table
                .get(field)
                .and_then(toml::Value::as_str)
                .expect(constants_str::DIAGNOSTIC_704FA6DD)
        };
        let crate_name = get_text(constants_str::CRATE);
        let compose_name = get_text(constants_str::VALUE_DB669AF6);
        let compose_file = get_text(constants_str::VALUE_739ED940);
        let dockerfile = get_text(constants_str::VALUE_254DB0FB);
        let image = get_text(constants_str::VALUE_6105D6CC);
        let kubernetes = get_text(constants_str::VALUE_94ABCB2D);
        let is_released = table
            .get(constants_str::RELEASE)
            .and_then(toml::Value::as_bool)
            .expect(constants_str::DIAGNOSTIC_E69FBCF1);
        let port = table
            .get(constants_str::VALUE_F8D397A3)
            .and_then(toml::Value::as_integer)
            .expect(constants_str::DIAGNOSTIC_8CC73F18);
        let compose = std::fs::read_to_string(repository_root.join(compose_file))
            .expect(constants_str::DIAGNOSTIC_37124D48);
        assert!(
            repository_root
                .join(crate_name)
                .join(constants_str::CARGO_TOML)
                .is_file()
        );
        assert!(repository_root.join(dockerfile).is_file());
        assert!(compose.contains(format!("  {compose_name}:\n").as_str()));
        assert!(compose.contains(format!("dockerfile: {dockerfile}").as_str()));
        assert!(compose.contains(format!("127.0.0.1:{port}:{port}").as_str()));
        let deployment = std::fs::read_to_string(repository_root.join(kubernetes))
            .expect(constants_str::DIAGNOSTIC_F20BE8A5);
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
fn test_continuous_integration_uses_the_pinned_application_database_image() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_36869B03);
    let compose = std::fs::read_to_string(repository_root.join(constants_str::VALUE_E45E45BA))
        .expect(constants_str::DIAGNOSTIC_B9E6DD80);
    let database_image = compose
        .split_once(constants_str::VALUE_BCE0FE4A)
        .and_then(|(_prefix, database)| {
            database
                .lines()
                .find(|line| line.trim().starts_with(constants_str::VALUE_A08A3033))
        })
        .map(str::trim)
        .expect(constants_str::DIAGNOSTIC_033BEB54);
    let ci =
        std::fs::read_to_string(repository_root.join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH))
            .expect(constants_str::DIAGNOSTIC_346C695A);
    assert!(ci.lines().any(|line| line.trim() == database_image));
    assert!(!ci.contains(constants_str::VALUE_ADE0980F));
}

#[test]
fn test_service_catalog_covers_every_build_and_runtime_projection() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_2F1A4B8C);
    let catalog = std::fs::read_to_string(repository_root.join(constants_str::VALUE_C1590960))
        .expect(constants_str::DIAGNOSTIC_7C3D9E10)
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_4A8F2C61);
    let services = catalog
        .get(constants_str::SERVICE)
        .and_then(toml::Value::as_array)
        .expect(constants_str::DIAGNOSTIC_9B6E0D42);
    let field_values = |field: &str| {
        services
            .iter()
            .map(|service| {
                service
                    .as_table()
                    .and_then(|table| table.get(field))
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned)
                    .expect(constants_str::DIAGNOSTIC_6E1C5A93)
            })
            .collect::<std::collections::BTreeSet<_>>()
    };
    let catalog_compose = field_values(constants_str::VALUE_DB669AF6);
    let catalog_dockerfiles = field_values(constants_str::VALUE_254DB0FB);
    let catalog_kubernetes = field_values(constants_str::VALUE_94ABCB2D);
    let released_images = services
        .iter()
        .filter(|service| {
            service
                .as_table()
                .expect(constants_str::DIAGNOSTIC_5B0E7C14)
                .get(constants_str::RELEASE)
                .and_then(toml::Value::as_bool)
                .expect(constants_str::DIAGNOSTIC_8D4A1F63)
        })
        .map(|service| {
            service
                .as_table()
                .and_then(|table| table.get(constants_str::VALUE_6105D6CC))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
                .expect(constants_str::DIAGNOSTIC_3F9C6A20)
        })
        .collect::<std::collections::BTreeSet<_>>();

    let compose = std::fs::read_to_string(repository_root.join(constants_str::VALUE_E45E45BA))
        .expect(constants_str::DIAGNOSTIC_1D7A3F85);
    let mut current_service = None;
    let mut compose_build_services = std::collections::BTreeSet::new();
    compose.lines().for_each(|line| {
        if line.starts_with(constants_str::TWO_SPACES)
            && !line.starts_with(constants_str::FOUR_SPACES)
            && line.ends_with(':')
        {
            current_service = Some(line.trim().trim_end_matches(':').to_owned());
        }
        if line.trim() == constants_str::VALUE_3CF4DC5D
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
                .any(|component| component.as_os_str() == constants_str::TARGET)
        })
        .map(|entry| entry.expect(constants_str::DIAGNOSTIC_7A2D5C91))
        .filter(|entry| {
            !entry.file_type().is_dir()
                && entry
                    .file_name()
                    .to_string_lossy()
                    .eq_ignore_ascii_case(constants_str::VALUE_DD2C0EB6)
        })
        .map(|entry| {
            entry
                .path()
                .strip_prefix(repository_root)
                .expect(constants_str::DIAGNOSTIC_5F9B2D74)
                .to_string_lossy()
                .into_owned()
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(dockerfiles, catalog_dockerfiles);

    let kubernetes_deployments =
        walkdir::WalkDir::new(repository_root.join(constants_str::VALUE_BC15D323))
            .into_iter()
            .map(|entry| entry.expect(constants_str::DIAGNOSTIC_1C8F4B60))
            .filter(|entry| !entry.file_type().is_dir())
            .filter_map(|entry| {
                let source = std::fs::read_to_string(entry.path())
                    .expect(constants_str::DIAGNOSTIC_9E3A6D27);
                source
                    .lines()
                    .any(|line| line.trim() == constants_str::VALUE_AB78925C)
                    .then(|| {
                        entry
                            .path()
                            .strip_prefix(repository_root)
                            .expect(constants_str::DIAGNOSTIC_8A0C4E16)
                            .to_string_lossy()
                            .into_owned()
                    })
            })
            .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(kubernetes_deployments, catalog_kubernetes);

    let release = std::fs::read_to_string(repository_root.join(constants_str::VALUE_87DB21A9))
        .expect(constants_str::DIAGNOSTIC_3E7B1A59);
    let release_matrix = release
        .split_once(constants_str::VALUE_5E783C26)
        .and_then(|(_prefix, matrix_and_steps)| {
            matrix_and_steps.split_once(constants_str::VALUE_C8999110)
        })
        .map(|(matrix, _steps)| matrix)
        .expect(constants_str::DIAGNOSTIC_4C8E2A70);
    let release_images = release_matrix
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix(constants_str::VALUE_0ACA6317)
                .map(str::to_owned)
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(release_images, released_images);

    let ci =
        std::fs::read_to_string(repository_root.join(constants_str::CODE_STYLE_CI_WORKFLOW_PATH))
            .expect(constants_str::DIAGNOSTIC_0D6F2C83);
    let ci_matrix = ci
        .split_once(constants_str::VALUE_1EAFB99B)
        .and_then(|(_prefix, matrix)| matrix.split_once(constants_str::VALUE_849338CC))
        .map(|(matrix, _suffix)| matrix)
        .expect(constants_str::DIAGNOSTIC_D5EF2BE8);
    let ci_images = ci_matrix
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix(constants_str::VALUE_0ACA6317)
                .map(str::to_owned)
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(ci_images, released_images);
}

fn unpinned_dockerfile_base_images(
    source_text_ref: crate::types::SourceTextRef<'_>,
) -> crate::types::SourceTextList {
    let from_parts = |line: &str| {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        if !words
            .first()
            .is_some_and(|directive| directive.eq_ignore_ascii_case(constants_str::VALUE_F4383C66))
        {
            return None;
        }
        let image_index = words
            .iter()
            .enumerate()
            .skip(constants_usize::ONE)
            .find(|(_, word)| !word.starts_with(constants_str::SHARED_VALUES_EMPTY))
            .map(|(index, _)| index)?;
        let image = words.get(image_index)?.to_string();
        let stage = words
            .get(image_index.saturating_add(constants_usize::ONE))
            .filter(|keyword| keyword.eq_ignore_ascii_case(constants_str::VALUE_DE148153))
            .and_then(|_| words.get(image_index.saturating_add(2usize)))
            .map(|stage| (*stage).to_ascii_lowercase());
        Some((image, stage))
    };
    let stage_names = source_text_ref
        .as_ref()
        .lines()
        .filter_map(from_parts)
        .filter_map(|(_image, stage)| stage)
        .collect::<std::collections::BTreeSet<_>>();
    source_text_ref
        .as_ref()
        .lines()
        .filter_map(from_parts)
        .map(|(image, _stage)| image)
        .filter(|image| {
            let is_stage = stage_names.contains(&image.to_ascii_lowercase());
            let is_scratch = image.eq_ignore_ascii_case(constants_str::VALUE_5A9CB6B5);
            let has_valid_digest = image
                .rsplit_once(constants_str::VALUE_6FBFA0EC)
                .is_some_and(|(name, digest)| {
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
fn test_catalog_dockerfiles_pin_every_external_base_image_by_digest() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect(constants_str::DIAGNOSTIC_D31C857A);
    let catalog = std::fs::read_to_string(repository_root.join(constants_str::VALUE_C1590960))
        .expect(constants_str::DIAGNOSTIC_A7641E3B)
        .parse::<toml::Table>()
        .expect(constants_str::DIAGNOSTIC_2B0C96D4);
    let services = catalog
        .get(constants_str::SERVICE)
        .and_then(toml::Value::as_array)
        .expect(constants_str::DIAGNOSTIC_74F02A1C);
    let mut errors = Vec::new();
    services.iter().for_each(|service| {
        let dockerfile = service
            .as_table()
            .and_then(|table| table.get(constants_str::VALUE_254DB0FB))
            .and_then(toml::Value::as_str)
            .expect(constants_str::DIAGNOSTIC_C1854D7F);
        let source = std::fs::read_to_string(repository_root.join(dockerfile))
            .expect(constants_str::DIAGNOSTIC_3FA21B68);
        unpinned_dockerfile_base_images(crate::types::SourceTextRef::from(source.as_str()))
            .into_iter()
            .for_each(|image| errors.push(format!("{dockerfile}: unpinned base image `{image}`")));
    });
    assert!(errors.is_empty(), "e40a7c16 {errors:#?}");
}

#[test]
fn test_dockerfile_base_image_policy_rejects_latest_and_allows_named_stages() {
    let violations = unpinned_dockerfile_base_images(crate::types::SourceTextRef::from(
        constants_str::VALUE_43F5436D,
    ));
    assert_eq!(
        violations.as_slice(),
        [
            String::from(constants_str::VALUE_7670C7B2),
            String::from(constants_str::VALUE_16BB83AC),
            String::from(constants_str::VALUE_9A532517),
        ]
    );
    assert!(
        unpinned_dockerfile_base_images(crate::types::SourceTextRef::from(
            constants_str::VALUE_889A7936
        ))
        .is_empty()
    );
}
