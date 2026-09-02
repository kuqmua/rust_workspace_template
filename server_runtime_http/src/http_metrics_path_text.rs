#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    proc_macro_newtype::BorrowStr,
    proc_macro_newtype::DerefInner,
)]
pub(super) struct HttpMetricsPathText(String);

impl TryFrom<String> for HttpMetricsPathText {
    type Error = crate::http_metrics_path_text_error::HttpMetricsPathTextError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.is_empty() || string.len() > constants_usize::VALUE_8_192 {
            Err(crate::http_metrics_path_text_error::HttpMetricsPathTextError)
        } else {
            Ok(Self(string))
        }
    }
}
