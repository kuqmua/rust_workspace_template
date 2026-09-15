#[must_use]
pub fn register_route<State>(
    frontend_contract_axum_router: crate::frontend_contract_axum_router::FrontendContractAxumRouter<
        State,
    >,
    contract_str: crate::contract_str::ContractStr,
    axum_route_method_router: crate::axum_route_method_router::AxumRouteMethodRouter<State>,
) -> crate::frontend_contract_axum_router::FrontendContractAxumRouter<State>
where
    State: Clone + Send + Sync + 'static,
{
    crate::frontend_contract_axum_router::FrontendContractAxumRouter::from(
        axum::Router::from(frontend_contract_axum_router).route(
            contract_str.as_ref(),
            axum::routing::MethodRouter::from(axum_route_method_router),
        ),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_register_route_accepts_contract_path_and_method_router() {
        let router = super::register_route(
            crate::frontend_contract_axum_router::FrontendContractAxumRouter::from(
                axum::Router::<()>::new(),
            ),
            crate::contract_str::ContractStr::from(constants_str::VALUE_0587C50E),
            crate::axum_route_method_router::AxumRouteMethodRouter::from(axum::routing::get(
                async || axum::http::StatusCode::OK,
            )),
        );
        drop(router);
    }
}
