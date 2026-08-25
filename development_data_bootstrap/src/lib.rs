pub mod domain_types;

#[must_use]
pub fn summarize_identity_bootstrap<Reports>(
    reports: Reports,
) -> domain_types::DevelopmentBootstrapSummary
where
    Reports: IntoIterator<Item = server_runtime_http::domain_types::IdentityBootstrapDecision>,
{
    reports.into_iter().fold(
        domain_types::DevelopmentBootstrapSummary::default(),
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
            server_runtime_http::domain_types::plan_identity_bootstrap(
                server_runtime_http::domain_types::IdentityPresence::Missing,
                server_runtime_http::domain_types::IdentityRolePresence::Present,
            ),
            server_runtime_http::domain_types::plan_identity_bootstrap(
                server_runtime_http::domain_types::IdentityPresence::Present,
                server_runtime_http::domain_types::IdentityRolePresence::Present,
            ),
            server_runtime_http::domain_types::plan_identity_bootstrap(
                server_runtime_http::domain_types::IdentityPresence::Missing,
                server_runtime_http::domain_types::IdentityRolePresence::Missing,
            ),
        ];
        let summary = super::summarize_identity_bootstrap(reports);
        assert_eq!(usize::from(summary.create()), constants_usize::ONE);
        assert_eq!(usize::from(summary.already_exists()), constants_usize::ONE);
        assert_eq!(usize::from(summary.missing_role()), constants_usize::ONE);
    }
}
