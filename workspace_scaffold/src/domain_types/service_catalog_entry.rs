use super::{
    ServiceComposeFile, ServiceComposeName, ServiceCrate, ServiceDockerfile, ServiceImage,
    ServiceKubernetesManifest, ServicePort, ServiceSocketEnv, ShouldRelease,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct ServiceCatalogEntry {
    pub(super) compose_file: ServiceComposeFile,
    pub(super) compose_name: ServiceComposeName,
    pub(super) crate_name: ServiceCrate,
    pub(super) dockerfile: ServiceDockerfile,
    pub(super) image: ServiceImage,
    pub(super) kubernetes_manifest: ServiceKubernetesManifest,
    pub(super) socket_env: ServiceSocketEnv,
    pub(super) port: ServicePort,
    pub(super) release: ShouldRelease,
}
