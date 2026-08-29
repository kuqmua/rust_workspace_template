#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    newtype::BorrowStr,
)]
pub(super) struct HttpMetricsPathText(pub(super) String);

impl TryFrom<String> for HttpMetricsPathText {
    type Error = crate::http_metrics_path_text_error::HttpMetricsPathTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > constants_usize::VALUE_8_192 {
            Err(crate::http_metrics_path_text_error::HttpMetricsPathTextError)
        } else {
            Ok(Self(value))
        }
    }
}
