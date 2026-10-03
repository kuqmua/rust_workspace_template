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

#[cfg(test)]
mod tests {
    #[test]
    fn test_optional_json_media_type_decision_covers_every_body_and_content_type() {
        assert!([
            (crate::optional_json_body_presence::OptionalJsonBodyPresence::Empty, crate::optional_json_content_type::OptionalJsonContentType::Missing, crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::Accept),
            (crate::optional_json_body_presence::OptionalJsonBodyPresence::NonEmpty, crate::optional_json_content_type::OptionalJsonContentType::Missing, crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType),
            (crate::optional_json_body_presence::OptionalJsonBodyPresence::Empty, crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson, crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::Accept),
            (crate::optional_json_body_presence::OptionalJsonBodyPresence::NonEmpty, crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson, crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::Accept),
            (crate::optional_json_body_presence::OptionalJsonBodyPresence::Empty, crate::optional_json_content_type::OptionalJsonContentType::NonJson, crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType),
            (crate::optional_json_body_presence::OptionalJsonBodyPresence::NonEmpty, crate::optional_json_content_type::OptionalJsonContentType::NonJson, crate::optional_json_content_type_decision::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType),
        ].into_iter().all(|(presence, content_type, expected)| {
            crate::resolve_optional_json_content_type_decision::resolve_optional_json_content_type_decision(presence, content_type) == expected
        }));
    }
}
