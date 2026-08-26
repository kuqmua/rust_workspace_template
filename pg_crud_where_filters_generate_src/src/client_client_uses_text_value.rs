#![allow(
    clippy::single_call_fn,
    reason = "the client emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(in crate::domain_types) fn client_uses_text_value(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSpecValid {
    spec.has_text_value_shape()
}
