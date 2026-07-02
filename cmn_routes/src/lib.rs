const ROUTE_GIT_INFO: RoutePath = RoutePath("/git_info");
const ROUTE_HEALTH_CHECK: RoutePath = RoutePath("/health_check");
const ROUTE_SWAGGER_UI: RoutePath = RoutePath("/swagger-ui");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommonRoutes<RouteState> {
    state: RouteState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutePath(&'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonRoute {
    GitInfo { path: RoutePath },
    HealthCheck { path: RoutePath },
    SwaggerUi { path: RoutePath },
}

pub trait CommonRoutesParameters: app_state::GetPgPool + git_info::GetGitCommitLink {}

impl<RouteState> CommonRoutes<RouteState> {
    #[must_use]
    pub const fn new(state: RouteState) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(&self) -> &RouteState {
        &self.state
    }
}

impl AsRef<str> for RoutePath {
    fn as_ref(&self) -> &str {
        self.0
    }
}

#[must_use]
pub const fn git_info_route() -> CommonRoute {
    CommonRoute::GitInfo {
        path: ROUTE_GIT_INFO,
    }
}

#[must_use]
pub const fn health_check_route() -> CommonRoute {
    CommonRoute::HealthCheck {
        path: ROUTE_HEALTH_CHECK,
    }
}

#[must_use]
pub const fn swagger_ui_route() -> CommonRoute {
    CommonRoute::SwaggerUi {
        path: ROUTE_SWAGGER_UI,
    }
}
