#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct RouteResponse<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    body: Route::Response,
}
impl<Route> RouteResponse<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    #[must_use]
    pub const fn body(&self) -> &Route::Response {
        &self.body
    }
}
