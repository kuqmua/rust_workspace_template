#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget,
)]
pub struct TransportBody(
    bounded_types::domain_types::vector::BoundedVec<u8, 0, { constants_usize::VALUE_16_777_216 }>,
);

impl TryFrom<Vec<u8>> for TransportBody {
    type Error = crate::FrontendContractBodyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(crate::FrontendContractBodyError::from)
    }
}
