#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum OpenApiSchemaMismatch {
    AdditionalProperty,
    AnyOf,
    Const,
    Enum,
    MissingReference,
    OneOf,
    RequiredProperty,
    Type,
}
