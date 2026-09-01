#[derive(generate_accessor::Getters)]
#[getters(bare)]
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
