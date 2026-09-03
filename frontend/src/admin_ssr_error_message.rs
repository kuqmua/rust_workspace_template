#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_display::Display,
)]
pub struct AdminSsrErrorMessage(to_err_string::error_text::ErrorText);
impl TryFrom<String> for AdminSsrErrorMessage {
    type Error = to_err_string::error_text::ErrorTextTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        to_err_string::error_text::ErrorText::try_from(string).map(Self)
    }
}
