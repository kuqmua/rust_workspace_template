#![allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]

#[test]
fn service_deployment_probes_use_registered_health_routes() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("416090ad");
    let catalog_source =
        std::fs::read_to_string(repository_root.join("deploy/services.toml")).expect("0c6173a3");
    let catalog = catalog_source.parse::<toml::Table>().expect("7ce7751f");
    let services = catalog
        .get("service")
        .and_then(toml::Value::as_array)
        .expect("4d41c98f");
    let live_path = common_routes::CommonRoute::HealthLive.path();
    let ready_path = common_routes::CommonRoute::HealthReady.path();
    services.iter().for_each(|service_value| {
        let table = service_value.as_table().expect("c04fc517");
        let get_text = |field| {
            table
                .get(field)
                .and_then(toml::Value::as_str)
                .expect("9971e2bf")
        };
        let service_name = get_text("compose");
        let compose_source =
            std::fs::read_to_string(repository_root.join(get_text("compose_file")))
                .expect("1928801b");
        let service_marker = format!("  {service_name}:\n");
        let compose_service = compose_source
            .split_once(service_marker.as_str())
            .map(|(_before, source)| source)
            .expect("def93cb8");
        assert!(compose_service.contains(ready_path.as_ref()), "4933eff6");
        let deployment_source =
            std::fs::read_to_string(repository_root.join(get_text("kubernetes")))
                .expect("631594d9");
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
        .expect("518d973f");
    let catalog_source =
        std::fs::read_to_string(repository_root.join("deploy/services.toml")).expect("2683c1a0");
    let catalog = catalog_source.parse::<toml::Table>().expect("8f1bea25");
    let services = catalog
        .get("service")
        .and_then(toml::Value::as_array)
        .expect("c6269736");
    let ci = std::fs::read_to_string(repository_root.join(".github/workflows/ci.yml"))
        .expect("f21736f4");
    let release = std::fs::read_to_string(repository_root.join(".github/workflows/release.yml"))
        .expect("a2bfc899");
    services.iter().for_each(|service| {
        let table = service.as_table().expect("24c7af1a");
        let get_text = |field| {
            table
                .get(field)
                .and_then(toml::Value::as_str)
                .expect("704fa6dd")
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
            .expect("e69fbcf1");
        let port = table
            .get("port")
            .and_then(toml::Value::as_integer)
            .expect("8cc73f18");
        let compose =
            std::fs::read_to_string(repository_root.join(compose_file)).expect("37124d48");
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
            std::fs::read_to_string(repository_root.join(kubernetes)).expect("f20be8a5");
        assert!(deployment.contains(format!("image: {image}:").as_str()));
        assert!(deployment.contains(format!("containerPort: {port}").as_str()));
        assert!(deployment.contains(format!("port: {port}").as_str()));
        if is_released {
            assert!(release.contains(format!("- name: {image}").as_str()));
            assert!(release.contains(format!("dockerfile: {dockerfile}").as_str()));
            assert!(ci.contains(format!("--tag {image}:").as_str()));
        }
    });
}

#[test]
fn continuous_integration_uses_the_pinned_application_database_image() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("36869b03");
    let compose =
        std::fs::read_to_string(repository_root.join("docker-compose.yml")).expect("b9e6dd80");
    let database_image = compose
        .split_once("  database:\n")
        .and_then(|(_prefix, database)| {
            database
                .lines()
                .find(|line| line.trim().starts_with("image:"))
        })
        .map(str::trim)
        .expect("033beb54");
    let ci = std::fs::read_to_string(repository_root.join(".github/workflows/ci.yml"))
        .expect("346c695a");
    assert!(ci.lines().any(|line| line.trim() == database_image));
    assert!(!ci.contains("postgresql_16_with_pg_jsonschema:latest"));
}
