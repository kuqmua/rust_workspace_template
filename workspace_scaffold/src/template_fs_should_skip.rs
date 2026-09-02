pub(crate) fn template_fs_should_skip(
    scaffold_path_ref: crate::scaffold_path_ref::ScaffoldPathRef<'_>,
) -> crate::should_skip::ShouldSkip {
    crate::should_skip::ShouldSkip::from(scaffold_path_ref.get().components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(
                constants_str::GIT
                    | constants_str::TARGET
                    | constants_str::WORKSPACE_SCAFFOLD_NODE_MODULES
            )
        )
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ignored_template_directories_are_explicit() {
        assert!(bool::from(
            crate::template_fs_should_skip::template_fs_should_skip(
                crate::scaffold_path_ref::ScaffoldPathRef::from(std::path::Path::new(
                    constants_str::VALUE_E1231D31
                ))
            )
        ));
        assert!(!bool::from(
            crate::template_fs_should_skip::template_fs_should_skip(
                crate::scaffold_path_ref::ScaffoldPathRef::from(std::path::Path::new(
                    constants_str::VALUE_4098B735
                ))
            )
        ));
    }
}
