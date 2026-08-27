use super::{
    RouteDatabaseUsage, RouteJsonBodyUsage, RouteResponseKind, RouteTestCapabilities,
    RouteTestCategories, RouteTestCategory,
};

#[must_use]
pub fn required_test_categories(capabilities: RouteTestCapabilities) -> RouteTestCategories {
    let categories = [
        Some(RouteTestCategory::FixtureHook),
        Some(RouteTestCategory::Metadata),
        (capabilities.database == RouteDatabaseUsage::Database)
            .then_some(RouteTestCategory::DatabaseFixture),
        (capabilities.json_body == RouteJsonBodyUsage::JsonBody)
            .then_some(RouteTestCategory::JsonRoundTrip),
        (capabilities.response == RouteResponseKind::Streaming)
            .then_some(RouteTestCategory::StreamingResponse),
    ]
    .into_iter()
    .flatten();
    RouteTestCategories::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(categories),
    )
}
