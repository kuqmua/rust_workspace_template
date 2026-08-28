pub(super) fn bind_count_matches(
    spec: crate::spec::FilterSpec,
    placeholders: crate::filter_placeholder_count::FilterPlaceholderCount,
) -> crate::spec::FilterSpecValid {
    spec.bind_count_matches(placeholders)
}
