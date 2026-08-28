#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct TypedClient<Transport> {
    path_prefix: crate::TransportPath,
    transport: Transport,
}
impl<Transport> TypedClient<Transport>
where
    Transport: crate::Transport,
{
    #[must_use]
    pub const fn new(transport: Transport, path_prefix: crate::TransportPath) -> Self {
        Self {
            path_prefix,
            transport,
        }
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    pub async fn send<Route>(
        &self,
        body: Route::Request,
    ) -> Result<Route::Response, crate::ClientError>
    where
        Route: crate::TypedRoute,
    {
        let route_path = crate::TransportPath::try_from(
            Route::metadata().path().as_ref().to_owned(),
        )
        .map_err(|error| crate::ClientError::Encode(super::create_form_value_error(error)))?;
        self.send_to::<Route>(&route_path, body).await
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    pub async fn send_parameterized<Route>(
        &self,
        parameter: &Route::Parameter,
        body: Route::Request,
    ) -> Result<Route::Response, crate::ClientError>
    where
        Route: crate::ParameterizedRoute,
    {
        let route_path = crate::TransportPath::try_from(String::from(Route::path(parameter)))
            .map_err(|error| crate::ClientError::Encode(super::create_form_value_error(error)))?;
        self.send_to::<Route>(&route_path, body).await
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    pub async fn send_contract(
        &self,
        contract: crate::RouteContract,
        route_path: crate::ContractStr,
    ) -> Result<crate::TransportBody, crate::ClientError> {
        let transport_path = crate::TransportPath::try_from(route_path.as_ref().to_owned())
            .map_err(|error| crate::ClientError::Encode(super::create_form_value_error(error)))?;
        let transport_body = crate::TransportBody::try_from(Vec::new())
            .map_err(|error| crate::ClientError::Encode(super::create_form_value_error(error)))?;
        let response = self
            .send_request(transport_body, &transport_path, contract)
            .await?;
        response
            .success_body(contract.success_status().transport_status())
            .cloned()
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    async fn send_to<Route>(
        &self,
        route_path: &crate::TransportPath,
        body: Route::Request,
    ) -> Result<Route::Response, crate::ClientError>
    where
        Route: crate::TypedRoute,
    {
        let metadata = Route::metadata();
        let transport_body = match Route::request_body() {
            crate::RouteRequestBody::Absent => crate::TransportBody::try_from(Vec::new()),
            crate::RouteRequestBody::Json => serde_json::to_vec(&body)
                .map_err(|error| crate::ClientError::Encode(super::create_form_value_error(error)))?
                .try_into(),
        }
        .map_err(|error: crate::FrontendContractBodyError| {
            crate::ClientError::Encode(super::create_form_value_error(error))
        })?;
        let response = self
            .send_request(transport_body, route_path, metadata.contract())
            .await?;
        let response_body = response.success_body(metadata.success_status().transport_status())?;
        let bytes = if response_body.as_ref().is_empty() {
            b"null".as_slice()
        } else {
            response_body.as_ref()
        };
        serde_json::from_slice(bytes)
            .map_err(|error| crate::ClientError::Decode(super::create_form_value_error(error)))
    }
    #[allow(clippy::future_not_send)] // Transport intentionally permits single-threaded WASM futures
    async fn send_request(
        &self,
        body: crate::TransportBody,
        route_path: &crate::TransportPath,
        contract: crate::RouteContract,
    ) -> Result<crate::TransportResponse, crate::ClientError> {
        let prefix_ref = self.path_prefix.as_ref().trim_end_matches('/');
        let route_path_ref = route_path.as_ref().trim_start_matches('/');
        let path_string = if prefix_ref.is_empty() {
            format!("/{route_path_ref}")
        } else if route_path_ref.is_empty() {
            prefix_ref.to_owned()
        } else {
            format!("{prefix_ref}/{route_path_ref}")
        };
        let path = crate::TransportPath::try_from(path_string)
            .map_err(|error| crate::ClientError::Encode(super::create_form_value_error(error)))?;
        self.transport
            .send(crate::TransportRequest::new(body, path, contract))
            .await
            .map_err(crate::ClientError::Transport)
    }
}
