#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::DerefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct HeaderStrRef<'header_str_lt>(&'header_str_lt str);
