#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct NonEmptyTrimmedText(String);
impl TryFrom<String> for NonEmptyTrimmedText {
    type Error = crate::bounded_text_policy_error::BoundedTextPolicyError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > constants_usize::VALUE_1_048_576 {
            return Err(Self::Error::TooLong);
        }
        let trimmed = string.trim();
        if trimmed.is_empty() {
            Err(Self::Error::Empty)
        } else if trimmed.contains('\0') {
            Err(Self::Error::ContainsNul)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}
