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
pub(super) struct HttpMetricsPathText(
    bounded_types::bounded_string::BoundedString<1usize, 8_192usize, false>,
);

impl TryFrom<String> for HttpMetricsPathText {
    type Error = crate::http_metrics_path_text_error::HttpMetricsPathTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > constants_usize::VALUE_8_192 {
            Err(crate::http_metrics_path_text_error::HttpMetricsPathTextError)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => crate::http_metrics_path_text_error::HttpMetricsPathTextError,
                })
        }
    }
}
