mod development_identity_count;
mod development_identity_creation_plan;
mod development_identity_creation_summary;
mod development_identity_specs;
mod development_identity_specs_error;
mod development_identity_specs_max_len;
pub mod domain_types;
pub(crate) use domain_types::{
    DevelopmentIdentityCount, DevelopmentIdentityCreationSummary, DevelopmentIdentitySpecs,
    DevelopmentIdentitySpecsError,
};

#[must_use]
pub fn summarize_identity_creation_decisions<Reports>(
    reports: Reports,
) -> DevelopmentIdentityCreationSummary
where
    Reports: IntoIterator<Item = server_runtime_http::domain_types::IdentityCreationDecision>,
{
    reports.into_iter().fold(
        DevelopmentIdentityCreationSummary::default(),
        |mut summary, decision| {
            summary.record(decision);
            summary
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn summarizes_desired_state_decisions() {
        let reports = [
            server_runtime_http::domain_types::plan_identity_creation(
                server_runtime_http::domain_types::IdentityPresence::Missing,
                server_runtime_http::domain_types::IdentityRolePresence::Present,
            ),
            server_runtime_http::domain_types::plan_identity_creation(
                server_runtime_http::domain_types::IdentityPresence::Present,
                server_runtime_http::domain_types::IdentityRolePresence::Present,
            ),
            server_runtime_http::domain_types::plan_identity_creation(
                server_runtime_http::domain_types::IdentityPresence::Missing,
                server_runtime_http::domain_types::IdentityRolePresence::Missing,
            ),
        ];
        let summary = super::summarize_identity_creation_decisions(reports);
        assert_eq!(usize::from(summary.create()), constants_usize::ONE);
        assert_eq!(usize::from(summary.already_exists()), constants_usize::ONE);
        assert_eq!(usize::from(summary.missing_role()), constants_usize::ONE);
    }
}
