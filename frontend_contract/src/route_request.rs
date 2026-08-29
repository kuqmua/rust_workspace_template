#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct RouteRequest<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    body: Route::Request,
}
impl<Route> RouteRequest<Route>
where
    Route: crate::typed_route::TypedRoute,
{
    #[must_use]
    pub const fn body(&self) -> &Route::Request {
        &self.body
    }
}
