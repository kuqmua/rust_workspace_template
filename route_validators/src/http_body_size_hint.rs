#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
pub struct HttpBodySizeHint(http_body::SizeHint);

impl HttpBodySizeHint {
    #[cfg(test)]
    pub(crate) fn upper(self) -> Option<u64> {
        self.0.upper()
    }
}

impl to_err_string::to_err_string::ToErrString for HttpBodySizeHint {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{:#?}", self.0))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
