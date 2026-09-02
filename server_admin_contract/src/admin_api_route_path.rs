pub(crate) fn admin_api_route_path(
    parameterized_route_path: frontend_contract::parameterized_route_path::ParameterizedRoutePath,
) -> crate::admin_route_path::AdminRoutePath {
    crate::admin_route_path::AdminRoutePath::try_from(format!(
        "{}{}{}",
        constants_str::V1,
        crate::admin_frontend_path::AdminFrontendPath::Root.get(),
        String::from(parameterized_route_path),
    ))
    .unwrap_or_default()
}
