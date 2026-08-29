#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct HttpBodySizeHint(pub(super) http_body::SizeHint);

impl to_err_string::to_err_string::ToErrString for HttpBodySizeHint {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{:#?}", self.0))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
