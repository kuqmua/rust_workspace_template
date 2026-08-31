#[cfg(test)]
mod tests {
    #[test]
    fn test_transport_body_enforces_shared_limit() {
        let oversized =
            vec![constants_u8::ZERO; constants_usize::VALUE_16_777_216 + constants_usize::ONE];
        assert_eq!(
            crate::transport_body::TransportBody::try_from(oversized),
            Err(crate::frontend_contract_body_error::FrontendContractBodyError::TooLarge),
        );
    }
}
