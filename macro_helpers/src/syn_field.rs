#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub struct SynField {
    pub identifier: crate::syn_field_identifier::SynFieldIdentifier,
    pub type0: crate::syn_field_type::SynFieldType,
    pub vis: crate::syn_field_vis::SynFieldVis,
}
