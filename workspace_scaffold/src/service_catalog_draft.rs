#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct ServiceCatalogDraft {
    pub(super) compose_file: Option<crate::service_compose_file::ServiceComposeFile>,
    pub(super) compose_name: Option<crate::service_compose_name::ServiceComposeName>,
    pub(super) crate_name: Option<crate::service_crate::ServiceCrate>,
    pub(super) dockerfile: Option<crate::service_dockerfile::ServiceDockerfile>,
    pub(super) image: Option<crate::service_image::ServiceImage>,
    pub(super) kubernetes_manifest:
        Option<crate::service_kubernetes_manifest::ServiceKubernetesManifest>,
    pub(super) socket_env: Option<crate::service_socket_env::ServiceSocketEnv>,
    pub(super) port: Option<crate::service_port::ServicePort>,
    pub(super) release: Option<crate::should_release::ShouldRelease>,
}
impl ServiceCatalogDraft {
    pub(super) fn finish(
        self,
    ) -> Result<
        crate::service_catalog_entry::ServiceCatalogEntry,
        crate::scaffold_error::ScaffoldError,
    > {
        Ok(crate::service_catalog_entry::ServiceCatalogEntry {
            compose_file: self
                .compose_file
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            compose_name: self
                .compose_name
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            crate_name: self
                .crate_name
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            dockerfile: self
                .dockerfile
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            image: self
                .image
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            kubernetes_manifest: self
                .kubernetes_manifest
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            port: self
                .port
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            release: self
                .release
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            socket_env: self
                .socket_env
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
        })
    }
}
