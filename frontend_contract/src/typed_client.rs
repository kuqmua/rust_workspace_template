#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct TypedClient<Transport> {
    path_prefix: crate::transport_path::TransportPath,
    transport: Transport,
}
impl<Transport> TypedClient<Transport>
where
    Transport: crate::transport::Transport,
{
    #[must_use]
    pub const fn new(
        transport: Transport,
        transport_path: crate::transport_path::TransportPath,
    ) -> Self {
        Self {
            path_prefix: transport_path,
            transport,
        }
    }
    #[allow(clippy::future_not_send, reason = "lint suppression is required here")]
    pub async fn send<Route>(
        &self,
        request: Route::Request,
    ) -> Result<Route::Response, crate::client_error::ClientError>
    where
        Route: crate::typed_route::TypedRoute,
    {
        let route_path = crate::transport_path::TransportPath::try_from(
            Route::metadata().path().as_ref().to_owned(),
        )
        .map_err(|error| {
            crate::client_error::ClientError::Encode(
                crate::create_form_value_error::create_form_value_error(error),
            )
        })?;
        self.send_to::<Route>(&route_path, request).await
    }
    #[allow(clippy::future_not_send, reason = "lint suppression is required here")]
    pub async fn send_parameterized<Route>(
        &self,
        parameter: &Route::Parameter,
        request: Route::Request,
    ) -> Result<Route::Response, crate::client_error::ClientError>
    where
        Route: crate::parameterized_route::ParameterizedRoute,
    {
        let route_path =
            crate::transport_path::TransportPath::try_from(String::from(Route::path(parameter)))
                .map_err(|error| {
                    crate::client_error::ClientError::Encode(
                        crate::create_form_value_error::create_form_value_error(error),
                    )
                })?;
        self.send_to::<Route>(&route_path, request).await
    }
    #[allow(clippy::future_not_send, reason = "lint suppression is required here")]
    pub async fn send_contract(
        &self,
        route_contract: crate::route_contract::RouteContract,
        contract_str: crate::contract_str::ContractStr,
    ) -> Result<crate::transport_body::TransportBody, crate::client_error::ClientError> {
        let transport_path =
            crate::transport_path::TransportPath::try_from(contract_str.as_ref().to_owned())
                .map_err(|error| {
                    crate::client_error::ClientError::Encode(
                        crate::create_form_value_error::create_form_value_error(error),
                    )
                })?;
        let transport_body =
            crate::transport_body::TransportBody::try_from(Vec::new()).map_err(|error| {
                crate::client_error::ClientError::Encode(
                    crate::create_form_value_error::create_form_value_error(error),
                )
            })?;
        let response = self
            .send_request(transport_body, &transport_path, route_contract)
            .await?;
        response
            .success_body(route_contract.success_status().transport_status())
            .cloned()
    }
    #[allow(clippy::future_not_send, reason = "lint suppression is required here")]
    async fn send_to<Route>(
        &self,
        transport_path: &crate::transport_path::TransportPath,
        request: Route::Request,
    ) -> Result<Route::Response, crate::client_error::ClientError>
    where
        Route: crate::typed_route::TypedRoute,
    {
        let metadata = Route::metadata();
        let transport_body = match Route::request_body() {
            crate::route_request_body::RouteRequestBody::Absent => {
                crate::transport_body::TransportBody::try_from(Vec::new())
            }
            crate::route_request_body::RouteRequestBody::Json => serde_json::to_vec(&request)
                .map_err(|error| {
                    crate::client_error::ClientError::Encode(
                        crate::create_form_value_error::create_form_value_error(error),
                    )
                })?
                .try_into(),
        }
        .map_err(
            |error: crate::frontend_contract_body_error::FrontendContractBodyError| {
                crate::client_error::ClientError::Encode(
                    crate::create_form_value_error::create_form_value_error(error),
                )
            },
        )?;
        let response = self
            .send_request(transport_body, transport_path, metadata.contract())
            .await?;
        let response_body = response.success_body(metadata.success_status().transport_status())?;
        let bytes = if response_body.as_ref().is_empty() {
            b"null".as_slice()
        } else {
            response_body.as_ref()
        };
        serde_json::from_slice(bytes).map_err(|error| {
            crate::client_error::ClientError::Decode(
                crate::create_form_value_error::create_form_value_error(error),
            )
        })
    }
    #[allow(clippy::future_not_send, reason = "lint suppression is required here")]
    async fn send_request(
        &self,
        transport_body: crate::transport_body::TransportBody,
        transport_path: &crate::transport_path::TransportPath,
        route_contract: crate::route_contract::RouteContract,
    ) -> Result<crate::transport_response::TransportResponse, crate::client_error::ClientError>
    {
        let prefix_ref = self.path_prefix.as_ref().trim_end_matches('/');
        let route_path_ref = transport_path.as_ref().trim_start_matches('/');
        let path_string = if prefix_ref.is_empty() {
            format!("/{route_path_ref}")
        } else if route_path_ref.is_empty() {
            prefix_ref.to_owned()
        } else {
            format!("{prefix_ref}/{route_path_ref}")
        };
        let path =
            crate::transport_path::TransportPath::try_from(path_string).map_err(|error| {
                crate::client_error::ClientError::Encode(
                    crate::create_form_value_error::create_form_value_error(error),
                )
            })?;
        self.transport
            .send(crate::transport_request::TransportRequest::new(
                transport_body,
                path,
                route_contract,
            ))
            .await
            .map_err(crate::client_error::ClientError::Transport)
    }
}
