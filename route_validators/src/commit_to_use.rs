#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_to_err_string_as_ref_str::ToErrStringAsRefStr,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct CommitToUse(&'static str);

impl CommitToUse {
    #[cfg(test)]
    pub(crate) const fn as_str(self) -> &'static str {
        self.0
    }
}
