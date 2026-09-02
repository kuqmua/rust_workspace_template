#[derive(
    Debug,
    Clone,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct SynField {
    identifier: crate::syn_field_identifier::SynFieldIdentifier,
    field_type: crate::syn_field_type::SynFieldType,
    visibility: crate::syn_field_vis::SynFieldVis,
}
