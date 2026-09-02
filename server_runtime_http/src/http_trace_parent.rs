#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::AsRefStr,
)]
pub struct HttpTraceParent(String);

impl TryFrom<String> for HttpTraceParent {
    type Error = crate::http_trace_parent_error::HttpTraceParentError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let bytes = string.as_bytes();
        if bytes.len() != 55usize
            || bytes.get(constants_usize::ZERO..3usize) != Some(b"00-")
            || bytes.get(35usize) != Some(&b'-')
            || bytes.get(52usize) != Some(&b'-')
            || !bytes
                .iter()
                .enumerate()
                .filter(|(index, _byte)| !matches!(index, 2usize | 35usize | 52usize))
                .all(|(_index, byte)| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(Self::Error::Format);
        }
        let Some(trace_id) = bytes.get(3usize..35usize) else {
            return Err(Self::Error::Format);
        };
        let Some(parent_id) = bytes.get(36usize..52usize) else {
            return Err(Self::Error::Format);
        };
        if trace_id.iter().all(|byte| *byte == b'0') {
            return Err(Self::Error::ZeroTraceId);
        }
        if parent_id.iter().all(|byte| *byte == b'0') {
            return Err(Self::Error::ZeroParentId);
        }
        Ok(Self(string))
    }
}
