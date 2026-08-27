pub async fn run_http_contract_fixture<Send, SendFuture>(
    expectation: super::HttpContractExpectation,
    send: Send,
) -> Result<(), super::HttpContractMismatch>
where
    Send: FnOnce(frontend_contract::domain_types::RouteMetadata) -> SendFuture,
    SendFuture: Future<Output = super::HttpContractObservation>,
{
    let observation = send(expectation.metadata).await;
    super::validate_route_contract_metadata(expectation.metadata, observation.metadata)
        .map_err(super::HttpContractMismatch::Metadata)?;
    if expectation.status != observation.status {
        return Err(super::HttpContractMismatch::Status {
            expected: expectation.status,
            observed: observation.status,
        });
    }
    match expectation.body_kind {
        super::HttpContractBodyKind::Empty if !observation.body.0.is_empty() => {
            Err(super::HttpContractMismatch::BodyExpectedEmpty)
        }
        super::HttpContractBodyKind::Json
            if serde_json::from_slice::<serde_json::Value>(&observation.body.0).is_err() =>
        {
            Err(super::HttpContractMismatch::BodyExpectedJson)
        }
        super::HttpContractBodyKind::Empty | super::HttpContractBodyKind::Json => Ok(()),
    }
}
