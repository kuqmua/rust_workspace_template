pub(super) fn synchronize_generated_file(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    begin: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    end: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    generated: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    should_write: crate::should_write::ShouldWrite,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let source =
        crate::template_fs_read_bounded_text::template_fs_read_bounded_text(scaffold_path_ref)?;
    if begin.get().is_empty()
        || end.get().is_empty()
        || source.as_ref().matches(begin.get()).count() != constants_usize::ONE
        || source.as_ref().matches(end.get()).count() != constants_usize::ONE
    {
        return Err(crate::scaffold_error::ScaffoldError::Marker);
    }
    let (prefix, after_begin) = source
        .as_ref()
        .split_once(begin.get())
        .ok_or(crate::scaffold_error::ScaffoldError::Marker)?;
    let (_previous, suffix) = after_begin
        .split_once(end.get())
        .ok_or(crate::scaffold_error::ScaffoldError::Marker)?;
    let expected = crate::scaffold_text::ScaffoldText::try_from(format!(
        "{prefix}{}{generated}{}{suffix}",
        begin.get(),
        end.get(),
        generated = generated.get()
    ))
    .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)?;
    if expected.as_ref() == source.as_ref() {
        return Ok(());
    }
    if bool::from(should_write) {
        crate::template_fs_write_text::template_fs_write_text(
            scaffold_path_ref,
            crate::scaffold_text_ref::ScaffoldTextRef::from(expected.as_ref()),
        )
    } else {
        Err(crate::scaffold_error::ScaffoldError::GeneratedDeployment)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_duplicate_generated_markers_are_rejected() {
        let path = std::env::temp_dir()
            .join(module_path!())
            .join(std::process::id().to_string());
        let begin = constants_str::VALUE_0BAD8889;
        let end = constants_str::VALUE_79B72852;
        let generated = constants_str::VALUE_48AA6CAE;
        std::fs::create_dir_all(path.parent().expect(constants_str::DIAGNOSTIC_FA89432B))
            .expect(constants_str::DIAGNOSTIC_0FE5EA67);
        std::fs::write(
            path.as_path(),
            format!("{begin}{generated}{end}{begin}{generated}{end}"),
        )
        .expect(constants_str::DIAGNOSTIC_FA0FC595);
        let result = crate::synchronize_generated_file::synchronize_generated_file(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(begin),
            crate::scaffold_text_ref::ScaffoldTextRef::from(end),
            crate::scaffold_text_ref::ScaffoldTextRef::from(generated),
            crate::should_write::ShouldWrite::from(false),
        );
        assert!(matches!(
            result,
            Err(crate::scaffold_error::ScaffoldError::Marker)
        ));
        std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_D27B9216);
    }
}
