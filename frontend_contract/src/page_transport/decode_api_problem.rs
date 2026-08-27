#[must_use]
pub fn decode_api_problem(body: &super::TransportBody) -> Option<super::super::ApiProblem> {
    serde_json::from_slice(body.as_ref()).ok()
}
