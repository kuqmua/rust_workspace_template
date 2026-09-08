#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct RsSourceFilesRef<'sources_lt>(
    &'sources_lt [crate::test_code_style_snapshot::RsSourceFile],
);

impl<'sources_lt> RsSourceFilesRef<'sources_lt> {
    pub(super) fn assert_external_modules_resolve(&self) {
        self.0
            .iter()
            .filter(|source_file| {
                source_file
                    .path()
                    .as_ref()
                    .file_stem()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|stem| matches!(stem, constants_str::LIB | constants_str::MAIN))
            })
            .for_each(|source_file| {
                source_file
                    .ast()
                    .as_ref()
                    .items
                    .iter()
                    .filter_map(|item| {
                        let syn::Item::Mod(item_mod) = item else {
                            return None;
                        };
                        item_mod.content.is_none().then_some(item_mod)
                    })
                    .for_each(|item_mod| {
                        let module_path = source_file.path().as_ref().with_file_name(format!(
                            "{}.{}",
                            item_mod.ident,
                            constants_str::RS
                        ));
                        assert!(
                            self.0
                                .iter()
                                .any(|module_source| module_source.path().as_ref() == module_path),
                            "e26ef279 missing external module source {}",
                            module_path.display()
                        );
                    });
            });
    }

    pub(super) fn external_module_declaration(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> Option<crate::syn_item_ref::SynItemRef<'sources_lt>> {
        self.0
            .iter()
            .find_map(|source_file| source_file.external_module_declaration(path_ref))
    }

    pub(super) fn is_test_module_path(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> crate::analyzer_bool::AnalyzerBool {
        crate::analyzer_bool::AnalyzerBool::from(
            self.0
                .iter()
                .filter_map(|source_file| source_file.external_module_declaration(path_ref))
                .any(|syn_item_ref| {
                    crate::code_style::cfg_test_attr_count(syn_item_ref) > constants_usize::ZERO
                }),
        )
    }

    pub(super) fn module_names(
        &self,
        path_ref: crate::path_ref::PathRef<'_>,
    ) -> crate::source_text_list::SourceTextList {
        crate::source_text_list::SourceTextList::from(
            self.external_module_declaration(path_ref)
                .into_iter()
                .filter_map(|syn_item_ref| {
                    if let syn::Item::Mod(item_mod) = syn_item_ref.as_ref() {
                        Some(item_mod.ident.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>(),
        )
    }
}
