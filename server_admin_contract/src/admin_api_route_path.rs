pub(crate) fn admin_api_route_path(
    suffix: frontend_contract::parameterized_route_path::ParameterizedRoutePath,
) -> crate::admin_route_path::AdminRoutePath {
    crate::admin_route_path::AdminRoutePath::try_from(format!(
        "{}{}{suffix}",
        constants_str::V1,
        crate::admin_frontend_path::AdminFrontendPath::Root.get(),
        suffix = String::from(suffix),
    ))
    .unwrap_or_default()
}
