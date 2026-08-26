#[allow(
    clippy::single_call_fn,
    reason = "deployment synchronization owns the release projection"
)]
pub(super) fn render_release_matrix(
    entries: super::ServiceCatalogEntriesRef<'_>,
) -> super::ScaffoldText {
    super::service_catalog_render_release_entries::render_release_entries(entries)
}
