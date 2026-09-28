pub(crate) fn template_fs_insert_once(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
    marker: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    replacement: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<(), crate::scaffold_error::ScaffoldError> {
    let contents =
        crate::template_fs_read_bounded_text::template_fs_read_bounded_text(scaffold_path_ref)?;
    if marker.get().is_empty() || replacement.get().is_empty() {
        return Err(crate::scaffold_error::ScaffoldError::Marker);
    }
    let (updated, found) = contents.as_ref().split(replacement.get()).enumerate().fold(
        (String::new(), false),
        |(mut updated, found), (index, segment)| {
            if index > constants_usize::ZERO {
                updated.push_str(replacement.get());
            }
            if !found && let Some((before, after)) = segment.split_once(marker.get()) {
                updated.push_str(before);
                updated.push_str(replacement.get());
                updated.push_str(after);
                return (updated, true);
            }
            updated.push_str(segment);
            (updated, found)
        },
    );
    if !found {
        return if contents.as_ref().contains(replacement.get()) {
            Ok(())
        } else {
            Err(crate::scaffold_error::ScaffoldError::Marker)
        };
    }
    std::fs::write(scaffold_path_ref.get(), updated)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_existing_replacement_does_not_hide_another_marker() {
        let path = std::env::temp_dir()
            .join(module_path!())
            .join(std::process::id().to_string());
        std::fs::create_dir_all(path.parent().expect(constants_str::DIAGNOSTIC_30F430BD))
            .expect(constants_str::DIAGNOSTIC_FCC03FEA);
        let marker = constants_str::WORKSPACE_SCAFFOLD_MANIFEST_MEMBER_MARKER;
        let replacement = format!(
            "{}{}",
            marker,
            constants_str::WORKSPACE_SCAFFOLD_NOTIFICATION_SERVICE
        );
        let source = format!("{}{}{}", replacement, constants_str::NEWLINE, marker);
        std::fs::write(path.as_path(), source).expect(constants_str::DIAGNOSTIC_FC612D4D);
        crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(marker),
            crate::scaffold_text_ref::ScaffoldTextRef::from(replacement.as_str()),
        )
        .expect(constants_str::DIAGNOSTIC_BA88718A);
        let updated =
            std::fs::read_to_string(path.as_path()).expect(constants_str::DIAGNOSTIC_6FF03EAC);
        assert_eq!(
            updated,
            format!("{}{}{}", replacement, constants_str::NEWLINE, replacement)
        );
        crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(marker),
            crate::scaffold_text_ref::ScaffoldTextRef::from(replacement.as_str()),
        )
        .expect(constants_str::DIAGNOSTIC_C6863D2D);
        let unchanged =
            std::fs::read_to_string(path.as_path()).expect(constants_str::DIAGNOSTIC_88938469);
        assert_eq!(unchanged, updated);
        let empty_marker_result = crate::template_fs_insert_once::template_fs_insert_once(
            crate::scaffold_path_ref::ScaffoldPathRef::from(path.as_path()),
            crate::scaffold_text_ref::ScaffoldTextRef::from(constants_str::EMPTY),
            crate::scaffold_text_ref::ScaffoldTextRef::from(replacement.as_str()),
        );
        assert!(matches!(
            empty_marker_result,
            Err(crate::scaffold_error::ScaffoldError::Marker)
        ));
        std::fs::remove_file(path).expect(constants_str::DIAGNOSTIC_119AE43E);
    }
}
