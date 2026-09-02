#[must_use]
pub fn classify_optional_json_content_type(
    http_content_type_text_ref: crate::http_content_type_text_ref::HttpContentTypeTextRef<'_>,
) -> crate::optional_json_content_type::OptionalJsonContentType {
    let Some(text) = http_content_type_text_ref
        .get()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    else {
        return crate::optional_json_content_type::OptionalJsonContentType::Missing;
    };
    if text.len() > constants_usize::VALUE_4_096 {
        return crate::optional_json_content_type::OptionalJsonContentType::NonJson;
    }
    if text
        .parse::<mime::Mime>()
        .is_ok_and(|media_type| media_type.essence_str() == constants_str::APPLICATION_JSON)
    {
        crate::optional_json_content_type::OptionalJsonContentType::ApplicationJson
    } else {
        crate::optional_json_content_type::OptionalJsonContentType::NonJson
    }
}
