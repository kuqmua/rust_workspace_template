#[cfg(test)]
mod tests {
    const ROOT_CARGO_TOML_EXCEPTIONS: [&str; 1] = ["../Cargo.toml"];
    const CLIPPY_LINT_EXCEPTIONS: [&str; 22] = [
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
    ];
    const INCLUDE_ASSET_MACRO_SOURCE_EXCEPTIONS: [&str; 0] = [];
    const PUBLIC_REEXPORT_SOURCE_INCLUSIONS: &[&str] = &[
        "../app_state/src/lib.rs",
        "../config_lib/src/lib.rs",
        "../git_info/src/lib.rs",
        "../loc_lib/src/lib.rs",
        "../macros_helpers/src/lib.rs",
        "../naming/src/lib.rs",
        "../pg_crud/src/lib.rs",
        "../pg_crud/pg_tbl/src/lib.rs",
        "../pg_crud/pg_types/src/lib.rs",
        "../route_validators/src/lib.rs",
    ];
    const DOMAIN_TYPE_POLICY_SOURCE_INCLUSIONS: &[&str] = &[
        "../app_state/src/lib.rs",
        "../cmn_routes/src/lib.rs",
        "../config_lib/config_lib_macros/src/lib.rs",
        "../config_lib/src/lib.rs",
        "../config_lib/src/str_from_enum_macros.rs",
        "../config_lib/src/types.rs",
        "../config_lib/try_from_env/src/lib.rs",
        "../gen_quotes/src/lib.rs",
        "../git_info/src/lib.rs",
        "../loc_lib/loc_test/src/main.rs",
        "../loc_lib/src/loc.rs",
        "../macros_helpers/gen_derive_ts_builder/src/lib.rs",
        "../macros_helpers/src/attr_ident_str.rs",
        "../macros_helpers/src/gen_field_loc_new_ts.rs",
        "../macros_helpers/src/gen_if_write_is_err_ts.rs",
        "../macros_helpers/src/gen_impl_dflt_ts.rs",
        "../macros_helpers/src/gen_impl_display_ts.rs",
        "../macros_helpers/src/gen_impl_from_ts.rs",
        "../macros_helpers/src/gen_impl_to_err_string_ts.rs",
        "../macros_helpers/src/gen_impl_try_from_ts.rs",
        "../macros_helpers/src/gen_new_or_try_new.rs",
        "../macros_helpers/src/gen_pub_type_al_ts.rs",
        "../macros_helpers/src/gen_simple_syn_punct.rs",
        "../macros_helpers/src/get_macro_attr.rs",
        "../macros_helpers/src/loc.rs",
        "../macros_helpers/src/loc_syn_field.rs",
        "../macros_helpers/src/pgn_start_end_init_ts.rs",
        "../macros_helpers/src/rs_file_path.rs",
        "../macros_helpers/src/status_code.rs",
        "../macros_helpers/src/syn_field.rs",
        "../macros_helpers/src/test_hlp.rs",
        "../macros_helpers/src/wrap_derive.rs",
        "../macros_helpers/src/write_string_into_file.rs",
        "../macros_helpers/src/write_ts_into_file.rs",
        "../naming/naming_cmn/src/lib.rs",
        "../naming/naming_macros/src/lib.rs",
        "../naming/src/lib.rs",
        "../optml/src/lib.rs",
        "../panic_loc/src/lib.rs",
        "../pg_crud/pg_crud_macros_cmn/src/flts.rs",
        "../pg_crud/pg_tbl/gen_pg_tbl_src/src/lib.rs",
        "../pg_crud/wh_flts/gen_wh_flts_src/src/lib.rs",
        "../pg_crud/pg_types/gen_pg_types_src/src/lib.rs",
        "../route_validators/src/check_body_size.rs",
        "../route_validators/src/check_commit.rs",
        "../route_validators/src/hdr_val.rs",
        "../route_validators/src/test_hlp.rs",
        "../pg_crud/pg_types/pg_types_cmn/src/lib.rs",
        "../server_tbl_example/src/lib.rs",
        "../server_app_state/src/lib.rs",
        "../server_config/src/lib.rs",
        "../server/src/main.rs",
        "../tests/src/domain_type_policy_fixture.rs",
        "../to_err_string/src/lib.rs",
        "../token_patterns/token_patterns_macros/src/lib.rs",
        "../token_patterns/src/lib.rs",
        "../workspace_macro_helpers/src/lib.rs",
    ];
    #[derive(Debug, Clone, Copy, optml::Optml)]
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
    #[derive(Debug, Clone, Copy, optml::Optml)]
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
    struct DbgVisitor {
        found: bool,
    }
    impl<'ast> syn::visit::Visit<'ast> for DbgVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if i.path
                .segments
                .last()
                .is_some_and(|v_4b8e1c7a| v_4b8e1c7a.ident == "dbg")
            {
                self.found = true;
            }
        }
    }
    struct TodoUnimplVisitor {
        todo_found: usize,
        unimplemented_found: usize,
    }
    impl<'ast> syn::visit::Visit<'ast> for TodoUnimplVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if let Some(last_segment) = i.path.segments.last() {
                match () {
                    () if last_segment.ident == "todo" => {
                        self.todo_found = self.todo_found.saturating_add(1);
                    }
                    () if last_segment.ident == "unimplemented" => {
                        self.unimplemented_found = self.unimplemented_found.saturating_add(1);
                    }
                    () => {}
                }
            }
        }
    }
    struct UnwrapVisitor {
        found_count: usize,
    }
    impl<'ast> syn::visit::Visit<'ast> for UnwrapVisitor {
        fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
            if i.method == "unwrap" && i.args.is_empty() {
                self.found_count = self.found_count.saturating_add(1);
            }
            syn::visit::visit_expr_method_call(self, i);
        }
    }
    struct RuntimePanicExpectUnwrapVisitor {
        ers: Vec<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for RuntimePanicExpectUnwrapVisitor {
        fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
            if i.method == "expect" {
                self.ers.push(".expect() call".to_owned());
            }
            if i.method == "unwrap" {
                self.ers.push(".unwrap() call".to_owned());
            }
            syn::visit::visit_expr_method_call(self, i);
        }
        fn visit_item(&mut self, i: &'ast syn::Item) {
            if has_test_only_cfg_attr(i) {
                return;
            }
            syn::visit::visit_item(self, i);
        }
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if i.path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "panic")
            {
                self.ers.push("panic!() call".to_owned());
            }
            syn::visit::visit_macro(self, i);
        }
    }
    struct RuntimeMutexVisitor {
        found_count: usize,
    }
    impl<'ast> syn::visit::Visit<'ast> for RuntimeMutexVisitor {
        fn visit_item(&mut self, i: &'ast syn::Item) {
            if has_test_only_cfg_attr(i) {
                return;
            }
            syn::visit::visit_item(self, i);
        }
        fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
            if path_has_segment(&i.path, "Mutex") {
                self.found_count = self.found_count.saturating_add(1);
            }
            syn::visit::visit_type_path(self, i);
        }
    }
    struct RuntimeArcVisitor {
        allow_arc_value_usage: bool,
        ers: Vec<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for RuntimeArcVisitor {
        fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
            if expr_call_path(i).is_some_and(|path| path_ends_with(path, &["Arc", "new"]))
                && !self.allow_arc_value_usage
            {
                self.ers
                    .push("Arc::new() outside approved cross-thread state construction".to_owned());
            }
            syn::visit::visit_expr_call(self, i);
        }
        fn visit_item(&mut self, i: &'ast syn::Item) {
            if has_test_only_cfg_attr(i) {
                return;
            }
            syn::visit::visit_item(self, i);
        }
        fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
            if type_contains_segment(&i.ty, "Arc") {
                let name = i.ident.to_string();
                if !name.contains("Shared") && !name.contains("DynArc") {
                    self.ers.push(format!(
                        "Arc type alias `{name}` must be explicitly named as shared cross-thread state"
                    ));
                }
            }
            syn::visit::visit_item_type(self, i);
        }
    }
    struct AsyncBlockingCallVisitor {
        async_fn_depth: usize,
        ers: Vec<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for AsyncBlockingCallVisitor {
        fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
            if self.async_fn_depth != 0
                && expr_call_path(i).is_some_and(path_is_blocking_async_call)
            {
                self.ers
                    .push("blocking call inside async function".to_owned());
            }
            syn::visit::visit_expr_call(self, i);
        }
        fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
            if self.async_fn_depth != 0 && method_is_blocking_async_call(&i.method.to_string()) {
                self.ers.push(format!(
                    ".{}() blocking method call inside async function",
                    i.method
                ));
            }
            syn::visit::visit_expr_method_call(self, i);
        }
        fn visit_item(&mut self, i: &'ast syn::Item) {
            if has_test_only_cfg_attr(i) {
                return;
            }
            syn::visit::visit_item(self, i);
        }
        fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
            let is_async = i.sig.asyncness.is_some();
            if is_async {
                self.async_fn_depth = self.async_fn_depth.saturating_add(1);
            }
            syn::visit::visit_item_fn(self, i);
            if is_async {
                self.async_fn_depth = self.async_fn_depth.saturating_sub(1);
            }
        }
    }
    struct UnitTestExternalServiceVisitor {
        ers: Vec<String>,
        test_depth: usize,
    }
    impl<'ast> syn::visit::Visit<'ast> for UnitTestExternalServiceVisitor {
        fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
            if self.test_depth != 0 && path_is_external_service_client(&i.path) {
                self.ers.push(format!(
                    "unit tests must not depend on external service client `{}`",
                    path_to_string(&i.path)
                ));
            }
            syn::visit::visit_expr_path(self, i);
        }
        fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
            let is_test = self.test_depth != 0 || item_fn_is_unit_test(i);
            if is_test {
                self.test_depth = self.test_depth.saturating_add(1);
            }
            syn::visit::visit_item_fn(self, i);
            if is_test {
                self.test_depth = self.test_depth.saturating_sub(1);
            }
        }
        fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
            let is_test = self.test_depth != 0 || i.attrs.iter().any(attr_is_test_only_cfg);
            if is_test {
                self.test_depth = self.test_depth.saturating_add(1);
            }
            syn::visit::visit_item_mod(self, i);
            if is_test {
                self.test_depth = self.test_depth.saturating_sub(1);
            }
        }
    }
    struct IncludeAssetMacroVisitor {
        ers: Vec<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for IncludeAssetMacroVisitor {
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if let Some(segment) = i.path.segments.last()
                && (segment.ident == "include_str" || segment.ident == "include_bytes")
            {
                self.ers.push(format!("contains {}!()", segment.ident));
            }
            syn::visit::visit_macro(self, i);
        }
    }
    struct UseImportVisitor {
        found_non_public_use_import: bool,
        found_use_rename: bool,
        public_use_roots: Vec<String>,
    }
    impl UseImportVisitor {
        fn use_tree_contains_rename(use_tree: &syn::UseTree) -> bool {
            match use_tree {
                syn::UseTree::Path(use_path) => Self::use_tree_contains_rename(&use_path.tree),
                syn::UseTree::Name(_) | syn::UseTree::Glob(_) => false,
                syn::UseTree::Rename(_) => true,
                syn::UseTree::Group(use_group) => {
                    use_group.items.iter().any(Self::use_tree_contains_rename)
                }
            }
        }
    }
    impl<'ast> syn::visit::Visit<'ast> for UseImportVisitor {
        fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
            if matches!(i.vis, syn::Visibility::Public(_)) {
                if let Some(root) = match &i.tree {
                    syn::UseTree::Path(use_path) => Some(use_path.ident.to_string()),
                    syn::UseTree::Rename(use_rename) => Some(use_rename.ident.to_string()),
                    syn::UseTree::Name(use_name) => Some(use_name.ident.to_string()),
                    syn::UseTree::Glob(_) | syn::UseTree::Group(_) => None,
                } {
                    self.public_use_roots.push(root);
                } else {
                    self.public_use_roots.push(String::from("*"));
                }
            } else {
                self.found_non_public_use_import = true;
            }
            if Self::use_tree_contains_rename(&i.tree) {
                self.found_use_rename = true;
            }
            syn::visit::visit_item_use(self, i);
        }
    }
    struct TypeAliasVisitor {
        ers: Vec<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for TypeAliasVisitor {
        fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
            self.ers.push(format!(
                "type alias `{}` found; use the explicit type at usage sites instead of creating a type alias",
                i.ident
            ));
            syn::visit::visit_item_type(self, i);
        }
    }
    struct DeclaredDomainTypeVisitor {
        names: std::collections::BTreeSet<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for DeclaredDomainTypeVisitor {
        fn visit_item(&mut self, i: &'ast syn::Item) {
            if has_test_only_cfg_attr(i) {
                return;
            }
            syn::visit::visit_item(self, i);
        }
        fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
            let _: bool = self.names.insert(i.ident.to_string());
            syn::visit::visit_item_enum(self, i);
        }
        fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
            let _: bool = self.names.insert(i.ident.to_string());
            syn::visit::visit_item_struct(self, i);
        }
        fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
            let _: bool = self.names.insert(i.ident.to_string());
            syn::visit::visit_item_trait(self, i);
        }
        fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
            let _: bool = self.names.insert(i.ident.to_string());
            syn::visit::visit_item_union(self, i);
        }
        fn visit_macro(&mut self, i: &'ast syn::Macro) {
            if path_ends_with(&i.path, &["gen_pg_types", "gen_pg_types"]) {
                collect_gen_pg_types_domain_names(&i.tokens.to_string(), &mut self.names);
            }
            if config_lib_domain_type_macro_path(&i.path) {
                collect_first_macro_ident_domain_name(&i.tokens.to_string(), &mut self.names);
            }
            if path_ends_with(&i.path, &["bool_enum_to_tokens"]) {
                collect_first_macro_ident_domain_name(&i.tokens.to_string(), &mut self.names);
            }
            if path_ends_with(&i.path, &["gen_derive_ts_builder", "gen_derive_ts_builder"]) {
                let _: bool = self.names.insert(String::from("DTsBuilder"));
            }
            syn::visit::visit_macro(self, i);
        }
    }
    struct DomainTypePolicyVisitor<'types> {
        ers: Vec<String>,
        generic_scopes: Vec<std::collections::BTreeSet<String>>,
        repo_types: &'types std::collections::BTreeSet<String>,
    }
    impl DomainTypePolicyVisitor<'_> {
        fn check_fields(
            &mut self,
            fields: &syn::Fields,
            ctx: &str,
            allow_single_newtype_raw: bool,
        ) {
            if allow_single_newtype_raw
                && matches!(fields, syn::Fields::Unnamed(unnamed_fields) if unnamed_fields.unnamed.len() == 1)
            {
                return;
            }
            for field in fields {
                self.check_ty(&field.ty, ctx);
            }
        }
        fn check_path_arguments(&mut self, arguments: &syn::PathArguments, ctx: &str) {
            match arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(ty) = arg {
                            self.check_ty(ty, ctx);
                        }
                    }
                }
                syn::PathArguments::Parenthesized(args) => {
                    for ty in &args.inputs {
                        self.check_ty(ty, ctx);
                    }
                    match &args.output {
                        syn::ReturnType::Default => {}
                        syn::ReturnType::Type(_, ty) => self.check_ty(ty, ctx),
                    }
                }
                syn::PathArguments::None => {}
            }
        }
        fn check_sig(&mut self, sig: &syn::Signature, ctx: &str) {
            self.push_generics(&sig.generics);
            for input in &sig.inputs {
                match input {
                    syn::FnArg::Receiver(_) => {}
                    syn::FnArg::Typed(pat_ty) => {
                        self.check_ty(&pat_ty.ty, &format!("{ctx} parameter"));
                    }
                }
            }
            match &sig.output {
                syn::ReturnType::Default => {}
                syn::ReturnType::Type(_, ty) => {
                    self.check_ty(ty, &format!("{ctx} return type"));
                }
            }
            self.pop_generics();
        }
        fn check_ty(&mut self, ty: &syn::Type, ctx: &str) {
            match ty {
                syn::Type::Array(ty_array) => self.check_ty(&ty_array.elem, ctx),
                syn::Type::Group(ty_group) => self.check_ty(&ty_group.elem, ctx),
                syn::Type::Paren(ty_paren) => self.check_ty(&ty_paren.elem, ctx),
                syn::Type::Path(ty_path) => self.check_ty_path(ty_path, ctx),
                syn::Type::Reference(ty_reference) => self.check_ty(&ty_reference.elem, ctx),
                syn::Type::Slice(ty_slice) => self.check_ty(&ty_slice.elem, ctx),
                syn::Type::Tuple(ty_tuple) => {
                    for elem in &ty_tuple.elems {
                        self.check_ty(elem, ctx);
                    }
                }
                syn::Type::BareFn(_)
                | syn::Type::ImplTrait(_)
                | syn::Type::Infer(_)
                | syn::Type::Macro(_)
                | syn::Type::Never(_)
                | syn::Type::Ptr(_)
                | syn::Type::TraitObject(_)
                | syn::Type::Verbatim(_)
                | _ => {}
            }
        }
        fn check_ty_path(&mut self, ty_path: &syn::TypePath, ctx: &str) {
            if let Some(qself) = &ty_path.qself {
                self.check_ty(&qself.ty, ctx);
                for segment in &ty_path.path.segments {
                    self.check_path_arguments(&segment.arguments, ctx);
                }
                return;
            }
            let Some(segment) = ty_path.path.segments.last() else {
                return;
            };
            let ident = segment.ident.to_string();
            if path_first_segment_is_self(&ty_path.path) {
                self.check_path_arguments(&segment.arguments, ctx);
                return;
            }
            if is_structural_generic_container(&ident) {
                self.check_path_arguments(&segment.arguments, ctx);
                return;
            }
            if self.path_starts_with_allowed_type_ident(&ty_path.path) {
                for path_segment in &ty_path.path.segments {
                    self.check_path_arguments(&path_segment.arguments, ctx);
                }
                return;
            }
            if self.is_allowed_type_ident(&ident) {
                self.check_path_arguments(&segment.arguments, ctx);
                return;
            }
            self.ers.push(format!(
                "{ctx} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                path_to_string(&ty_path.path)
            ));
            self.check_path_arguments(&segment.arguments, ctx);
        }
        fn is_allowed_type_ident(&self, ident: &str) -> bool {
            ident == "Self"
                || self.repo_types.contains(ident)
                || self
                    .generic_scopes
                    .iter()
                    .rev()
                    .any(|scope| scope.contains(ident))
        }
        fn path_starts_with_allowed_type_ident(&self, path: &syn::Path) -> bool {
            path.segments.len() > 1
                && path
                    .segments
                    .first()
                    .is_some_and(|segment| self.is_allowed_type_ident(&segment.ident.to_string()))
        }
        fn pop_generics(&mut self) {
            let popped = self.generic_scopes.pop();
            assert!(popped.is_some(), "1cb23b63");
        }
        fn push_generics(&mut self, generics: &syn::Generics) {
            let mut names = std::collections::BTreeSet::new();
            for param in &generics.params {
                if let syn::GenericParam::Type(type_param) = param {
                    let _: bool = names.insert(type_param.ident.to_string());
                }
            }
            self.generic_scopes.push(names);
        }
    }
    impl<'ast> syn::visit::Visit<'ast> for DomainTypePolicyVisitor<'_> {
        fn visit_item(&mut self, i: &'ast syn::Item) {
            if has_test_only_cfg_attr(i) {
                return;
            }
            syn::visit::visit_item(self, i);
        }
        fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
            self.push_generics(&i.generics);
            for variant in &i.variants {
                self.check_fields(
                    &variant.fields,
                    &format!("enum `{}` variant", i.ident),
                    false,
                );
            }
            self.pop_generics();
        }
        fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
            if item_fn_is_proc_macro(i) {
                return;
            }
            self.check_sig(&i.sig, &format!("function `{}`", i.sig.ident));
        }
        fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
            if i.trait_.is_some() {
                return;
            }
            self.push_generics(&i.generics);
            for item in &i.items {
                if let syn::ImplItem::Fn(item_fn) = item
                    && !attrs_contain_test_only_cfg(&item_fn.attrs)
                {
                    self.check_sig(&item_fn.sig, &format!("method `{}`", item_fn.sig.ident));
                }
            }
            self.pop_generics();
        }
        fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
            self.push_generics(&i.generics);
            self.check_fields(&i.fields, &format!("struct `{}` field", i.ident), true);
            self.pop_generics();
        }
        fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
            self.push_generics(&i.generics);
            for item in &i.items {
                if let syn::TraitItem::Fn(item_fn) = item
                    && !attrs_contain_test_only_cfg(&item_fn.attrs)
                {
                    self.check_sig(
                        &item_fn.sig,
                        &format!("trait method `{}`", item_fn.sig.ident),
                    );
                }
            }
            self.pop_generics();
        }
    }
    #[test]
    fn all_crates_have_publish_false() {
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
    fn all_crates_have_workspace_lints() {
        assert_root_workspace_cargo_policy("d5f1a4e7", |path, parsed, ers| {
            match parsed
                .get("lints")
                .and_then(|v_8f2a3d6b| v_8f2a3d6b.as_table())
            {
                Some(lints_tbl) => {
                    if lints_tbl.get("workspace") != Some(&toml::Value::Boolean(true)) {
                        ers.push(format!(
                            "{}: [lints] missing `workspace = true`",
                            path.display()
                        ));
                    }
                }
                None => {
                    ers.push(format!("{}: missing [lints] section", path.display()));
                }
            }
        });
    }
    #[test]
    fn all_crates_use_edition_2024() {
        assert_root_workspace_cargo_policy("a3d7f1c8", |path, parsed, ers| {
            let edition = parsed
                .get("package")
                .and_then(|v_6d9f2a3e| v_6d9f2a3e.get("edition"))
                .and_then(toml::Value::as_str);
            if edition != Some("2024") {
                ers.push(format!("{}: edition is not \"2024\"", path.display()));
            }
        });
    }
    #[test]
    fn all_files_are_english_only() {
        let mut ers = Vec::new();
        let exceptions = [
            "../pg_crud/pg_crud_cmn/src/lib.rs", //contain utf-8 String test
            "../CODE_IMPROVEMENT_PLAN.md",
            "../DEVELOPMENT_PLAN.md",
        ];
        for el_d87f0495 in project_dir()
            .into_iter()
            .filter_entry(|el_6870bc3d| !is_ignored_dir_entry_name(el_6870bc3d.file_name()))
            .filter_map(Result::ok)
        {
            let path = el_d87f0495.path();
            if !is_allowed_english_check_file(path) {
                continue;
            }
            if is_exception(path, &exceptions) {
                continue;
            }
            let Ok(v) = std::fs::read_to_string(path) else {
                continue; //skip binary non-utf8 files
            };
            ers.extend(collect_non_english_symbol_ers(path, &v));
        }
        assert_joined_ers_empty_with_ctx(&ers, "8db37a2f", "non-english symbols:");
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
                        if let Some(syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        })) = i.args.first()
                        {
                            let v = lit_str.value();
                            if v.len() == 8 {
                                self.uuids.push(v);
                            } else {
                                self.ers.push(format!("arg len is not 8: {v}"));
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
        for_each_rs_syn_file(|path, ast| {
            let visitor = visit_syn_file(
                ast,
                ExpectVisitor {
                    method_name: expect_or_panic.method_name(),
                    uuids: Vec::new(),
                    ers: Vec::new(),
                },
            );
            all_uuids.extend(visitor.uuids);
            all_ers.extend(
                visitor
                    .ers
                    .into_iter()
                    .map(|el_2b9891bd| format!("{path:?}: {el_2b9891bd}")),
            );
        });
        let duplicates = find_duplicate_strings(&all_uuids);
        if !duplicates.is_empty() {
            all_ers.push(format!("duplicate UUIDs found: {duplicates:?}"));
        }
        assert!(all_ers.is_empty(), "6062a9e9 {all_ers:#?}",);
    }
    #[test]
    fn check_expect_contains_only_unq_uuid_v4() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Expect);
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_clippy_contains_all_clippy_lints() {
        assert_workspace_lints_match(
            RustOrClippy::Clippy,
            "clippy-driver",
            true,
            "8895ca50",
            &CLIPPY_LINT_EXCEPTIONS,
        );
    }
    #[test]
    fn check_if_workspace_cargo_toml_workspace_lints_rust_contains_all_rust_lints() {
        assert_workspace_lints_match(
            RustOrClippy::Rust,
            "rustc",
            false,
            "3c20b457",
            //todo on commit momment seems like this lints still not added to rustc, but in the list of rustc -W help
            &[
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
                "unqualified_local_imports", //need to use some kind of different test flag or something for this
                "unreachable_cfg_select_predicates",
                "default_overrides_default_fields",
                "linker_info",
                "duplicate_features",
                "deprecated_llvm_intrinsic",
                "tail_call_track_caller",
            ],
        );
    }
    #[allow(clippy::single_call_fn)] // shared lint-compare wrapper keeps clippy/rust lint test flow aligned and reduces duplicate wiring
    fn assert_workspace_lints_match(
        rust_or_clippy: RustOrClippy,
        tool: &str,
        parse_only_clippy: bool,
        exp_id: &'static str,
        exceptions: &[&str],
    ) {
        let lints_vec_from_cargo_toml = lints_vec_from_cargo_toml_workspace(rust_or_clippy);
        let lints_from_cmd = lints_from_help_cmd(tool, parse_only_clippy, exp_id);
        compare_lints_vecs(
            rust_or_clippy,
            &lints_vec_from_cargo_toml,
            &lints_from_cmd,
            exceptions,
        );
    }
    #[allow(clippy::single_call_fn)] // helper intentionally stays extracted so command parsing remains decoupled from lint comparison orchestration
    fn lints_from_help_cmd(
        tool: &str,
        parse_only_clippy: bool,
        exp_id: &'static str,
    ) -> Vec<String> {
        let output = std::process::Command::new(tool)
            .args(["-W", "help"])
            .stdout(std::process::Stdio::piped())
            .output()
            .unwrap_or_else(|_| panic!("{exp_id}"));
        assert_cmd_output_ok(&output, "95d4595a", "cc4670a2");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let regex = if parse_only_clippy {
            regex::Regex::new(r"(?m)^\s*clippy::([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                .expect("fbf14346")
        } else {
            regex::Regex::new(r"(?m)^\s*([a-z0-9][a-z0-9_-]+)\s+(allow|warn|deny|forbid)\b")
                .expect("60d99c87")
        };
        regex
            .captures_iter(&stdout)
            .map(|el_70833f93| normalize_lint_name(&el_70833f93[1]))
            .collect::<Vec<String>>()
    }
    #[allow(clippy::single_call_fn)] // shared command-output assertions keep status/stderr checks reusable for command-driven tests
    fn assert_cmd_output_ok(
        output: &std::process::Output,
        status_exp_id: &'static str,
        stderr_exp_id: &'static str,
    ) {
        assert!(output.status.success(), "{status_exp_id}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.trim().is_empty(), "{stderr_exp_id}");
    }
    #[allow(clippy::single_call_fn)] // centralizes lint-name normalization used by command output parsing
    fn normalize_lint_name(v: &str) -> String {
        v.replace('-', "_")
    }
    #[test]
    fn check_panic_contains_only_unq_uuid_v4() {
        check_expect_or_panic_contains_only_unq_uuid_v4(ExpectOrPanic::Panic);
    }
    #[test]
    fn check_rs_files_contains_only_unq_uuid_v4() {
        let rgx = regex::Regex::new(
            r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\b"
        ).expect("e098a1ff");
        let mut seen = std::collections::HashSet::new();
        for_each_rs_file_content(|_, v| {
            for el_714b3d9c in rgx.find_iter(v) {
                let uuid = uuid::Uuid::parse_str(el_714b3d9c.as_str()).expect("c9711efd");
                assert!(uuid.get_version_num() == 4, "49b49b21");
                assert!(seen.insert(uuid), "4cf9d239");
            }
        });
    }
    #[test]
    fn check_workspace_dependencies_having_exact_version() {
        let workspace = workspace_tbl_from_cargo_toml();
        for (_, v_5c36cb98) in
            toml_val_as_tbl_ref(workspace.get("dependencies").expect("2376f58e"), "e117fa5a")
        {
            validate_workspace_dep_spec(v_5c36cb98);
        }
    }
    #[allow(clippy::single_call_fn)] // keeps workspace-dependency shape checks reusable and focused in one helper
    fn validate_workspace_dep_spec(v: &toml::Value) {
        let v_tbl = toml_val_as_tbl_ref(v, "cb693a3f");
        if let Some(path_v) = v_tbl.get("path") {
            match path_v {
                toml::Value::String(_) => {
                    validate_workspace_path_dep_version(v_tbl);
                    match v_tbl.len() {
                        2 => (),
                        3 => validate_workspace_dep_default_features(v_tbl),
                        _ => panic!("f6a3b9d1 {v_tbl:#?}"),
                    }
                    return;
                }
                toml::Value::Table(_)
                | toml::Value::Integer(_)
                | toml::Value::Float(_)
                | toml::Value::Boolean(_)
                | toml::Value::Datetime(_)
                | toml::Value::Array(_) => panic!("6ca03a1f"),
            }
        }
        validate_workspace_dep_version(v_tbl);
        match v_tbl.len() {
            1 => {}
            2 => validate_workspace_dep_features_or_default_features(v_tbl),
            3 => {
                validate_workspace_dep_features(v_tbl);
                match v_tbl.get("default-features").expect("847a138f") {
                    &toml::Value::Boolean(_) => (),
                    &toml::Value::String(_)
                    | &toml::Value::Table(_)
                    | &toml::Value::Integer(_)
                    | &toml::Value::Float(_)
                    | &toml::Value::Datetime(_)
                    | &toml::Value::Array(_) => panic!("b320164b"),
                }
            }
            _ => panic!("f1139378 {v_tbl:#?}"),
        }
    }
    #[allow(clippy::single_call_fn)] // path workspace deps must keep concrete package versions for external tooling policy checks
    fn validate_workspace_path_dep_version(v_tbl: &toml::value::Table) {
        match v_tbl.get("version").expect("bf2e4a7c") {
            toml::Value::String(version_string) => {
                assert_eq!(version_string, "0.1.0", "8c3d5f91");
            }
            toml::Value::Table(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("a6c7e3d2"),
        }
    }
    #[allow(clippy::single_call_fn)] // keeps two-key dependency tables strict while allowing featureless default-features opt-out
    fn validate_workspace_dep_features_or_default_features(v_tbl: &toml::value::Table) {
        if v_tbl.contains_key("features") {
            validate_workspace_dep_features(v_tbl);
        } else {
            validate_workspace_dep_default_features(v_tbl);
        }
    }
    #[allow(clippy::single_call_fn)] // shared shape check for dependency tables that explicitly opt out of default features
    fn validate_workspace_dep_default_features(v_tbl: &toml::value::Table) {
        match v_tbl.get("default-features").expect("d2a8c4e1") {
            &toml::Value::Boolean(_) => (),
            &toml::Value::String(_)
            | &toml::Value::Table(_)
            | &toml::Value::Integer(_)
            | &toml::Value::Float(_)
            | &toml::Value::Datetime(_)
            | &toml::Value::Array(_) => panic!("e5f7b1c3"),
        }
    }
    #[allow(clippy::single_call_fn)] // separates version shape assertion from dependency-table flow and keeps IDs stable
    fn validate_workspace_dep_version(v_tbl: &toml::value::Table) {
        match v_tbl.get("version").expect("d5b2b269") {
            toml::Value::String(version_string) => {
                assert!(is_exact_three_part_version(version_string), "6640b9bf");
            }
            toml::Value::Table(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("a3410a37"),
        }
    }
    #[allow(clippy::single_call_fn)] // extracted to avoid repeated feature-type checks for dependency tables
    fn validate_workspace_dep_features(v_tbl: &toml::value::Table) {
        match v_tbl.get("features").expect("473577d5") {
            &toml::Value::Array(_) => (),
            &toml::Value::String(_)
            | &toml::Value::Table(_)
            | &toml::Value::Integer(_)
            | &toml::Value::Float(_)
            | &toml::Value::Boolean(_)
            | &toml::Value::Datetime(_) => panic!("38ba32e9"),
        }
    }
    #[allow(clippy::single_call_fn)] // isolates exact-version parsing so version-format checks are reusable and testable
    fn is_exact_three_part_version(v: &str) -> bool {
        let Some(rest) = v.strip_prefix('=') else {
            return false;
        };
        let mut iter = rest.split('.');
        if !take_next_u64_part(&mut iter)
            || !take_next_u64_part(&mut iter)
            || !take_next_u64_part(&mut iter)
        {
            return false;
        }
        iter.next().is_none()
    }
    #[allow(clippy::single_call_fn)] // keeps exact-version parser steps reusable while avoiding repeated parse blocks
    fn take_next_u64_part(iter: &mut std::str::Split<'_, char>) -> bool {
        iter.next()
            .and_then(|part| part.parse::<u64>().ok())
            .is_some()
    }
    #[allow(clippy::single_call_fn)] // helper intentionally stays extracted so lint diff logic remains reusable and independently readable
    fn compare_lints_vecs(
        rust_or_clippy: RustOrClippy,
        lints_vec_from_cargo_toml: &[String],
        lints_to_check: &[String],
        lints_not_in_cargo_toml_vec_exceptions: &[&str],
    ) {
        let rust_or_clippy_name = rust_or_clippy.name();
        let lints_from_cargo_set = str_set(lints_vec_from_cargo_toml);
        let lints_to_check_set = str_set(lints_to_check);
        let lints_exceptions_set = lints_not_in_cargo_toml_vec_exceptions
            .iter()
            .copied()
            .collect::<std::collections::HashSet<&str>>();
        let (lints_not_in_cargo_toml, lints_missing_by_exception) = split_lints_missing_from_cargo(
            lints_to_check,
            &lints_from_cargo_set,
            &lints_exceptions_set,
        );
        for lint in lints_missing_by_exception {
            println!("todo!() {rust_or_clippy_name} {lint} 158b5c43-05fa-4b8f-b6fe-9cda49d26997");
        }
        assert!(
            lints_not_in_cargo_toml.is_empty(),
            "d2b7ba9f {lints_not_in_cargo_toml:?}"
        );
        let outdated_lints_in_file =
            collect_missing_items(lints_vec_from_cargo_toml, &lints_to_check_set);
        assert!(outdated_lints_in_file.is_empty(), "93787d2d");
    }
    #[allow(clippy::single_call_fn)] // shared parser keeps .env line-to-key extraction reusable and test behavior centralized
    fn parse_env_key_line(line: &str) -> Option<&str> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }
        trimmed.split_once('=').map(|(key, _)| key)
    }
    fn env_keys_from_file(path: &str) -> Vec<String> {
        std::fs::read_to_string(path)
            .expect("b3a7c1e4")
            .lines()
            .filter_map(parse_env_key_line)
            .map(str::to_owned)
            .collect()
    }
    #[test]
    fn env_and_envexample_have_same_keys() {
        let env_keys = env_keys_from_file("../server/.env");
        let example_keys = env_keys_from_file("../server/.envexample");
        let env_keys_set = str_set(&env_keys);
        let example_keys_set = str_set(&example_keys);
        let mut ers = collect_missing_key_ers(&env_keys, &example_keys_set, ".env", ".envexample");
        ers.extend(collect_missing_key_ers(
            &example_keys,
            &env_keys_set,
            ".envexample",
            ".env",
        ));
        assert_joined_ers_empty_sorted(&mut ers, "c8d2f1a3");
    }
    #[allow(clippy::single_call_fn)] // shared set-difference collector keeps missing-item checks reusable across lint and env-key tests
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
    #[allow(clippy::single_call_fn)] // centralized formatter keeps env key mismatch diagnostics consistent
    fn collect_missing_key_ers(
        source_keys: &[String],
        target_set: &std::collections::HashSet<&str>,
        source_file: &str,
        target_file: &str,
    ) -> Vec<String> {
        collect_missing_items(source_keys, target_set)
            .into_iter()
            .map(|key| format!("key `{key}` in {source_file} but missing from {target_file}"))
            .collect::<Vec<String>>()
    }
    #[allow(clippy::single_call_fn)] // split keeps lint exception handling explicit while reusing missing-item collection
    fn split_lints_missing_from_cargo<'lints>(
        lints_to_check: &'lints [String],
        lints_from_cargo_set: &std::collections::HashSet<&str>,
        lints_exceptions_set: &std::collections::HashSet<&str>,
    ) -> (Vec<&'lints str>, Vec<&'lints str>) {
        let mut lints_not_in_cargo_toml = Vec::new();
        let mut lints_missing_by_exception = Vec::new();
        for lint in collect_missing_items(lints_to_check, lints_from_cargo_set) {
            if lints_exceptions_set.contains(lint) {
                lints_missing_by_exception.push(lint);
            } else {
                lints_not_in_cargo_toml.push(lint);
            }
        }
        (lints_not_in_cargo_toml, lints_missing_by_exception)
    }
    fn is_exception(path: &std::path::Path, exceptions: &[&str]) -> bool {
        exceptions.iter().any(|exception| path.ends_with(exception))
    }
    #[allow(clippy::single_call_fn)] // helper intentionally stays extracted so workspace-lints table parsing remains separate from test driver wiring
    fn lints_vec_from_cargo_toml_workspace(rust_or_clippy: RustOrClippy) -> Vec<String> {
        let workspace = workspace_tbl_from_cargo_toml();
        let lints = toml_val_as_tbl_ref(workspace.get("lints").expect("82eaea37"), "cae226cd");
        let toml_v_tbl = toml_val_as_tbl_ref(
            lints.get(rust_or_clippy.name()).expect("dbd02f72"),
            "6f4580ce",
        );
        toml_v_tbl.keys().cloned().collect::<Vec<String>>()
    }
    #[allow(clippy::single_call_fn)] // reusable collector stays split from assertion helper for callsites that need raw error vectors
    fn collect_cargo_toml_ers(
        exceptions: &[&str],
        mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
    ) -> Vec<String> {
        let mut ers = Vec::new();
        for_each_cargo_toml_project_file(exceptions, |path| {
            let Some(parsed) = read_toml_table(path) else {
                return;
            };
            mk_ers(path, &parsed, &mut ers);
        });
        ers
    }
    #[allow(clippy::single_call_fn)] // centralizes repeated cargo-toml assertion shape used by multiple tests
    fn assert_cargo_toml_ers_empty(
        exceptions: &[&str],
        exp_id: &'static str,
        mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
    ) {
        let ers = collect_cargo_toml_ers(exceptions, |path, parsed, ers| {
            mk_ers(path, parsed, ers);
        });
        assert_joined_ers_empty(&ers, exp_id);
    }
    #[allow(clippy::single_call_fn)] // shared workspace-root cargo policy assertion keeps root exceptions and joined-diagnostic behavior consistent across package-metadata checks
    fn assert_root_workspace_cargo_policy(
        exp_id: &'static str,
        mut mk_ers: impl FnMut(&std::path::Path, &toml::Table, &mut Vec<String>),
    ) {
        assert_cargo_toml_ers_empty(&ROOT_CARGO_TOML_EXCEPTIONS, exp_id, |path, parsed, ers| {
            mk_ers(path, parsed, ers);
        });
    }
    #[allow(clippy::single_call_fn)] // shared joined-error assertion keeps multi-line diagnostics consistent across workspace policy tests
    fn assert_joined_ers_empty(ers: &[String], exp_id: &'static str) {
        assert_joined_ers_empty_with_ctx(ers, exp_id, "");
    }
    #[allow(clippy::single_call_fn)] // shared assertion with context keeps multiline diagnostics reusable without duplicating message-format glue
    fn assert_joined_ers_empty_with_ctx(ers: &[String], exp_id: &'static str, ctx: &str) {
        if ctx.is_empty() {
            assert!(ers.is_empty(), "{exp_id}\n{}", ers.join("\n"));
        } else {
            assert!(ers.is_empty(), "{exp_id} {ctx}\n{}", ers.join("\n"));
        }
    }
    #[allow(clippy::single_call_fn)] // shared sort+assert helper keeps joined diagnostics deterministic for tests that accumulate path-dependent errors
    fn assert_joined_ers_empty_sorted(ers: &mut [String], exp_id: &'static str) {
        ers.sort();
        assert_joined_ers_empty(ers, exp_id);
    }
    #[allow(clippy::single_call_fn)] // shared helper avoids repeated conversion of vec<string> into set<&str>
    fn str_set(v: &[String]) -> std::collections::HashSet<&str> {
        v.iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<&str>>()
    }
    #[allow(clippy::single_call_fn)] // shared duplicate finder keeps uniqueness checks reusable and consistent
    fn find_duplicate_strings(v: &[String]) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for el_45f4b8bc in v {
            if !seen.insert(el_45f4b8bc.as_str()) {
                duplicates.push(el_45f4b8bc.to_owned());
            }
        }
        duplicates
    }
    #[allow(clippy::single_call_fn)] // reusable collector stays available for AST-policy tests and keeps collection logic separate from assertion wrappers
    fn collect_rs_ast_ers(
        mut mk_ers: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
    ) -> Vec<String> {
        let mut ers = Vec::new();
        for_each_rs_syn_file(|path, ast| {
            mk_ers(path, ast, &mut ers);
        });
        ers
    }
    #[allow(clippy::single_call_fn)] // shared visitor runner keeps AST test callsites focused on assertion logic rather than visit boilerplate
    fn visit_syn_file<V>(ast: &syn::File, mut visitor: V) -> V
    where
        V: for<'ast> syn::visit::Visit<'ast>,
    {
        syn::visit::Visit::visit_file(&mut visitor, ast);
        visitor
    }
    #[allow(clippy::single_call_fn)] // shared assertion wrapper keeps AST-policy tests focused on visitor logic while reusing collection and joined-report formatting
    fn assert_rs_ast_ers_empty_with_ctx(
        exp_id: &'static str,
        ctx: &str,
        mut mk_ers: impl FnMut(&std::path::Path, &syn::File, &mut Vec<String>),
    ) {
        let ers = collect_rs_ast_ers(|path, ast, ers| {
            mk_ers(path, ast, ers);
        });
        assert_joined_ers_empty_with_ctx(&ers, exp_id, ctx);
    }
    #[allow(clippy::single_call_fn)] // shared parser keeps Cargo.toml read+parse behavior centralized for policy collectors
    fn read_toml_table(path: &std::path::Path) -> Option<toml::Table> {
        let v = std::fs::read_to_string(path).ok()?;
        v.parse::<toml::Table>().ok()
    }
    #[test]
    fn no_dbg_macro_in_source_code() {
        assert_rs_ast_ers_empty_with_ctx("f1c7a4e3", "dbg!() found:", |path, ast, ers| {
            let visitor = visit_syn_file(ast, DbgVisitor { found: false });
            if visitor.found {
                ers.push(format!("{}: contains dbg!()", path.display()));
            }
        });
    }
    #[test]
    fn no_empty_lines_in_rust_files() {
        let mut ers = Vec::new();
        for_each_rs_file_content(|path, v| {
            ers.extend(collect_empty_line_ers(path, v));
        });
        assert_joined_ers_empty_with_ctx(&ers, "3d2fc8a1", "empty lines found in Rust files:");
    }
    #[allow(clippy::single_call_fn)] // isolates empty-line diagnostics so file-level test stays focused on traversal and assertion
    fn collect_empty_line_ers(path: &std::path::Path, v: &str) -> Vec<String> {
        let mut lines_iter = v.lines();
        if let Some(first_line) = lines_iter.next()
            && first_line.trim().is_empty()
            && lines_iter.next().is_none()
        {
            return Vec::new();
        }
        v.lines()
            .enumerate()
            .filter(|(_, line)| line.trim().is_empty())
            .map(|(line_nbr, _)| {
                format!(
                    "{}:{} empty line",
                    path.display(),
                    line_nbr.saturating_add(1)
                )
            })
            .collect::<Vec<String>>()
    }
    #[allow(clippy::single_call_fn)] // isolates non-english diagnostics so file-level test stays focused on traversal and assertion
    fn collect_non_english_symbol_ers(path: &std::path::Path, v: &str) -> Vec<String> {
        let mut ers = Vec::new();
        for (line_idx, line) in v.lines().enumerate() {
            let line_number = line_idx.saturating_add(1);
            for ch in line.chars() {
                if !is_allowed_english_char(ch) {
                    ers.push(format!(
                        "{}:{} non-english symbol `{}` (U+{:04X})",
                        path.display(),
                        line_number,
                        ch,
                        u32::from(ch)
                    ));
                }
            }
        }
        ers
    }
    #[allow(clippy::single_call_fn)] // shared character predicate keeps english-only symbol policy centralized
    fn is_allowed_english_char(ch: char) -> bool {
        matches!(ch, '\n' | '\r' | '\t' | '\u{2014}' | '\u{2194}') || ch.is_ascii()
    }
    #[test]
    fn no_todo_or_unimplemented_macro_in_source_code() {
        assert_rs_ast_ers_empty_with_ctx(
            "c4e9a2d7",
            "todo!/unimplemented! found:",
            |path, ast, ers| {
                let visitor = visit_syn_file(
                    ast,
                    TodoUnimplVisitor {
                        todo_found: 0,
                        unimplemented_found: 0,
                    },
                );
                push_repeated_file_er(ers, path, "contains todo!()", visitor.todo_found);
                push_repeated_file_er(
                    ers,
                    path,
                    "contains unimplemented!()",
                    visitor.unimplemented_found,
                );
            },
        );
    }
    #[test]
    fn no_macro_rules_in_source_code() {
        let macro_name = "macro_rules";
        let forbidden = format!("{macro_name}!");
        let mut ers = Vec::new();
        for_each_rs_file_content(|path, v| {
            if v.contains(&forbidden) {
                ers.push(format!(
                    "{}: contains {forbidden}; use a workspace proc-macro crate instead",
                    path.display()
                ));
            }
        });
        assert_joined_ers_empty_with_ctx(
            &ers,
            "b6e2a9f4",
            "macro_rules found; use workspace proc-macro crates instead:",
        );
    }
    #[test]
    fn no_include_asset_macros_outside_allowlist() {
        assert_rs_ast_ers_empty_with_ctx(
            "a6d4f2c9",
            "include_str!() or include_bytes!() found outside explicit generated/test fixture allowlist:",
            |path, ast, ers| {
                if is_exception(path, &INCLUDE_ASSET_MACRO_SOURCE_EXCEPTIONS) {
                    return;
                }
                let visitor = visit_syn_file(ast, IncludeAssetMacroVisitor { ers: Vec::new() });
                ers.extend(visitor.ers.into_iter().map(|er| {
                    format!(
                        "{}: {er}; add only generated/test fixture files to INCLUDE_ASSET_MACRO_SOURCE_EXCEPTIONS",
                        path.display()
                    )
                }));
            },
        );
    }
    #[test]
    fn no_non_public_use_imports_in_rust_sources() {
        assert_rs_ast_ers_empty_with_ctx(
            "b4e7c2a9",
            "use imports found outside explicit facade re-export files; prefer explicit paths at usage sites:",
            |path, ast, ers| {
                let visitor = visit_syn_file(
                    ast,
                    UseImportVisitor {
                        found_non_public_use_import: false,
                        found_use_rename: false,
                        public_use_roots: Vec::new(),
                    },
                );
                if visitor.found_non_public_use_import {
                    ers.push(format!(
                        "{}: found non-public use import; use the explicit path at the usage site",
                        path.display()
                    ));
                }
                let local_mod_names = ast
                    .items
                    .iter()
                    .filter_map(|item| {
                        if let syn::Item::Mod(item_mod) = item {
                            Some(item_mod.ident.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<std::collections::HashSet<_>>();
                if !is_public_reexport_source_path(path) {
                    for public_use_root in &visitor.public_use_roots {
                        if !local_mod_names.contains(public_use_root) {
                            ers.push(format!(
                                "{}: found public use import rooted at `{public_use_root}` outside facade re-export allowlist; use the explicit path at the usage site or add only intentional facade files to PUBLIC_REEXPORT_SOURCE_INCLUSIONS",
                                path.display()
                            ));
                        }
                    }
                }
                if visitor.found_use_rename {
                    ers.push(format!(
                        "{}: found use rename with `as`; use the original item name or rename the item at its definition",
                        path.display()
                    ));
                }
            },
        );
    }
    #[test]
    fn no_type_aliases_in_rust_sources() {
        assert_rs_ast_ers_empty_with_ctx(
            "c6e4f7a1",
            "type aliases found; use explicit types at usage sites:",
            |path, ast, ers| {
                let visitor = visit_syn_file(ast, TypeAliasVisitor { ers: Vec::new() });
                ers.extend(
                    visitor
                        .ers
                        .into_iter()
                        .map(|er| format!("{}: {er}", path.display())),
                );
            },
        );
    }
    #[test]
    fn domain_boundaries_use_repository_declared_types() {
        let repo_types = declared_domain_type_names();
        assert_rs_ast_ers_empty_with_ctx(
            "a7f9c3e1",
            "raw external or primitive types found in domain boundaries; use repository domain wrapper types initialized with From/TryFrom:",
            |path, ast, ers| {
                if !domain_type_policy_should_check_path(path) {
                    return;
                }
                let visitor = visit_syn_file(
                    ast,
                    DomainTypePolicyVisitor {
                        ers: Vec::new(),
                        generic_scopes: Vec::new(),
                        repo_types: &repo_types,
                    },
                );
                ers.extend(
                    visitor
                        .ers
                        .into_iter()
                        .map(|er| format!("{}: {er}", path.display())),
                );
            },
        );
    }
    #[test]
    fn no_unwrap_in_source_code() {
        assert_rs_ast_ers_empty_with_ctx("e8b3a6d2", "unwrap() found:", |path, ast, ers| {
            let visitor = visit_syn_file(ast, UnwrapVisitor { found_count: 0 });
            push_repeated_file_er(ers, path, "unwrap() call", visitor.found_count);
        });
    }
    #[test]
    fn runtime_code_does_not_use_expect_unwrap_or_panic() {
        assert_rs_ast_ers_empty_with_ctx(
            "c71f2a8d",
            "runtime code contains forbidden expect/unwrap/panic calls; use Result with a \
             thiserror-like error enum instead:",
            |path, ast, ers| {
                if !is_runtime_policy_source_path(path) {
                    return;
                }
                let visitor =
                    visit_syn_file(ast, RuntimePanicExpectUnwrapVisitor { ers: Vec::new() });
                ers.extend(
                    visitor
                        .ers
                        .into_iter()
                        .map(|er| format!("{}: {er}", path.display())),
                );
            },
        );
    }
    #[test]
    fn runtime_code_does_not_use_mutex() {
        assert_rs_ast_ers_empty_with_ctx(
            "e3f8a1c5",
            "runtime code contains Mutex; use it only for justified interior mutability:",
            |path, ast, ers| {
                if !is_runtime_policy_source_path(path) {
                    return;
                }
                let visitor = visit_syn_file(ast, RuntimeMutexVisitor { found_count: 0 });
                push_repeated_file_er(ers, path, "Mutex type usage", visitor.found_count);
            },
        );
    }
    #[test]
    fn runtime_arc_usage_is_limited_to_cross_thread_state() {
        assert_rs_ast_ers_empty_with_ctx(
            "f9c2d4a8",
            "runtime Arc usage must be limited to explicit cross-thread shared state:",
            |path, ast, ers| {
                if !is_runtime_policy_source_path(path) {
                    return;
                }
                let visitor = visit_syn_file(
                    ast,
                    RuntimeArcVisitor {
                        allow_arc_value_usage: path.ends_with("server/src/main.rs"),
                        ers: Vec::new(),
                    },
                );
                ers.extend(
                    visitor
                        .ers
                        .into_iter()
                        .map(|er| format!("{}: {er}", path.display())),
                );
            },
        );
    }
    #[test]
    fn async_functions_do_not_make_blocking_executor_calls() {
        assert_rs_ast_ers_empty_with_ctx(
            "a8e1c6f3",
            "async functions contain blocking executor calls:",
            |path, ast, ers| {
                if !is_runtime_policy_source_path(path) {
                    return;
                }
                let visitor = visit_syn_file(
                    ast,
                    AsyncBlockingCallVisitor {
                        async_fn_depth: 0,
                        ers: Vec::new(),
                    },
                );
                ers.extend(
                    visitor
                        .ers
                        .into_iter()
                        .map(|er| format!("{}: {er}", path.display())),
                );
            },
        );
    }
    #[test]
    fn unit_tests_do_not_create_external_service_clients() {
        assert_rs_ast_ers_empty_with_ctx(
            "d1f5b9c7",
            "unit tests contain external-service clients; use deterministic local fakes instead:",
            |path, ast, ers| {
                let visitor = visit_syn_file(
                    ast,
                    UnitTestExternalServiceVisitor {
                        test_depth: 0,
                        ers: Vec::new(),
                    },
                );
                ers.extend(
                    visitor
                        .ers
                        .into_iter()
                        .map(|er| format!("{}: {er}", path.display())),
                );
            },
        );
    }
    #[allow(clippy::single_call_fn)] // shared repeated-file error helper keeps AST visitor diagnostics consistent
    fn push_repeated_file_er(
        ers: &mut Vec<String>,
        path: &std::path::Path,
        msg: &str,
        times: usize,
    ) {
        for _ in 0..times {
            ers.push(format!("{}: {msg}", path.display()));
        }
    }
    fn project_dir() -> walkdir::WalkDir {
        walkdir::WalkDir::new("../")
    }
    #[allow(clippy::single_call_fn)] // shared ignore predicate keeps directory filtering rules consistent across walkers
    fn is_ignored_dir_entry_name(name: &std::ffi::OsStr) -> bool {
        name == "target" || name == ".git"
    }
    #[allow(clippy::single_call_fn)] // shared traversal keeps Cargo.toml filtering rules centralized while avoiding temporary vec allocation
    fn for_each_cargo_toml_project_file(
        exceptions: &[&str],
        mut on_file: impl FnMut(&std::path::Path),
    ) {
        for entry in project_dir()
            .into_iter()
            .filter_entry(|el| !is_ignored_dir_entry_name(el.file_name()))
            .filter_map(Result::ok)
            .filter(|el| el.file_name() == "Cargo.toml")
            .filter(|el| !is_exception(el.path(), exceptions))
        {
            on_file(entry.path());
        }
    }
    #[allow(clippy::single_call_fn)] // iterator builder is intentionally separated for readability and traversal reuse entrypoint
    fn rs_project_files() -> impl Iterator<Item = walkdir::DirEntry> {
        project_dir()
            .into_iter()
            .filter_entry(|el| {
                !is_ignored_dir_entry_name(el.file_name())
                    && (el.file_type().is_dir() || is_rs_file_path(el.path()))
            })
            .filter_map(Result::ok)
            .filter(|el| is_rs_file_path(el.path()))
    }
    #[allow(clippy::single_call_fn)] // shared extension gate keeps english-only file selection centralized and reusable
    fn is_allowed_english_check_file(path: &std::path::Path) -> bool {
        path.is_file()
            && is_allowed_english_check_ext(path.extension().and_then(std::ffi::OsStr::to_str))
    }
    #[allow(clippy::single_call_fn)] // shared extension predicate keeps source-policy file-kind checks consistent
    fn is_allowed_english_check_ext(ext: Option<&str>) -> bool {
        matches!(
            ext,
            Some("rs" | "toml" | "md" | "txt" | "yml" | "yaml" | "json")
        )
    }
    #[allow(clippy::single_call_fn)] // shared rust-extension predicate keeps rs walker filters consistent
    fn is_rs_file_path(path: &std::path::Path) -> bool {
        path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs")
    }
    fn path_has_segment(path: &syn::Path, segment: &str) -> bool {
        path.segments.iter().any(|el| el.ident == segment)
    }
    fn path_ends_with(path: &syn::Path, segments: &[&str]) -> bool {
        path.segments.len() >= segments.len()
            && path
                .segments
                .iter()
                .rev()
                .zip(segments.iter().rev())
                .all(|(got, exp)| got.ident == *exp)
    }
    #[allow(clippy::single_call_fn)] // names Self-path handling separately from domain type path traversal
    fn path_first_segment_is_self(path: &syn::Path) -> bool {
        path.segments
            .first()
            .is_some_and(|segment| segment.ident == "Self")
    }
    fn expr_call_path(call: &syn::ExprCall) -> Option<&syn::Path> {
        match call.func.as_ref() {
            syn::Expr::Path(path) => Some(&path.path),
            syn::Expr::Array(_)
            | syn::Expr::Assign(_)
            | syn::Expr::Async(_)
            | syn::Expr::Await(_)
            | syn::Expr::Binary(_)
            | syn::Expr::Block(_)
            | syn::Expr::Break(_)
            | syn::Expr::Call(_)
            | syn::Expr::Cast(_)
            | syn::Expr::Closure(_)
            | syn::Expr::Const(_)
            | syn::Expr::Continue(_)
            | syn::Expr::Field(_)
            | syn::Expr::ForLoop(_)
            | syn::Expr::Group(_)
            | syn::Expr::If(_)
            | syn::Expr::Index(_)
            | syn::Expr::Infer(_)
            | syn::Expr::Let(_)
            | syn::Expr::Lit(_)
            | syn::Expr::Loop(_)
            | syn::Expr::Macro(_)
            | syn::Expr::Match(_)
            | syn::Expr::MethodCall(_)
            | syn::Expr::Paren(_)
            | syn::Expr::Range(_)
            | syn::Expr::RawAddr(_)
            | syn::Expr::Reference(_)
            | syn::Expr::Repeat(_)
            | syn::Expr::Return(_)
            | syn::Expr::Struct(_)
            | syn::Expr::Try(_)
            | syn::Expr::TryBlock(_)
            | syn::Expr::Tuple(_)
            | syn::Expr::Unary(_)
            | syn::Expr::Unsafe(_)
            | syn::Expr::Verbatim(_)
            | syn::Expr::While(_)
            | syn::Expr::Yield(_)
            | _ => None,
        }
    }
    #[allow(clippy::single_call_fn)] // extracts repo macro domain type discovery from the visitor traversal
    fn collect_gen_pg_types_domain_names(
        tokens: &str,
        names: &mut std::collections::BTreeSet<String>,
    ) {
        let re = regex::Regex::new("\"([A-Za-z0-9]+As[A-Za-z0-9]+)\"").expect("f4e61b29");
        for captures in re.captures_iter(tokens) {
            let Some(base) = captures.get(1).map(|el| el.as_str()) else {
                continue;
            };
            let Some((prefix, suffix)) = base.split_once("As") else {
                continue;
            };
            let _: bool = names.insert(format!("{prefix}AsNn{suffix}"));
            let _: bool = names.insert(format!("Opt{prefix}AsNl{suffix}"));
        }
    }
    #[allow(clippy::single_call_fn)] // config_lib helper macros declare domain wrapper structs from their first argument
    fn config_lib_domain_type_macro_path(path: &syn::Path) -> bool {
        path_ends_with(
            path,
            &["config_lib_macros", "impl_try_from_non_empty_string"],
        ) || path_ends_with(path, &["config_lib_macros", "impl_try_from_secret_url"])
            || path_ends_with(path, &["config_lib_macros", "impl_try_from_parse"])
            || path_ends_with(
                path,
                &["config_lib_macros", "impl_try_from_parse_string_er"],
            )
    }
    #[allow(clippy::single_call_fn)] // macro-generated domain wrapper names are the first ident in these macro inputs
    fn collect_first_macro_ident_domain_name(
        tokens: &str,
        names: &mut std::collections::BTreeSet<String>,
    ) {
        let re = regex::Regex::new(r"^\s*([A-Za-z][A-Za-z0-9_]*)\s*,").expect("fc65b7c4");
        if let Some(name) = re
            .captures(tokens)
            .and_then(|captures| captures.get(1))
            .map(|name| name.as_str())
        {
            let _: bool = names.insert(name.to_owned());
        }
    }
    #[allow(clippy::single_call_fn)] // keeps Arc type policy readable apart from syn type matching
    fn type_contains_segment(ty: &syn::Type, segment: &str) -> bool {
        match ty {
            syn::Type::Path(path) => path_has_segment(&path.path, segment),
            syn::Type::Array(_)
            | syn::Type::BareFn(_)
            | syn::Type::Group(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Paren(_)
            | syn::Type::Ptr(_)
            | syn::Type::Reference(_)
            | syn::Type::Slice(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Tuple(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        }
    }
    #[allow(clippy::single_call_fn)] // names the async-blocking method policy separately from traversal code
    fn method_is_blocking_async_call(method: &str) -> bool {
        matches!(
            method,
            "block_on" | "block_in_place" | "blocking_recv" | "blocking_send"
        )
    }
    #[allow(clippy::single_call_fn)] // names the async-blocking function policy separately from traversal code
    fn path_is_blocking_async_call(path: &syn::Path) -> bool {
        path_ends_with(path, &["futures", "executor", "block_on"])
            || path_ends_with(path, &["tokio", "task", "block_in_place"])
            || path_ends_with(path, &["std", "thread", "sleep"])
    }
    #[allow(clippy::single_call_fn)] // names the external-service unit-test policy separately from traversal code
    fn path_is_external_service_client(path: &syn::Path) -> bool {
        path_ends_with(path, &["reqwest", "Client", "new"])
            || path_ends_with(path, &["std", "net", "TcpStream", "connect"])
            || path_ends_with(path, &["std", "net", "TcpListener", "bind"])
            || path_ends_with(path, &["std", "net", "UdpSocket", "bind"])
            || path_ends_with(path, &["tokio", "net", "TcpStream", "connect"])
            || path_ends_with(path, &["tokio", "net", "TcpListener", "bind"])
            || path_ends_with(path, &["tokio", "net", "UdpSocket", "bind"])
    }
    #[allow(clippy::single_call_fn)] // extracted to keep the domain policy test focused on assertion flow
    fn declared_domain_type_names() -> std::collections::BTreeSet<String> {
        let mut names = std::collections::BTreeSet::new();
        for_each_rs_syn_file(|_, ast| {
            let visitor = visit_syn_file(
                ast,
                DeclaredDomainTypeVisitor {
                    names: std::collections::BTreeSet::new(),
                },
            );
            names.extend(visitor.names);
        });
        names
    }
    #[allow(clippy::single_call_fn)] // keeps phased domain policy rollout controlled from one predicate
    fn domain_type_policy_should_check_path(path: &std::path::Path) -> bool {
        if is_exception(path, DOMAIN_TYPE_POLICY_SOURCE_INCLUSIONS) {
            return true;
        }
        if std::env::var_os("DOMAIN_TYPE_POLICY_CHECK_ALL").is_none() {
            return false;
        }
        let Some(cargo_toml_path) = nearest_cargo_toml_path(path) else {
            return false;
        };
        let Some(parsed) = read_toml_table(&cargo_toml_path) else {
            return false;
        };
        !is_test_crate(&parsed)
    }
    #[allow(clippy::single_call_fn)] // keeps public re-export allowlist separate from use-import visitor diagnostics
    fn is_public_reexport_source_path(path: &std::path::Path) -> bool {
        is_exception(path, PUBLIC_REEXPORT_SOURCE_INCLUSIONS)
    }
    #[allow(clippy::single_call_fn)] // keeps transparent container policy separate from path validation
    fn is_structural_generic_container(ident: &str) -> bool {
        matches!(
            ident,
            "Option"
                | "Result"
                | "Vec"
                | "Box"
                | "Cow"
                | "Arc"
                | "Rc"
                | "Pin"
                | "PhantomData"
                | "HashMap"
                | "BTreeMap"
                | "HashSet"
                | "BTreeSet"
        )
    }
    #[allow(clippy::single_call_fn)] // proc-macro entrypoints must keep the compiler-required TokenStream ABI
    fn item_fn_is_proc_macro(item: &syn::ItemFn) -> bool {
        item.attrs.iter().any(|attr| {
            attr.path().is_ident("proc_macro")
                || attr.path().is_ident("proc_macro_derive")
                || attr.path().is_ident("proc_macro_attribute")
        })
    }
    #[allow(clippy::single_call_fn)] // shared attr-list wrapper avoids exposing cfg-test matching at visitor callsites
    fn attrs_contain_test_only_cfg(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(attr_is_test_only_cfg)
    }
    #[allow(clippy::single_call_fn)] // keeps unit-test detection reusable inside nested test module traversal
    fn item_fn_is_unit_test(item: &syn::ItemFn) -> bool {
        item.attrs
            .iter()
            .any(|attr| attr.path().is_ident("test") || attr_is_test_only_cfg(attr))
    }
    #[allow(clippy::single_call_fn)] // keeps external-service error messages stable and readable
    fn path_to_string(path: &syn::Path) -> String {
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<String>>()
            .join("::")
    }
    #[allow(clippy::single_call_fn)] // centralizes production-source filtering for panic/expect/unwrap policy
    fn is_runtime_policy_source_path(path: &std::path::Path) -> bool {
        if path.file_name().and_then(std::ffi::OsStr::to_str) == Some("test_hlp.rs") {
            return false;
        }
        if !path
            .components()
            .any(|component| component.as_os_str() == "src")
        {
            return false;
        }
        let Some(cargo_toml_path) = nearest_cargo_toml_path(path) else {
            return false;
        };
        let Some(parsed) = read_toml_table(&cargo_toml_path) else {
            return false;
        };
        !is_proc_macro_crate(&parsed) && !is_test_crate(&parsed)
    }
    #[allow(clippy::single_call_fn)] // walks upward from a source file to the owning crate manifest
    fn nearest_cargo_toml_path(path: &std::path::Path) -> Option<std::path::PathBuf> {
        for ancestor in path.ancestors() {
            let cargo_toml_path = ancestor.join("Cargo.toml");
            if cargo_toml_path.exists() {
                return Some(cargo_toml_path);
            }
        }
        None
    }
    #[allow(clippy::single_call_fn)] // package-name based test crate filter keeps generated/test-only crates outside runtime policy
    fn is_test_crate(parsed: &toml::Table) -> bool {
        parsed
            .get("package")
            .and_then(toml::Value::as_table)
            .and_then(|package| package.get("name"))
            .and_then(toml::Value::as_str)
            .is_some_and(|name| name == "tests" || name.contains("_test") || name.ends_with("test"))
    }
    #[allow(clippy::single_call_fn)] // proc-macro crates are allowed to panic by repository policy
    fn is_proc_macro_crate(parsed: &toml::Table) -> bool {
        parsed
            .get("lib")
            .and_then(toml::Value::as_table)
            .and_then(|lib| lib.get("proc-macro"))
            == Some(&toml::Value::Boolean(true))
    }
    #[allow(clippy::single_call_fn)] // keeps cfg(test) handling local to runtime AST policy visitor
    fn has_test_only_cfg_attr(i: &syn::Item) -> bool {
        match i {
            syn::Item::Const(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Enum(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::ExternCrate(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Fn(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::ForeignMod(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Impl(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Macro(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Mod(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Static(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Struct(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Trait(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::TraitAlias(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Type(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Union(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Use(item) => item.attrs.iter().any(attr_is_test_only_cfg),
            syn::Item::Verbatim(_) | _ => false,
        }
    }
    #[allow(clippy::single_call_fn)] // accepts cfg(test) and cfg(feature = "test-utils") as non-production code
    fn attr_is_test_only_cfg(attr: &syn::Attribute) -> bool {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        let mut is_test_only_cfg = false;
        drop(attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("test") {
                is_test_only_cfg = true;
            }
            if meta.path.is_ident("feature") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                if lit.value() == "test-utils" {
                    is_test_only_cfg = true;
                }
            }
            Ok(())
        }));
        is_test_only_cfg
    }
    #[allow(clippy::single_call_fn)] // shared rust-file reader keeps skip-on-read-error behavior centralized across source policy checks
    fn for_each_rs_file_content(mut on_file: impl FnMut(&std::path::Path, &str)) {
        for entry in rs_project_files() {
            let path = entry.path();
            let Ok(v) = std::fs::read_to_string(path) else {
                continue;
            };
            on_file(path, &v);
        }
    }
    #[allow(clippy::single_call_fn)] // shared rust-file parser keeps read+parse flow reusable for AST-based checks and visitors
    fn for_each_rs_syn_file(mut on_file: impl FnMut(&std::path::Path, &syn::File)) {
        for_each_rs_file_content(|path, v| {
            let ast = syn::parse_file(v).expect("5e7a83eb");
            on_file(path, &ast);
        });
    }
    fn workspace_tbl_from_cargo_toml() -> toml::value::Table {
        let mut tbl = std::fs::read_to_string("../Cargo.toml")
            .expect("39a0d238")
            .parse::<toml::Table>()
            .expect("beb11586");
        toml_val_as_tbl(tbl.remove("workspace").expect("f728192d"), "2bfb0b62")
    }
    #[allow(clippy::single_call_fn)] // shared owned-value table extractor keeps table-shape validation reusable where ownership is required
    fn toml_val_as_tbl(v: toml::Value, uuid: &str) -> toml::value::Table {
        match v {
            toml::Value::Table(t) => t,
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("{uuid}"),
        }
    }
    fn toml_val_as_tbl_ref<'value_lt>(
        v: &'value_lt toml::Value,
        uuid: &str,
    ) -> &'value_lt toml::value::Table {
        match v {
            toml::Value::Table(t) => t,
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => panic!("{uuid}"),
        }
    }
    #[test]
    fn workspace_crates_must_use_workspace_dependencies() {
        assert_cargo_toml_ers_empty(
            &[
                "../Cargo.toml", //workspace
            ],
            "5f8a6d17",
            collect_non_workspace_dep_ers,
        );
    }
    #[test]
    fn workspace_dependencies_use_inline_table_style() {
        let rgx = regex::Regex::new(r"(?m)^\s*[A-Za-z0-9_-]+\.workspace\s*=\s*true\s*$")
            .expect("ac15d6b9");
        let mut ers = Vec::new();
        for_each_cargo_toml_project_file(&[], |path| {
            let v = std::fs::read_to_string(path).expect("762c1d9e");
            for mtch in rgx.find_iter(&v) {
                let line_nbr = v
                    .bytes()
                    .take(mtch.start())
                    .filter(|byte| *byte == b'\n')
                    .count()
                    .saturating_add(1);
                ers.push(format!(
                    "{}:{line_nbr} use `dep = {{ workspace = true }}` instead of dotted workspace dependency style",
                    path.display()
                ));
            }
        });
        assert_joined_ers_empty_with_ctx(
            &ers,
            "d7a3c5b1",
            "dotted workspace dependency style found:",
        );
    }
    #[allow(clippy::single_call_fn)] // shared collector keeps workspace-dependency policy checks reusable and centralized
    fn collect_non_workspace_dep_ers(
        path: &std::path::Path,
        parsed: &toml::Table,
        ers: &mut Vec<String>,
    ) {
        for dep_section in ["dependencies", "dev-dependencies", "build-dependencies"] {
            if let Some(deps) = parsed.get(dep_section).and_then(toml::Value::as_table) {
                for (dep_name, dep_value) in deps {
                    if !workspace_dep_entry_is_valid(dep_value) {
                        ers.push(workspace_dep_entry_er(path, dep_name, dep_section));
                    }
                }
            }
        }
    }
    #[allow(clippy::single_call_fn)] // keeps dependency-policy validation centralized for dependencies/dev-dependencies/build-dependencies checks
    fn workspace_dep_entry_is_valid(dep_value: &toml::Value) -> bool {
        match dep_value {
            toml::Value::Table(dep_tbl) => {
                dep_tbl.contains_key("path")
                    || dep_tbl.get("workspace") == Some(&toml::Value::Boolean(true))
            }
            toml::Value::String(_)
            | toml::Value::Integer(_)
            | toml::Value::Float(_)
            | toml::Value::Boolean(_)
            | toml::Value::Datetime(_)
            | toml::Value::Array(_) => false,
        }
    }
    #[allow(clippy::single_call_fn)] // shared message builder keeps dependency-policy errors identical across call sites
    fn workspace_dep_entry_er(path: &std::path::Path, dep_name: &str, dep_section: &str) -> String {
        format!(
            "{}: dependency `{dep_name}` in [{dep_section}] must use `dep = {{ workspace = true }}` (only `path = ...` is allowed as exception)",
            path.display(),
        )
    }
    #[test]
    fn workspace_members_exist_on_disk() {
        let workspace = workspace_tbl_from_cargo_toml();
        let members = workspace_members_as_strs(&workspace, "7f3a1c4e");
        let mut ers = collect_workspace_member_missing_cargo_toml_ers(&members);
        assert_joined_ers_empty_sorted(&mut ers, "a4e3b8d1");
    }
    #[test]
    fn workspace_members_sorted_alphabetically() {
        let workspace = workspace_tbl_from_cargo_toml();
        let members_vec = workspace_members_as_strs(&workspace, "c1d4f7a2");
        let mut sorted = members_vec.clone();
        sorted.sort_unstable();
        let mut ers = Vec::new();
        for (k_4b1e6a8c, (got, expected)) in members_vec.iter().zip(sorted.iter()).enumerate() {
            if got != expected {
                ers.push(format!(
                    "index {k_4b1e6a8c}: got `{got}`, expected `{expected}`"
                ));
            }
        }
        assert_joined_ers_empty_with_ctx(&ers, "b7c2e5f8", "members not sorted:");
    }
    #[allow(clippy::single_call_fn)] // dedicated collector keeps workspace-members existence diagnostics reusable and deterministic with caller-managed sorting
    fn collect_workspace_member_missing_cargo_toml_ers(members: &[&str]) -> Vec<String> {
        let mut ers = Vec::new();
        for member_str in members {
            let path = std::path::Path::new("..")
                .join(member_str)
                .join("Cargo.toml");
            if !path.exists() {
                ers.push(format!(
                    "member `{member_str}` Cargo.toml not found at {}",
                    path.display()
                ));
            }
        }
        ers
    }
    #[allow(clippy::single_call_fn)] // central member extraction keeps workspace-members readers strict and reusable across membership checks
    fn workspace_members_as_strs<'members_lt>(
        workspace: &'members_lt toml::value::Table,
        exp_id: &'static str,
    ) -> Vec<&'members_lt str> {
        let Some(members) = workspace.get("members").and_then(toml::Value::as_array) else {
            panic!("{exp_id}");
        };
        let mut output = Vec::with_capacity(members.len());
        for member in members {
            match member.as_str() {
                Some(member_str) => output.push(member_str),
                None => panic!("{exp_id}"),
            }
        }
        output
    }
}
