use super::ParameterizedRoutePathTryFromStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
)]
pub struct ParameterizedRoutePath(String);
impl TryFrom<String> for ParameterizedRoutePath {
    type Error = ParameterizedRoutePathTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(ParameterizedRoutePathTryFromStringError)
        } else {
            Ok(Self(value))
        }
    }
}
