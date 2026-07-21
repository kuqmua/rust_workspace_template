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

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefTarget)]
pub struct RouteContractMismatches(Vec<RouteContractMismatch>);

#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct HttpContractStatus(u16);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContractBody(Vec<u8>);
impl TryFrom<Vec<u8>> for HttpContractBody {
    type Error = crate::FrontendContractBodyError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > crate::FRONTEND_CONTRACT_BODY_MAX_BYTES {
            Err(crate::FrontendContractBodyError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpContractBodyKind {
    Empty,
    Json,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpContractObservation {
    body: HttpContractBody,
    metadata: crate::RouteMetadata,
    status: HttpContractStatus,
}
impl HttpContractObservation {
    #[must_use]
    pub const fn new(
        metadata: crate::RouteMetadata,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpContractExpectation {
    body_kind: HttpContractBodyKind,
    metadata: crate::RouteMetadata,
    status: HttpContractStatus,
}
impl HttpContractExpectation {
    #[must_use]
    pub const fn new(
        metadata: crate::RouteMetadata,
        status: HttpContractStatus,
        body_kind: HttpContractBodyKind,
    ) -> Self {
        Self {
            body_kind,
            metadata,
            status,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
    Send: FnOnce(crate::RouteMetadata) -> SendFuture,
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
                crate::RouteMethod::Get,
                str_constants::ROUTE_READ,
                str_constants::ROUTE,
            )
        }
    }

    fn metadata(
        method: crate::RouteMethod,
        operation_id: &'static str,
        path: &'static str,
    ) -> crate::RouteMetadata {
        crate::RouteMetadata::new(method, operation_id.into(), path.into())
    }

    #[test]
    fn equal_metadata_satisfies_contract() {
        let metadata = metadata(
            crate::RouteMethod::Get,
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
                crate::RouteMethod::Get,
                str_constants::ROUTE_READ,
                str_constants::ROUTE,
            )),
            Ok(())
        );
    }

    #[test]
    fn http_fixture_checks_status_and_json_body() {
        let metadata = metadata(
            crate::RouteMethod::Get,
            str_constants::ROUTE_READ,
            str_constants::ROUTE,
        );
        let result = futures::executor::block_on(super::run_http_contract_fixture(
            super::HttpContractExpectation::new(
                metadata,
                200u16.into(),
                super::HttpContractBodyKind::Json,
            ),
            async |observed_metadata| {
                super::HttpContractObservation::new(
                    observed_metadata,
                    200u16.into(),
                    super::HttpContractBody::try_from(br#"{"ok":true}"#.to_vec())
                        .expect("08bddb5e"),
                )
            },
        ));
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn every_metadata_difference_is_reported() {
        let expected = metadata(
            crate::RouteMethod::Get,
            str_constants::ROUTE_READ,
            str_constants::ROUTE,
        );
        let observed = metadata(
            crate::RouteMethod::Post,
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
