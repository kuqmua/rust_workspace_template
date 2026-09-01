#[derive(
    Debug,
    Clone,
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub struct SynField {
    identifier: crate::syn_field_identifier::SynFieldIdentifier,
    field_type: crate::syn_field_type::SynFieldType,
    visibility: crate::syn_field_vis::SynFieldVis,
}
