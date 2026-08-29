use super::domain_types::BoundedTextPolicyError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct RequiredNulFreeBoundedText(String);
impl TryFrom<String> for RequiredNulFreeBoundedText {
    type Error = BoundedTextPolicyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_1_048_576 {
            return Err(Self::Error::TooLong);
        }
        if value.is_empty() {
            Err(Self::Error::Empty)
        } else if value.contains('\0') {
            Err(Self::Error::ContainsNul)
        } else {
            Ok(Self(value))
        }
    }
}
