#[derive(Debug, serde::Serialize, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
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
        git_commit_link_cow: git_info::git_commit_link_cow::GitCommitLinkCow,
        error_text: to_err_string::error_text::ErrorText,
        open_api_specification_path: crate::open_api_specification_path::OpenApiSpecificationPath,
    ) -> Self {
        Self {
            commit: git_commit_link_cow,
            message: error_text,
            open_api_specification: open_api_specification_path,
        }
    }

    #[cfg(test)]
    pub(crate) fn matches(
        &self,
        expected_commit: &str,
        error_text: &to_err_string::error_text::ErrorText,
        expected_open_api_specification: &str,
    ) -> bool {
        self.commit.as_ref() == expected_commit
            && self.message == *error_text
            && *self.open_api_specification == expected_open_api_specification
    }
}
