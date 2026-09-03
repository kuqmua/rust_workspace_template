#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
pub struct AdminRoutePath(Box<str>);
impl TryFrom<String> for AdminRoutePath {
    type Error = crate::admin_route_path_error::AdminRoutePathError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > constants_usize::VALUE_8_192 {
            Err(crate::admin_route_path_error::AdminRoutePathError::TooLong)
        } else {
            Ok(Self(string.into_boxed_str()))
        }
    }
}
