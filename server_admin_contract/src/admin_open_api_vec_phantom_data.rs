#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct AdminOpenApiVecPhantomData<T>(std::marker::PhantomData<T>);
