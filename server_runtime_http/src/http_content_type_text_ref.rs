#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[accessor(pub(crate))]
pub struct HttpContentTypeTextRef<'value_lt>(Option<&'value_lt str>);
