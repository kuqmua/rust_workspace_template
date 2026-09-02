pub(crate) fn client_uses_text_value(
    filter_spec: crate::filter_spec::FilterSpec,
) -> crate::filter_spec_valid::FilterSpecValid {
    filter_spec.has_text_value_shape()
}
