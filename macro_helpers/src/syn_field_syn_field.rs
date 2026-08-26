#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub struct SynField {
    pub identifier: super::syn_field_identifier::SynFieldIdentifier,
    pub type0: super::syn_field_type::SynFieldType,
    pub vis: super::syn_field_vis::SynFieldVis,
}
