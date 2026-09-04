#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct FormValueError(to_err_string::error_text::ErrorText);
impl TryFrom<String> for FormValueError {
    type Error = to_err_string::error_text::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::error_text::ErrorText::try_from(value).map(Self)
    }
}
