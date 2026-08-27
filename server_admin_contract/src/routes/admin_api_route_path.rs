use super::{AdminFrontendPath, AdminRoutePath};

pub(in crate::domain_types) fn admin_api_route_path(
    suffix: frontend_contract::domain_types::ParameterizedRoutePath,
) -> AdminRoutePath {
    AdminRoutePath::try_from(format!(
        "{}{}{suffix}",
        constants_str::V1,
        AdminFrontendPath::Root.get(),
        suffix = String::from(suffix),
    ))
    .unwrap_or_default()
}
