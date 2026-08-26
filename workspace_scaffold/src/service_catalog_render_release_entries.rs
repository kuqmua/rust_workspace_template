pub(super) fn service_catalog_render_release_entries(
    entries: super::ServiceCatalogEntriesRef<'_>,
) -> super::ScaffoldText {
    let output_capacity = entries
        .0
        .iter()
        .filter(|entry| bool::from(entry.release))
        .map(|entry| {
            constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT
                .len()
                .saturating_add(entry.image.as_ref().len())
                .saturating_add(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT.len())
                .saturating_add(entry.dockerfile.as_ref().len())
                .saturating_add(constants_usize::ONE)
        })
        .sum::<usize>();
    super::ScaffoldText::try_from(
        entries
            .0
            .iter()
            .filter(|entry| bool::from(entry.release))
            .fold(
                String::with_capacity(output_capacity),
                |mut output, entry| {
                    output.push_str(constants_str::WORKSPACE_SCAFFOLD_MATRIX_NAME_INDENT);
                    output.push_str(entry.image.as_ref());
                    output.push_str(constants_str::WORKSPACE_SCAFFOLD_MATRIX_DOCKERFILE_INDENT);
                    output.push_str(entry.dockerfile.as_ref());
                    output.push('\n');
                    output
                },
            ),
    )
    .unwrap_or_else(super::ScaffoldText::from)
}
