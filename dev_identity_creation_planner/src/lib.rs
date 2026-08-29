pub mod development_identity_count;
pub mod development_identity_creation_plan;
pub mod development_identity_creation_summary;
pub mod development_identity_specs;
pub mod development_identity_specs_error;
pub mod development_identity_specs_max_len;

#[must_use]
pub fn summarize_identity_creation_decisions<Reports>(
    reports: Reports,
) -> development_identity_creation_summary::DevelopmentIdentityCreationSummary
where
    Reports: IntoIterator<
        Item = server_runtime_core::identity_creation_decision::IdentityCreationDecision,
    >,
{
    reports.into_iter().fold(
        development_identity_creation_summary::DevelopmentIdentityCreationSummary::default(),
        |mut summary, decision| {
            summary.record(decision);
            summary
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn plan_preserves_typed_identity_specs() {
        let plan = crate::development_identity_creation_plan::DevelopmentIdentityCreationPlan::new(
            crate::development_identity_specs::DevelopmentIdentitySpecs::try_from(vec![
                server_runtime_core::identity_spec::IdentitySpec::new(1u8, 2u8, 3u8, 4u8),
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
            server_runtime_core::identity_spec::IdentitySpec::new(1u8, 2u8, 3u8, 4u8)
        })
        .take(
            super::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN
                .saturating_add(constants_usize::ONE),
        )
        .collect::<Vec<_>>();
        assert_eq!(
            crate::development_identity_specs::DevelopmentIdentitySpecs::try_from(values),
            Err(crate::development_identity_specs_error::DevelopmentIdentitySpecsError)
        );
    }

    #[test]
    fn summarizes_desired_state_decisions() {
        let reports = [
            server_runtime_core::plan_identity_creation::plan_identity_creation(
                server_runtime_core::identity_presence::IdentityPresence::Missing,
                server_runtime_core::identity_role_presence::IdentityRolePresence::Present,
            ),
            server_runtime_core::plan_identity_creation::plan_identity_creation(
                server_runtime_core::identity_presence::IdentityPresence::Present,
                server_runtime_core::identity_role_presence::IdentityRolePresence::Present,
            ),
            server_runtime_core::plan_identity_creation::plan_identity_creation(
                server_runtime_core::identity_presence::IdentityPresence::Missing,
                server_runtime_core::identity_role_presence::IdentityRolePresence::Missing,
            ),
        ];
        let summary = super::summarize_identity_creation_decisions(reports);
        assert_eq!(usize::from(summary.create()), constants_usize::ONE);
        assert_eq!(usize::from(summary.already_exists()), constants_usize::ONE);
        assert_eq!(usize::from(summary.missing_role()), constants_usize::ONE);
    }
}
