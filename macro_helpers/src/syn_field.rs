#![allow(
    clippy::module_name_repetitions,
    reason = "nested modules and wrapper types retain exact owner-derived names required by the module naming policy"
)]

#[path = "syn_field_identifier.rs"]
pub mod syn_field_identifier;
#[path = "syn_field_type.rs"]
pub mod syn_field_type;
#[path = "syn_field_vis.rs"]
pub mod syn_field_vis;

#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub struct SynField {
    pub identifier: syn_field_identifier::SynFieldIdentifier,
    pub type0: syn_field_type::SynFieldType,
    pub vis: syn_field_vis::SynFieldVis,
}
