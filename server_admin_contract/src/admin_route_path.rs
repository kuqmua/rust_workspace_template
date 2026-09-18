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
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::admin_route_path_error::AdminRoutePathError::TooLong)
        } else {
            Ok(Self(value.into_boxed_str()))
        }
    }
}

impl From<crate::admin_user_id::AdminUserId> for AdminRoutePath {
    fn from(value: crate::admin_user_id::AdminUserId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_frontend_path::AdminFrontendPath::Users.get(),
                value
            )
            .into_boxed_str(),
        )
    }
}
