#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
pub(super) struct AdminOpenApiVecPhantomData<T>(std::marker::PhantomData<T>);
