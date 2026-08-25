#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub(in crate::domain_types::app) struct AdminHttpStatus(u16);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = 16_384usize, chars)]
pub(in crate::domain_types::app) struct AdminCsrApiUrl(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(in crate::domain_types::app) struct AdminCsrApiUrlSuffixRef<'suffix_lt>(&'suffix_lt str);

pub(in crate::domain_types::app) fn admin_api_url(
    route: server_admin_contract::domain_types::AdminRoute,
) -> Result<AdminCsrApiUrl, crate::domain_types::app::state::AdminTableLoadError> {
    admin_route_path_url(route.path())
}

pub(in crate::domain_types::app) fn admin_route_path_url(
    path: server_admin_contract::domain_types::AdminRoutePath,
) -> Result<AdminCsrApiUrl, crate::domain_types::app::state::AdminTableLoadError> {
    AdminCsrApiUrl::try_from(path.to_string())
        .map_err(|_error| crate::domain_types::app::state::AdminTableLoadError::Query)
}

pub(in crate::domain_types::app) fn admin_api_url_with_suffix(
    route: server_admin_contract::domain_types::AdminRoute,
    suffix: AdminCsrApiUrlSuffixRef<'_>,
) -> Result<AdminCsrApiUrl, crate::domain_types::app::state::AdminTableLoadError> {
    AdminCsrApiUrl::try_from(format!("{}{}", route.path(), suffix.as_ref()))
        .map_err(|_error| crate::domain_types::app::state::AdminTableLoadError::Query)
}
