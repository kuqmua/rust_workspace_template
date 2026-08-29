pub(super) fn bind_count_matches(
    spec: crate::filter_spec::FilterSpec,
    placeholders: crate::filter_placeholder_count::FilterPlaceholderCount,
) -> crate::filter_spec_valid::FilterSpecValid {
    spec.bind_count_matches(placeholders)
}
