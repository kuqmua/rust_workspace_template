pub(in crate::domain_types::auth::html::actions) fn router()
-> super::super::super::super::AxumAdminStateRouter {
    super::super::super::super::AxumAdminStateRouter::from(
        super::AdminHtmlSessionActionRouteRegistry::registry_router(),
    )
}
