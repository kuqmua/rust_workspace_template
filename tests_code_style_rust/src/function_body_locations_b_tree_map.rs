#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Default,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(super) struct FunctionBodyLocationsBTreeMap(
    std::collections::BTreeMap<
        crate::function_body_hash::FunctionBodyHash,
        crate::source_text_list::SourceTextList,
    >,
);
