pub fn try_maybe_write_token_stream_into_file<P>(
    should_write_token_stream_into_file: super::ShouldWriteTokenStreamIntoFile,
    file_name: P,
    ts: super::ProcMacro2TokenStreamRef<'_>,
    format_with_cargofmt: &super::FormatWithCargofmt,
) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
{
    if !matches!(
        should_write_token_stream_into_file,
        super::ShouldWriteTokenStreamIntoFile::True
    ) {
        return Ok(());
    }
    let string_cnt = ts.as_ref().to_string();
    let wr_outcome = crate::domain_types::string_writer::try_write_string_into_file_with_outcome(
        file_name,
        crate::domain_types::string_writer::StringFileContentRef::from(string_cnt.as_str()),
    )?;
    if bool::from(wr_outcome.is_changed())
        && matches!(format_with_cargofmt, super::FormatWithCargofmt::True)
    {
        let path = wr_outcome.path();
        let mut command = crate::domain_types::tool_command::ToolCommand::new(
            crate::domain_types::tool_command::ToolProgramRef::from(constants_str::RUSTFMT),
        );
        let path_text = path.as_ref().to_string_lossy();
        let status = command
            .arg(crate::domain_types::tool_command::ToolArgRef::from(
                path_text.as_ref(),
            ))
            .status()?;
        if !status.success() {
            return Err(std::io::Error::other(format!(
                "rustfmt failed for {}",
                path.as_ref().display()
            )));
        }
    }
    Ok(())
}
