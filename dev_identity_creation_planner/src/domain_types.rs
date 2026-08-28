pub use crate::development_identity_count::DevelopmentIdentityCount;
pub use crate::development_identity_creation_plan::DevelopmentIdentityCreationPlan;
pub use crate::development_identity_creation_summary::DevelopmentIdentityCreationSummary;
pub use crate::development_identity_specs::DevelopmentIdentitySpecs;
pub use crate::development_identity_specs_error::DevelopmentIdentitySpecsError;
#[cfg(test)]
use crate::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN;

#[cfg(test)]
mod tests {
    #[test]
    fn plan_preserves_typed_identity_specs() {
        let plan = super::DevelopmentIdentityCreationPlan::new(
            super::DevelopmentIdentitySpecs::try_from(vec![
                server_runtime_http::domain_types::IdentitySpec::new(1u8, 2u8, 3u8, 4u8),
            ])
            .expect("743c519b plan_preserves_typed_identity_specs invariant must hold"),
        );
        let identity = plan
            .identities()
            .first()
            .expect("b9368d0c plan_preserves_typed_identity_specs invariant must hold");
        assert_eq!(identity.login(), &1u8);
    }

    #[test]
    fn identity_specs_rejects_more_than_supported_entries() {
        let values = std::iter::repeat_with(|| {
            server_runtime_http::domain_types::IdentitySpec::new(1u8, 2u8, 3u8, 4u8)
        })
        .take(super::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN.saturating_add(constants_usize::ONE))
        .collect::<Vec<_>>();
        assert_eq!(
            super::DevelopmentIdentitySpecs::try_from(values),
            Err(super::DevelopmentIdentitySpecsError)
        );
    }
}
