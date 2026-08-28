#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
pub trait RouteRegistrationContract: Copy {
    fn method(self) -> super::RouteMethod;
    fn path(self) -> RegisteredRoutePath;
}

#[cfg(not(target_arch = "wasm32"))]
pub use crate::axum_route_method_router::AxumRouteMethodRouter;
pub use crate::registered_route_path::RegisteredRoutePath;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::route_method_router::route_method_router;
#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn route_method_router_supports_every_contract_method() {
        async fn endpoint() -> axum::http::StatusCode {
            axum::http::StatusCode::NO_CONTENT
        }

        [
            crate::RouteMethod::Connect,
            crate::RouteMethod::Delete,
            crate::RouteMethod::Get,
            crate::RouteMethod::Head,
            crate::RouteMethod::Options,
            crate::RouteMethod::Patch,
            crate::RouteMethod::Post,
            crate::RouteMethod::Put,
            crate::RouteMethod::Trace,
        ]
        .into_iter()
        .for_each(|method| {
            let _router = super::route_method_router::<(), _, _>(method, endpoint);
        });
    }

    #[test]
    fn registered_route_path_preserves_the_static_path() {
        assert_eq!(super::RegisteredRoutePath::from("/health").get(), "/health");
    }
}
