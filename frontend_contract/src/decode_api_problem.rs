#[must_use]
pub fn decode_api_problem(
    transport_body: &crate::transport_body::TransportBody,
) -> Option<crate::api_problem::ApiProblem> {
    serde_json::from_slice(transport_body.as_ref()).ok()
}
