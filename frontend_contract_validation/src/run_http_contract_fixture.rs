pub async fn run_http_contract_fixture<Send, SendFuture>(
    expectation: crate::route_contract_validation::HttpContractExpectation,
    send: Send,
) -> Result<(), crate::route_contract_validation::HttpContractMismatch>
where
    Send: FnOnce(frontend_contract::RouteMetadata) -> SendFuture,
    SendFuture: Future<Output = crate::route_contract_validation::HttpContractObservation>,
{
    let observation = send(expectation.metadata).await;
    crate::route_contract_validation::validate_route_contract_metadata(
        expectation.metadata,
        observation.metadata,
    )
    .map_err(crate::route_contract_validation::HttpContractMismatch::Metadata)?;
    if expectation.status != observation.status {
        return Err(
            crate::route_contract_validation::HttpContractMismatch::Status {
                expected: expectation.status,
                observed: observation.status,
            },
        );
    }
    match expectation.body_kind {
        crate::route_contract_validation::HttpContractBodyKind::Empty
            if !observation.body.0.is_empty() =>
        {
            Err(crate::route_contract_validation::HttpContractMismatch::BodyExpectedEmpty)
        }
        crate::route_contract_validation::HttpContractBodyKind::Json
            if serde_json::from_slice::<serde_json::Value>(&observation.body.0).is_err() =>
        {
            Err(crate::route_contract_validation::HttpContractMismatch::BodyExpectedJson)
        }
        crate::route_contract_validation::HttpContractBodyKind::Empty
        | crate::route_contract_validation::HttpContractBodyKind::Json => Ok(()),
    }
}
