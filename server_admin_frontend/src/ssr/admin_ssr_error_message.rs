#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::Display,
)]
pub struct AdminSsrErrorMessage(to_err_string::domain_types::ErrorText);
impl TryFrom<String> for AdminSsrErrorMessage {
    type Error = to_err_string::domain_types::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::domain_types::ErrorText::try_from(value).map(Self)
    }
}
