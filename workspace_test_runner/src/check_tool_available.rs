#[allow(
    clippy::single_call_fn,
    reason = "check tool available remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn check_tool_available(
    tool_path: crate::tool_path::ToolPath,
) -> crate::tool_available::ToolAvailable {
    crate::tool_available::ToolAvailable::from(std::path::Path::new(tool_path.get()).exists())
}
