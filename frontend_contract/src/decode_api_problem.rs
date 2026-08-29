#[must_use]
pub fn decode_api_problem(
    body: &crate::transport_body::TransportBody,
) -> Option<crate::api_problem::ApiProblem> {
    serde_json::from_slice(body.as_ref()).ok()
}
