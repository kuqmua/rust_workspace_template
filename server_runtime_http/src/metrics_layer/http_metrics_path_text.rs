#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    newtype::BorrowStr,
)]
pub(super) struct HttpMetricsPathText(pub(super) String);

impl TryFrom<String> for HttpMetricsPathText {
    type Error = super::HttpMetricsPathTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > constants_usize::VALUE_8_192 {
            Err(super::HttpMetricsPathTextError)
        } else {
            Ok(Self(value))
        }
    }
}
