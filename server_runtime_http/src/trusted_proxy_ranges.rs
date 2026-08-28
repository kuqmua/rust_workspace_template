#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq)]
pub struct TrustedProxyRanges(
    bounded_types::BoundedVec<super::TrustedProxyRange, 0, { constants_usize::VALUE_128 }>,
);

impl TryFrom<Vec<super::TrustedProxyRange>> for TrustedProxyRanges {
    type Error = super::TrustedProxyRangesError;

    fn try_from(value: Vec<super::TrustedProxyRange>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(super::TrustedProxyRangesError::from)
    }
}

impl TrustedProxyRanges {
    pub(super) fn contains(&self, candidate: super::ParsedIpAddr) -> super::StdRangeContains {
        super::StdRangeContains::from(self.0.iter().any(|range| range.contains(candidate).get()))
    }
}
