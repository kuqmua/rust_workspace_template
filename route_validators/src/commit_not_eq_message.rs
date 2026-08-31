#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::ToErrStringAsRefStr,
    newtype::FromInner,
)]
pub struct CommitNotEqMessage(&'static str);

impl CommitNotEqMessage {
    #[cfg(test)]
    pub(crate) const fn as_str(self) -> &'static str {
        self.0
    }
}
