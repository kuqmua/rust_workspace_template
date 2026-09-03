#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct ProjectGitCommitLinkRef(&'static str);
