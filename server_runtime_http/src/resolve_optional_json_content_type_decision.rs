pub const fn resolve_optional_json_content_type_decision(
    optional_json_body_presence: crate::optional_json_body_presence::OptionalJsonBodyPresence,
    optional_json_content_type: crate::optional_json_content_type::OptionalJsonContentType,
) -> crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision {
    match (optional_json_body_presence, optional_json_content_type) {
        (_, crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson)
        | (crate::optional_json_body_presence::OptionalJsonBodyPresence::Empty, crate::optional_json_content_type::OptionalJsonContentType::Missing) => {
            crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::Accept
        }
        _ => crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType,
    }
}
