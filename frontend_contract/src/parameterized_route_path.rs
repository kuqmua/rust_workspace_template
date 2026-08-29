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
    type Error = crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::parameterized_route_path_try_from_string_error::ParameterizedRoutePathTryFromStringError)
        } else {
            Ok(Self(value))
        }
    }
}
