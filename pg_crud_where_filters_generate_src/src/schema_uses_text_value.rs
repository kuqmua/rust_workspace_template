pub(crate) fn schema_uses_text_value(
    spec: crate::filter_spec::FilterSpec,
) -> crate::filter_spec_valid::FilterSpecValid {
    spec.has_text_value_shape()
}
