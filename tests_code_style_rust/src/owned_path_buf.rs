#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_borrow_path::BorrowPath,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct OwnedPathBuf(std::path::PathBuf);
