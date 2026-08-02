#[derive(optml::Optml, Clone, Copy, Debug, newtype::Display, newtype::FromInner)]
pub(in crate::app) struct AdminHttpStatus(u16);

#[derive(
    optml::Optml, Clone, Debug, newtype::AsRefStr, newtype::BoundedString, newtype::Display,
)]
#[bounded_string(max = 16_384usize, chars)]
pub(in crate::app) struct AdminCsrApiUrl(String);

#[derive(optml::Optml, Clone, Copy, Debug, newtype::AsRefStr, newtype::FromInner)]
pub(in crate::app) struct AdminCsrApiUrlSuffixRef<'suffix_lt>(&'suffix_lt str);

pub(in crate::app) fn admin_api_url(
    route: server_admin_contract::AdminRoute,
) -> Result<AdminCsrApiUrl, crate::app::state::AdminTableLoadError> {
    admin_route_path_url(route.path())
}

pub(in crate::app) fn admin_route_path_url(
    path: server_admin_contract::AdminRoutePath,
) -> Result<AdminCsrApiUrl, crate::app::state::AdminTableLoadError> {
    AdminCsrApiUrl::try_from(path.to_string())
        .map_err(|_error| crate::app::state::AdminTableLoadError::Query)
}

pub(in crate::app) fn admin_api_url_with_suffix(
    route: server_admin_contract::AdminRoute,
    suffix: AdminCsrApiUrlSuffixRef<'_>,
) -> Result<AdminCsrApiUrl, crate::app::state::AdminTableLoadError> {
    AdminCsrApiUrl::try_from(format!("{}{}", route.path(), suffix.as_ref()))
        .map_err(|_error| crate::app::state::AdminTableLoadError::Query)
}
