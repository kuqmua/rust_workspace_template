#![allow(
    clippy::module_name_repetitions,
    reason = "nested modules and wrapper types retain exact owner-derived names required by the module naming policy"
)]

#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub struct SynField {
    pub identifier: crate::domain_types::syn_field::SynFieldIdentifier,
    pub type0: crate::domain_types::syn_field::SynFieldType,
    pub vis: crate::domain_types::syn_field::SynFieldVis,
}
