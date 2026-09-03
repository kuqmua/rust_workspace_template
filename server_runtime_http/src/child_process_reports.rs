#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct ChildProcessReports(
    bounded_types::bounded_vec::BoundedVec<
        crate::child_process_report::ChildProcessReport,
        0,
        { usize::MAX },
    >,
);
