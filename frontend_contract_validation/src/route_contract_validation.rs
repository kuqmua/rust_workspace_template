#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteContractMismatch {
    Method {
        expected: frontend_contract::ContractStr,
        observed: frontend_contract::ContractStr,
    },
    OpenApiOperationId {
        expected: frontend_contract::ContractStr,
        observed: frontend_contract::ContractStr,
    },
    Path {
        expected: frontend_contract::ContractStr,
        observed: frontend_contract::ContractStr,
    },
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct RouteContractMismatches(
    bounded_types::BoundedVec<RouteContractMismatch, 0, { usize::MAX }>,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, newtype::TryFrom,
)]
#[try_from(
    error = frontend_contract::HttpStatusTryFromU16Error,
    validator = HttpContractStatus::validate
)]
pub struct HttpContractStatus(u16);
impl HttpContractStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    fn validate(value: &u16) -> Result<(), frontend_contract::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(frontend_contract::HttpStatusTryFromU16Error)
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractBody(
    bounded_types::BoundedVec<u8, 0, { frontend_contract::FRONTEND_CONTRACT_BODY_MAX_BYTES }>,
);
impl TryFrom<Vec<u8>> for HttpContractBody {
    type Error = frontend_contract::FrontendContractBodyError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(frontend_contract::FrontendContractBodyError::from)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpContractBodyKind {
    Empty,
    Json,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractObservation {
    body: HttpContractBody,
    metadata: frontend_contract::RouteMetadata,
    status: HttpContractStatus,
}
impl HttpContractObservation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::RouteMetadata,
        status: HttpContractStatus,
        body: HttpContractBody,
    ) -> Self {
        Self {
            body,
            metadata,
            status,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct HttpContractExpectation {
    metadata: frontend_contract::RouteMetadata,
    status: HttpContractStatus,
    body_kind: HttpContractBodyKind,
}
impl HttpContractExpectation {
    #[must_use]
    pub const fn new(
        metadata: frontend_contract::RouteMetadata,
        status: HttpContractStatus,
        body_kind: HttpContractBodyKind,
    ) -> Self {
        Self {
            metadata,
            status,
            body_kind,
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum HttpContractMismatch {
    BodyExpectedEmpty,
    BodyExpectedJson,
    Metadata(RouteContractMismatches),
    Status {
        expected: HttpContractStatus,
        observed: HttpContractStatus,
    },
}

pub async fn run_http_contract_fixture<Send, SendFuture>(
    expectation: HttpContractExpectation,
    send: Send,
) -> Result<(), HttpContractMismatch>
where
    Send: FnOnce(frontend_contract::RouteMetadata) -> SendFuture,
    SendFuture: Future<Output = HttpContractObservation>,
{
    let observation = send(expectation.metadata).await;
    validate_route_contract_metadata(expectation.metadata, observation.metadata)
        .map_err(HttpContractMismatch::Metadata)?;
    if expectation.status != observation.status {
        return Err(HttpContractMismatch::Status {
            expected: expectation.status,
            observed: observation.status,
        });
    }
    match expectation.body_kind {
        HttpContractBodyKind::Empty if !observation.body.0.is_empty() => {
            Err(HttpContractMismatch::BodyExpectedEmpty)
        }
        HttpContractBodyKind::Json
            if serde_json::from_slice::<serde_json::Value>(&observation.body.0).is_err() =>
        {
            Err(HttpContractMismatch::BodyExpectedJson)
        }
        HttpContractBodyKind::Empty | HttpContractBodyKind::Json => Ok(()),
    }
}

pub fn validate_route_contract_metadata(
    expected: frontend_contract::RouteMetadata,
    observed: frontend_contract::RouteMetadata,
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
        Err(RouteContractMismatches::from(
            bounded_types::BoundedVec::from_max_iter(mismatches),
        ))
    }
}

pub fn validate_typed_route_contract<Route>(
    observed: frontend_contract::RouteMetadata,
) -> Result<(), RouteContractMismatches>
where
    Route: frontend_contract::TypedRoute,
{
    validate_route_contract_metadata(Route::metadata(), observed)
}

#[cfg(test)]
mod tests {
    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct ReadRoute;
    impl frontend_contract::TypedRoute for ReadRoute {
        type Request = ();
        type Response = ();
        type Transport = frontend_contract::PublicTransport;

        fn metadata() -> frontend_contract::RouteMetadata {
            metadata(
                frontend_contract::RouteMethod::Get,
                str_constants::ROUTE_READ,
                str_constants::ROUTE,
            )
        }
    }

    fn metadata(
        method: frontend_contract::RouteMethod,
        operation_id: &'static str,
        path: &'static str,
    ) -> frontend_contract::RouteMetadata {
        frontend_contract::RouteMetadata::new(method, operation_id.into(), path.into())
    }

    #[test]
    fn equal_metadata_satisfies_contract() {
        let metadata = metadata(
            frontend_contract::RouteMethod::Get,
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
                frontend_contract::RouteMethod::Get,
                str_constants::ROUTE_READ,
                str_constants::ROUTE,
            )),
            Ok(())
        );
    }

    #[test]
    fn http_fixture_checks_status_and_json_body() {
        let metadata = metadata(
            frontend_contract::RouteMethod::Get,
            str_constants::ROUTE_READ,
            str_constants::ROUTE,
        );
        let result = futures::executor::block_on(super::run_http_contract_fixture(
            super::HttpContractExpectation::new(
                metadata,
                super::HttpContractStatus::try_from(200u16).expect(
                    "a76c9e6b http_fixture_checks_status_and_json_body invariant must hold",
                ),
                super::HttpContractBodyKind::Json,
            ),
            async |observed_metadata| {
                super::HttpContractObservation::new(
                    observed_metadata,
                    super::HttpContractStatus::try_from(200u16).expect(
                        "d0abdccc http_fixture_checks_status_and_json_body invariant must hold",
                    ),
                    super::HttpContractBody::try_from(br#"{"ok":true}"#.to_vec()).expect(
                        "08bddb5e http_fixture_checks_status_and_json_body invariant must hold",
                    ),
                )
            },
        ));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn every_metadata_difference_is_reported() {
        let expected = metadata(
            frontend_contract::RouteMethod::Get,
            str_constants::ROUTE_READ,
            str_constants::ROUTE,
        );
        let observed = metadata(
            frontend_contract::RouteMethod::Post,
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
            mismatches.as_ref().get(usize_constants::ONE),
            Some(super::RouteContractMismatch::OpenApiOperationId { .. })
        ));
        assert!(matches!(
            mismatches.as_ref().get(2usize),
            Some(super::RouteContractMismatch::Path { .. })
        ));
    }
}
