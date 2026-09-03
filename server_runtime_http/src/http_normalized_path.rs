#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct HttpNormalizedPath(String);

impl TryFrom<String> for HttpNormalizedPath {
    type Error = crate::http_normalized_path_error::HttpNormalizedPathError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        match string.len().cmp(&constants_usize::VALUE_8_192) {
            std::cmp::Ordering::Greater => {
                Err(crate::http_normalized_path_error::HttpNormalizedPathError::TooLarge)
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => Ok(Self(string)),
        }
    }
}
