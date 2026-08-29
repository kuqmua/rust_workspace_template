#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct HttpNormalizedPath(String);

impl TryFrom<String> for HttpNormalizedPath {
    type Error = crate::http_normalized_path_error::HttpNormalizedPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::http_normalized_path_error::HttpNormalizedPathError)
        } else {
            Ok(Self(value))
        }
    }
}
