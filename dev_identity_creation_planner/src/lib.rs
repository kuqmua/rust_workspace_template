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
    fn test_plan_preserves_typed_identity_specs() {
        let plan = crate::development_identity_creation_plan::DevelopmentIdentityCreationPlan::new(
            crate::development_identity_specs::DevelopmentIdentitySpecs::try_from(vec![
                server_runtime_core::identity_spec::IdentitySpec::new(1u8, 2u8, 3u8, 4u8),
            ])
            .expect(constants_str::DIAGNOSTIC_743C519B),
        );
        let identity = plan
            .identities()
            .first()
            .expect(constants_str::DIAGNOSTIC_B9368D0C);
        assert_eq!(identity.login(), &1u8);
    }

    #[test]
    fn test_identity_specs_rejects_more_than_supported_entries() {
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
            Err(crate::development_identity_specs_error::DevelopmentIdentitySpecsError::TooMany)
        );
    }

    #[test]
    fn test_summarizes_desired_state_decisions() {
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
    #[test]
    fn test_identity_summary_empty_input_and_repeated_decisions() {
        let empty = crate::summarize_identity_creation_decisions(std::iter::empty());
        assert_eq!(usize::from(empty.create()), constants_usize::ZERO);
        assert_eq!(usize::from(empty.already_exists()), constants_usize::ZERO);
        assert_eq!(usize::from(empty.missing_role()), constants_usize::ZERO);
        let summary = crate::summarize_identity_creation_decisions([
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create,
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::AlreadyExists,
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create,
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::MissingRole,
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::AlreadyExists,
            server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create,
        ]);
        assert_eq!(usize::from(summary.create()), constants_usize::THREE);
        assert_eq!(usize::from(summary.already_exists()), constants_usize::TWO);
        assert_eq!(usize::from(summary.missing_role()), constants_usize::ONE);
    }

    #[test]
    fn test_identity_specs_accept_empty_and_exact_capacity_and_preserve_every_field() {
        [constants_usize::ZERO, crate::development_identity_specs_max_len::DEVELOPMENT_IDENTITY_SPECS_MAX_LEN]
            .into_iter()
            .fold((), |(), count| {
                let identities = (constants_usize::ZERO..count)
                    .map(|index| server_runtime_core::identity_spec::IdentitySpec::new(index, 2u8, 3u8, 4u8))
                    .collect::<Vec<_>>();
                assert!(crate::development_identity_specs::DevelopmentIdentitySpecs::try_from(identities).is_ok_and(|specs| {
                    let plan = crate::development_identity_creation_plan::DevelopmentIdentityCreationPlan::new(specs);
                    plan.identities().len() == count && plan.identities().iter().enumerate().all(|(index, identity)| {
                        identity.login() == &index && identity.display_name() == &2u8 && identity.role() == &3u8 && identity.secret_source() == &4u8
                    })
                }));
            });
    }
    #[test]
    fn test_identity_summary_records_only_the_selected_decision_count() {
        let initial = crate::development_identity_creation_summary::DevelopmentIdentityCreationSummary::default();
        assert_eq!(usize::from(initial.create()), constants_usize::ZERO);
        assert_eq!(usize::from(initial.already_exists()), constants_usize::ZERO);
        assert_eq!(usize::from(initial.missing_role()), constants_usize::ZERO);
        let final_summary = [
            (server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create, 1usize, 0usize, 0usize),
            (server_runtime_core::identity_creation_decision::IdentityCreationDecision::AlreadyExists, 1usize, 1usize, 0usize),
            (server_runtime_core::identity_creation_decision::IdentityCreationDecision::MissingRole, 1usize, 1usize, 1usize),
            (server_runtime_core::identity_creation_decision::IdentityCreationDecision::Create, 2usize, 1usize, 1usize),
        ].into_iter().fold(initial, |mut summary, (identity_creation_decision, create, already_exists, missing_role)| {
            summary.record(identity_creation_decision);
            assert_eq!(usize::from(summary.create()), create);
            assert_eq!(usize::from(summary.already_exists()), already_exists);
            assert_eq!(usize::from(summary.missing_role()), missing_role);
            summary
        });
        assert_eq!(usize::from(final_summary.create()), 2usize);
        assert_eq!(usize::from(final_summary.already_exists()), 1usize);
        assert_eq!(usize::from(final_summary.missing_role()), 1usize);
    }
}
