#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct OpenApiSchemaReferenceRef<'reference_lt>(&'reference_lt str);
