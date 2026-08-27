pub(in super::super::super) fn router() -> super::super::super::super::super::AxumAdminStateRouter {
    super::super::super::super::super::AxumAdminStateRouter::from(
        super::AdminHtmlRoleActionRouteRegistry::router(),
    )
}
