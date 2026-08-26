#![allow(clippy::single_call_fn)] // schema shape projection has one contract consumer

pub(in crate::domain_types) fn schema_uses_text_value(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSpecValid {
    spec.has_text_value_shape()
}
