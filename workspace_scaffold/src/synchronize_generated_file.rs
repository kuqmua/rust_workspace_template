use super::{ScaffoldError, ScaffoldPathRef, ScaffoldText, ScaffoldTextRef, ShouldWrite};

pub(super) fn synchronize_generated_file(
    path: ScaffoldPathRef<'_>,
    begin: ScaffoldTextRef<'_>,
    end: ScaffoldTextRef<'_>,
    generated: ScaffoldTextRef<'_>,
    write_changes: ShouldWrite,
) -> Result<(), ScaffoldError> {
    let source = crate::template_fs_read_bounded_text::template_fs_read_bounded_text(path)?;
    let (prefix, after_begin) = source
        .as_ref()
        .split_once(begin.0)
        .ok_or(ScaffoldError::Marker)?;
    let (_previous, suffix) = after_begin.split_once(end.0).ok_or(ScaffoldError::Marker)?;
    let expected = ScaffoldText::try_from(format!(
        "{prefix}{}{generated}{}{suffix}",
        begin.0,
        end.0,
        generated = generated.0
    ))
    .map_err(|_error| ScaffoldError::Catalog)?;
    if expected.as_ref() == source.as_ref() {
        return Ok(());
    }
    if bool::from(write_changes) {
        crate::template_fs_write_text::template_fs_write_text(
            path,
            ScaffoldTextRef::from(expected.as_ref()),
        )
    } else {
        Err(ScaffoldError::GeneratedDeployment)
    }
}
