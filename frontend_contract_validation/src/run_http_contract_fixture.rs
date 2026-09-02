pub async fn run_http_contract_fixture<Send, SendFuture>(
    http_contract_expectation: crate::http_contract_expectation::HttpContractExpectation,
    send: Send,
) -> Result<(), crate::http_contract_mismatch::HttpContractMismatch>
where
    Send: FnOnce(frontend_contract::route_metadata::RouteMetadata) -> SendFuture,
    SendFuture: Future<Output = crate::http_contract_observation::HttpContractObservation>,
{
    let (metadata, status, body_kind) = http_contract_expectation.parts();
    let observation = send(metadata).await;
    let (body, observed_metadata, observed_status) = observation.parts();
    crate::validate_route_contract_metadata::validate_route_contract_metadata(
        metadata,
        observed_metadata,
    )
    .map_err(crate::http_contract_mismatch::HttpContractMismatch::Metadata)?;
    if status != observed_status {
        return Err(
            crate::http_contract_mismatch::HttpContractMismatch::Status {
                expected: status,
                observed: observed_status,
            },
        );
    }
    match body_kind {
        crate::http_contract_body_kind::HttpContractBodyKind::Empty if !body.is_empty() => {
            Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedEmpty)
        }
        crate::http_contract_body_kind::HttpContractBodyKind::Json
            if serde_json::from_slice::<serde_json::Value>(body).is_err() =>
        {
            Err(crate::http_contract_mismatch::HttpContractMismatch::BodyExpectedJson)
        }
        crate::http_contract_body_kind::HttpContractBodyKind::Empty
        | crate::http_contract_body_kind::HttpContractBodyKind::Json => Ok(()),
    }
}
