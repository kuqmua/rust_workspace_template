#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default, proc_macro_getters::Getters,
)]
#[getters(get_mut)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct ServiceCatalogDraft {
    compose_file: Option<crate::service_compose_file::ServiceComposeFile>,
    compose_name: Option<crate::service_compose_name::ServiceComposeName>,
    crate_name: Option<crate::service_crate::ServiceCrate>,
    dockerfile: Option<crate::service_dockerfile::ServiceDockerfile>,
    image: Option<crate::service_image::ServiceImage>,
    kubernetes_manifest: Option<crate::service_kubernetes_manifest::ServiceKubernetesManifest>,
    socket_env: Option<crate::service_socket_env::ServiceSocketEnv>,
    port: Option<crate::service_port::ServicePort>,
    release: Option<crate::should_release::ShouldRelease>,
}
impl ServiceCatalogDraft {
    pub(super) fn finish(
        self,
    ) -> Result<
        crate::service_catalog_entry::ServiceCatalogEntry,
        crate::scaffold_error::ScaffoldError,
    > {
        Ok(crate::service_catalog_entry::ServiceCatalogEntry::new(
            self.compose_file
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.compose_name
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.crate_name
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.dockerfile
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.image
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.kubernetes_manifest
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.socket_env
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.port
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
            self.release
                .ok_or(crate::scaffold_error::ScaffoldError::Catalog)?,
        ))
    }
}
