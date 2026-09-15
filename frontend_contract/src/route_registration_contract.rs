#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
pub trait RouteRegistrationContract: Copy {
    fn registration_method(self) -> crate::route_method::RouteMethod;
    fn registration_path(self) -> crate::contract_str::ContractStr;
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "route registration contract uses iterator traversal to comply with the workspace no-for-loop policy"
    )]
    fn test_route_method_router_supports_every_contract_method() {
        async fn endpoint() -> axum::http::StatusCode {
            axum::http::StatusCode::NO_CONTENT
        }

        [
            crate::route_method::RouteMethod::Connect,
            crate::route_method::RouteMethod::Delete,
            crate::route_method::RouteMethod::Get,
            crate::route_method::RouteMethod::Head,
            crate::route_method::RouteMethod::Options,
            crate::route_method::RouteMethod::Patch,
            crate::route_method::RouteMethod::Post,
            crate::route_method::RouteMethod::Put,
            crate::route_method::RouteMethod::Trace,
        ]
        .into_iter()
        .for_each(|method| {
            let _router =
                crate::route_method_router::route_method_router::<(), _, _>(method, endpoint);
        });
    }

    #[test]
    fn test_route_registration_contract_path_preserves_the_static_path() {
        assert_eq!(
            crate::contract_str::ContractStr::from(constants_str::VALUE_0587C50E).as_ref(),
            constants_str::VALUE_0587C50E
        );
    }
}
