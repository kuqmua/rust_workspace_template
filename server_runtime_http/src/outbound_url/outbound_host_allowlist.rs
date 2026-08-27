#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundHostAllowlist(
    bounded_types::domain_types::vector::BoundedVec<super::OutboundAllowedHost, 1, 64>,
);

impl TryFrom<Vec<super::OutboundAllowedHost>> for OutboundHostAllowlist {
    type Error = super::OutboundHostAllowlistError;

    fn try_from(mut value: Vec<super::OutboundAllowedHost>) -> Result<Self, Self::Error> {
        value.sort_unstable();
        value.dedup();
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|error| match error {
                bounded_types::domain_types::BoundedValueError::BelowMin { .. } => {
                    super::OutboundHostAllowlistError::Empty
                }
                bounded_types::domain_types::BoundedValueError::AboveMax { .. }
                | bounded_types::domain_types::BoundedValueError::InvalidBounds { .. } => {
                    super::OutboundHostAllowlistError::TooManyHosts
                }
            })
    }
}

impl OutboundHostAllowlist {
    pub fn validate(
        &self,
        url: &super::ReqwestOutboundUrl,
    ) -> Result<(), super::OutboundHostAllowlistError> {
        let host = url
            .0
            .host_str()
            .ok_or(super::OutboundHostAllowlistError::InvalidHost)?;
        if self
            .0
            .binary_search_by(|allowed| allowed.0.as_str().cmp(host))
            .is_ok()
        {
            Ok(())
        } else {
            Err(super::OutboundHostAllowlistError::HostNotAllowed)
        }
    }
}
