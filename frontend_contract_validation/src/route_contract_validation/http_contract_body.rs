#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractBody(
    pub(super)  bounded_types::domain_types::vector::BoundedVec<
        u8,
        0,
        { frontend_contract::domain_types::FRONTEND_CONTRACT_BODY_MAX_BYTES },
    >,
);

impl TryFrom<Vec<u8>> for HttpContractBody {
    type Error = frontend_contract::domain_types::FrontendContractBodyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(frontend_contract::domain_types::FrontendContractBodyError::from)
    }
}
