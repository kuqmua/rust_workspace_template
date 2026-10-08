#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_deref_inner::DerefInner,
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_http_contract_body_preserves_exact_byte_bounds_and_content() {
        assert!(
            crate::http_contract_body::HttpContractBody::try_from(Vec::new())
                .is_ok_and(|body| body.as_slice().is_empty())
        );
        let maximum = constants_usize::VALUE_16_777_216;
        assert!(
            crate::http_contract_body::HttpContractBody::try_from(vec![255u8; maximum]).is_ok_and(
                |body| {
                    body.as_slice().len() == maximum
                        && body.as_slice().iter().all(|byte| *byte == 255u8)
                }
            )
        );
        assert_eq!(crate::http_contract_body::HttpContractBody::try_from(vec![255u8; maximum + 1usize]), Err(frontend_contract::frontend_contract_body_error::FrontendContractBodyError::TooLarge));
    }
}
