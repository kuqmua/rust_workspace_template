#[test]
fn notification_deployment_probes_use_registered_health_routes() {
    let repository_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("416090ad");
    let compose_source =
        std::fs::read_to_string(repository_root.join("docker-compose.yml")).expect("0c6173a3");
    let notification_service = compose_source
        .split_once("  notification_service:\n")
        .map(|(_before, service)| service)
        .expect("def93cb8");
    assert!(
        notification_service.contains(str_constants::COMMON_ROUTES_HEALTH_READY),
        "4933eff6"
    );
    let deployment_source =
        std::fs::read_to_string(repository_root.join("deploy/k8s/base/notification-service.yaml"))
            .expect("631594d9");
    assert_eq!(
        deployment_source
            .lines()
            .filter(|line| {
                line.trim() == format!("path: {}", str_constants::COMMON_ROUTES_HEALTH_READY)
            })
            .count(),
        2usize,
        "886062ce"
    );
    assert_eq!(
        deployment_source
            .lines()
            .filter(|line| {
                line.trim() == format!("path: {}", str_constants::COMMON_ROUTES_HEALTH_LIVE)
            })
            .count(),
        1usize,
        "4173ba47"
    );
}
