#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    Decode(crate::FormValueError),
    Encode(crate::FormValueError),
    Problem(crate::ApiProblem),
    Status {
        actual: super::TransportStatus,
        expected: super::TransportStatus,
    },
    Transport(super::TransportError),
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
