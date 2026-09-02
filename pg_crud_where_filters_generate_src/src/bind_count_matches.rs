pub(super) fn bind_count_matches(
    filter_spec: crate::filter_spec::FilterSpec,
    filter_placeholder_count: crate::filter_placeholder_count::FilterPlaceholderCount,
) -> crate::filter_spec_valid::FilterSpecValid {
    filter_spec.bind_count_matches(filter_placeholder_count)
}
