#[must_use]
pub fn missing_required_test_categories(
    route_test_capabilities: crate::route_test_capabilities::RouteTestCapabilities,
    available_categories: &[crate::route_test_category::RouteTestCategory],
) -> crate::route_test_categories::RouteTestCategories {
    crate::route_test_categories::RouteTestCategories::from(
        bounded_types::bounded_vec::BoundedVec::from_max_iter(
            bounded_types::bounded_vec::BoundedVec::from(
                crate::required_test_categories::required_test_categories(route_test_capabilities),
            )
            .into_iter()
            .filter(|category| !available_categories.contains(category)),
        ),
    )
}
