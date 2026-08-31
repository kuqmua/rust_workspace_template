#[derive(Debug, serde::Serialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct NotFoundPayload {
    commit: git_info::git_commit_link_cow::GitCommitLinkCow,
    message: to_err_string::error_text::ErrorText,
    open_api_specification: crate::open_api_specification_path::OpenApiSpecificationPath,
}

impl NotFoundPayload {
    #[allow(
        clippy::single_call_fn,
        reason = "the payload owner constructs its private serialized fields for the fallback adapter"
    )]
    pub(super) const fn from_parts(
        commit: git_info::git_commit_link_cow::GitCommitLinkCow,
        message: to_err_string::error_text::ErrorText,
        open_api_specification: crate::open_api_specification_path::OpenApiSpecificationPath,
    ) -> Self {
        Self {
            commit,
            message,
            open_api_specification,
        }
    }

    #[cfg(test)]
    pub(crate) fn matches(
        &self,
        expected_commit: &str,
        expected_message: &to_err_string::error_text::ErrorText,
        expected_open_api_specification: &str,
    ) -> bool {
        self.commit.as_ref() == expected_commit
            && self.message == *expected_message
            && *self.open_api_specification == expected_open_api_specification
    }
}
