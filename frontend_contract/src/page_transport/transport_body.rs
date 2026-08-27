#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget,
)]
pub struct TransportBody(
    bounded_types::domain_types::vector::BoundedVec<
        u8,
        0,
        { super::super::FRONTEND_CONTRACT_BODY_MAX_BYTES },
    >,
);

impl TryFrom<Vec<u8>> for TransportBody {
    type Error = super::super::FrontendContractBodyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(super::super::FrontendContractBodyError::from)
    }
}
