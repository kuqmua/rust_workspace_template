pub async fn run_http_contract_fixture<Send, SendFuture>(
    http_contract_expectation: crate::http_contract_expectation::HttpContractExpectation,
    send: Send,
) -> Result<(), crate::http_contract_mismatch::HttpContractMismatch>
where
    Send: FnOnce(frontend_contract::route_metadata::RouteMetadata) -> SendFuture,
    SendFuture: Future<Output = crate::http_contract_observation::HttpContractObservation>,
{
    let metadata = http_contract_expectation.metadata();
    let status = http_contract_expectation.status();
    let body_kind = http_contract_expectation.body_kind();
    let observation = send(metadata).await;
    crate::validate_route_contract_metadata::validate_route_contract_metadata(
        metadata,
        observation.metadata(),
    )
    .map_err(crate::http_contract_mismatch::HttpContractMismatch::Metadata)?;
    if status != observation.status() {
        return Err(
            crate::http_contract_mismatch::HttpContractMismatch::Status {
                expected: status,
                observed: observation.status(),
            },
        );
    }
    match body_kind {
        crate::http_contract_body_kind::HttpContractBodyKind::Empty
            if !observation.body().is_empty() =>
        {
            Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedEmpty)
        }
        crate::http_contract_body_kind::HttpContractBodyKind::Json
            if serde_json::from_slice::<serde_json::Value>(observation.body()).is_err() =>
        {
            Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedJson)
        }
        crate::http_contract_body_kind::HttpContractBodyKind::Empty
        | crate::http_contract_body_kind::HttpContractBodyKind::Json => Ok(()),
    }
}
