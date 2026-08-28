pub(crate) fn template_fs_should_skip(path: crate::ScaffoldPathRef<'_>) -> crate::ShouldSkip {
    crate::ShouldSkip::from(path.get().components().any(|component| {
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
    fn ignored_template_directories_are_explicit() {
        assert!(bool::from(super::template_fs_should_skip(
            crate::ScaffoldPathRef::from(std::path::Path::new("target/generated"))
        )));
        assert!(!bool::from(super::template_fs_should_skip(
            crate::ScaffoldPathRef::from(std::path::Path::new("server/src"))
        )));
    }
}
