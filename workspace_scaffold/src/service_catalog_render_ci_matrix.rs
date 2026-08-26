#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns the CI projection"
)]
pub(super) fn render_ci_matrix(
    entries: super::ServiceCatalogEntriesRef<'_>,
) -> super::ScaffoldText {
    super::service_catalog_render_release_entries::render_release_entries(entries)
}
