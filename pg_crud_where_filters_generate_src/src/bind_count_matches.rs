#[allow(
    clippy::single_call_fn,
    reason = "the bind count validation boundary is intentionally isolated from descriptor emitters"
)]
pub(super) fn bind_count_matches(
    spec: crate::domain_types::spec::FilterSpec,
    placeholders: crate::domain_types::filter_placeholder_count::FilterPlaceholderCount,
) -> crate::domain_types::spec::FilterSpecValid {
    spec.bind_count_matches(placeholders)
}
