pub(super) fn service_catalog_render_release_entries(
    service_catalog_entries_ref: crate::service_catalog_entries_ref::ServiceCatalogEntriesRef<'_>,
) -> crate::scaffold_text::ScaffoldText {
    let output_capacity = service_catalog_entries_ref
        .get()
        .iter()
        .filter(|entry| bool::from(*entry.get_release()))
        .map(|entry| {
            constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT
                .len()
                .saturating_add(entry.get_image().as_ref().len())
                .saturating_add(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT.len())
                .saturating_add(entry.get_dockerfile().as_ref().len())
                .saturating_add(constants_usize::ONE)
        })
        .sum::<usize>();
    crate::scaffold_text::ScaffoldText::try_from(
        service_catalog_entries_ref
            .get()
            .iter()
            .filter(|entry| bool::from(*entry.get_release()))
            .fold(
                String::with_capacity(output_capacity),
                |mut output, entry| {
                    output.push_str(constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT);
                    output.push_str(entry.get_image().as_ref());
                    output.push_str(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT);
                    output.push_str(entry.get_dockerfile().as_ref());
                    output.push('\n');
                    output
                },
            ),
    )
    .unwrap_or_else(crate::scaffold_text::ScaffoldText::from)
}
