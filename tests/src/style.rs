#[cfg(test)]
mod tests {
    const ROOT_CARGO_TOML_EXCEPTIONS: [&str; 1] = ["../Cargo.toml"];
    const CLIPPY_LINT_EXCEPTIONS: [&str; 24] = [
        "absolute_paths",
        "disallowed_fields",
        "unnecessary_trailing_comma",
        "manual_pop_if",
        "assign_ops",
        "extend_from_slice",
        "match_on_vec_items",
        "misaligned_transmute",
        "option_map_or_err_ok",
        "pub_enum_variant_names",
        "range_step_by_zero",
        "regex_macro",
        "replace_consts",
        "should_assert_eq",
        "string_to_string",
        "unsafe_vector_initialization",
        "unstable_as_mut_slice",
        "unstable_as_slice",
        "unused_collect",
        "wrong_pub_self_convention",
        "manual_noop_waker",
        "manual_option_zip",
        "useless_borrows_in_formatting",
        "inline_modules",
    ];

    #[derive(Debug, Clone, Copy)]
    enum ExpectOrPanic {
        Expect,
        Panic,
    }
    impl ExpectOrPanic {
        const fn method_name(self) -> &'static str {
            match self {
                Self::Expect => "expect",
                Self::Panic => "panic",
            }
        }
    }
    #[derive(Debug, Clone, Copy)]
    enum RustOrClippy {
        Clippy,
        Rust,
    }
    impl RustOrClippy {
        fn name(&self) -> &str {
            match *self {
                Self::Rust => "rust",
                Self::Clippy => "clippy",
            }
        }
    }

    struct DeclaredProjectTypeNames(std::collections::HashSet<String>);

    struct BoundaryTypeErrors(Vec<String>);

    struct BoundaryGenericTypeParameterNames(std::collections::HashSet<String>);

    #[derive(Clone, Copy)]
    struct ConversionImplState(bool);

    #[derive(Clone, Copy)]
    struct ExternalTraitImplState(bool);

    #[derive(Clone, Copy)]
    struct FunctionBoundaryCheckState(bool);

    struct DeclaredTypeVisitor {
        names: DeclaredProjectTypeNames,
    }

    struct BoundaryTypeVisitor<'names> {
        checks_function_boundaries: FunctionBoundaryCheckState,
        declared_project_type_names: &'names DeclaredProjectTypeNames,
        errors: BoundaryTypeErrors,
        impl_generic_type_parameter_names: BoundaryGenericTypeParameterNames,
        is_conversion_impl: ConversionImplState,
        is_external_trait_impl: ExternalTraitImplState,
    }

    impl<'ast> syn::visit::Visit<'ast> for DeclaredTypeVisitor {
        fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
            let _was_inserted = self.names.0.insert(i.ident.to_string());
        }

        fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
            let _was_inserted = self.names.0.insert(i.ident.to_string());
        }

        fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
            let _was_inserted = self.names.0.insert(i.ident.to_string());
        }

        fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
            let _was_inserted = self.names.0.insert(i.ident.to_string());
        }
    }

    impl BoundaryTypeVisitor<'_> {
        fn is_external_trait_path(&self, trait_path: &syn::Path) -> bool {
            let Some(last_segment) = trait_path.segments.last() else {
                return true;
            };
            !self
                .declared_project_type_names
                .0
                .contains(last_segment.ident.to_string().as_str())
        }

        fn record_type(
            &mut self,
            owner_kind: &'static str,
            owner_name: &str,
            location: &'static str,
            ty: &syn::Type,
            generic_type_parameter_names: &std::collections::HashSet<String>,
        ) {
            collect_forbidden_non_project_type_usages(
                &mut self.errors.0,
                owner_kind,
                owner_name,
                location,
                ty,
                &self.declared_project_type_names.0,
                generic_type_parameter_names,
            );
        }
    }

    impl<'ast> syn::visit::Visit<'ast> for BoundaryTypeVisitor<'_> {
        fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
            if !self.checks_function_boundaries.0 {
                return;
            }
            if self.is_conversion_impl.0 || self.is_external_trait_impl.0 {
                return;
            }
            let mut generic_type_parameter_names = self.impl_generic_type_parameter_names.0.clone();
            generic_type_parameter_names.extend(collect_type_parameter_names(&i.sig.generics));
            for input in &i.sig.inputs {
                match input.clone() {
                    syn::FnArg::Receiver(_) => {}
                    syn::FnArg::Typed(pat_type) => self.record_type(
                        "method",
                        &i.sig.ident.to_string(),
                        "parameter",
                        &pat_type.ty,
                        &generic_type_parameter_names,
                    ),
                }
            }
            if let syn::ReturnType::Type(_return_arrow, return_type) = i.sig.output.clone() {
                self.record_type(
                    "method",
                    &i.sig.ident.to_string(),
                    "return value",
                    &return_type,
                    &generic_type_parameter_names,
                );
            }
        }

        fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
            let generic_type_parameter_names = collect_type_parameter_names(&i.generics);
            for variant in &i.variants {
                for field in &variant.fields {
                    self.record_type(
                        "enum variant",
                        &format!("{}::{}", i.ident, variant.ident),
                        "payload",
                        &field.ty,
                        &generic_type_parameter_names,
                    );
                }
            }
        }

        fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
            if !self.checks_function_boundaries.0 {
                return;
            }
            let generic_type_parameter_names = collect_type_parameter_names(&i.sig.generics);
            for input in &i.sig.inputs {
                match input.clone() {
                    syn::FnArg::Receiver(_) => {}
                    syn::FnArg::Typed(pat_type) => self.record_type(
                        "function",
                        &i.sig.ident.to_string(),
                        "parameter",
                        &pat_type.ty,
                        &generic_type_parameter_names,
                    ),
                }
            }
            if let syn::ReturnType::Type(_return_arrow, return_type) = i.sig.output.clone() {
                self.record_type(
                    "function",
                    &i.sig.ident.to_string(),
                    "return value",
                    &return_type,
                    &generic_type_parameter_names,
                );
            }
        }

        fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
            let previous_impl_generic_type_parameter_names = core::mem::replace(
                &mut self.impl_generic_type_parameter_names,
                BoundaryGenericTypeParameterNames(collect_type_parameter_names(&i.generics)),
            );
            let previous_is_conversion_impl = core::mem::replace(
                &mut self.is_conversion_impl,
                ConversionImplState(i.trait_.clone().is_some_and(
                    |(_bang_token, trait_path, _for_token)| {
                        trait_path.segments.first().is_some_and(|segment| {
                            matches!(
                                segment.ident.to_string().as_str(),
                                "AsRef" | "From" | "TryFrom"
                            )
                        })
                    },
                )),
            );
            let is_external_trait_impl =
                i.trait_
                    .clone()
                    .is_some_and(|(_bang_token, trait_path, _for_token)| {
                        self.is_external_trait_path(&trait_path)
                    });
            let previous_is_external_trait_impl = core::mem::replace(
                &mut self.is_external_trait_impl,
                ExternalTraitImplState(is_external_trait_impl),
            );
            syn::visit::visit_item_impl(self, i);
            self.is_external_trait_impl = previous_is_external_trait_impl;
            self.is_conversion_impl = previous_is_conversion_impl;
            self.impl_generic_type_parameter_names = previous_impl_generic_type_parameter_names;
        }

        fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
            if matches!(i.fields, syn::Fields::Unnamed(_)) && i.fields.iter().count() == 1 {
                return;
            }
            let generic_type_parameter_names = collect_type_parameter_names(&i.generics);
            for field in &i.fields {
                self.record_type(
                    "struct",
                    &i.ident.to_string(),
                    "field",
                    &field.ty,
                    &generic_type_parameter_names,
                );
            }
        }

        fn visit_trait_item_fn(&mut self, i: &'ast syn::TraitItemFn) {
            if !self.checks_function_boundaries.0 {
                return;
            }
            let generic_type_parameter_names = collect_type_parameter_names(&i.sig.generics);
            for input in &i.sig.inputs {
                match input.clone() {
                    syn::FnArg::Receiver(_) => {}
                    syn::FnArg::Typed(pat_type) => self.record_type(
                        "trait method",
                        &i.sig.ident.to_string(),
                        "parameter",
                        &pat_type.ty,
                        &generic_type_parameter_names,
                    ),
                }
            }
            if let syn::ReturnType::Type(_return_arrow, return_type) = i.sig.output.clone() {
                self.record_type(
                    "trait method",
                    &i.sig.ident.to_string(),
                    "return value",
                    &return_type,
                    &generic_type_parameter_names,
                );
            }
        }
    }

    fn collect_files_recursively(
        directory_path: &std::path::Path,
        files: &mut Vec<std::path::PathBuf>,
    ) {
        let Ok(directory_entries) = std::fs::read_dir(directory_path) else {
            return;
        };

        for directory_entry_result in directory_entries {
            let Ok(directory_entry) = directory_entry_result else {
                continue;
            };
            let Ok(file_type) = directory_entry.file_type() else {
                continue;
            };
            let path = directory_entry.path();

            if file_type.is_dir() {
                if path.ends_with(".git") || path.ends_with("target") {
                    continue;
                }
                collect_files_recursively(&path, files);
                continue;
            }

            files.push(path);
        }
    }

    fn collect_workspace_files(workspace_root: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut files = Vec::new();
        collect_files_recursively(workspace_root, &mut files);
        files
    }

    fn workspace_root_path() -> std::path::PathBuf {
        let crate_manifest_directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let parent_directory = crate_manifest_directory
            .parent()
            .map(std::path::Path::to_path_buf);
        parent_directory.unwrap_or(crate_manifest_directory)
    }

    fn non_test_source_segment(file_content: &str) -> &str {
        if let Some((before_test_segment, _after_test_segment)) =
            file_content.split_once("#[cfg(test)]")
        {
            return before_test_segment;
        }

        file_content
    }

    fn read_file(path: &std::path::Path) -> String {
        std::fs::read_to_string(path).unwrap_or_default()
    }

    fn workflow_file_paths(workspace_root: &std::path::Path) -> Vec<std::path::PathBuf> {
        let workflows_directory_path = workspace_root.join(".github").join("workflows");
        if !workflows_directory_path.exists() {
            return Vec::new();
        }
        let Ok(directory_entries) = std::fs::read_dir(&workflows_directory_path) else {
            return Vec::new();
        };
        directory_entries
            .into_iter()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "yml"))
            .collect()
    }

    fn continuous_integration_workflow_content() -> String {
        read_file(
            &workspace_root_path()
                .join(".github")
                .join("workflows")
                .join("ci.yml"),
        )
    }

    fn assert_continuous_integration_workflow_contains(required_text: &str) {
        let workflow_content = continuous_integration_workflow_content();
        assert!(
            workflow_content.contains(required_text),
            "CI workflow must contain `{required_text}`"
        );
    }

    fn rust_source_files(workspace_files: &[std::path::PathBuf]) -> Vec<&std::path::PathBuf> {
        workspace_files
            .iter()
            .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("rs"))
            .collect()
    }

    fn is_policy_test_source_file(path: &std::path::Path) -> bool {
        path.ends_with("tests/src/lib.rs") || path.ends_with("tests/src/style.rs")
    }

    fn forbids_pattern_usage_in_rust_sources(
        forbidden_pattern: &str,
        forbidden_name: &str,
    ) -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains(forbidden_pattern) {
                return Err(format!(
                    "found {} in Rust source: {}. preferred alternative: Result",
                    forbidden_name,
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    fn workspace_command_succeeds(
        command_name: &str,
        command_arguments: &[&str],
    ) -> Result<(), String> {
        let command_output_result = std::process::Command::new(command_name)
            .args(command_arguments)
            .current_dir(workspace_root_path())
            .output();
        let command_output = match command_output_result {
            Ok(successful_command_output) => successful_command_output,
            Err(error) => {
                return Err(format!(
                    "failed to run command `{command_name}` with args `{}`: {error}",
                    command_arguments.join(" ")
                ));
            }
        };
        if command_output.status.success() {
            return Ok(());
        }
        let standard_output = String::from_utf8(command_output.stdout)
            .map_err(|error| format!("94ffb721 {error}"))?;
        let standard_error = String::from_utf8(command_output.stderr)
            .map_err(|error| format!("09ceb39c {error}"))?;
        Err(format!(
            "utility command failed: `{}` `{}`\nstdout:\n{}\nstderr:\n{}",
            command_name,
            command_arguments.join(" "),
            standard_output,
            standard_error
        ))
    }

    fn is_str_or_slice_type_path(ty: &syn::Type) -> bool {
        match ty.clone() {
            syn::Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "str"),
            syn::Type::Group(type_group) => is_str_or_slice_type_path(&type_group.elem),
            syn::Type::Paren(type_paren) => is_str_or_slice_type_path(&type_paren.elem),
            syn::Type::Reference(type_reference) => is_str_or_slice_type_path(&type_reference.elem),
            syn::Type::Slice(_) => true,
            syn::Type::Array(_)
            | syn::Type::BareFn(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        }
    }

    fn is_forbidden_unbounded_domain_type_path(ty: &syn::Type) -> bool {
        match ty.clone() {
            syn::Type::Path(type_path) => {
                let Some(segment) = type_path.path.segments.last() else {
                    return false;
                };
                let unbounded_type_names = [
                    "String",
                    "str",
                    "Vec",
                    "HashMap",
                    "BTreeMap",
                    "std::collections::HashSet",
                    "BTreeSet",
                    "std::path::PathBuf",
                    "OsString",
                    "VecDeque",
                    "BinaryHeap",
                ];
                if unbounded_type_names
                    .into_iter()
                    .any(|type_name| segment.ident == type_name)
                {
                    return true;
                }
                if segment.ident == "Cow" || segment.ident == "Box" {
                    let syn::PathArguments::AngleBracketed(angle_bracketed_arguments) =
                        segment.arguments.clone()
                    else {
                        return false;
                    };
                    return angle_bracketed_arguments
                        .args
                        .into_iter()
                        .any(|generic_argument| {
                            if let syn::GenericArgument::Type(argument_type) = generic_argument {
                                return is_str_or_slice_type_path(&argument_type);
                            }
                            false
                        });
                }
                false
            }
            syn::Type::Group(type_group) => {
                is_forbidden_unbounded_domain_type_path(&type_group.elem)
            }
            syn::Type::Paren(type_paren) => {
                is_forbidden_unbounded_domain_type_path(&type_paren.elem)
            }
            syn::Type::Reference(type_reference) => {
                is_forbidden_unbounded_domain_type_path(&type_reference.elem)
            }
            syn::Type::Array(_)
            | syn::Type::BareFn(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::Slice(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        }
    }

    fn is_usize_type_path(ty: &syn::Type) -> bool {
        match ty.clone() {
            syn::Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "usize"),
            syn::Type::Group(type_group) => is_usize_type_path(&type_group.elem),
            syn::Type::Paren(type_paren) => is_usize_type_path(&type_paren.elem),
            syn::Type::Reference(type_reference) => is_usize_type_path(&type_reference.elem),
            syn::Type::Array(_)
            | syn::Type::BareFn(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::Slice(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        }
    }

    fn collect_type_parameter_names(generics: &syn::Generics) -> std::collections::HashSet<String> {
        generics
            .type_params()
            .map(|type_parameter| type_parameter.ident.to_string())
            .collect()
    }

    fn record_forbidden_non_project_type_path(
        errors: &mut Vec<String>,
        owner_kind: &'static str,
        owner_name: &str,
        location: &'static str,
        type_path: &syn::TypePath,
        declared_project_type_names: &std::collections::HashSet<String>,
        generic_type_parameter_names: &std::collections::HashSet<String>,
    ) {
        let Some(first_segment) = type_path.path.segments.first() else {
            errors.push(format!(
                "{owner_kind} `{owner_name}` uses non-project type in {location}: `{}`",
                syn::__private::ToTokens::to_token_stream(&type_path.path)
            ));
            return;
        };
        let first_segment_name = first_segment.ident.to_string();
        let last_segment_name = type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string());
        let allowed_interop_roots = ["proc_macro", "proc_macro2"];
        let allowed_wrapper_roots = ["Option", "Self"];
        let is_allowed_type = declared_project_type_names.contains(first_segment_name.as_str())
            || last_segment_name
                .as_deref()
                .is_some_and(|name| declared_project_type_names.contains(name))
            || generic_type_parameter_names.contains(first_segment_name.as_str())
            || allowed_interop_roots.contains(&first_segment_name.as_str())
            || allowed_wrapper_roots.contains(&first_segment_name.as_str());
        if is_allowed_type {
            return;
        }
        errors.push(format!(
            "{owner_kind} `{owner_name}` uses non-project type in {location}: `{}`",
            syn::__private::ToTokens::to_token_stream(&type_path.path)
        ));
    }

    fn collect_forbidden_trait_bound_type_usages(
        errors: &mut Vec<String>,
        owner_kind: &'static str,
        owner_name: &str,
        location: &'static str,
        type_param_bounds: syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>,
        declared_project_type_names: &std::collections::HashSet<String>,
        generic_type_parameter_names: &std::collections::HashSet<String>,
    ) {
        for type_param_bound in type_param_bounds {
            if let syn::TypeParamBound::Trait(trait_bound) = type_param_bound {
                record_forbidden_non_project_type_path(
                    errors,
                    owner_kind,
                    owner_name,
                    location,
                    &syn::TypePath {
                        qself: None,
                        path: trait_bound.path,
                    },
                    declared_project_type_names,
                    generic_type_parameter_names,
                );
            }
        }
    }

    fn collect_forbidden_nested_type_usages(
        errors: &mut Vec<String>,
        owner_kind: &'static str,
        owner_name: &str,
        location: &'static str,
        ty: &syn::Type,
        declared_project_type_names: &std::collections::HashSet<String>,
        generic_type_parameter_names: &std::collections::HashSet<String>,
    ) {
        collect_forbidden_non_project_type_usages(
            errors,
            owner_kind,
            owner_name,
            location,
            ty,
            declared_project_type_names,
            generic_type_parameter_names,
        );
    }

    fn collect_forbidden_non_project_type_usages(
        errors: &mut Vec<String>,
        owner_kind: &'static str,
        owner_name: &str,
        location: &'static str,
        ty: &syn::Type,
        declared_project_type_names: &std::collections::HashSet<String>,
        generic_type_parameter_names: &std::collections::HashSet<String>,
    ) {
        if let Some(inner_type) = match ty.clone() {
            syn::Type::Array(type_array) => Some(type_array.elem),
            syn::Type::Group(type_group) => Some(type_group.elem),
            syn::Type::Paren(type_paren) => Some(type_paren.elem),
            syn::Type::Reference(type_reference) => Some(type_reference.elem),
            syn::Type::Slice(type_slice) => Some(type_slice.elem),
            syn::Type::BareFn(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Path(_)
            | syn::Type::Ptr(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => None,
        } {
            return collect_forbidden_nested_type_usages(
                errors,
                owner_kind,
                owner_name,
                location,
                &inner_type,
                declared_project_type_names,
                generic_type_parameter_names,
            );
        }

        match ty.clone() {
            syn::Type::ImplTrait(type_impl_trait) => collect_forbidden_trait_bound_type_usages(
                errors,
                owner_kind,
                owner_name,
                location,
                type_impl_trait.bounds,
                declared_project_type_names,
                generic_type_parameter_names,
            ),
            syn::Type::Path(type_path) => {
                record_forbidden_non_project_type_path(
                    errors,
                    owner_kind,
                    owner_name,
                    location,
                    &type_path,
                    declared_project_type_names,
                    generic_type_parameter_names,
                );
                for path_segment in type_path.path.segments {
                    if let syn::PathArguments::AngleBracketed(angle_bracketed_arguments) =
                        path_segment.arguments
                    {
                        for generic_argument in angle_bracketed_arguments.args {
                            if let syn::GenericArgument::Type(generic_argument_type) =
                                generic_argument
                            {
                                collect_forbidden_nested_type_usages(
                                    errors,
                                    owner_kind,
                                    owner_name,
                                    location,
                                    &generic_argument_type,
                                    declared_project_type_names,
                                    generic_type_parameter_names,
                                );
                            }
                        }
                    }
                }
            }
            syn::Type::TraitObject(type_trait_object) => collect_forbidden_trait_bound_type_usages(
                errors,
                owner_kind,
                owner_name,
                location,
                type_trait_object.bounds,
                declared_project_type_names,
                generic_type_parameter_names,
            ),
            syn::Type::Tuple(type_tuple) => {
                for tuple_element in type_tuple.elems {
                    collect_forbidden_nested_type_usages(
                        errors,
                        owner_kind,
                        owner_name,
                        location,
                        &tuple_element,
                        declared_project_type_names,
                        generic_type_parameter_names,
                    );
                }
            }
            syn::Type::BareFn(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::Verbatim(_)
            | _ => {}
        }
    }

    fn record_forbidden_usize_domain_type(
        errors: &mut Vec<String>,
        owner_kind: &'static str,
        owner_name: &str,
        location: &'static str,
        ty: &syn::Type,
    ) {
        if is_usize_type_path(ty) {
            errors.push(format!(
                "{owner_kind} `{owner_name}` uses usize as a domain/API type in {location}: `{}`",
                syn::__private::ToTokens::to_token_stream(ty)
            ));
        }
    }

    fn record_forbidden_unbounded_domain_type(
        errors: &mut Vec<String>,
        owner_kind: &'static str,
        owner_name: &str,
        location: &'static str,
        ty: &syn::Type,
    ) {
        if is_forbidden_unbounded_domain_type_path(ty) {
            errors.push(format!(
                "{owner_kind} `{owner_name}` uses unbounded standard domain type in {location}: \
                 `{}`",
                syn::__private::ToTokens::to_token_stream(ty)
            ));
        }
    }

    fn runtime_rust_source_files<'file_paths>(
        workspace_root: &std::path::Path,
        workspace_files: &'file_paths [std::path::PathBuf],
    ) -> Vec<&'file_paths std::path::PathBuf> {
        rust_source_files(workspace_files)
            .into_iter()
            .filter(|rust_file| !is_policy_test_source_file(rust_file))
            .filter(|rust_file| {
                !rust_file.starts_with(workspace_root.join("tests").join("trybuild"))
            })
            .collect()
    }

    fn assert_forbidden_pattern_not_in_non_test_code(
        forbidden_pattern: &str,
        violation_message_prefix: &str,
    ) {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains(forbidden_pattern),
                "{violation_message_prefix}: {}",
                rust_file.display()
            );
        }
    }

    fn assert_forbidden_pattern_not_in_rust_sources_except_test_lib(
        forbidden_pattern: &str,
        violation_message_prefix: &str,
    ) {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains(forbidden_pattern),
                "{violation_message_prefix}: {}",
                rust_file.display()
            );
        }
    }

    fn project_dir() -> walkdir::WalkDir {
        walkdir::WalkDir::new("../")
    }

    fn is_ignored_dir_entry_name(name: &std::ffi::OsStr) -> bool {
        name == "target" || name == ".git"
    }

    fn for_each_rs_file_content(mut on_file: impl FnMut(&std::path::Path, &str)) {
        for entry in project_dir()
            .into_iter()
            .filter_entry(|el| {
                !is_ignored_dir_entry_name(el.file_name())
                    && (el.file_type().is_dir()
                        || el.path().extension().and_then(std::ffi::OsStr::to_str) == Some("rs"))
            })
            .filter_map(Result::ok)
            .filter(|el| el.path().extension().and_then(std::ffi::OsStr::to_str) == Some("rs"))
        {
            let path = entry.path();
            let Ok(content) = std::fs::read_to_string(path) else {
                continue;
            };
            on_file(path, &content);
        }
    }

    fn workspace_tbl_from_cargo_toml() -> Result<toml::value::Table, String> {
        let mut tbl = std::fs::read_to_string("../Cargo.toml")
            .map_err(|error| format!("39a0d238 {error}"))?
            .parse::<toml::value::Table>()
            .map_err(|error| format!("beb11586 {error}"))?;
        match tbl
            .remove("workspace")
            .ok_or_else(|| "f728192d".to_owned())?
        {
            toml::Value::Table(table) => Ok(table),
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => Err("2bfb0b62".to_owned()),
        }
    }

    fn toml_val_as_tbl_ref<'value_lt>(
        value: &'value_lt toml::Value,
        uuid: &str,
    ) -> Result<&'value_lt toml::value::Table, String> {
        value.as_table().ok_or_else(|| uuid.to_owned())
    }

    fn collect_missing_items<'items>(
        items: &'items [String],
        present_set: &std::collections::HashSet<&str>,
    ) -> Vec<&'items str> {
        items
            .iter()
            .map(String::as_str)
            .filter(|item| !present_set.contains(item))
            .collect::<Vec<&str>>()
    }

    fn is_exception(path: &std::path::Path, exceptions: &[&str]) -> bool {
        exceptions.iter().any(|exception| path.ends_with(exception))
    }

    fn assert_root_workspace_cargo_policy(
        exp_id: &'static str,
        mut mk_ers: impl FnMut(&std::path::Path, &toml::value::Table, &mut Vec<String>),
    ) {
        let ers = {
            let mut collected_ers = Vec::new();
            for entry in project_dir()
                .into_iter()
                .filter_entry(|el| !is_ignored_dir_entry_name(el.file_name()))
                .filter_map(Result::ok)
                .filter(|el| el.file_name() == "Cargo.toml")
                .filter(|el| !is_exception(el.path(), &ROOT_CARGO_TOML_EXCEPTIONS))
            {
                let path = entry.path();
                let parsed = match std::fs::read_to_string(path) {
                    Ok(content) => match content.parse::<toml::value::Table>() {
                        Ok(table) => table,
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                };
                mk_ers(path, &parsed, &mut collected_ers);
            }
            collected_ers
        };
        assert_joined_ers_empty(&ers, exp_id);
    }

    fn assert_joined_ers_empty(ers: &[String], exp_id: &'static str) {
        assert_joined_ers_empty_with_ctx(ers, exp_id, "");
    }

    fn assert_joined_ers_empty_with_ctx(ers: &[String], exp_id: &'static str, ctx: &str) {
        if ctx.is_empty() {
            assert!(ers.is_empty(), "{exp_id}\n{}", ers.join("\n"));
        } else {
            assert!(ers.is_empty(), "{exp_id} {ctx}\n{}", ers.join("\n"));
        }
    }

    fn str_set(items: &[String]) -> std::collections::HashSet<&str> {
        items
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<&str>>()
    }

    fn assert_workspace_lints_match(
        rust_or_clippy: RustOrClippy,
        tool: &str,
        parse_only_clippy: bool,
        exp_id: &'static str,
        exceptions: &[&str],
    ) -> Result<(), String> {
        let lints_vec_from_cargo_toml = {
            let workspace = workspace_tbl_from_cargo_toml()?;
            let lints = toml_val_as_tbl_ref(
                workspace
                    .get("lints")
                    .ok_or_else(|| "82eaea37".to_owned())?,
                "cae226cd",
            )?;
            let toml_v_tbl = toml_val_as_tbl_ref(
                lints
                    .get(rust_or_clippy.name())
                    .ok_or_else(|| "dbd02f72".to_owned())?,
                "6f4580ce",
            )?;
            toml_v_tbl.keys().cloned().collect::<Vec<String>>()
        };
        let lints_from_cmd = {
            let output = std::process::Command::new(tool)
                .args(["-W", "help"])
                .stdout(std::process::Stdio::piped())
                .output()
                .map_err(|error| format!("{exp_id} {error}"))?;
            if !output.status.success() {
                return Err("95d4595a".to_owned());
            }
            let stderr = String::from_utf8(output.stderr.clone())
                .map_err(|error| format!("3c1d9f87 {error}"))?;
            if !stderr.trim().is_empty() {
                return Err("cc4670a2".to_owned());
            }
            let stdout =
                String::from_utf8(output.stdout).map_err(|error| format!("5ef7b23a {error}"))?;
            let regex = if parse_only_clippy {
                regex::Regex::new(
                    r"(?m)^\s*clippy::([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b",
                )
                .map_err(|error| format!("fbf14346 {error}"))?
            } else {
                regex::Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                    .map_err(|error| format!("60d99c87 {error}"))?
            };
            regex
                .captures_iter(&stdout)
                .map(|el_70833f93| {
                    el_70833f93
                        .get(1)
                        .ok_or_else(|| "4f9c2e87".to_owned())
                        .map(|match_capture| match_capture.as_str().replace('-', "_"))
                })
                .collect::<Result<Vec<String>, String>>()?
        };
        {
            let lints_from_cargo_set = str_set(&lints_vec_from_cargo_toml);
            let lints_to_check_set = str_set(&lints_from_cmd);
            let lints_exceptions_set = exceptions
                .iter()
                .copied()
                .collect::<std::collections::HashSet<&str>>();
            let lints_not_in_cargo_toml = {
                let mut lints_not_in_cargo_toml = Vec::new();
                for lint in collect_missing_items(&lints_from_cmd, &lints_from_cargo_set) {
                    if !lints_exceptions_set.contains(lint) {
                        lints_not_in_cargo_toml.push(lint);
                    }
                }
                lints_not_in_cargo_toml
            };
            if !lints_not_in_cargo_toml.is_empty() {
                return Err(format!("1c5a9308 {lints_not_in_cargo_toml:?}"));
            }
            let outdated_lints_in_file =
                collect_missing_items(&lints_vec_from_cargo_toml, &lints_to_check_set);
            if !outdated_lints_in_file.is_empty() {
                return Err("93787d2d".to_owned());
            }
        }
        Ok(())
    }

    fn validate_workspace_dep_features(v_tbl: &toml::value::Table) -> Result<(), String> {
        if v_tbl
            .get("features")
            .ok_or_else(|| "473577d5".to_owned())?
            .as_array()
            .is_some()
        {
            return Ok(());
        }
        Err("27bcfb1c".to_owned())
    }

    fn take_next_u64_part(iter: &mut core::str::Split<'_, char>) -> bool {
        iter.next()
            .and_then(|part| part.parse::<u64>().ok())
            .is_some()
    }

    fn workspace_members_as_strs<'members_lt>(
        workspace: &'members_lt toml::value::Table,
        exp_id: &'static str,
    ) -> Result<Vec<&'members_lt str>, String> {
        let members = workspace
            .get("members")
            .and_then(toml::Value::as_array)
            .ok_or_else(|| exp_id.to_owned())?;
        let mut output = Vec::with_capacity(members.len());
        for member in members {
            match member.as_str() {
                Some(member_str) => output.push(member_str),
                None => return Err(exp_id.to_owned()),
            }
        }
        Ok(output)
    }

    fn check_expect_or_panic_contains_only_unq_uuid_v4(expect_or_panic: ExpectOrPanic) {
        struct ExpectVisitor {
            ers: Vec<String>,
            method_name: &'static str,
            uuids: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for ExpectVisitor {
            fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
                if i.method == self.method_name {
                    if i.args.len() == 1 {
                        if let Some(argument_expression) = i.args.first() {
                            let argument_token_stream =
                                syn::__private::ToTokens::to_token_stream(argument_expression)
                                    .to_string();
                            if argument_token_stream.starts_with('\"')
                                && argument_token_stream.ends_with('\"')
                            {
                                let value = argument_token_stream.trim_matches('\"').to_owned();
                                if value.len() == 8 {
                                    self.uuids.push(value);
                                } else {
                                    self.ers.push(format!("arg len is not 8: {value}"));
                                }
                            } else {
                                self.ers.push("arg is not string literal".to_owned());
                            }
                        } else {
                            self.ers.push("arg is not string literal".to_owned());
                        }
                    } else {
                        self.ers.push("with != 1 arg".to_owned());
                    }
                }
                syn::visit::visit_expr_method_call(self, i);
            }
        }
        let mut all_uuids = Vec::new();
        let mut all_ers = Vec::new();
        for_each_rs_file_content(|path, content| {
            let Ok(ast) = syn::parse_file(content) else {
                all_ers.push(format!("{path:?}: 5e7a83eb"));
                return;
            };
            let mut visitor = ExpectVisitor {
                method_name: expect_or_panic.method_name(),
                uuids: Vec::new(),
                ers: Vec::new(),
            };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            all_uuids.extend(visitor.uuids);
            all_ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|el_2b9891bd| format!("{path:?}: {el_2b9891bd}")),
            );
        });
        let duplicates = {
            let mut seen = std::collections::HashSet::new();
            let mut duplicates = Vec::new();
            for el_45f4b8bc in &all_uuids {
                if !seen.insert(el_45f4b8bc.as_str()) {
                    duplicates.push(el_45f4b8bc.to_owned());
                }
            }
            duplicates
        };
        if !duplicates.is_empty() {
            all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}");
    }

    // --- Meta tests ---

    #[test]
    fn enforces_publish_false_in_all_crates() {
        assert_root_workspace_cargo_policy("f2a8c5d3", |path, parsed, ers| {
            let publish = parsed
                .get("package")
                .and_then(|v_1c7b4e9d| v_1c7b4e9d.get("publish"));
            if publish != Some(&toml::Value::Boolean(false)) {
                ers.push(format!("{}: missing `publish = false`", path.display()));
            }
        });
    }

    #[test]
    fn enforces_workspace_lints_in_all_crates() {
        assert_root_workspace_cargo_policy("d5f1a4e7", |path, parsed, ers| {
            match parsed
                .get("lints")
                .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
            {
                Some(lints_tbl) => {
                    if lints_tbl.get("workspace") != Some(&toml::Value::Boolean(true)) {
                        ers.push(format!("{}: [lints] missing `workspace = true`", path.display()));
                    }
                }
                None => {
                    ers.push(format!("{}: missing [lints] section", path.display()));
                }
            }
        });
    }

    #[test]
    fn enforces_english_only_in_source_files() {
        let mut ers = Vec::new();
        let exceptions: [&str; 0] = [];
        for el_d87f0495 in project_dir()
            .into_iter()
            .filter_entry(|el_6870bc3d| !is_ignored_dir_entry_name(el_6870bc3d.file_name()))
            .filter_map(Result::ok)
        {
            let path = el_d87f0495.path();
            if !(path.is_file()
                && matches!(
                    path.extension().and_then(std::ffi::OsStr::to_str),
                    Some("rs" | "toml" | "md" | "txt" | "yml" | "yaml" | "json")
                ))
            {
                continue;
            }
            if is_exception(path, &exceptions) {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(path) else {
                continue;
            };
            ers.extend({
                let mut collected = Vec::new();
                for (line_idx, line) in content.lines().enumerate() {
                    let line_number = line_idx.saturating_add(1);
                    for ch in line.chars() {
                        if !(matches!(ch, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}')
                            || ch.is_ascii())
                        {
                            collected.push(format!(
                                "{}:{} non-english symbol `{}` (U+{:04X})",
                                path.display(),
                                line_number,
                                ch,
                                u32::from(ch)
                            ));
                        }
                    }
                }
                collected
            });
        }
        assert_joined_ers_empty_with_ctx(&ers, "8db37a2f", "non-english symbols:");
    }

    #[test]
    fn enforces_unique_uuid_in_expect_messages() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Expect);
    }

    #[test]
    fn enforces_unique_uuid_in_panic_messages() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Panic);
    }

    #[test]
    fn enforces_all_clippy_lints_in_workspace() -> Result<(), String> {
        assert_workspace_lints_match(
            RustOrClippy::Clippy,
            "clippy-driver",
            true,
            "8895ca50",
            &CLIPPY_LINT_EXCEPTIONS,
        )
    }

    #[test]
    fn enforces_all_rust_lints_in_workspace() -> Result<(), String> {
        assert_workspace_lints_match(RustOrClippy::Rust, "rustc", false, "3c20b457", &[
            "fuzzy_provenance_casts",
            "lossy_provenance_casts",
            "multiple_supertrait_upcastable",
            "must_not_suspend",
            "non_exhaustive_omitted_patterns",
            "supertrait_item_shadowing_definition",
            "supertrait_item_shadowing_usage",
            "aarch_64_softfloat_neon",
            "dflt_overrides_dflt_fields",
            "test_unstable_lint",
            "resolving_to_items_shadowing_supertrait_items",
            "shadowing_supertrait_items",
            "unqualified_local_imports",
            "unreachable_cfg_select_predicates",
            "default_overrides_default_fields",
            "linker_info",
            "duplicate_features",
            "deprecated_llvm_intrinsic",
            "tail_call_track_caller",
            "dead_code_pub_in_binary",
        ])
    }

    #[test]
    fn enforces_unique_uuid_in_rs_files() -> Result<(), String> {
        let rgx = regex::Regex::new(
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b"
        )
        .map_err(|error| format!("e098a1ff {error}"))?;
        let mut seen = std::collections::HashSet::new();
        let mut validation_error = None;
        for_each_rs_file_content(|_, content| {
            if validation_error.is_some() {
                return;
            }
            for el_714b3d9c in rgx.find_iter(content) {
                let Ok(uuid) = uuid::Uuid::parse_str(el_714b3d9c.as_str()) else {
                    validation_error = Some("c9711efd".to_owned());
                    return;
                };
                if uuid.get_version_num() != 4 {
                    validation_error = Some("49b49b21".to_owned());
                    return;
                }
                if !seen.insert(uuid) {
                    validation_error = Some("4cf9d239".to_owned());
                    return;
                }
            }
        });
        if let Some(error) = validation_error {
            return Err(error);
        }
        Ok(())
    }

    #[test]
    fn enforces_reused_string_literals_in_test_rust_code() -> Result<(), String> {
        struct StringLiteralVisitor {
            values: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for StringLiteralVisitor {
            fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
                if let syn::Lit::Str(literal_string) = i.lit.clone() {
                    self.values.push(literal_string.value());
                }
                syn::visit::visit_expr_lit(self, i);
            }
        }

        let workspace_root = workspace_root_path();
        let tests_directory = workspace_root.join("tests");
        let mut literal_locations_by_value = Vec::<(String, Vec<String>)>::new();
        for walk_directory_entry in walkdir::WalkDir::new(&tests_directory)
            .into_iter()
            .filter_entry(|walk_filter_entry| {
                !is_ignored_dir_entry_name(walk_filter_entry.file_name())
            })
            .filter_map(Result::ok)
        {
            let rust_file = walk_directory_entry.path();
            if !rust_file.is_file()
                || rust_file.extension().and_then(std::ffi::OsStr::to_str) != Some("rs")
                || is_policy_test_source_file(rust_file)
            {
                continue;
            }
            let file_content = read_file(rust_file);
            let syntax_tree = syn::parse_file(&file_content)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            let mut visitor = StringLiteralVisitor { values: Vec::new() };
            syn::visit::Visit::visit_file(&mut visitor, &syntax_tree);
            for literal_value in visitor.values {
                if let Some(existing_literal_index) = literal_locations_by_value
                    .iter_mut()
                    .position(|literal_locations| literal_locations.0 == literal_value)
                {
                    if let Some(literal_locations) =
                        literal_locations_by_value.get_mut(existing_literal_index)
                    {
                        literal_locations.1.push(rust_file.display().to_string());
                    }
                } else {
                    literal_locations_by_value
                        .push((literal_value, vec![rust_file.display().to_string()]));
                }
            }
        }

        let mut errors = Vec::new();
        for (literal_value, locations) in literal_locations_by_value {
            if locations.len() > 1 {
                errors.push(format!(
                    "duplicated string literal in test Rust code: {literal_value:?} in \
                     {locations:?}"
                ));
            }
        }
        assert_joined_ers_empty(&errors, "6f7348ad");
        Ok(())
    }

    #[test]
    fn forbids_use_imports_in_rust_sources() -> Result<(), String> {
        struct UseImportVisitor {
            found_use_imports: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for UseImportVisitor {
            fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
                self.found_use_imports
                    .push(syn::__private::ToTokens::to_token_stream(&i.tree).to_string());
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();
        for rust_file in rust_source_files(&workspace_files) {
            let file_content = read_file(rust_file);
            let parsed_file = syn::parse_file(&file_content)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            let mut visitor = UseImportVisitor {
                found_use_imports: Vec::new(),
            };
            syn::visit::Visit::visit_file(&mut visitor, &parsed_file);
            if !visitor.found_use_imports.is_empty() {
                errors.push(format!(
                    "{}: found use imports: {}. Use explicit full paths at usage sites instead",
                    rust_file.display(),
                    visitor.found_use_imports.join(", ")
                ));
            }
        }
        assert_joined_ers_empty(&errors, "0f3e5a9d");
        Ok(())
    }

    #[test]
    fn forbids_type_aliases_outside_associated_error_types() -> Result<(), String> {
        struct TypeAliasVisitor {
            aliases: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for TypeAliasVisitor {
            fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
                self.aliases.push(i.ident.to_string());
                syn::visit::visit_item_type(self, i);
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let parsed_file = syn::parse_file(&file_content)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            let mut visitor = TypeAliasVisitor {
                aliases: Vec::new(),
            };
            syn::visit::Visit::visit_file(&mut visitor, &parsed_file);
            if !visitor.aliases.is_empty() {
                errors.push(format!(
                    "{}: found type aliases: {}",
                    rust_file.display(),
                    visitor.aliases.join(", ")
                ));
            }
        }
        assert_joined_ers_empty(&errors, "c91d3df8");
        Ok(())
    }

    #[test]
    fn forbids_const_aliases_to_path_constants_in_runtime_sources() -> Result<(), String> {
        struct ConstAliasVisitor {
            aliases: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for ConstAliasVisitor {
            fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
                let item_const_expression = i.expr.as_ref().clone();
                if let syn::Expr::Path(expr_path) = item_const_expression {
                    let target_path =
                        syn::__private::ToTokens::to_token_stream(&expr_path.path).to_string();
                    self.aliases.push(format!(
                        "const `{}` aliases `{target_path}`; delete local const `{}` and replace \
                         all uses of `{}` with `{target_path}`",
                        i.ident, i.ident, i.ident
                    ));
                }
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let parsed_file = syn::parse_file(&file_content)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            let mut visitor = ConstAliasVisitor {
                aliases: Vec::new(),
            };
            syn::visit::Visit::visit_file(&mut visitor, &parsed_file);
            if !visitor.aliases.is_empty() {
                errors.push(format!(
                    "{}: found const aliases to path constants: {}. Use the original imported \
                     constant directly instead of redeclaring an alias",
                    rust_file.display(),
                    visitor.aliases.join(", ")
                ));
            }
        }
        assert_joined_ers_empty(&errors, "62e0a2ce");
        Ok(())
    }

    #[test]
    fn enforces_exact_version_in_workspace_dependencies() -> Result<(), String> {
        let workspace = workspace_tbl_from_cargo_toml()?;
        for (_, v_5c36cb98) in toml_val_as_tbl_ref(
            workspace
                .get("dependencies")
                .ok_or_else(|| "2376f58e".to_owned())?,
            "e117fa5a",
        )? {
            {
                let v_tbl = toml_val_as_tbl_ref(v_5c36cb98, "cb693a3f")?;
                if let Some(path_v) = v_tbl.get("path") {
                    if path_v.as_str().is_none() {
                        return Err("6ca03a1f".to_owned());
                    }
                } else {
                    match v_tbl
                        .get("version")
                        .ok_or_else(|| "d5b2b269".to_owned())?
                        .as_str()
                    {
                        Some(version_string) => {
                            if !version_string.strip_prefix('=').is_some_and(|rest| {
                                let mut iter = rest.split('.');
                                take_next_u64_part(&mut iter)
                                    && take_next_u64_part(&mut iter)
                                    && take_next_u64_part(&mut iter)
                                    && iter.next().is_none()
                            }) {
                                return Err("6640b9bf".to_owned());
                            }
                        }
                        None => return Err("a3410a37".to_owned()),
                    }
                    match v_tbl.len() {
                        1 => {}
                        2 => {
                            if v_tbl.contains_key("features") {
                                validate_workspace_dep_features(v_tbl)?;
                            }
                        }
                        3 => {
                            validate_workspace_dep_features(v_tbl)?;
                            match v_tbl
                                .get("default-features")
                                .ok_or_else(|| "847a138f".to_owned())?
                            {
                                &toml::Value::Boolean(_) => (),
                                &toml::Value::String(_)
                                | &toml::Value::Table(_)
                                | &toml::Value::Integer(_)
                                | &toml::Value::Float(_)
                                | &toml::Value::Datetime(_)
                                | &toml::Value::Array(_) => return Err("b320164b".to_owned()),
                            }
                        }
                        _ => return Err(format!("f1139378 {v_tbl:#?}")),
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn enforces_workspace_members_exist_on_disk() -> Result<(), String> {
        let workspace = workspace_tbl_from_cargo_toml()?;
        let members = workspace_members_as_strs(&workspace, "7f3a1c4e")?;
        let mut ers = {
            let mut collected = Vec::new();
            for member_str in members {
                let member_path = std::path::Path::new("..")
                    .join(member_str)
                    .join("Cargo.toml");
                if !member_path.exists() {
                    collected.push(format!(
                        "member `{member_str}` Cargo.toml not found at {}",
                        member_path.display()
                    ));
                }
            }
            collected
        };
        ers.sort();
        assert_joined_ers_empty(&ers, "a4e3b8d1");
        Ok(())
    }

    #[test]
    fn enforces_workspace_members_sorted_alphabetically() -> Result<(), String> {
        let workspace = workspace_tbl_from_cargo_toml()?;
        let members_vec = workspace_members_as_strs(&workspace, "c1d4f7a2")?;
        let mut sorted = members_vec.clone();
        sorted.sort_unstable();
        let mut ers = Vec::new();
        for (k_4b1e6a8c, (got, expected)) in members_vec.iter().zip(sorted.iter()).enumerate() {
            if got != expected {
                ers.push(format!("index {k_4b1e6a8c}: got `{got}`, expected `{expected}`"));
            }
        }
        assert_joined_ers_empty_with_ctx(&ers, "b7c2e5f8", "members not sorted:");
        Ok(())
    }

    #[test]
    fn enforces_service_environment_examples_define_required_mapcam_api_environment() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            assert!(
                content.contains("MAPCAM_API") || !content.contains("MAPCAM"),
                "mapcam API environment must be explicit in {}",
                environment_example.display()
            );
        }
    }

    #[test]
    fn enforces_service_environment_examples_define_non_empty_required_values() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            for line in content.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                    continue;
                }
                if let Some((name, value)) = trimmed_line.split_once('=') {
                    assert!(
                        !name.trim().is_empty() && !value.trim().is_empty(),
                        "environment example variable must have non-empty name and value in {}: {}",
                        environment_example.display(),
                        trimmed_line
                    );
                }
            }
        }
    }

    #[test]
    fn enforces_service_environment_examples_have_unique_mapcam_logins() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut seen_logins = std::collections::HashSet::new();
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            for line in content.lines() {
                if let Some((name, value)) = line.split_once('=') {
                    if name.contains("LOGIN") {
                        assert!(
                            seen_logins.insert(value.trim().to_owned()),
                            "duplicate service login in {}: {}",
                            environment_example.display(),
                            value.trim()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn enforces_service_environment_examples_have_unique_service_ports() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut seen_ports = std::collections::HashSet::new();
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            for line in content.lines() {
                if let Some((name, value)) = line.split_once('=') {
                    if name.contains("PORT") {
                        assert!(
                            seen_ports.insert(value.trim().to_owned()),
                            "duplicate service port in {}: {}",
                            environment_example.display(),
                            value.trim()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn enforces_service_environment_examples_parse_bind_and_mapcam_api_ports() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            for line in content.lines() {
                if let Some((name, value)) = line.split_once('=') {
                    if name.contains("PORT") {
                        assert!(
                            value.trim().parse::<u16>().is_ok(),
                            "port value must parse as u16 in {}: {}",
                            environment_example.display(),
                            line
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn enforces_service_environment_examples_request_origin_matches_mapcam_api_address() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            if content.contains("REQUEST_ORIGIN") && content.contains("MAPCAM_API") {
                assert!(
                    content.contains("http://") || content.contains("https://"),
                    "request origin and API address examples must include scheme in {}",
                    environment_example.display()
                );
            }
        }
    }

    #[test]
    fn enforces_service_environment_examples_required_variable_order() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for environment_example in workspace_files.iter().filter(|path| {
            path.file_name()
                .is_some_and(|file_name| file_name == ".env.example")
        }) {
            let content = read_file(environment_example);
            let names = content
                .lines()
                .filter_map(|line| line.split_once('=').map(|(name, _)| name.trim().to_owned()))
                .collect::<Vec<String>>();
            let mut sorted_names = names.clone();
            sorted_names.sort();
            assert!(
                names == sorted_names,
                "environment example variables must be sorted in {}",
                environment_example.display()
            );
        }
    }

    #[test]
    fn enforces_microservice_runner_crates_have_environment_examples() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for manifest_path in workspace_files
            .iter()
            .filter(|path| path.ends_with("Cargo.toml"))
            .filter(|path| !path.ends_with("tests/Cargo.toml"))
        {
            let Some(crate_directory) = manifest_path.parent() else {
                continue;
            };
            if crate_directory.join("src").join("main.rs").exists() {
                let environment_example_path = crate_directory.join(".env.example");
                if crate_directory.ends_with("server") {
                    continue;
                }
                assert!(
                    environment_example_path.exists(),
                    "runnable service crate should define .env.example: {}",
                    crate_directory.display()
                );
            }
        }
    }

    #[test]
    fn enforces_service_startup_code_has_no_hardcoded_default_bind_ports() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("127.0.0.1:") && !source_segment.contains("0.0.0.0:"),
                "service startup code must not hardcode bind ports: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_mapcam_crates_use_workspace_thiserror_dependency() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        for manifest_path in workspace_files
            .iter()
            .filter(|path| path.ends_with("Cargo.toml"))
            .filter(|path| {
                !path.ends_with("Cargo.toml") || *path != &workspace_root.join("Cargo.toml")
            })
        {
            let manifest_content = read_file(manifest_path);
            if manifest_content.contains("thiserror") {
                assert!(
                    manifest_content.contains("thiserror.workspace = true")
                        || manifest_content.contains("thiserror = { workspace = true"),
                    "thiserror must be inherited from workspace dependencies in {}",
                    manifest_path.display()
                );
            }
        }
    }

    // --- Policy tests ---

    #[test]
    fn forbids_todo_in_non_test_code() {
        assert_forbidden_pattern_not_in_non_test_code("todo!(", "found todo! in non-test code");
    }

    #[test]
    fn forbids_unimplemented_in_non_test_code() {
        assert_forbidden_pattern_not_in_non_test_code(
            "unimplemented!(",
            "found unimplemented! in non-test code",
        );
    }

    #[test]
    fn forbids_unreachable_in_non_test_code() {
        assert_forbidden_pattern_not_in_non_test_code(
            "unreachable!(",
            "found unreachable! in non-test code",
        );
    }

    #[test]
    fn forbids_panic_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("panic!(") {
                return Err(format!(
                    "found panic! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_assert_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("assert!(") {
                return Err(format!(
                    "found assert! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_assert_eq_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("assert_eq!(") {
                return Err(format!(
                    "found assert_eq! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_assert_ne_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("assert_ne!(") {
                return Err(format!(
                    "found assert_ne! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_debug_assert_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("debug_assert!(") {
                return Err(format!(
                    "found debug_assert! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_debug_assert_eq_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("debug_assert_eq!(") {
                return Err(format!(
                    "found debug_assert_eq! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_debug_assert_ne_usage_in_rust_sources() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            if file_content.contains("debug_assert_ne!(") {
                return Err(format!(
                    "found debug_assert_ne! in Rust source: {}. preferred alternative: Result",
                    rust_file.display()
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn forbids_unbounded_standard_domain_types_in_rust_apis() -> Result<(), String> {
        struct UnboundedDomainTypeContractVisitor {
            errors: Vec<String>,
        }

        impl<'ast> syn::visit::Visit<'ast> for UnboundedDomainTypeContractVisitor {
            fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
                for variant in &i.variants {
                    for field in &variant.fields {
                        record_forbidden_unbounded_domain_type(
                            &mut self.errors,
                            "enum variant",
                            &format!("{}::{}", i.ident, variant.ident),
                            "payload",
                            &field.ty,
                        );
                    }
                }
            }

            fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
                for input in &i.sig.inputs {
                    match input.clone() {
                        syn::FnArg::Receiver(_) => {}
                        syn::FnArg::Typed(pat_type) => {
                            record_forbidden_unbounded_domain_type(
                                &mut self.errors,
                                "function",
                                &i.sig.ident.to_string(),
                                "parameter",
                                &pat_type.ty,
                            );
                        }
                    }
                }
            }

            fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
                if i.fields.iter().count() == 1 {
                    return;
                }
                for field in &i.fields {
                    record_forbidden_unbounded_domain_type(
                        &mut self.errors,
                        "struct",
                        &i.ident.to_string(),
                        "field",
                        &field.ty,
                    );
                }
            }

            fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
                if is_forbidden_unbounded_domain_type_path(&i.ty) {
                    self.errors.push(format!(
                        "type alias `{}` targets forbidden unbounded standard domain type: `{}`",
                        i.ident,
                        syn::__private::ToTokens::to_token_stream(&i.ty)
                    ));
                }
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();

        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let ast =
                syn::parse_file(&file_content).map_err(|error| format!("c52615ce {error}"))?;
            let mut visitor = UnboundedDomainTypeContractVisitor { errors: Vec::new() };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            errors.extend(
                visitor
                    .errors
                    .into_iter()
                    .map(|error| format!("{}: {error}", rust_file.display())),
            );
        }

        if errors.is_empty() {
            return Ok(());
        }

        Err(format!("43134d11\n{}", errors.join("\n")))
    }

    #[test]
    fn forbids_unbounded_standard_collection_and_path_types_in_rust_apis() -> Result<(), String> {
        forbids_unbounded_standard_domain_types_in_rust_apis()
    }

    #[test]
    fn forbids_non_project_types_in_runtime_boundary_positions() -> Result<(), String> {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut runtime_source_files = runtime_rust_source_files(&workspace_root, &workspace_files);
        runtime_source_files.extend(
            rust_source_files(&workspace_files)
                .into_iter()
                .filter(|rust_file| is_policy_test_source_file(rust_file)),
        );
        let mut declared_type_visitor = DeclaredTypeVisitor {
            names: DeclaredProjectTypeNames(std::collections::HashSet::new()),
        };
        let mut parsed_runtime_files = Vec::new();
        for rust_file in runtime_source_files {
            let file_content = read_file(rust_file);
            let source_segment = if is_policy_test_source_file(rust_file) {
                file_content.as_str()
            } else {
                non_test_source_segment(&file_content)
            };
            let ast = syn::parse_file(source_segment)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            syn::visit::Visit::visit_file(&mut declared_type_visitor, &ast);
            parsed_runtime_files.push((rust_file, ast));
        }

        let mut errors = Vec::new();
        for (rust_file, ast) in parsed_runtime_files {
            let mut visitor = BoundaryTypeVisitor {
                checks_function_boundaries: FunctionBoundaryCheckState(
                    !is_policy_test_source_file(rust_file),
                ),
                declared_project_type_names: &declared_type_visitor.names,
                errors: BoundaryTypeErrors(Vec::new()),
                impl_generic_type_parameter_names: BoundaryGenericTypeParameterNames(
                    std::collections::HashSet::new(),
                ),
                is_conversion_impl: ConversionImplState(false),
                is_external_trait_impl: ExternalTraitImplState(false),
            };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            errors.extend(
                visitor
                    .errors
                    .0
                    .into_iter()
                    .map(|error| format!("{}: {error}", rust_file.display())),
            );
        }
        assert_joined_ers_empty(&errors, "59bfcf74");
        Ok(())
    }

    #[test]
    fn forbids_usize_in_rust_api_function_parameters_enum_payloads_and_type_aliases()
    -> Result<(), String> {
        struct UsizeDomainTypeContractVisitor {
            errors: Vec<String>,
        }

        impl<'ast> syn::visit::Visit<'ast> for UsizeDomainTypeContractVisitor {
            fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
                for variant in &i.variants {
                    for field in &variant.fields {
                        record_forbidden_usize_domain_type(
                            &mut self.errors,
                            "enum variant",
                            &format!("{}::{}", i.ident, variant.ident),
                            "payload",
                            &field.ty,
                        );
                    }
                }
            }

            fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
                for input in &i.sig.inputs {
                    match input.clone() {
                        syn::FnArg::Receiver(_) => {}
                        syn::FnArg::Typed(pat_type) => {
                            record_forbidden_usize_domain_type(
                                &mut self.errors,
                                "function",
                                &i.sig.ident.to_string(),
                                "parameter",
                                &pat_type.ty,
                            );
                        }
                    }
                }
            }

            fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
                if is_usize_type_path(&i.ty) {
                    self.errors.push(format!(
                        "type alias `{}` targets forbidden usize domain/API type: `{}`",
                        i.ident,
                        syn::__private::ToTokens::to_token_stream(&i.ty)
                    ));
                }
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();

        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let ast =
                syn::parse_file(&file_content).map_err(|error| format!("83f68d5b {error}"))?;
            let mut visitor = UsizeDomainTypeContractVisitor { errors: Vec::new() };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            errors.extend(
                visitor
                    .errors
                    .into_iter()
                    .map(|error| format!("{}: {error}", rust_file.display())),
            );
        }

        if errors.is_empty() {
            return Ok(());
        }

        Err(format!("546cf567\n{}", errors.join("\n")))
    }

    #[test]
    fn forbids_serde_json_value_fields_in_runtime_domain_and_api_types() -> Result<(), String> {
        struct SerdeJsonValueFieldVisitor {
            errors: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueFieldVisitor {
            fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
                for field in &i.fields {
                    let type_text =
                        syn::__private::ToTokens::to_token_stream(&field.ty).to_string();
                    if type_text.contains("serde_json :: toml::Value") || type_text == "toml::Value"
                    {
                        self.errors.push(format!(
                            "struct `{}` contains serde_json::toml::Value field `{type_text}`",
                            i.ident
                        ));
                    }
                }
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let ast = syn::parse_file(&file_content)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            let mut visitor = SerdeJsonValueFieldVisitor { errors: Vec::new() };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            errors.extend(
                visitor
                    .errors
                    .into_iter()
                    .map(|error| format!("{}: {error}", rust_file.display())),
            );
        }
        assert_joined_ers_empty(&errors, "13e298ee");
        Ok(())
    }

    #[test]
    fn forbids_new_raw_domain_primitive_fields_in_runtime_types() -> Result<(), String> {
        struct RawPrimitiveFieldVisitor {
            errors: Vec<String>,
        }
        impl<'ast> syn::visit::Visit<'ast> for RawPrimitiveFieldVisitor {
            fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
                let primitive_types = [
                    "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8",
                    "u16", "u32", "u64", "u128", "usize",
                ];
                for field in &i.fields {
                    let field_type_text =
                        syn::__private::ToTokens::to_token_stream(&field.ty).to_string();
                    if primitive_types.contains(&field_type_text.as_str()) {
                        self.errors.push(format!(
                            "struct `{}` contains raw primitive field `{}`",
                            i.ident, field_type_text
                        ));
                    }
                }
            }
        }

        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let ast = syn::parse_file(&file_content)
                .map_err(|error| format!("failed to parse {}: {error}", rust_file.display()))?;
            let mut visitor = RawPrimitiveFieldVisitor { errors: Vec::new() };
            syn::visit::Visit::visit_file(&mut visitor, &ast);
            errors.extend(
                visitor
                    .errors
                    .into_iter()
                    .map(|error| format!("{}: {error}", rust_file.display())),
            );
        }
        assert_joined_ers_empty(&errors, "58130edb");
        Ok(())
    }

    #[test]
    fn forbids_raw_schema_identifiers_in_scoped_sql_query_literals() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let schema_identifier_tokens = ["SELECT ", "INSERT INTO ", "UPDATE ", "DELETE FROM "];

        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for source_line in source_segment.lines() {
                let trimmed_source_line = source_line.trim();
                assert!(
                    !schema_identifier_tokens
                        .iter()
                        .any(|token| trimmed_source_line.contains(token)),
                    "raw SQL query literal needs shared schema constants: {}: {}",
                    rust_file.display(),
                    trimmed_source_line
                );
            }
        }
    }

    #[test]
    fn forbids_raw_table_names_after_sql_table_clauses_in_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let raw_table_clause_patterns = [
            "FROM users",
            "JOIN users",
            "UPDATE users",
            "INTO users",
            "TABLE users",
        ];

        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for source_line in source_segment.lines() {
                assert!(
                    !raw_table_clause_patterns
                        .iter()
                        .any(|pattern| source_line.contains(pattern)),
                    "raw table name appears after SQL table clause: {}: {}",
                    rust_file.display(),
                    source_line.trim()
                );
            }
        }
    }

    #[test]
    fn forbids_raw_postgres_schema_fragments_in_rust_sql_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let raw_postgres_fragments = ["public.", "information_schema.", "::jsonb", "::geometry"];

        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for source_line in source_segment.lines() {
                assert!(
                    !raw_postgres_fragments
                        .iter()
                        .any(|fragment| source_line.contains(fragment)),
                    "raw PostgreSQL schema fragment appears in Rust SQL source: {}: {}",
                    rust_file.display(),
                    source_line.trim()
                );
            }
        }
    }

    #[test]
    fn enforces_migration_schema_and_seed_data_split() {
        let workspace_root = workspace_root_path();
        let migrations_directory = workspace_root.join("migrations");
        if !migrations_directory.exists() {
            return;
        }
        let workspace_files = collect_workspace_files(&migrations_directory);
        for migration_file in workspace_files {
            let migration_content = read_file(&migration_file);
            let migration_file_name = migration_file
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default();
            if migration_file_name.contains("schema") {
                assert!(
                    !migration_content.contains("INSERT INTO "),
                    "schema migration must not contain seed data: {}",
                    migration_file.display()
                );
            }
            if migration_file_name.contains("seed") {
                assert!(
                    !migration_content.contains("CREATE TABLE ")
                        && !migration_content.contains("ALTER TABLE "),
                    "seed migration must not contain schema DDL: {}",
                    migration_file.display()
                );
            }
        }
    }

    #[test]
    fn type_boundary_compile_fail_contracts_hold() {
        let compile_fail_cases = trybuild::TestCases::new();
        compile_fail_cases.compile_fail("trybuild/*.rs");
    }

    #[test]
    fn optml_dependency_is_available_for_trybuild_contracts() {
        #[derive(optml::Optml)]
        struct OptmlTrybuildDependency;
        let optml_trybuild_dependency_type_name = core::any::type_name::<OptmlTrybuildDependency>();
        assert!(
            optml_trybuild_dependency_type_name.contains("OptmlTrybuildDependency"),
            "optml trybuild dependency marker type name changed"
        );
    }

    #[test]
    fn forbids_source_dropping_map_err_pattern() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains("map_err(|_|"),
                "found source-dropping map_err pattern in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_global_mutable_or_lazy_singleton_patterns() {
        assert_forbidden_pattern_not_in_non_test_code(
            "static mut",
            "found static mut in non-test code",
        );
        assert_forbidden_pattern_not_in_non_test_code(
            "lazy_static!",
            "found lazy_static singleton in non-test code",
        );
        assert_forbidden_pattern_not_in_non_test_code(
            "thread_local!(",
            "found thread_local singleton in non-test code",
        );
        assert_forbidden_pattern_not_in_non_test_code(
            "once_cell::sync::Lazy",
            "found once_cell::sync::Lazy singleton in non-test code",
        );
        assert_forbidden_pattern_not_in_non_test_code(
            "LazyLock::new(",
            "found LazyLock singleton in non-test code",
        );
    }

    #[test]
    fn enforces_deterministic_test_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if !rust_file
                .components()
                .any(|component| component.as_os_str() == "tests")
            {
                continue;
            }
            if is_policy_test_source_file(rust_file) {
                continue;
            }

            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("sleep("),
                "found sleep in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("SystemTime::now("),
                "found wall-clock time in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("Utc::now("),
                "found timezone-dependent call in tests: {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("Local::now("),
                "found local-time call in tests: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn requires_dependency_policy_checks_in_ci_and_readme_file() {
        let workspace_root = workspace_root_path();
        let ci_workflow_path = workspace_root
            .join(".github")
            .join("workflows")
            .join("ci.yml");
        let ci_workflow_content = read_file(&ci_workflow_path);

        assert!(
            ci_workflow_content.contains("cargo deny check"),
            "missing dependency policy check in {}",
            ci_workflow_path.display()
        );
        assert!(
            ci_workflow_content.contains("cargo udeps")
                || ci_workflow_content.contains("cargo +nightly udeps"),
            "missing unused dependency check in {}",
            ci_workflow_path.display()
        );

        let readme_path = workspace_root.join("README.md");
        let readme_content = read_file(&readme_path);
        assert!(
            !readme_content.trim().is_empty(),
            "README.md exists but is empty: {}",
            readme_path.display()
        );
    }

    #[test]
    fn requires_taplo_format_check_in_ci() {
        assert_continuous_integration_workflow_contains("taplo fmt --check");
    }

    #[test]
    fn requires_taplo_lint_in_ci() {
        assert_continuous_integration_workflow_contains("taplo lint");
    }

    #[test]
    fn requires_typos_check_in_ci() {
        assert_continuous_integration_workflow_contains("typos");
    }

    #[test]
    fn requires_actionlint_check_in_ci() {
        assert_continuous_integration_workflow_contains("actionlint");
    }

    #[test]
    fn requires_cargo_format_check_in_ci() {
        assert_continuous_integration_workflow_contains("cargo fmt");
    }

    #[test]
    fn requires_cargo_clippy_strict_check_in_ci() {
        assert_continuous_integration_workflow_contains(
            "cargo clippy --all-targets --all-features -- -D warnings",
        );
    }

    #[test]
    fn requires_cargo_deny_check_in_ci() {
        assert_continuous_integration_workflow_contains(
            "cargo deny check advisories bans licenses sources",
        );
    }

    #[test]
    fn requires_cargo_machete_check_in_ci() {
        assert_continuous_integration_workflow_contains("cargo machete");
    }

    #[test]
    fn requires_cargo_semver_checks_in_ci() {
        let workflow_content = continuous_integration_workflow_content();
        assert!(
            workflow_content.contains("cargo-semver-checks")
                || workflow_content.contains("cargo semver-checks"),
            "CI workflow must contain cargo semver checks"
        );
    }

    #[test]
    fn requires_cargo_audit_in_ci() {
        assert_continuous_integration_workflow_contains("cargo audit");
    }

    #[test]
    fn requires_cargo_metadata_locked_format_version_one_in_ci() {
        assert_continuous_integration_workflow_contains(
            "cargo metadata --locked --format-version 1",
        );
    }

    #[test]
    fn requires_cargo_hack_feature_powerset_check_in_ci() {
        assert_continuous_integration_workflow_contains(
            "cargo hack check --workspace --feature-powerset --no-dev-deps",
        );
    }

    #[test]
    fn requires_cargo_udeps_check_in_ci() {
        assert_continuous_integration_workflow_contains(
            "cargo +nightly udeps --workspace --all-targets --all-features",
        );
    }

    #[test]
    fn requires_cargo_llvm_coverage_summary_check_in_ci() {
        assert_continuous_integration_workflow_contains(
            "cargo llvm-cov --workspace --all-features --all-targets --summary-only",
        );
    }

    #[test]
    fn requires_gitleaks_check_in_ci() {
        assert_continuous_integration_workflow_contains("gitleaks detect");
    }

    #[test]
    fn requires_trivy_filesystem_check_in_ci() {
        assert_continuous_integration_workflow_contains("trivy fs");
    }

    #[test]
    fn requires_cargo_bench_save_baseline_check_in_ci() {
        assert_continuous_integration_workflow_contains("cargo bench save baseline");
    }

    #[test]
    fn requires_cargo_bench_baseline_compare_check_in_ci() {
        assert_continuous_integration_workflow_contains("cargo bench baseline compare");
    }

    #[test]
    fn runs_taplo_format_check_command() -> Result<(), String> {
        workspace_command_succeeds("taplo", &[
            "fmt",
            "--check",
            ".cargo/config.toml",
            ".typos.toml",
            "Cargo.toml",
            "clippy.toml",
            "deny.toml",
            "optml/Cargo.toml",
            "rust-toolchain.toml",
            "rustfmt.toml",
            "server/Cargo.toml",
            "tests/Cargo.toml",
        ])
    }

    #[test]
    fn runs_taplo_lint_command() -> Result<(), String> {
        workspace_command_succeeds("taplo", &["lint"])
    }

    #[test]
    fn runs_typos_check_command() -> Result<(), String> {
        workspace_command_succeeds("typos", &[])
    }

    #[test]
    fn runs_actionlint_check_command() -> Result<(), String> {
        workspace_command_succeeds("actionlint", &[])
    }

    #[test]
    fn runs_cargo_format_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &["fmt", "--check"])
    }

    #[test]
    fn runs_cargo_clippy_strict_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ])
    }

    #[test]
    fn runs_cargo_deny_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &[
            "deny",
            "check",
            "advisories",
            "bans",
            "licenses",
            "sources",
        ])
    }

    #[test]
    fn runs_cargo_machete_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo-machete", &["--help"])
    }

    #[test]
    fn runs_cargo_semver_checks_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &["semver-checks", "--help"])
    }

    #[test]
    fn runs_cargo_audit_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &["audit"])
    }

    #[test]
    fn runs_cargo_metadata_locked_format_version_one_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &["metadata", "--locked", "--format-version", "1"])
    }

    #[test]
    fn runs_cargo_hack_feature_powerset_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &["hack", "check", "--workspace", "--feature-powerset"])
    }

    #[test]
    fn runs_cargo_udeps_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &[
            "+nightly",
            "udeps",
            "--workspace",
            "--all-targets",
            "--all-features",
        ])
    }

    #[test]
    fn runs_cargo_llvm_coverage_summary_check_command() -> Result<(), String> {
        workspace_command_succeeds("cargo", &["llvm-cov", "--help"])
    }

    #[test]
    fn forbids_unwrap_usage_in_rust_sources() -> Result<(), String> {
        forbids_pattern_usage_in_rust_sources("unwrap(", "unwrap")
    }

    #[test]
    fn forbids_error_masking_shortcuts_in_rust_sources() -> Result<(), String> {
        forbids_pattern_usage_in_rust_sources("unwrap_or_default(", "unwrap_or_default")?;
        forbids_pattern_usage_in_rust_sources("unwrap_or(", "unwrap_or")?;
        Ok(())
    }

    #[test]
    fn forbids_expect_usage_in_rust_sources() -> Result<(), String> {
        forbids_pattern_usage_in_rust_sources("expect(", "expect")
    }

    #[test]
    fn forbids_abort_usage_in_rust_sources() -> Result<(), String> {
        forbids_pattern_usage_in_rust_sources("abort(", "abort")
    }

    #[test]
    fn forbids_process_abort_call_in_all_non_policy_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut errors = Vec::new();
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            if source_segment.contains("std::process::abort(")
                || source_segment.contains("process::abort(")
            {
                errors.push(format!("{}: found process abort call", rust_file.display()));
            }
        }
        assert_joined_ers_empty(&errors, "75a70d93");
    }

    #[test]
    fn forbids_environment_configuration_fallbacks_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let forbidden_patterns = [
            "env::var(",
            "std::env::var(",
            ".unwrap_or(",
            ".unwrap_or_default(",
            ".unwrap_or_else(",
        ];
        let mut errors = Vec::new();
        for rust_file in runtime_rust_source_files(&workspace_root, &workspace_files) {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for forbidden_pattern in forbidden_patterns {
                if source_segment.contains(forbidden_pattern) {
                    errors.push(format!(
                        "{}: environment/default fallback pattern `{forbidden_pattern}`",
                        rust_file.display()
                    ));
                }
            }
        }
        assert_joined_ers_empty(&errors, "b9860c85");
    }

    #[test]
    fn forbids_transmute_usage_in_rust_sources() -> Result<(), String> {
        forbids_pattern_usage_in_rust_sources("transmute(", "transmute")
    }

    #[test]
    fn forbids_direct_command_new_usage() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("std::process::Command::new("),
                "direct std::process::Command::new usage is forbidden: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_direct_environment_and_filesystem_access_inventory() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut violations = Vec::new();

        for rust_file in rust_source_files(&workspace_files) {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let occurrence_count = source_segment
                .matches("std::env")
                .count()
                .saturating_add(source_segment.matches("std::fs").count());
            if occurrence_count != 0 {
                violations.push(format!(
                    "{} has {} direct std::env/std::fs occurrence(s) in non-test code; move \
                     access behind an adapter or add a reviewed allowlist entry with a boundary \
                     reason",
                    rust_file.display(),
                    occurrence_count
                ));
            }
        }

        assert_joined_ers_empty(&violations, "3568c8b9");
    }

    #[test]
    fn enforces_spawn_ownership_inventory() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut violations = Vec::new();

        for rust_file in rust_source_files(&workspace_files) {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            let occurrence_count = file_content
                .matches("tokio::spawn(")
                .count()
                .saturating_add(file_content.matches("std::thread::spawn(").count());
            if occurrence_count != 0 {
                violations.push(format!(
                    "{} has {} direct tokio::spawn/std::thread::spawn occurrence(s); record \
                     owner, shutdown trigger, error path, and join/abort policy",
                    rust_file.display(),
                    occurrence_count
                ));
            }
        }

        assert_joined_ers_empty(&violations, "fc367bb3");
    }

    #[test]
    fn enforces_expect_attribute_inventory() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let mut violations = Vec::new();

        for rust_file in rust_source_files(&workspace_files) {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            let occurrence_count = file_content
                .matches("#[expect(")
                .count()
                .saturating_add(file_content.matches("#![expect(").count());
            if occurrence_count != 0 {
                violations.push(format!(
                    "{} has {} expect attribute occurrence(s); add a reviewed inventory entry \
                     with a boundary reason",
                    rust_file.display(),
                    occurrence_count
                ));
            }
        }

        assert_joined_ers_empty(&violations, "d1e53f57");
    }

    #[test]
    fn forbids_allow_lint_attributes_in_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("#[allow("),
                "found #[allow(...)] attribute in {}",
                rust_file.display()
            );
            assert!(
                !file_content.contains("#![allow("),
                "found #![allow(...)] attribute in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_dependency_declarations_for_member_crates() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);

        let member_manifest_paths = workspace_files
            .iter()
            .filter(|path| path.ends_with("Cargo.toml"))
            .filter(|path| {
                !path.ends_with("Cargo.toml") || *path != &workspace_root.join("Cargo.toml")
            })
            .filter(|path| path.starts_with(workspace_root.join("server")))
            .collect::<Vec<_>>();

        for manifest_path in member_manifest_paths {
            let manifest_content = read_file(manifest_path);
            let mut is_in_dependency_section = false;
            for line in manifest_content.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.starts_with('[') {
                    is_in_dependency_section = trimmed_line == "[dependencies]"
                        || trimmed_line == "[dev-dependencies]"
                        || trimmed_line == "[build-dependencies]";
                    continue;
                }
                if !is_in_dependency_section
                    || trimmed_line.is_empty()
                    || trimmed_line.starts_with('#')
                {
                    continue;
                }
                assert!(
                    trimmed_line.contains(".workspace = true")
                        || trimmed_line.contains("workspace = true"),
                    "dependency entry is not workspace-based in {}: {}",
                    manifest_path.display(),
                    trimmed_line
                );
                assert!(
                    !trimmed_line.contains("version = "),
                    "member manifest must not pin crate versions directly in {}: {}",
                    manifest_path.display(),
                    trimmed_line
                );
            }
        }
    }

    #[test]
    fn requires_crates_io_dependencies_to_live_in_workspace_dependencies() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);
        let mut is_in_workspace_dependencies_section = false;

        for line in root_manifest_content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('[') {
                is_in_workspace_dependencies_section = trimmed_line == "[workspace.dependencies]";
                continue;
            }
            if !is_in_workspace_dependencies_section
                || trimmed_line.is_empty()
                || trimmed_line.starts_with('#')
            {
                continue;
            }
            if trimmed_line.contains("version = ") && !trimmed_line.contains("path = ") {
                if trimmed_line.starts_with("axum = ")
                    || trimmed_line.starts_with("tokio = ")
                    || trimmed_line.starts_with("tracing-subscriber = ")
                {
                    continue;
                }
                assert!(
                    trimmed_line.contains("default-features = false"),
                    "workspace crates.io dependency must disable default features in {}: {}",
                    root_manifest_path.display(),
                    trimmed_line
                );
            }
        }
    }

    #[test]
    fn forbids_makefile_justfile_and_shell_scripts() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let shell_file_extensions = ["sh", "bash", "zsh", "fish"];

        assert!(
            !workspace_files
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "Makefile")),
            "Makefile is forbidden by policy"
        );
        assert!(
            !workspace_files
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "Justfile")),
            "Justfile is forbidden by policy"
        );
        assert!(
            !workspace_files.iter().any(|path| {
                path.extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|extension| shell_file_extensions.contains(&extension))
            }),
            "shell script files (*.sh, *.bash, *.zsh, *.fish) are forbidden by policy"
        );
        assert!(
            !workspace_files.iter().any(|path| {
                read_file(path).lines().next().is_some_and(|first_line| {
                    first_line.starts_with("#!/") && first_line.contains("sh")
                })
            }),
            "files with shell shebang are forbidden by policy"
        );
    }

    #[test]
    fn forbids_direct_index_zero_or_one_access_patterns() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            for line in file_content.lines() {
                let compact_line = line.replace(' ', "");
                assert!(
                    !compact_line.contains("[0]"),
                    "found forbidden [0] indexing pattern in {}: {}",
                    rust_file.display(),
                    line.trim()
                );
                assert!(
                    !compact_line.contains("[1]"),
                    "found forbidden [1] indexing pattern in {}: {}",
                    rust_file.display(),
                    line.trim()
                );
            }
        }
    }

    #[test]
    fn forbids_regular_loops_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);

            assert!(
                !source_segment.contains("\nfor "),
                "found regular for-loop in non-test code: {}",
                rust_file.display()
            );
            assert!(
                !source_segment.contains("\nwhile "),
                "found regular while-loop in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_numeric_as_casts_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let forbidden_as_patterns = [
            " as i8",
            " as i16",
            " as i32",
            " as i64",
            " as i128",
            " as isize",
            " as u8",
            " as u16",
            " as u32",
            " as u64",
            " as u128",
            " as usize",
            " as f32",
            " as f64",
        ];

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            for forbidden_as_pattern in forbidden_as_patterns {
                assert!(
                    !source_segment.contains(forbidden_as_pattern),
                    "found numeric `as` cast in non-test code: {} pattern: {}",
                    rust_file.display(),
                    forbidden_as_pattern
                );
            }
        }
    }

    #[test]
    fn forbids_axum_from_fn_layer_usage() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            assert!(
                !source_segment.contains(".layer(from_fn("),
                "found forbidden axum from_fn layer in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn forbids_json_macro_usage_in_repository_rust_sources() {
        assert_forbidden_pattern_not_in_rust_sources_except_test_lib(
            "json!(",
            "found forbidden json! macro usage in",
        );
    }

    #[test]
    fn forbids_macro_rules_usage_in_repository_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let forbidden_pattern = concat!("macro", "_rules!");
        let forbidden_pattern_with_space = concat!("macro", "_rules !");
        let mut errors = Vec::new();
        for rust_file in rust_source_files(&workspace_files) {
            let file_content = read_file(rust_file);
            if file_content.contains(forbidden_pattern) {
                errors.push(format!(
                    "{}: found {}. Create a dedicated proc-macro crate instead",
                    rust_file.display(),
                    forbidden_pattern
                ));
            }
            if file_content.contains(forbidden_pattern_with_space) {
                errors.push(format!(
                    "{}: found {}. Create a dedicated proc-macro crate instead",
                    rust_file.display(),
                    forbidden_pattern_with_space
                ));
            }
        }
        assert_joined_ers_empty(&errors, "991e4509");
    }

    #[test]
    fn forbids_include_macro_usage_in_repository_rust_sources() {
        assert_forbidden_pattern_not_in_rust_sources_except_test_lib(
            "include!(",
            "found forbidden include! macro usage in",
        );
    }

    #[test]
    fn forbids_include_str_macro_usage_in_non_test_code() {
        assert_forbidden_pattern_not_in_non_test_code(
            "include_str!(",
            "found forbidden include_str! macro usage in non-test code",
        );
    }

    #[test]
    fn forbids_include_bytes_macro_usage_in_non_test_code() {
        assert_forbidden_pattern_not_in_non_test_code(
            "include_bytes!(",
            "found forbidden include_bytes! macro usage in non-test code",
        );
    }

    #[test]
    fn forbids_serde_json_value_usage_in_repository_rust_sources() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }
            let file_content = read_file(rust_file);
            assert!(
                !file_content.contains("serde_json::toml::Value"),
                "found forbidden serde_json::toml::Value usage in {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_hardened_release_profile_configuration() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);

        assert!(
            root_manifest_content.contains("[profile.release]"),
            "missing [profile.release] section in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("lto = \"fat\""),
            "release profile must enable fat LTO in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("codegen-units = 1"),
            "release profile must use codegen-units = 1 in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("panic = \"abort\""),
            "release profile must use panic = \"abort\" in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("strip = \"symbols\""),
            "release profile must strip symbols in {}",
            root_manifest_path.display()
        );
    }

    #[test]
    fn enforces_workspace_verify_alias_order() {
        let workspace_root = workspace_root_path();
        let cargo_config_path = workspace_root.join(".cargo").join("config.toml");
        let cargo_config_content = read_file(&cargo_config_path);
        let expected_verify_alias = "workspace-verify = \"!cargo fmt && cargo clippy \
                                     --all-targets --all-features -- -D warnings && cargo test\"";

        assert!(
            cargo_config_content.contains(expected_verify_alias),
            "workspace-verify alias must execute fmt -> clippy -> test in {}",
            cargo_config_path.display()
        );
    }

    #[test]
    fn enforces_nightly_toolchain_channel() {
        let workspace_root = workspace_root_path();
        let rust_toolchain_path = workspace_root.join("rust-toolchain.toml");
        let rust_toolchain_content = read_file(&rust_toolchain_path);

        assert!(
            rust_toolchain_content.contains("channel = \"nightly\""),
            "workspace template requires nightly toolchain in {}",
            rust_toolchain_path.display()
        );
    }

    #[test]
    fn forbids_debug_print_macros_in_non_test_code_except_entrypoint_output() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);
        let server_main_path = workspace_root.join("server").join("src").join("main.rs");

        for rust_file in rust_files {
            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let path_is_server_main = rust_file == &server_main_path;
            if !path_is_server_main {
                assert!(
                    !source_segment.contains("println!("),
                    "found println! in non-test non-entrypoint code: {}",
                    rust_file.display()
                );
                assert!(
                    !source_segment.contains("eprintln!("),
                    "found eprintln! in non-test non-entrypoint code: {}",
                    rust_file.display()
                );
            }
            assert!(
                !source_segment.contains("dbg!("),
                "found dbg! in non-test code: {}",
                rust_file.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_alias_set_for_daily_workflows() {
        let workspace_root = workspace_root_path();
        let cargo_config_path = workspace_root.join(".cargo").join("config.toml");
        let cargo_config_content = read_file(&cargo_config_path);

        let required_aliases = [
            "workspace-format = ",
            "workspace-lint = ",
            "workspace-test = ",
            "workspace-hack = ",
            "workspace-deny = ",
            "workspace-udeps = ",
            "workspace-verify = ",
        ];

        for required_alias in required_aliases {
            assert!(
                cargo_config_content.contains(required_alias),
                "missing required cargo alias `{}` in {}",
                required_alias,
                cargo_config_path.display()
            );
        }
    }

    #[test]
    fn enforces_workspace_resolver_and_member_list_contract() {
        let workspace_root = workspace_root_path();
        let root_manifest_path = workspace_root.join("Cargo.toml");
        let root_manifest_content = read_file(&root_manifest_path);

        assert!(
            root_manifest_content.contains("resolver = \"2\""),
            "workspace root must define resolver = \"2\" in {}",
            root_manifest_path.display()
        );
        assert!(
            root_manifest_content.contains("\"server\""),
            "workspace members must include server crate in {}",
            root_manifest_path.display()
        );
    }

    #[test]
    fn forbids_server_from_direct_thiserror_dependency() {
        let workspace_root = workspace_root_path();
        let server_manifest_path = workspace_root.join("server").join("Cargo.toml");
        let server_manifest_content = read_file(&server_manifest_path);

        assert!(
            !server_manifest_content.contains("thiserror"),
            "server crate must not depend on thiserror directly in {}",
            server_manifest_path.display()
        );
    }

    #[test]
    fn enforces_documented_local_verification_order() {
        let workspace_root = workspace_root_path();
        let readme_path = workspace_root.join("README.md");
        let readme_content = read_file(&readme_path);
        let contributing_path = workspace_root.join("CONTRIBUTING.md");
        let contributing_content = read_file(&contributing_path);
        let required_verification_sequence =
            "cargo fmt\ncargo clippy --all-targets --all-features -- -D warnings\ncargo test";

        assert!(
            readme_content.contains(required_verification_sequence),
            "README must document fmt -> clippy -> test order in {}",
            readme_path.display()
        );
        assert!(
            contributing_content.contains(required_verification_sequence),
            "CONTRIBUTING must document fmt -> clippy -> test order in {}",
            contributing_path.display()
        );
    }

    #[test]
    fn enforces_workflow_level_minimal_permissions() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            assert!(
                workflow_content.contains("permissions:\n  contents: read"),
                "workflow must define top-level minimal permissions in {}",
                workflow_file_path.display()
            );
        }
    }

    #[test]
    fn permits_parallel_workflow_runs() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            assert!(
                !workflow_content.contains("concurrency:"),
                "workflow must not define concurrency when parallel CI runs are allowed in {}",
                workflow_file_path.display()
            );
        }
    }

    #[test]
    fn enforces_timeout_minutes_for_each_workflow_job() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            let runs_on_count = workflow_content.matches("runs-on:").count();
            let timeout_minutes_count = workflow_content.matches("timeout-minutes:").count();
            assert!(
                runs_on_count == timeout_minutes_count,
                "every workflow job must define timeout-minutes in {}: runs-on={}, \
                 timeout-minutes={}",
                workflow_file_path.display(),
                runs_on_count,
                timeout_minutes_count
            );
        }
    }

    #[test]
    fn enforces_marketplace_actions_to_be_pinned_by_full_commit_sha() {
        let workspace_root = workspace_root_path();
        let workflows = workflow_file_paths(&workspace_root);

        for workflow_file_path in workflows {
            let workflow_content = read_file(&workflow_file_path);
            for workflow_line in workflow_content.lines() {
                let Some((_, action_reference_value)) = workflow_line.split_once("uses:") else {
                    continue;
                };
                let action_reference = action_reference_value.trim();
                if action_reference.starts_with("./") || action_reference.starts_with("docker://") {
                    continue;
                }
                let Some((action_name, action_version_reference)) =
                    action_reference.split_once('@')
                else {
                    continue;
                };
                if !action_name.contains('/') {
                    continue;
                }
                assert!(
                    action_version_reference.len() == 40
                        && action_version_reference
                            .chars()
                            .all(|character| character.is_ascii_hexdigit()),
                    "GitHub Action must be pinned by full 40-char SHA in {}: {}",
                    workflow_file_path.display(),
                    workflow_line.trim()
                );
            }
        }
    }

    #[test]
    fn forbids_public_struct_fields_in_non_test_code() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let rust_files = rust_source_files(&workspace_files);

        for rust_file in rust_files {
            if is_policy_test_source_file(rust_file) {
                continue;
            }

            let file_content = read_file(rust_file);
            let source_segment = non_test_source_segment(&file_content);
            let mut is_inside_public_struct = false;
            let mut public_struct_brace_depth = 0;

            for source_line in source_segment.lines() {
                let trimmed_line = source_line.trim_start();
                if !is_inside_public_struct
                    && trimmed_line.starts_with("pub struct ")
                    && trimmed_line.contains('{')
                {
                    is_inside_public_struct = true;
                    public_struct_brace_depth = source_line.matches('{').count();
                    public_struct_brace_depth =
                        public_struct_brace_depth.saturating_sub(source_line.matches('}').count());
                    continue;
                }

                if is_inside_public_struct {
                    assert!(
                        !trimmed_line.starts_with("pub "),
                        "found public struct field in non-test code: {}: {}",
                        rust_file.display(),
                        source_line.trim()
                    );

                    public_struct_brace_depth += source_line.matches('{').count();
                    public_struct_brace_depth =
                        public_struct_brace_depth.saturating_sub(source_line.matches('}').count());
                    if public_struct_brace_depth == 0 {
                        is_inside_public_struct = false;
                    }
                }
            }
        }
    }

    #[test]
    fn forbids_placeholder_repository_metadata_in_workspace_crates() {
        let workspace_root = workspace_root_path();
        let workspace_files = collect_workspace_files(&workspace_root);
        let manifest_paths = workspace_files
            .iter()
            .filter(|path| path.ends_with("Cargo.toml"))
            .filter(|path| *path != &workspace_root.join("Cargo.toml"));
        let placeholder_patterns = [
            "description = \"description\"",
            "repository = \"repository\"",
            "readme = \"readme\"",
            "keywords = [\"keyword\"]",
            "categories = [\"category\"]",
            "repository = \"https://github.com/user/repo\"",
        ];

        for manifest_path in manifest_paths {
            let manifest_content = read_file(manifest_path);
            for placeholder_pattern in placeholder_patterns {
                assert!(
                    !manifest_content.contains(placeholder_pattern),
                    "placeholder repository metadata must be replaced in {}: {}",
                    manifest_path.display(),
                    placeholder_pattern
                );
            }
        }
    }
}
