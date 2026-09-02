#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
)]
pub struct FormValueError(to_err_string::error_text::ErrorText);
impl TryFrom<String> for FormValueError {
    type Error = to_err_string::error_text::ErrorTextTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        to_err_string::error_text::ErrorText::try_from(string).map(Self)
    }
}
