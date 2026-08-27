use super::{
    RouteTestCapabilities, RouteTestCategories, RouteTestCategory, required_test_categories,
};

#[must_use]
pub fn missing_required_test_categories(
    capabilities: RouteTestCapabilities,
    available_categories: &[RouteTestCategory],
) -> RouteTestCategories {
    RouteTestCategories::from(
        bounded_types::domain_types::vector::BoundedVec::from_max_iter(
            bounded_types::domain_types::vector::BoundedVec::from(required_test_categories(
                capabilities,
            ))
            .into_iter()
            .filter(|category| !available_categories.contains(category)),
        ),
    )
}
