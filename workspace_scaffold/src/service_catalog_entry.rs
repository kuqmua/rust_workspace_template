#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
pub(super) struct ServiceCatalogEntry {
    compose_file: crate::service_compose_file::ServiceComposeFile,
    compose_name: crate::service_compose_name::ServiceComposeName,
    crate_name: crate::service_crate::ServiceCrate,
    dockerfile: crate::service_dockerfile::ServiceDockerfile,
    image: crate::service_image::ServiceImage,
    kubernetes_manifest: crate::service_kubernetes_manifest::ServiceKubernetesManifest,
    socket_env: crate::service_socket_env::ServiceSocketEnv,
    port: crate::service_port::ServicePort,
    release: crate::should_release::ShouldRelease,
}
