#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::DerefInner,
)]
pub struct HttpContractBody(
    bounded_types::bounded_vec::BoundedVec<u8, 0, { constants_usize::VALUE_16_777_216 }>,
);

impl TryFrom<Vec<u8>> for HttpContractBody {
    type Error = frontend_contract::frontend_contract_body_error::FrontendContractBodyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(
                frontend_contract::frontend_contract_body_error::FrontendContractBodyError::from,
            )
    }
}
