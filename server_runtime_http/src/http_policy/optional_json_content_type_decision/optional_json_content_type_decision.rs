#[must_use]
pub const fn optional_json_content_type_decision(
    body: super::super::OptionalJsonBodyPresence,
    content_type: super::super::OptionalJsonContentType,
) -> super::OptionalJsonContentTypeDecision {
    match (body, content_type) {
        (_, super::super::OptionalJsonContentType::ApplicationJson)
        | (
            super::super::OptionalJsonBodyPresence::Empty,
            super::super::OptionalJsonContentType::Missing,
        ) => super::OptionalJsonContentTypeDecision::Accept,
        _ => super::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType,
    }
}
