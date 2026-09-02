#[must_use]
pub fn required_test_categories(
    route_test_capabilities: crate::route_test_capabilities::RouteTestCapabilities,
) -> crate::route_test_categories::RouteTestCategories {
    let categories = [
        Some(crate::route_test_category::RouteTestCategory::FixtureHook),
        Some(crate::route_test_category::RouteTestCategory::Metadata),
        (*route_test_capabilities.get_database()
            == crate::route_database_usage::RouteDatabaseUsage::Database)
            .then_some(crate::route_test_category::RouteTestCategory::DatabaseFixture),
        (*route_test_capabilities.get_json_body()
            == crate::route_json_body_usage::RouteJsonBodyUsage::JsonBody)
            .then_some(crate::route_test_category::RouteTestCategory::JsonRoundTrip),
        (*route_test_capabilities.get_response()
            == crate::route_response_kind::RouteResponseKind::Streaming)
            .then_some(crate::route_test_category::RouteTestCategory::StreamingResponse),
    ]
    .into_iter()
    .flatten();
    crate::route_test_categories::RouteTestCategories::from(
        bounded_types::bounded_vec::BoundedVec::from_max_iter(categories),
    )
}
