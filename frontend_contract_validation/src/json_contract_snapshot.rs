#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct JsonContractSnapshot(
    bounded_types::bounded_string::BoundedString<0usize, 1_048_576usize, false>,
);
impl TryFrom<String> for JsonContractSnapshot {
    type Error = crate::json_contract_snapshot_error::JsonContractSnapshotError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(Self::Error::TooLong)
    }
}
