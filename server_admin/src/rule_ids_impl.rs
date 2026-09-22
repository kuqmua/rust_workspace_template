#[allow(
    clippy::single_call_fn,
    reason = "rule ids impl remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn rule_ids_impl(
    admin_html_form_text: &crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<server_admin_contract::admin_rule_ids::AdminRuleIds, crate::admin_error::AdminError> {
    crate::assignment_ids_impl::assignment_ids_impl::<
        server_admin_contract::admin_rule_id::AdminRuleId,
        _,
        server_admin_contract::admin_rule_ids::AdminRuleIds,
        _,
    >(admin_html_form_text)
}
