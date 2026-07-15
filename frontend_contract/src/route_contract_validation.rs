#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteContractMismatch {
    Method {
        expected: crate::ContractStr,
        observed: crate::ContractStr,
    },
    OpenApiOperationId {
        expected: crate::ContractStr,
        observed: crate::ContractStr,
    },
    Path {
        expected: crate::ContractStr,
        observed: crate::ContractStr,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteContractMismatches(Vec<RouteContractMismatch>);

impl AsRef<[RouteContractMismatch]> for RouteContractMismatches {
    fn as_ref(&self) -> &[RouteContractMismatch] {
        self.0.as_slice()
    }
}

pub fn validate_route_contract_metadata(
    expected: crate::RouteMetadata,
    observed: crate::RouteMetadata,
) -> Result<(), RouteContractMismatches> {
    let mut mismatches = Vec::with_capacity(3usize);
    if expected.method() != observed.method() {
        mismatches.push(RouteContractMismatch::Method {
            expected: expected.method(),
            observed: observed.method(),
        });
    }
    if expected.openapi_operation_id() != observed.openapi_operation_id() {
        mismatches.push(RouteContractMismatch::OpenApiOperationId {
            expected: expected.openapi_operation_id(),
            observed: observed.openapi_operation_id(),
        });
    }
    if expected.path() != observed.path() {
        mismatches.push(RouteContractMismatch::Path {
            expected: expected.path(),
            observed: observed.path(),
        });
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(RouteContractMismatches(mismatches))
    }
}

pub fn validate_typed_route_contract<Route>(
    observed: crate::RouteMetadata,
) -> Result<(), RouteContractMismatches>
where
    Route: crate::TypedRoute,
{
    validate_route_contract_metadata(Route::metadata(), observed)
}

#[cfg(test)]
mod tests {
    struct ReadRoute;
    impl crate::TypedRoute for ReadRoute {
        type Request = ();
        type Response = ();
        type Transport = crate::PublicTransport;

        fn metadata() -> crate::RouteMetadata {
            metadata(
                str_constants::GET,
                str_constants::ROUTE_READ,
                str_constants::ROUTE,
            )
        }
    }

    fn metadata(
        method: &'static str,
        operation_id: &'static str,
        path: &'static str,
    ) -> crate::RouteMetadata {
        crate::RouteMetadata::new(method.into(), operation_id.into(), path.into())
    }

    #[test]
    fn equal_metadata_satisfies_contract() {
        let metadata = metadata(
            str_constants::GET,
            str_constants::ROUTE_READ,
            str_constants::ROUTE,
        );
        assert_eq!(
            super::validate_route_contract_metadata(metadata, metadata),
            Ok(())
        );
    }

    #[test]
    fn typed_route_is_the_contract_source_of_truth() {
        assert_eq!(
            super::validate_typed_route_contract::<ReadRoute>(metadata(
                str_constants::GET,
                str_constants::ROUTE_READ,
                str_constants::ROUTE,
            )),
            Ok(())
        );
    }

    #[test]
    fn every_metadata_difference_is_reported() {
        let expected = metadata(
            str_constants::GET,
            str_constants::ROUTE_READ,
            str_constants::ROUTE,
        );
        let observed = metadata(
            str_constants::POST,
            str_constants::ADMIN_ALT,
            str_constants::NOT_AN_API_ROUTE,
        );
        let mismatches = super::validate_route_contract_metadata(expected, observed)
            .expect_err(str_constants::VALUE_5067F83C);
        assert_eq!(mismatches.as_ref().len(), 3usize);
        assert!(matches!(
            mismatches.as_ref().first(),
            Some(super::RouteContractMismatch::Method { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(1usize),
            Some(super::RouteContractMismatch::OpenApiOperationId { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(2usize),
            Some(super::RouteContractMismatch::Path { .. })
        ));
    }
}
