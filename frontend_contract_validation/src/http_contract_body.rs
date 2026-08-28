#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct HttpContractBody(
    pub(super) bounded_types::BoundedVec<u8, 0, { constants_usize::VALUE_16_777_216 }>,
);

impl TryFrom<Vec<u8>> for HttpContractBody {
    type Error = frontend_contract::FrontendContractBodyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(frontend_contract::FrontendContractBodyError::from)
    }
}
