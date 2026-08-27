#[path = "optional_json_content_type_decision/optional_json_content_type_decision.rs"]
mod optional_json_content_type_decision;

pub use optional_json_content_type_decision::optional_json_content_type_decision;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptionalJsonContentTypeDecision {
    Accept,
    RejectUnsupportedMediaType,
}
