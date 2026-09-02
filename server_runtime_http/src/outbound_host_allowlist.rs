#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundHostAllowlist(
    bounded_types::bounded_vec::BoundedVec<
        crate::outbound_allowed_host::OutboundAllowedHost,
        1,
        64,
    >,
);

impl TryFrom<Vec<crate::outbound_allowed_host::OutboundAllowedHost>> for OutboundHostAllowlist {
    type Error = crate::outbound_host_allowlist_error::OutboundHostAllowlistError;

    fn try_from(
        mut vec: Vec<crate::outbound_allowed_host::OutboundAllowedHost>,
    ) -> Result<Self, Self::Error> {
        vec.sort();
        vec.dedup();
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self)
            .map_err(|error| match error {
                bounded_types::bounded_value_error::BoundedValueError::BelowMin { .. } => {
                    crate::outbound_host_allowlist_error::OutboundHostAllowlistError::Empty
                }
                bounded_types::bounded_value_error::BoundedValueError::AboveMax { .. }
                | bounded_types::bounded_value_error::BoundedValueError::InvalidBounds { .. } => {
                    crate::outbound_host_allowlist_error::OutboundHostAllowlistError::TooManyHosts
                }
            })
    }
}

impl OutboundHostAllowlist {
    pub fn validate(
        &self,
        reqwest_outbound_url: &crate::reqwest_outbound_url::ReqwestOutboundUrl,
    ) -> Result<(), crate::outbound_host_allowlist_error::OutboundHostAllowlistError> {
        let host = reqwest_outbound_url
            .host_str()
            .ok_or(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::InvalidHost)?;
        if self
            .0
            .binary_search_by(|allowed| allowed.as_str().cmp(host))
            .is_ok()
        {
            Ok(())
        } else {
            Err(crate::outbound_host_allowlist_error::OutboundHostAllowlistError::HostNotAllowed)
        }
    }
}
