#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SettingsUpdate, crate::update_settings),
)]
pub(crate) struct AdminHtmlSettingsActionRouteRegistry;
