#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefInner,
)]
pub struct HttpContractBody(
    bounded_types::bounded_vec::BoundedVec<u8, 0, { constants_usize::VALUE_16_777_216 }>,
);

impl TryFrom<Vec<u8>> for HttpContractBody {
    type Error = frontend_contract::frontend_contract_body_error::FrontendContractBodyError;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self)
            .map_err(
                frontend_contract::frontend_contract_body_error::FrontendContractBodyError::from,
            )
    }
}
