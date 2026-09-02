#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype::ToErrStringAsRefStr,
    proc_macro_newtype::FromInner,
)]
pub struct CommitToUse(&'static str);

impl CommitToUse {
    #[cfg(test)]
    pub(crate) const fn as_str(self) -> &'static str {
        self.0
    }
}
