pub fn validate_route_contract_metadata(
    expected: frontend_contract::domain_types::RouteMetadata,
    observed: frontend_contract::domain_types::RouteMetadata,
) -> Result<(), super::RouteContractMismatches> {
    let mut mismatches = Vec::with_capacity(3usize);
    if expected.method() != observed.method() {
        mismatches.push(super::RouteContractMismatch::Method {
            expected: expected.method(),
            observed: observed.method(),
        });
    }
    if expected.openapi_operation_id() != observed.openapi_operation_id() {
        mismatches.push(super::RouteContractMismatch::OpenApiOperationId {
            expected: expected.openapi_operation_id(),
            observed: observed.openapi_operation_id(),
        });
    }
    if expected.path() != observed.path() {
        mismatches.push(super::RouteContractMismatch::Path {
            expected: expected.path(),
            observed: observed.path(),
        });
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(super::RouteContractMismatches::from(
            bounded_types::domain_types::vector::BoundedVec::from_max_iter(mismatches),
        ))
    }
}
