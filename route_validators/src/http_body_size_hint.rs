#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct HttpBodySizeHint(pub(super) http_body::SizeHint);

impl to_err_string::domain_types::ToErrString for HttpBodySizeHint {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(format!("{:#?}", self.0))
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
