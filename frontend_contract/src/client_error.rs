#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    Decode(crate::form_value_error::FormValueError),
    Encode(crate::form_value_error::FormValueError),
    Problem(crate::api_problem::ApiProblem),
    Status {
        actual: crate::transport_status::TransportStatus,
        expected: crate::transport_status::TransportStatus,
    },
    Transport(crate::transport_error::TransportError),
    UnexpectedResponse,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(value) => write!(f, "failed to decode response: {value}"),
            Self::Encode(value) => write!(f, "failed to encode request: {value}"),
            Self::Problem(value) => value.detail().as_ref().fmt(f),
            Self::Status { actual, expected } => {
                write!(f, "expected HTTP {expected}, received HTTP {actual}")
            }
            Self::Transport(value) => write!(f, "transport failed: {value}"),
            Self::UnexpectedResponse => {
                f.write_str(constants_str::SERVER_RETURNED_AN_ERROR_RESPONSE)
            }
        }
    }
}
