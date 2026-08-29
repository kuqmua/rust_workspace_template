pub async fn run_http_contract_fixture<Send, SendFuture>(
    expectation: crate::http_contract_expectation::HttpContractExpectation,
    send: Send,
) -> Result<(), crate::http_contract_mismatch::HttpContractMismatch>
where
    Send: FnOnce(frontend_contract::route_metadata::RouteMetadata) -> SendFuture,
    SendFuture: Future<Output = crate::http_contract_observation::HttpContractObservation>,
{
    let observation = send(expectation.metadata).await;
    crate::validate_route_contract_metadata::validate_route_contract_metadata(
        expectation.metadata,
        observation.metadata,
    )
    .map_err(crate::http_contract_mismatch::HttpContractMismatch::Metadata)?;
    if expectation.status != observation.status {
        return Err(
            crate::http_contract_mismatch::HttpContractMismatch::Status {
                expected: expectation.status,
                observed: observation.status,
            },
        );
    }
    match expectation.body_kind {
        crate::http_contract_body_kind::HttpContractBodyKind::Empty
            if !observation.body.0.is_empty() =>
        {
            Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedEmpty)
        }
        crate::http_contract_body_kind::HttpContractBodyKind::Json
            if serde_json::from_slice::<serde_json::Value>(&observation.body.0).is_err() =>
        {
            Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedJson)
        }
        crate::http_contract_body_kind::HttpContractBodyKind::Empty
        | crate::http_contract_body_kind::HttpContractBodyKind::Json => Ok(()),
    }
}
