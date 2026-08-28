pub(crate) fn schema_uses_text_value(
    spec: crate::spec::FilterSpec,
) -> crate::spec::FilterSpecValid {
    spec.has_text_value_shape()
}
