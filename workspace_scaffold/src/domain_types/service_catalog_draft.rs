#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{
    ScaffoldError, ServiceCatalogEntry, ServiceComposeFile, ServiceComposeName, ServiceCrate,
    ServiceDockerfile, ServiceImage, ServiceKubernetesManifest, ServicePort, ServiceSocketEnv,
    ShouldRelease,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct ServiceCatalogDraft {
    pub(super) compose_file: Option<ServiceComposeFile>,
    pub(super) compose_name: Option<ServiceComposeName>,
    pub(super) crate_name: Option<ServiceCrate>,
    pub(super) dockerfile: Option<ServiceDockerfile>,
    pub(super) image: Option<ServiceImage>,
    pub(super) kubernetes_manifest: Option<ServiceKubernetesManifest>,
    pub(super) socket_env: Option<ServiceSocketEnv>,
    pub(super) port: Option<ServicePort>,
    pub(super) release: Option<ShouldRelease>,
}
impl ServiceCatalogDraft {
    pub(super) fn finish(self) -> Result<ServiceCatalogEntry, ScaffoldError> {
        Ok(ServiceCatalogEntry {
            compose_file: self.compose_file.ok_or(ScaffoldError::Catalog)?,
            compose_name: self.compose_name.ok_or(ScaffoldError::Catalog)?,
            crate_name: self.crate_name.ok_or(ScaffoldError::Catalog)?,
            dockerfile: self.dockerfile.ok_or(ScaffoldError::Catalog)?,
            image: self.image.ok_or(ScaffoldError::Catalog)?,
            kubernetes_manifest: self.kubernetes_manifest.ok_or(ScaffoldError::Catalog)?,
            port: self.port.ok_or(ScaffoldError::Catalog)?,
            release: self.release.ok_or(ScaffoldError::Catalog)?,
            socket_env: self.socket_env.ok_or(ScaffoldError::Catalog)?,
        })
    }
}
