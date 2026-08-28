use super::{OptionalJsonBodyPresence, OptionalJsonContentType, OptionalJsonContentTypeDecision};

pub const fn resolve_optional_json_content_type_decision(
    body: OptionalJsonBodyPresence,
    content_type: OptionalJsonContentType,
) -> OptionalJsonContentTypeDecision {
    match (body, content_type) {
        (_, OptionalJsonContentType::ApplicationJson)
        | (OptionalJsonBodyPresence::Empty, OptionalJsonContentType::Missing) => {
            OptionalJsonContentTypeDecision::Accept
        }
        _ => OptionalJsonContentTypeDecision::RejectUnsupportedMediaType,
    }
}
