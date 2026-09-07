#[derive(
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(crate) struct StdIoWriteRef<'writer, Writer: std::io::Write>(&'writer mut Writer);
