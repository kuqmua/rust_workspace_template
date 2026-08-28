#[must_use]
pub fn decode_api_problem(body: &super::TransportBody) -> Option<crate::ApiProblem> {
    serde_json::from_slice(body.as_ref()).ok()
}
