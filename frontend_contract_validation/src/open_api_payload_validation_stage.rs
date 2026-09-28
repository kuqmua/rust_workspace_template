#[derive(Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum OpenApiPayloadValidationStage {
    AdditionalProperties,
    AdditionalPropertyChild,
    AllOf,
    AllOfChild,
    AnyOf,
    AnyOfChild,
    CheckValue,
    Complete,
    ItemChild,
    Items,
    OneOf,
    OneOfChild,
    Properties,
    PropertyChild,
    Resolve,
}
