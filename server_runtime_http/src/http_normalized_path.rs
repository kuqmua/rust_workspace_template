#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct HttpNormalizedPath(String);

impl TryFrom<String> for HttpNormalizedPath {
    type Error = crate::http_normalized_path_error::HttpNormalizedPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len().cmp(&constants_usize::VALUE_8_192) {
            std::cmp::Ordering::Greater => {
                Err(crate::http_normalized_path_error::HttpNormalizedPathError::TooLarge)
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => Ok(Self(value)),
        }
    }
}
