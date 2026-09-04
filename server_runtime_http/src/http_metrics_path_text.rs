#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    proc_macro_newtype_borrow_str::BorrowStr,
    proc_macro_newtype_deref_inner::DerefInner,
)]
pub(super) struct HttpMetricsPathText(String);

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
