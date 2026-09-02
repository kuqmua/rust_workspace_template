#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default, Eq, PartialEq,
)]
pub struct TrustedProxyRanges(
    bounded_types::bounded_vec::BoundedVec<
        crate::trusted_proxy_range::TrustedProxyRange,
        0,
        { constants_usize::VALUE_128 },
    >,
);

impl TryFrom<Vec<crate::trusted_proxy_range::TrustedProxyRange>> for TrustedProxyRanges {
    type Error = crate::trusted_proxy_ranges_error::TrustedProxyRangesError;

    fn try_from(
        value: Vec<crate::trusted_proxy_range::TrustedProxyRange>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(crate::trusted_proxy_ranges_error::TrustedProxyRangesError::from)
    }
}

impl TrustedProxyRanges {
    pub(super) fn contains(
        &self,
        candidate: crate::parsed_ip_addr::ParsedIpAddr,
    ) -> crate::std_range_contains::StdRangeContains {
        crate::std_range_contains::StdRangeContains::from(
            self.0.iter().any(|range| range.contains(candidate).get()),
        )
    }
}
