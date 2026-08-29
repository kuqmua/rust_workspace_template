pub fn validate_route_contract_metadata(
    expected: frontend_contract::route_metadata::RouteMetadata,
    observed: frontend_contract::route_metadata::RouteMetadata,
) -> Result<(), crate::route_contract_mismatches::RouteContractMismatches> {
    let mut mismatches = Vec::with_capacity(3usize);
    if expected.method() != observed.method() {
        mismatches.push(
            crate::route_contract_mismatch::RouteContractMismatch::Method {
                expected: expected.method(),
                observed: observed.method(),
            },
        );
    }
    if expected.openapi_operation_id() != observed.openapi_operation_id() {
        mismatches.push(
            crate::route_contract_mismatch::RouteContractMismatch::OpenApiOperationId {
                expected: expected.openapi_operation_id(),
                observed: observed.openapi_operation_id(),
            },
        );
    }
    if expected.path() != observed.path() {
        mismatches.push(
            crate::route_contract_mismatch::RouteContractMismatch::Path {
                expected: expected.path(),
                observed: observed.path(),
            },
        );
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(
            crate::route_contract_mismatches::RouteContractMismatches::from(
                bounded_types::bounded_vec::BoundedVec::from_max_iter(mismatches),
            ),
        )
    }
}
