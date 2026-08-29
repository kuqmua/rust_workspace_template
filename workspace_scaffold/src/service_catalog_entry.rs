#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct ServiceCatalogEntry {
    pub(super) compose_file: crate::service_compose_file::ServiceComposeFile,
    pub(super) compose_name: crate::service_compose_name::ServiceComposeName,
    pub(super) crate_name: crate::service_crate::ServiceCrate,
    pub(super) dockerfile: crate::service_dockerfile::ServiceDockerfile,
    pub(super) image: crate::service_image::ServiceImage,
    pub(super) kubernetes_manifest: crate::service_kubernetes_manifest::ServiceKubernetesManifest,
    pub(super) socket_env: crate::service_socket_env::ServiceSocketEnv,
    pub(super) port: crate::service_port::ServicePort,
    pub(super) release: crate::should_release::ShouldRelease,
}
