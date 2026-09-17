#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_borrow_str::BorrowStr,
)]
pub(crate) struct EnvKey(bounded_types::bounded_string::BoundedString<1usize, 1_024usize, false>);
impl TryFrom<String> for EnvKey {
    type Error = crate::init_string_error::InitStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 1_024usize {
            Err(Self::Error::Invalid)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(value)
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    }
                    | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Invalid,
                })
        }
    }
}
