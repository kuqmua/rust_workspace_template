#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DbgVisitor {
    found: crate::types::AnalyzerBool,
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    Default,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct CustomTypeNameVisitor {
    names: crate::types::SourceTextList,
}

impl<'ast> syn::visit::Visit<'ast> for CustomTypeNameVisitor {
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        if !item_enum
            .ident
            .to_string()
            .starts_with(constants_str::GENERATED_PRIVATE_TYPE_PREFIX)
        {
            self.names.push(item_enum.ident.to_string());
        }
        syn::visit::visit_item_enum(self, item_enum);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let test_only = item_mod.attrs.iter().any(|attr| {
            attr.path().is_ident(constants_str::CFG_ALT)
                && matches!(
                    &attr.meta,
                    syn::Meta::List(list)
                        if list.tokens.to_string().contains(constants_str::TEST_ALT_3)
                )
        });
        if !test_only {
            syn::visit::visit_item_mod(self, item_mod);
        }
    }

    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if !item_struct
            .ident
            .to_string()
            .starts_with(constants_str::GENERATED_PRIVATE_TYPE_PREFIX)
        {
            self.names.push(item_struct.ident.to_string());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }

    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        self.names.push(item_trait.ident.to_string());
        syn::visit::visit_item_trait(self, item_trait);
    }

    fn visit_item_trait_alias(&mut self, item_trait_alias: &'ast syn::ItemTraitAlias) {
        self.names.push(item_trait_alias.ident.to_string());
        syn::visit::visit_item_trait_alias(self, item_trait_alias);
    }

    fn visit_item_type(&mut self, item_type: &'ast syn::ItemType) {
        self.names.push(item_type.ident.to_string());
        syn::visit::visit_item_type(self, item_type);
    }

    fn visit_item_union(&mut self, item_union: &'ast syn::ItemUnion) {
        self.names.push(item_union.ident.to_string());
        syn::visit::visit_item_union(self, item_union);
    }
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    Default,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct FreeFnNameVisitor {
    names: crate::types::SourceTextList,
}

impl<'ast> syn::visit::Visit<'ast> for FreeFnNameVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        self.names.push(item_fn.sig.ident.to_string());
        syn::visit::visit_item_fn(self, item_fn);
    }
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    Default,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct OptimalMemoryLayoutVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl OptimalMemoryLayoutVisitor {
    fn check_attrs(&mut self, ident: &syn::Ident, attrs: &[syn::Attribute], str: &str) {
        let mut derives_optimal_memory_layout = false;
        attrs
            .iter()
            .filter(|attr| attr.path().is_ident(constants_str::DERIVE))
            .for_each(|attr| {
                drop(attr.parse_nested_meta(|metadata| {
                    if metadata
                        .path
                        .segments
                        .last()
                        .is_some_and(|segment| segment.ident == constants_str::VALUE_00714460)
                    {
                        derives_optimal_memory_layout = true;
                    }
                    Ok(())
                }));
            });
        if !derives_optimal_memory_layout {
            self.errors.push(format!(
                "{str} `{ident}` must derive `proc_macro_optimal_memory_layout::OptimalMemoryLayout`"
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for OptimalMemoryLayoutVisitor {
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        self.check_attrs(&item_enum.ident, &item_enum.attrs, constants_str::ENUM);
        syn::visit::visit_item_enum(self, item_enum);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        self.check_attrs(
            &item_struct.ident,
            &item_struct.attrs,
            constants_str::STRUCT,
        );
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DbgVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if r#macro
            .path
            .segments
            .last()
            .is_some_and(|v_4b8e1c7a| v_4b8e1c7a.ident == constants_str::DBG)
        {
            self.found.set_true();
        }
    }
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct TodoUnimplVisitor {
    todo_found: crate::types::AnalyzerCount,
    unimplemented_found: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for TodoUnimplVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if let Some(last_segment) = r#macro.path.segments.last() {
            match () {
                () if last_segment.ident == constants_str::TODO => {
                    self.todo_found.saturating_inc();
                }
                () if last_segment.ident == constants_str::UNIMPLEMENTED => {
                    self.unimplemented_found.saturating_inc();
                }
                () => {}
            }
        }
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct UnwrapVisitor {
    found_count: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnwrapVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::UNWRAP && expr_method_call.args.is_empty() {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ForLoopVisitor {
    found_count: crate::types::AnalyzerCount,
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct SourceDroppingMapErrVisitor {
    found_count: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SourceDroppingMapErrVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::CODE_STYLE_MAP_ERR
            && expr_method_call.args.first().is_some_and(|argument| {
                matches!(argument, syn::Expr::Closure(closure) if closure.inputs.iter().any(|input| matches!(input, syn::Pat::Wild(_))))
            })
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct NumericAsCastVisitor {
    found_count: crate::types::AnalyzerCount,
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct SerdeJsonValueFieldVisitor {
    violations: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueFieldVisitor {
    fn visit_field(&mut self, field: &'ast syn::Field) {
        let mut type_visitor = SerdeJsonValueTypeVisitor::default();
        syn::visit::Visit::visit_type(&mut type_visitor, &field.ty);
        if type_visitor.found.get() {
            self.violations
                .push(constants_str::CODE_STYLE_UNNAMED_ITEM.to_owned());
        }
        syn::visit::visit_field(self, field);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(
            &syn::Item::Struct(item_struct.clone()),
        ))
        .get()
            || item_struct.ident == constants_str::CODE_STYLE_SERDE_JSON_ADMIN_AUDIT_DETAILS
        {
            return;
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct SerdeJsonValueTypeVisitor {
    found: crate::types::AnalyzerBool,
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct PublicStructFieldVisitor {
    violations: crate::types::DiagnosticMessages,
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
pub(super) struct GeneratedPublicStructFieldVisitor {
    violations: crate::types::DiagnosticMessages,
}

impl GeneratedPublicStructFieldVisitor {
    fn visit_tokens(&mut self, token_stream: proc_macro2::TokenStream) {
        let token_trees = token_stream
            .into_iter()
            .collect::<Vec<proc_macro2::TokenTree>>();
        token_trees
            .iter()
            .filter_map(|token| match token {
                proc_macro2::TokenTree::Group(group) => Some(group.stream()),
                proc_macro2::TokenTree::Ident(_)
                | proc_macro2::TokenTree::Punct(_)
                | proc_macro2::TokenTree::Literal(_) => None,
            })
            .for_each(|group_tokens| self.visit_tokens(group_tokens));
        let window_text = |window: &[proc_macro2::TokenTree]| {
            window
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(constants_str::SINGLE_SPACE)
        };
        self.violations.extend(
            token_trees
                .windows(4usize)
                .filter(|window| {
                    matches!(window, [proc_macro2::TokenTree::Ident(public), proc_macro2::TokenTree::Punct(hash), proc_macro2::TokenTree::Ident(_), proc_macro2::TokenTree::Punct(colon)] if public == constants_str::PUB_KEYWORD && hash.as_char() == '#' && colon.as_char() == ':')
                        || matches!(window, [proc_macro2::TokenTree::Ident(public), proc_macro2::TokenTree::Group(scope), proc_macro2::TokenTree::Ident(_), proc_macro2::TokenTree::Punct(colon)] if public == constants_str::PUB_KEYWORD && scope.delimiter() == proc_macro2::Delimiter::Parenthesis && colon.as_char() == ':')
                })
                .map(window_text),
        );
        self.violations.extend(
            token_trees
                .windows(5usize)
                .filter(|window| {
                    matches!(window, [proc_macro2::TokenTree::Ident(public), proc_macro2::TokenTree::Group(scope), proc_macro2::TokenTree::Punct(hash), proc_macro2::TokenTree::Ident(_), proc_macro2::TokenTree::Punct(colon)] if public == constants_str::PUB_KEYWORD && scope.delimiter() == proc_macro2::Delimiter::Parenthesis && hash.as_char() == '#' && colon.as_char() == ':')
                        || matches!(window, [proc_macro2::TokenTree::Punct(first_hash), proc_macro2::TokenTree::Ident(visibility), proc_macro2::TokenTree::Punct(second_hash), proc_macro2::TokenTree::Ident(_), proc_macro2::TokenTree::Punct(colon)] if first_hash.as_char() == '#' && visibility.to_string().contains(constants_str::VISIBILITY_IDENTIFIER_FRAGMENT) && second_hash.as_char() == '#' && colon.as_char() == ':')
                })
                .map(window_text),
        );
        self.violations.extend(
            token_trees
                .windows(3usize)
                .filter(|window| {
                    matches!(window, [proc_macro2::TokenTree::Ident(public), proc_macro2::TokenTree::Ident(_), proc_macro2::TokenTree::Punct(colon)] if public == constants_str::PUB_KEYWORD && colon.as_char() == ':')
                })
                .map(window_text),
        );
    }
}

impl<'ast> syn::visit::Visit<'ast> for GeneratedPublicStructFieldVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if r#macro.path.segments.last().is_some_and(|segment| {
            segment.ident == constants_str::QUOTE_MACRO_NAME
                || segment.ident == constants_str::QUOTE_SPANNED_MACRO_NAME
        }) {
            self.visit_tokens(r#macro.tokens.clone());
        }
        syn::visit::visit_macro(self, r#macro);
    }
}

impl<'ast> syn::visit::Visit<'ast> for PublicStructFieldVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        item_struct
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !matches!(field.vis, syn::Visibility::Inherited))
            .for_each(|(index, field)| {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| index.to_string(), ToString::to_string);
                self.violations
                    .push(format!("{}::{field_name}", item_struct.ident));
            });
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueTypeVisitor {
    fn visit_type_path(&mut self, type_path: &'ast syn::TypePath) {
        let mut segments = type_path.path.segments.iter().rev();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_VALUE)
            && segments
                .next()
                .is_some_and(|segment| segment.ident == constants_str::SERDE_JSON)
        {
            self.found.set_true();
        }
        syn::visit::visit_type_path(self, type_path);
    }
}
impl<'ast> syn::visit::Visit<'ast> for NumericAsCastVisitor {
    fn visit_expr_cast(&mut self, expr_cast: &'ast syn::ExprCast) {
        let numeric = if let syn::Type::Path(path) = expr_cast.ty.as_ref() {
            if path.qself.is_none() {
                path.path.segments.last().is_some_and(|segment| {
                    matches!(
                        segment.ident.to_string().as_str(),
                        constants_str::CODE_STYLE_I8
                            | constants_str::CODE_STYLE_I16
                            | constants_str::CODE_STYLE_I32
                            | constants_str::CODE_STYLE_I64
                            | constants_str::CODE_STYLE_I128
                            | constants_str::CODE_STYLE_ISIZE
                            | constants_str::CODE_STYLE_U8
                            | constants_str::CODE_STYLE_U16
                            | constants_str::CODE_STYLE_U32
                            | constants_str::CODE_STYLE_U64
                            | constants_str::CODE_STYLE_U128
                            | constants_str::CODE_STYLE_USIZE
                            | constants_str::CODE_STYLE_F32
                            | constants_str::CODE_STYLE_F64
                    )
                })
            } else {
                false
            }
        } else {
            false
        };
        if numeric {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_cast(self, expr_cast);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForLoopVisitor {
    fn visit_expr_for_loop(&mut self, expr_for_loop: &'ast syn::ExprForLoop) {
        self.found_count.saturating_inc();
        syn::visit::visit_expr_for_loop(self, expr_for_loop);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct IncludeAssetMacroVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for IncludeAssetMacroVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if let Some(segment) = r#macro.path.segments.last()
            && (segment.ident == constants_str::INCLUDE_STR
                || segment.ident == constants_str::INCLUDE_BYTES)
        {
            self.errors.push(format!("contains {}!()", segment.ident));
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DirectPathCallVisitor {
    calls: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct UnboundedReadVisitor {
    calls: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for UnboundedReadVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let Some(path) =
            crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(expr_call))
        {
            let call = crate::code_style::path_to_string(path);
            if matches!(
                call.as_ref(),
                constants_str::STD_PATH_FS_PATH_READ
                    | constants_str::STD_PATH_FS_PATH_READ_TO_STRING
                    | constants_str::TOKIO_PATH_FS_PATH_READ
                    | constants_str::TOKIO_PATH_FS_PATH_READ_TO_STRING
            ) {
                self.calls.push(call.as_ref().to_owned());
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::PG_CRUD_PG_TEXT {
            self.calls.push(format!(".{}()", expr_method_call.method));
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectPathCallVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let Some(path) =
            crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(expr_call))
        {
            self.calls
                .push(crate::code_style::path_to_string(path).as_ref().to_owned());
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct LostSpawnVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for LostSpawnVisitor {
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        let discarded = match stmt {
            syn::Stmt::Expr(expression, _) => crate::code_style::unowned_spawn_expr(expression),
            syn::Stmt::Local(local) => local.init.as_ref().is_some_and(|init| {
                crate::code_style::unowned_spawn_expr(init.expr.as_ref())
                    && match &local.pat {
                        syn::Pat::Wild(_) => true,
                        syn::Pat::Ident(identifier) => {
                            identifier.ident.to_string().starts_with('_')
                        }
                        syn::Pat::Const(_)
                        | syn::Pat::Lit(_)
                        | syn::Pat::Macro(_)
                        | syn::Pat::Or(_)
                        | syn::Pat::Paren(_)
                        | syn::Pat::Path(_)
                        | syn::Pat::Range(_)
                        | syn::Pat::Reference(_)
                        | syn::Pat::Rest(_)
                        | syn::Pat::Slice(_)
                        | syn::Pat::Struct(_)
                        | syn::Pat::Tuple(_)
                        | syn::Pat::TupleStruct(_)
                        | syn::Pat::Type(_)
                        | syn::Pat::Verbatim(_)
                        | _ => false,
                    }
            }),
            syn::Stmt::Item(_) | syn::Stmt::Macro(_) => false,
        };
        if discarded {
            self.errors.push(
                constants_str::SPAWN_RESULT_IS_DISCARDED_RETAIN_AND_SUPERVISE_TASK.to_owned(),
            );
        }
        syn::visit::visit_stmt(self, stmt);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct TestNondeterminismVisitor {
    calls: crate::types::DiagnosticMessages,
    test_depth: crate::types::AnalyzerCount,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct SensitiveTextDebugDeriveVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct SensitiveErrorFormatVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct GeneratedRandomnessVisitor {
    calls: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct StaticStateVisitor {
    identifiers: crate::types::SourceTextList,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PrintMacroVisitor {
    calls: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ProductionLinePrintMacroVisitor {
    calls: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct DoubleUnderscoreNamingVisitor {
    identifiers: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ShortFunctionNamingVisitor {
    identifiers: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct OpaqueShortFieldNamingVisitor {
    identifiers: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct OpaqueSerdeRenameVisitor {
    identifiers: crate::types::DiagnosticMessages,
}
impl OpaqueSerdeRenameVisitor {
    fn check_identifier(&mut self, string: String) {
        if matches!(
            string.as_str(),
            stringify!(asc)
                | stringify!(aud)
                | stringify!(desc)
                | stringify!(exp)
                | stringify!(iat)
                | stringify!(iss)
                | stringify!(jti)
                | stringify!(sub)
                | stringify!(v)
        ) {
            self.identifiers.push(string);
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for OpaqueSerdeRenameVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if attribute.path().is_ident(stringify!(serde)) {
            let syn::Meta::List(attribute_list) = &attribute.meta else {
                syn::visit::visit_attribute(self, attribute);
                return;
            };
            let parser = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated;
            match syn::parse::Parser::parse2(parser, attribute_list.tokens.clone()) {
                Ok(metadata) => metadata.into_iter().for_each(|meta| match meta {
                    syn::Meta::NameValue(name_value)
                        if name_value.path.is_ident(stringify!(rename)) =>
                    {
                        if let syn::Expr::Lit(expression_literal) = name_value.value
                            && let syn::Lit::Str(identifier) = expression_literal.lit
                        {
                            self.check_identifier(identifier.value());
                        }
                    }
                    syn::Meta::List(rename_list)
                        if rename_list.path.is_ident(stringify!(rename)) =>
                    {
                        match syn::parse::Parser::parse2(parser, rename_list.tokens) {
                            Ok(rename_metadata) => {
                                rename_metadata.into_iter().for_each(|rename_meta| {
                                    if let syn::Meta::NameValue(name_value) = rename_meta
                                        && (name_value.path.is_ident(stringify!(serialize))
                                            || name_value.path.is_ident(stringify!(deserialize)))
                                        && let syn::Expr::Lit(expression_literal) = name_value.value
                                        && let syn::Lit::Str(identifier) = expression_literal.lit
                                    {
                                        self.check_identifier(identifier.value());
                                    }
                                });
                            }
                            Err(error) => self.identifiers.push(error.to_string()),
                        }
                    }
                    syn::Meta::Path(_) | syn::Meta::List(_) | syn::Meta::NameValue(_) => {}
                }),
                Err(error) => self.identifiers.push(error.to_string()),
            }
        }
        syn::visit::visit_attribute(self, attribute);
    }
}
impl<'ast> syn::visit::Visit<'ast> for OpaqueShortFieldNamingVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        self.identifiers.extend(
            item_struct
                .fields
                .iter()
                .filter_map(|field| field.ident.as_ref())
                .map(ToString::to_string)
                .filter(|identifier_text| {
                    matches!(
                        identifier_text.as_str(),
                        stringify!(aud)
                            | stringify!(exp)
                            | stringify!(iat)
                            | stringify!(index)
                            | stringify!(iss)
                            | stringify!(jti)
                            | stringify!(lt)
                            | stringify!(sub)
                            | stringify!(type0)
                            | stringify!(v)
                            | stringify!(vis)
                    )
                }),
        );
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl ShortFunctionNamingVisitor {
    fn check_identifier(&mut self, ident: &syn::Ident) {
        let identifier_text = ident.to_string();
        if identifier_text.starts_with(constants_str::WORKSPACE_SHORT_MAKE_PREFIX) {
            self.identifiers.push(identifier_text);
        }
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct PublicLogicVisitor {
    found: crate::types::AnalyzerBool,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct OwnedTestVisitor {
    found: crate::types::AnalyzerBool,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct AllowReasonVisitor {
    errors: crate::types::DiagnosticMessages,
    lines: crate::types::SourceTextList,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[getters(get_mut)]
pub(super) struct DiagnosticIdVisitor {
    errors: crate::types::DiagnosticMessages,
    ids: crate::types::SourceTextList,
}
impl DiagnosticIdVisitor {
    pub(super) fn record(
        &mut self,
        kind: crate::types::SourceTextRef<'_>,
        value: crate::types::SourceTextRef<'_>,
    ) {
        let optional_prefix = value
            .get()
            .get(..8usize)
            .filter(|prefix| {
                prefix
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            })
            .filter(|_| {
                value.get().len() == 8usize
                    || value.get().get(8usize..).is_some_and(|suffix| {
                        suffix.starts_with(constants_str::VALUE_45822F54) || suffix.starts_with(' ')
                    })
            });
        if let Some(prefix) = optional_prefix {
            let has_context = value
                .get()
                .get(8usize..)
                .and_then(|suffix| {
                    suffix
                        .strip_prefix(constants_str::VALUE_45822F54)
                        .or_else(|| suffix.strip_prefix(' '))
                })
                .is_some_and(|context| context.split_whitespace().count() >= 2usize);
            if kind.as_ref() == constants_str::CODE_STYLE_EXPECT_METHOD_NAME && !has_context {
                self.errors.push(format!(
                    "expect message diagnostic ID must be followed by at least two context words: {value:?}",
                    value = value.as_ref(),
                ));
            } else {
                self.ids.push(prefix.to_owned());
            }
        } else {
            self.errors.push(format!(
                "{kind} message must start with a unique eight-character lowercase hexadecimal diagnostic ID: {value:?}",
                kind = kind.as_ref(),
                value = value.as_ref(),
            ));
        }
    }
    fn record_constant_path(
        &mut self,
        source_text_ref: crate::types::SourceTextRef<'_>,
        expr: &syn::Expr,
    ) -> bool {
        let syn::Expr::Path(path) = expr else {
            return false;
        };
        if path.qself.is_some() {
            return false;
        }
        let Some(identifier) = path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
        else {
            return false;
        };
        let Some(diagnostic_id) = identifier
            .strip_prefix(constants_str::CODE_STYLE_DIAGNOSTIC_CONSTANT_PREFIX)
            .and_then(|suffix| suffix.get(..8usize))
            .map(str::to_ascii_lowercase)
        else {
            return false;
        };
        self.record(
            source_text_ref,
            crate::types::SourceTextRef::from(
                format!("{diagnostic_id} stored diagnostic").as_str(),
            ),
        );
        true
    }
}
impl<'ast> syn::visit::Visit<'ast> for DiagnosticIdVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::CODE_STYLE_EXPECT_METHOD_NAME {
            match expr_method_call.args.first() {
                Some(syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) if expr_method_call.args.len() == constants_usize::ONE => self.record(
                    crate::types::SourceTextRef::from(constants_str::CODE_STYLE_EXPECT_METHOD_NAME),
                    crate::types::SourceTextRef::from(lit_str.value().as_str()),
                ),
                Some(argument)
                    if expr_method_call.args.len() == constants_usize::ONE
                        && self.record_constant_path(
                            crate::types::SourceTextRef::from(
                                constants_str::CODE_STYLE_EXPECT_METHOD_NAME,
                            ),
                            argument,
                        ) => {}
                Some(_) | None => self.errors.push(constants_str::VALUE_3C063239.to_owned()),
            }
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if crate::code_style::macro_path_is_quote(crate::types::SynPathRef::from(&r#macro.path))
            .get()
        {
            crate::code_style::scan_generated_diagnostic_tokens(&r#macro.tokens, self);
        }
        if r#macro
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_PANIC_METHOD_NAME)
        {
            match r#macro.tokens.clone().into_iter().next() {
                Some(proc_macro2::TokenTree::Literal(literal)) => {
                    match syn::parse_str::<syn::LitStr>(literal.to_string().as_str()) {
                        Ok(lit_str) => {
                            let value = lit_str.value();
                            if !crate::code_style::panic_uses_dynamic_diagnostic_id(
                                crate::types::SourceTextRef::from(value.as_str()),
                            )
                            .get()
                            {
                                self.record(
                                    crate::types::SourceTextRef::from(
                                        constants_str::CODE_STYLE_PANIC_METHOD_NAME,
                                    ),
                                    crate::types::SourceTextRef::from(value.as_str()),
                                );
                            }
                        }
                        Err(_error) => self.errors.push(constants_str::VALUE_CCFFF72E.to_owned()),
                    }
                }
                Some(_) | None => self.errors.push(constants_str::VALUE_CCFFF72E.to_owned()),
            }
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
#[allow(
    clippy::needless_for_each,
    reason = "repository source policy requires iterator methods instead of for loops"
)]
impl<'ast> syn::visit::Visit<'ast> for SensitiveTextDebugDeriveVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if crate::code_style::sensitive_text_wrapper_identifier(crate::types::SourceTextRef::from(
            item_struct.ident.to_string().as_str(),
        ))
        .get()
            && item_struct
                .fields
                .iter()
                .any(|field| crate::code_style::type_contains_sensitive_text_or_bytes(&field.ty))
        {
            [
                constants_str::VALUE_1A03BD2F,
                constants_str::VALUE_34E108C0,
                constants_str::VALUE_4A177217,
                constants_str::VALUE_00857394,
            ]
            .into_iter()
            .for_each(|derive_name| {
                if item_struct.attrs.iter().any(|attr| {
                    crate::code_style::derive_attr_has_terminal(
                        crate::types::SynAttributeRef::from(attr),
                        crate::types::SourceTextRef::from(derive_name),
                    )
                    .get()
                }) {
                    self.errors.push(format!(
                        "sensitive text wrapper `{}` derives `{derive_name}` without redaction",
                        item_struct.ident
                    ));
                }
            });
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl SensitiveErrorFormatVisitor {
    fn inspect_fields(&mut self, attrs: &[syn::Attribute], fields: &syn::Fields) {
        let templates = attrs
            .iter()
            .filter(|attr| attr.path().is_ident(constants_str::CONFIG_TRACING_ERROR))
            .filter_map(|attr| match &attr.meta {
                syn::Meta::List(list) => Some(list.tokens.to_string()),
                syn::Meta::Path(_) | syn::Meta::NameValue(_) => None,
            })
            .collect::<Vec<String>>();
        if templates.is_empty() {
            return;
        }
        fields.iter().enumerate().for_each(|(index, field)| {
            let named_placeholder = field.ident.as_ref().and_then(|identifier| {
                crate::code_style::sensitive_text_wrapper_identifier(crate::types::SourceTextRef::from(
                    identifier.to_string().as_str(),
                ))
                .get()
                .then(|| format!("{{{identifier}"))
            });
            let tuple_placeholder = field
                .ident
                .is_none()
                .then(|| format!("{{{index}"))
                .filter(|_| crate::code_style::type_contains_sensitive_text_or_bytes(&field.ty));
            [named_placeholder, tuple_placeholder]
                .into_iter()
                .flatten()
                .for_each(|field_placeholder| {
                    if templates
                        .iter()
                        .any(|template| template.contains(field_placeholder.as_str()))
                    {
                        self.errors.push(format!(
                            "error formatter exposes sensitive field placeholder `{field_placeholder}`"
                        ));
                    }
                });
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SensitiveErrorFormatVisitor {
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        item_enum
            .variants
            .iter()
            .for_each(|variant| self.inspect_fields(&variant.attrs, &variant.fields));
        syn::visit::visit_item_enum(self, item_enum);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        self.inspect_fields(&item_struct.attrs, &item_struct.fields);
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for GeneratedRandomnessVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if crate::code_style::macro_path_is_quote(crate::types::SynPathRef::from(&r#macro.path))
            .get()
        {
            let compact = r#macro
                .tokens
                .to_string()
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect::<String>();
            [
                constants_str::UUID_PATH_UUID_PATH_NEW_V4,
                constants_str::UUID_PATH_UUID_PATH_NEW_V7,
                constants_str::RAND_PATH_RNG,
                constants_str::RAND_PATH_RANDOM,
                constants_str::RAND_PATH_RANDOM_RANGE,
                constants_str::RAND_PATH_THREAD_RNG,
                constants_str::GETRANDOM_PATH_FILL,
                constants_str::GETRANDOM_PATH_U32,
                constants_str::GETRANDOM_PATH_U64,
            ]
            .into_iter()
            .filter(|path| compact.contains(path))
            .for_each(|path| self.calls.push(path.to_owned()));
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StaticStateVisitor {
    fn visit_item_static(&mut self, item_static: &'ast syn::ItemStatic) {
        self.identifiers.push(item_static.ident.to_string());
        syn::visit::visit_item_static(self, item_static);
    }
}
impl<'ast> syn::visit::Visit<'ast> for PrintMacroVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if r#macro.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::SHARED_VALUES_PRINT
                    | constants_str::SHARED_VALUES_PRINTLN
                    | constants_str::SHARED_VALUES_EPRINT
                    | constants_str::SHARED_VALUES_EPRINTLN
            )
        }) {
            self.calls.push(
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&r#macro.path))
                    .as_ref()
                    .to_owned(),
            );
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ProductionLinePrintMacroVisitor {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if item_fn.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::TEST_ALT_3)
                || crate::code_style::attr_is_test_only_cfg(crate::types::SynAttributeRef::from(
                    attr,
                ))
                .get()
        }) {
            return;
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if r#macro.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::SHARED_VALUES_PRINTLN | constants_str::SHARED_VALUES_EPRINTLN
            )
        }) {
            self.calls.push(
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&r#macro.path))
                    .as_ref()
                    .to_owned(),
            );
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DoubleUnderscoreNamingVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let identifier = item_fn.sig.ident.to_string();
        if identifier.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE) {
            self.identifiers.push(identifier);
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let identifier = item_mod.ident.to_string();
        if identifier.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE) {
            self.identifiers.push(identifier);
        }
        syn::visit::visit_item_mod(self, item_mod);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ShortFunctionNamingVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        self.check_identifier(&item_fn.sig.ident);
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        item_impl.items.iter().for_each(|item| {
            if let syn::ImplItem::Fn(function) = item {
                self.check_identifier(&function.sig.ident);
            }
        });
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicLogicVisitor {
    fn visit_impl_item_fn(&mut self, impl_item_fn: &'ast syn::ImplItemFn) {
        if matches!(impl_item_fn.vis, syn::Visibility::Public(_)) {
            self.found.set_true();
        }
        syn::visit::visit_impl_item_fn(self, impl_item_fn);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if matches!(item_fn.vis, syn::Visibility::Public(_)) {
            self.found.set_true();
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        if matches!(item_trait.vis, syn::Visibility::Public(_))
            && item_trait.items.iter().any(|item| {
                matches!(
                    item,
                    syn::TraitItem::Fn(function) if function.default.is_some()
                )
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_item_trait(self, item_trait);
    }
}
impl<'ast> syn::visit::Visit<'ast> for OwnedTestVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::TEST_ALT_3)
        {
            self.found.set_true();
        }
        syn::visit::visit_attribute(self, attribute);
    }
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        if item_mod.ident == constants_str::TESTS_ALT
            && item_mod.attrs.iter().any(|attribute| {
                crate::code_style::attr_is_test_only_cfg(crate::types::SynAttributeRef::from(
                    attribute,
                ))
                .get()
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_item_mod(self, item_mod);
    }
}
impl<'ast> syn::visit::Visit<'ast> for AllowReasonVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        let is_lint_suppression = attribute.path().segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::VALUE_41008373 | constants_str::CODE_STYLE_EXPECT_METHOD_NAME
            )
        });
        if is_lint_suppression {
            let has_reason_argument = match &attribute.meta {
                syn::Meta::List(list) => list
                    .tokens
                    .to_string()
                    .contains(constants_str::VALUE_45F4C964),
                syn::Meta::Path(_) | syn::Meta::NameValue(_) => false,
            };
            let span = syn::spanned::Spanned::span(attribute);
            let start_line = span.start().line;
            let end_line = span.end().line;
            let has_same_line_reason = self
                .lines
                .get(end_line.saturating_sub(constants_usize::ONE))
                .and_then(|line| {
                    line.split_once(constants_str::VALUE_A2C23396)
                        .map(|(_attribute, reason)| reason)
                })
                .is_some_and(|reason| !reason.trim().is_empty());
            let has_preceding_reason = start_line
                .checked_sub(2usize)
                .and_then(|line_index| self.lines.get(line_index))
                .is_some_and(|line| {
                    line.trim_start()
                        .strip_prefix(constants_str::VALUE_A2C23396)
                        .is_some_and(|reason| !reason.trim().is_empty())
                });
            if !has_reason_argument && !has_same_line_reason && !has_preceding_reason {
                let path = attribute
                    .path()
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<String>>()
                    .join(constants_str::PATH_SEPARATOR);
                let attribute_text = match &attribute.meta {
                    syn::Meta::List(list) => format!("#[{path}({})]", list.tokens),
                    syn::Meta::NameValue(_) => format!("#[{path} = value]"),
                    syn::Meta::Path(_) => format!("#[{path}]"),
                };
                self.errors.push(format!(
                    "line {start_line}: lint suppression `{attribute_text}` requires an explicit reason"
                ));
            }
        }
        syn::visit::visit_attribute(self, attribute);
    }
}
impl<'ast> syn::visit::Visit<'ast> for TestNondeterminismVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if self.test_depth.get() != 0
            && let Some(path) =
                crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(expr_call))
        {
            let text = crate::code_style::path_to_string(path);
            if matches!(
                text.as_ref(),
                constants_str::RAND_PATH_RNG
                    | constants_str::RAND_PATH_RANDOM
                    | constants_str::RAND_PATH_RANDOM_RANGE
                    | constants_str::RAND_PATH_THREAD_RNG
                    | constants_str::GETRANDOM_PATH_FILL
                    | constants_str::GETRANDOM_PATH_U32
                    | constants_str::GETRANDOM_PATH_U64
                    | constants_str::STD_PATH_THREAD_PATH_SLEEP
                    | constants_str::STD_PATH_TIME_PATH_INSTANT_PATH_NOW
                    | constants_str::STD_PATH_TIME_PATH_SYSTEMTIME_PATH_NOW
                    | constants_str::TOKIO_PATH_TIME_PATH_INSTANT_PATH_NOW
                    | constants_str::TOKIO_PATH_TIME_PATH_SLEEP
                    | constants_str::UUID_PATH_UUID_PATH_NEW_V4
                    | constants_str::UUID_PATH_UUID_PATH_NEW_V7
            ) || text.as_ref().ends_with(constants_str::PATH_UTC_PATH_NOW)
                || text.as_ref().ends_with(constants_str::PATH_LOCAL_PATH_NOW)
                || text.as_ref().ends_with(constants_str::PATH_FROM_OS_RNG)
            {
                self.calls.push(text.as_ref().to_owned());
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
        if self.test_depth.get() != 0 {
            let text =
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&expr_path.path));
            if matches!(
                text.as_ref(),
                constants_str::RAND_PATH_RNGS_PATH_OS_RNG | constants_str::RAND_CORE_PATH_OS_RNG
            ) {
                self.calls.push(text.as_ref().to_owned());
            }
        }
        syn::visit::visit_expr_path(self, expr_path);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let is_test =
            crate::code_style::item_fn_is_unit_test(crate::types::SynItemFnRef::from(item_fn))
                .get();
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, item_fn);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let is_test = item_mod.attrs.iter().any(|attr| {
            crate::code_style::attr_is_test_only_cfg(crate::types::SynAttributeRef::from(attr))
                .get()
        });
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_mod(self, item_mod);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "alignment order required by optimal_memory_layout takes precedence over alphabetical field order"
)]
pub(super) struct UseImportVisitor {
    public_use_roots: crate::types::SourceTextList,
    found_non_public_use_import: crate::types::AnalyzerBool,
    found_use_rename: crate::types::AnalyzerBool,
}
#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::single_call_fn,
    reason = "use-tree analyzer helpers are grouped by root extraction, reviewed exceptions, and recursive rename detection to keep policy logic independently testable"
)]
impl UseImportVisitor {
    fn is_reviewed_private_use(use_tree: &syn::UseTree) -> bool {
        matches!(use_tree, syn::UseTree::Path(path) if path.ident == stringify!(leptos))
    }

    fn use_tree_root(use_tree: &syn::UseTree) -> crate::types::SourceText {
        crate::types::SourceText::try_from(match use_tree {
            syn::UseTree::Path(path) => path.ident.to_string(),
            syn::UseTree::Name(name) => name.ident.to_string(),
            syn::UseTree::Rename(rename) => rename.ident.to_string(),
            syn::UseTree::Glob(_) => constants_str::ASTERISK.to_owned(),
            syn::UseTree::Group(_) => constants_str::BRACED_ELLIPSIS.to_owned(),
        })
        .expect(constants_str::DIAGNOSTIC_E7AB40C1)
    }

    fn use_tree_contains_rename(
        syn_use_tree_ref: crate::types::SynUseTreeRef<'_>,
    ) -> crate::types::AnalyzerBool {
        crate::types::AnalyzerBool::from(match syn_use_tree_ref.as_ref() {
            syn::UseTree::Path(use_path) => {
                Self::use_tree_contains_rename(crate::types::SynUseTreeRef::from(&*use_path.tree))
                    .get()
            }
            syn::UseTree::Name(_) | syn::UseTree::Glob(_) => false,
            syn::UseTree::Rename(_) => true,
            syn::UseTree::Group(use_group) => use_group.items.iter().any(|item| {
                Self::use_tree_contains_rename(crate::types::SynUseTreeRef::from(item)).get()
            }),
        })
    }
}
impl<'ast> syn::visit::Visit<'ast> for UseImportVisitor {
    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        if matches!(item_use.vis, syn::Visibility::Inherited)
            && !Self::is_reviewed_private_use(&item_use.tree)
        {
            self.found_non_public_use_import.set_true();
        }
        if !matches!(item_use.vis, syn::Visibility::Inherited) {
            self.public_use_roots
                .push(String::from(Self::use_tree_root(&item_use.tree)));
        }
        if Self::use_tree_contains_rename(crate::types::SynUseTreeRef::from(&item_use.tree)).get() {
            self.found_use_rename.set_true();
        }
        syn::visit::visit_item_use(self, item_use);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct TypeAliasVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for TypeAliasVisitor {
    fn visit_item_type(&mut self, item_type: &'ast syn::ItemType) {
        self.errors.push(format!(
                "type alias `{}` found; use the explicit type at usage sites instead of creating a type alias",
                item_type.ident
            ));
        syn::visit::visit_item_type(self, item_type);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct EmptyEnumVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl EmptyEnumVisitor {
    fn check(&mut self, item_enum: &syn::ItemEnum) {
        if item_enum.variants.is_empty() {
            self.errors.push(format!(
                "enum `{}` has no variants; use an inhabited domain type or return the concrete type from infallible functions",
                item_enum.ident
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for EmptyEnumVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if let syn::Meta::List(meta) = &attribute.meta
            && let Ok(item) = syn::parse2::<syn::ItemEnum>(meta.tokens.clone())
        {
            self.check(&item);
        }
        syn::visit::visit_attribute(self, attribute);
    }

    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        self.check(item_enum);
        syn::visit::visit_item_enum(self, item_enum);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct InfallibleResultVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl InfallibleResultVisitor {
    fn type_is_infallible(ty: &syn::Type) -> bool {
        let syn::Type::Path(path) = ty else {
            return false;
        };
        path.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == stringify!(Infallible))
    }
}
impl<'ast> syn::visit::Visit<'ast> for InfallibleResultVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let result_error = |ty: &'ast syn::Type| -> Option<&'ast syn::Type> {
            let syn::Type::Path(path) = ty else {
                return None;
            };
            let segment = path.path.segments.last()?;
            if segment.ident != stringify!(Result) {
                return None;
            }
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };
            arguments
                .args
                .iter()
                .filter_map(|argument| {
                    let syn::GenericArgument::Type(argument_type) = argument else {
                        return None;
                    };
                    Some(argument_type)
                })
                .nth(constants_usize::ONE)
        };
        if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output
            && result_error(ty).is_some_and(Self::type_is_infallible)
        {
            self.errors.push(format!(
                "function `{}` returns `Result` with `Infallible`; return the concrete success type",
                item_fn.sig.ident
            ));
        }
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let mut fields = item_struct.fields.iter();
        if let Some(field) = fields.next()
            && fields.next().is_none()
            && Self::type_is_infallible(&field.ty)
        {
            self.errors.push(format!(
                "struct `{}` wraps `Infallible`; remove the wrapper and return the concrete success type",
                item_struct.ident
            ));
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ConstantAliasVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for ConstantAliasVisitor {
    fn visit_item_const(&mut self, item_const: &'ast syn::ItemConst) {
        let local_constant_name = item_const.ident.to_string();
        if local_constant_name == constants_str::UNDERSCORE {
            return;
        }
        if let syn::Expr::Path(expression_path) = item_const.expr.as_ref()
            && expression_path.qself.is_none()
            && expression_path.path.segments.last().is_some_and(|segment| {
                let segment_identifier = segment.ident.to_string();
                segment_identifier
                    .chars()
                    .any(|symbol| symbol.is_ascii_alphabetic())
                    && segment_identifier.chars().all(|symbol| {
                        symbol.is_ascii_uppercase() || symbol.is_ascii_digit() || symbol == '_'
                    })
            })
        {
            self.errors.push(format!(
                "`{local_constant_name}` aliases `{}`; use the source constant directly",
                crate::code_style::path_to_string(crate::types::SynPathRef::from(
                    &expression_path.path
                ))
                .as_ref()
            ));
        }
        syn::visit::visit_item_const(self, item_const);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ForwardingDerefVisitor {
    errors: crate::types::DiagnosticMessages,
    inner_types: std::collections::BTreeMap<String, syn::Type>,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ForwardingBorrowVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingBorrowVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_borrow_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_BORROW_TRAIT_IDENTIFIER
            })
        });
        let forwards_inner = item_impl.items.iter().any(|item| {
            let syn::ImplItem::Fn(function) = item else {
                return false;
            };
            function.sig.ident == constants_str::CODE_STYLE_BORROW_FN_IDENTIFIER
                && function.block.stmts.len() == constants_usize::ONE
                && function.block.stmts.first().is_some_and(|statement| {
                    let syn::Stmt::Expr(expression, None) = statement else {
                        return false;
                    };
                    let is_inner_field = |expr: &syn::Expr| {
                        let syn::Expr::Field(field) = expr else {
                            return false;
                        };
                        let syn::Expr::Path(receiver) = field.base.as_ref() else {
                            return false;
                        };
                        receiver
                            .path
                            .is_ident(constants_str::CODE_STYLE_SELF_VALUE_IDENTIFIER)
                            && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
                    };
                    is_inner_field(expression)
                        || matches!(expression, syn::Expr::Reference(reference) if is_inner_field(reference.expr.as_ref()))
                        || matches!(expression, syn::Expr::MethodCall(call) if is_inner_field(call.receiver.as_ref()))
                })
        });
        if is_borrow_impl && forwards_inner {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_BORROW.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingDerefVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_deref_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DEREF_TRAIT_IDENTIFIER
            })
        });
        let wrapped_type_name = if let syn::Type::Path(path) = item_impl.self_ty.as_ref() {
            path.path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
        } else {
            None
        };
        let target_type = item_impl.items.iter().find_map(|item| {
            let syn::ImplItem::Type(associated_type) = item else {
                return None;
            };
            (associated_type.ident == constants_str::CODE_STYLE_TARGET_ASSOCIATED_TYPE_IDENTIFIER)
                .then_some(&associated_type.ty)
        });
        let targets_inner = wrapped_type_name
            .as_ref()
            .and_then(|name| self.inner_types.get(name))
            .zip(target_type)
            .is_some_and(|(inner_type, deref_target_type)| inner_type == deref_target_type);
        let forwards_inner = item_impl.items.iter().any(|item| {
            let syn::ImplItem::Fn(function) = item else {
                return false;
            };
            function.sig.ident == constants_str::CODE_STYLE_DEREF_FN_IDENTIFIER
                && function.block.stmts.len() == constants_usize::ONE
                && function.block.stmts.first().is_some_and(|statement| {
                    let syn::Stmt::Expr(expression, None) = statement else {
                        return false;
                    };
                    let syn::Expr::Reference(reference) = expression else {
                        return false;
                    };
                    let syn::Expr::Field(field) = reference.expr.as_ref() else {
                        return false;
                    };
                    let syn::Expr::Path(receiver) = field.base.as_ref() else {
                        return false;
                    };
                    receiver
                        .path
                        .is_ident(constants_str::CODE_STYLE_SELF_VALUE_IDENTIFIER)
                        && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
                })
        });
        if is_deref_impl && targets_inner && forwards_inner {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_DEREF.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &item_struct.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
        {
            let _previous = self
                .inner_types
                .insert(item_struct.ident.to_string(), field.ty.clone());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ForwardingDisplayVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ManualErrorImplVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ManualNotImplVisitor {
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ConstDisplayImplVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for ConstDisplayImplVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_display_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DISPLAY_TRAIT_IDENTIFIER
            })
        });
        let writes_constant = item_impl.items.len() == constants_usize::ONE
            && item_impl.items.first().is_some_and(|item| {
                let syn::ImplItem::Fn(function) = item else {
                    return false;
                };
                function.block.stmts.len() == constants_usize::ONE
                    && function.block.stmts.first().is_some_and(|statement| {
                        let syn::Stmt::Expr(syn::Expr::MethodCall(call), None) = statement else {
                            return false;
                        };
                        call.method == constants_str::CODE_STYLE_WRITE_STR_FN_IDENTIFIER
                            && call.args.len() == constants_usize::ONE
                            && call.args.first().is_some_and(|argument| {
                                matches!(argument, syn::Expr::Path(path) if path.path.segments.first().is_some_and(|segment| segment.ident == constants_str::STR_CONSTANTS_CRATE_IDENTIFIER))
                            })
                    })
            });
        if is_display_impl && writes_constant {
            let start = syn::spanned::Spanned::span(item_impl).start();
            self.errors.push(format!(
                "constant Display implementation at {}:{}; derive proc_macro_newtype::DisplayConst instead",
                start.line, start.column
            ));
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualNotImplVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_not_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_NOT_TRAIT_IDENTIFIER
            })
        });
        if is_not_impl {
            let start = syn::spanned::Spanned::span(item_impl).start();
            self.errors.push(format!(
                "manual Not implementation at {}:{}; derive proc_macro_newtype::NotInner instead",
                start.line, start.column
            ));
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualErrorImplVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_error_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_ERROR_TRAIT_IDENTIFIER
            })
        });
        if is_error_impl {
            let start = syn::spanned::Spanned::span(item_impl).start();
            self.errors.push(format!(
                "manual Error implementation at {}:{}; derive thiserror::Error instead",
                start.line, start.column
            ));
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct JsonCallVisitor {
    found: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for JsonCallVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if matches!(
            expr_call.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_AXUM_JSON_IDENTIFIER
                })
        ) {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct JsonIntoResponseErrorVisitor<'names_lt> {
    errors: crate::types::DiagnosticMessages,
    thiserror_enum_names: &'names_lt crate::types::SourceTextBTreeSet,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct TupleResponseVisitor {
    found: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for TupleResponseVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if expr_call
            .args
            .iter()
            .any(|argument| matches!(argument, syn::Expr::Tuple(_)))
            && matches!(
                expr_call.func.as_ref(),
                syn::Expr::Path(path)
                    if path.path.segments.last().is_some_and(|segment| {
                        segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_METHOD_IDENTIFIER
                    })
            )
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, expr_call);
    }

    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::CODE_STYLE_INTO_RESPONSE_METHOD_IDENTIFIER
            && matches!(expr_method_call.receiver.as_ref(), syn::Expr::Tuple(_))
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
}
impl<'ast> syn::visit::Visit<'ast> for JsonIntoResponseErrorVisitor<'_> {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_into_response = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_TRAIT_IDENTIFIER
            })
        });
        if is_into_response {
            let mut json_visitor = JsonCallVisitor::default();
            syn::visit::Visit::visit_item_impl(&mut json_visitor, item_impl);
            let mut tuple_visitor = TupleResponseVisitor::default();
            syn::visit::Visit::visit_item_impl(&mut tuple_visitor, item_impl);
            if json_visitor.found.get() || tuple_visitor.found.get() {
                let name = crate::code_style::item_impl_self_ty_identifier(
                    crate::types::SynItemImplRef::from(item_impl),
                )
                .map_or_else(
                    || String::from(constants_str::NON_PATH_TARGET),
                    String::from,
                );
                if !self.thiserror_enum_names.contains(name.as_str()) {
                    self.errors.push(format!(
                        "JSON API error response type `{name}` must be an enum deriving thiserror::Error"
                    ));
                }
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct ThiserrorEnumVisitor {
    location_names: crate::types::SourceTextBTreeSet,
    names: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for ThiserrorEnumVisitor {
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        let derives_thiserror = item_enum.attrs.iter().any(|attr| {
            if !attr.path().is_ident(constants_str::DERIVE) {
                return false;
            }
            let mut found = false;
            drop(attr.parse_nested_meta(|meta| {
                found |= meta.path.segments.first().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_THISERROR_CRATE_IDENTIFIER
                }) && meta.path.segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_ERROR_TRAIT_IDENTIFIER
                });
                Ok(())
            }));
            found
        });
        if derives_thiserror {
            let _: bool = self.names.insert(item_enum.ident.to_string());
            let derives_location =
                item_enum.attrs.iter().any(|attr| {
                    if !attr.path().is_ident(constants_str::DERIVE) {
                        return false;
                    }
                    let mut found = false;
                    drop(attr.parse_nested_meta(|meta| {
                        found |=
                            meta.path.segments.first().is_some_and(|segment| {
                                segment.ident == constants_str::LOCATION_ALT
                            }) && meta
                                .path
                                .segments
                                .last()
                                .is_some_and(|segment| segment.ident == constants_str::LOCATION);
                        Ok(())
                    }));
                    found
                });
            let has_location = derives_location
                || item_enum.variants.iter().any(|variant| {
                    variant.fields.iter().any(|field| {
                        let syn::Type::Path(path) = &field.ty else {
                            return false;
                        };
                        path.path
                            .segments
                            .first()
                            .is_some_and(|segment| segment.ident == constants_str::LOCATION_LIB)
                            && path
                                .path
                                .segments
                                .last()
                                .is_some_and(|segment| segment.ident == constants_str::LOCATION)
                    })
                });
            if has_location {
                let _: bool = self.location_names.insert(item_enum.ident.to_string());
            }
        }
        syn::visit::visit_item_enum(self, item_enum);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ApiErrorLocationVisitor<'names_lt> {
    errors: crate::types::DiagnosticMessages,
    thiserror_location_enum_names: &'names_lt crate::types::SourceTextBTreeSet,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct IntoResponseTypeVisitor {
    names: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for IntoResponseTypeVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_into_response = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_TRAIT_IDENTIFIER
            })
        });
        if is_into_response
            && let Some(name) = crate::code_style::item_impl_self_ty_identifier(
                crate::types::SynItemImplRef::from(item_impl),
            )
        {
            let _: bool = self.names.insert(String::from(name));
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ApiErrorSourceVisitor<'names_lt> {
    api_error_names: &'names_lt crate::types::SourceTextBTreeSet,
    errors: crate::types::DiagnosticMessages,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
#[getters(get_mut)]
pub(super) struct RouteOperationErrorVisitor {
    errors: crate::types::DiagnosticMessages,
    names: crate::types::SourceTextBTreeSet,
    operations: crate::types::SourceTextBTreeSet,
    registered: crate::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for RouteOperationErrorVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let route_error_name = item_fn.attrs.iter().find_map(|attr| {
            attr.path().segments.last().and_then(|segment| {
                let path = (segment.ident == constants_str::CODE_STYLE_ROUTE_ERROR_IDENTIFIER)
                    .then(|| attr.parse_args::<syn::Path>().ok())
                    .flatten()?;
                path.segments.last().map(|value| value.ident.to_string())
            })
        });
        let is_route_operation = route_error_name.is_some()
            || item_fn.attrs.iter().any(|attr| {
                attr.path().segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_ROUTE_OPENAPI_IDENTIFIER
                        || segment.ident == constants_str::CODE_STYLE_ROUTE_OPERATION_IDENTIFIER
                })
            });
        if is_route_operation {
            let _: bool = self.operations.insert(item_fn.sig.ident.to_string());
            let error_name = route_error_name.or_else(|| match &item_fn.sig.output {
                syn::ReturnType::Type(_, output) => match output.as_ref() {
                    syn::Type::Path(result) => result
                        .path
                        .segments
                        .last()
                        .and_then(|segment| match &segment.arguments {
                            syn::PathArguments::AngleBracketed(arguments) => {
                                arguments.args.iter().nth(constants_usize::ONE)
                            }
                            syn::PathArguments::None | syn::PathArguments::Parenthesized(_) => None,
                        })
                        .and_then(|argument| match argument {
                            syn::GenericArgument::Type(syn::Type::Path(error)) => {
                                error.path.segments.last()
                            }
                            syn::GenericArgument::AssocConst(_)
                            | syn::GenericArgument::AssocType(_)
                            | syn::GenericArgument::Const(_)
                            | syn::GenericArgument::Constraint(_)
                            | syn::GenericArgument::Lifetime(_)
                            | syn::GenericArgument::Type(_)
                            | _ => None,
                        })
                        .map(|segment| segment.ident.to_string()),
                    syn::Type::Array(_)
                    | syn::Type::FnPtr(_)
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
                    | _ => None,
                },
                syn::ReturnType::Default => None,
            });
            match error_name {
                Some(name) if self.names.insert(name.clone()) => {}
                Some(name) => self.errors.push(format!(
                    "route operation `{}` reuses error type `{name}`",
                    item_fn.sig.ident
                )),
                None => {}
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_macro(&mut self, item_macro: &'ast syn::ItemMacro) {
        if item_macro.mac.path.segments.last().is_some_and(|segment| {
            segment.ident == constants_str::CODE_STYLE_ENDPOINT_REGISTRY_IDENTIFIER
        }) {
            item_macro
                .mac
                .tokens
                .clone()
                .into_iter()
                .filter_map(|token| match token {
                    proc_macro2::TokenTree::Group(group)
                        if group.delimiter() == proc_macro2::Delimiter::Parenthesis
                            && group.stream().into_iter().any(|child| {
                                matches!(
                                    child,
                                    proc_macro2::TokenTree::Punct(punctuation)
                                        if punctuation.as_char() == ','
                                )
                            }) =>
                    {
                        group
                            .stream()
                            .into_iter()
                            .filter_map(|child| match child {
                                proc_macro2::TokenTree::Ident(identifier) => {
                                    Some(identifier.to_string())
                                }
                                proc_macro2::TokenTree::Group(_)
                                | proc_macro2::TokenTree::Literal(_)
                                | proc_macro2::TokenTree::Punct(_) => None,
                            })
                            .last()
                    }
                    proc_macro2::TokenTree::Group(_)
                    | proc_macro2::TokenTree::Ident(_)
                    | proc_macro2::TokenTree::Literal(_)
                    | proc_macro2::TokenTree::Punct(_) => None,
                })
                .for_each(|endpoint| {
                    let _: bool = self.registered.insert(endpoint);
                });
        }
        syn::visit::visit_item_macro(self, item_macro);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ApiErrorSourceVisitor<'_> {
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        if self
            .api_error_names
            .contains(item_enum.ident.to_string().as_str())
        {
            item_enum.variants.iter().for_each(|variant| {
                variant.fields.iter().for_each(|field| {
                    let is_source = field.attrs.iter().any(|attr| {
                        attr.path()
                            .is_ident(constants_str::CODE_STYLE_SOURCE_ATTRIBUTE_IDENTIFIER)
                    });
                    let is_observed = matches!(
                        &field.ty,
                        syn::Type::Path(path)
                            if path.path.segments.last().is_some_and(|segment| {
                                segment.ident
                                    == constants_str::CODE_STYLE_OBSERVED_ERROR_IDENTIFIER
                            })
                    );
                    if is_source && !is_observed {
                        self.errors.push(format!(
                            "API response error `{}::{}` source must be wrapped in ObservedError",
                            item_enum.ident, variant.ident
                        ));
                    }
                });
            });
        }
        syn::visit::visit_item_enum(self, item_enum);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ApiErrorLocationVisitor<'_> {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_into_response = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_TRAIT_IDENTIFIER
            })
        });
        if is_into_response {
            let name = crate::code_style::item_impl_self_ty_identifier(
                crate::types::SynItemImplRef::from(item_impl),
            )
            .map_or_else(
                || String::from(constants_str::NON_PATH_TARGET),
                String::from,
            );
            if self.thiserror_location_enum_names.contains(name.as_str()) {
                self.errors.push(format!(
                    "API response error enum `{name}` must keep source location in HttpErrorDiagnostic"
                ));
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingDisplayVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_inner_field = |expression: &syn::Expr| {
            let referenced_expression = if let syn::Expr::Reference(reference) = expression {
                reference.expr.as_ref()
            } else {
                expression
            };
            let syn::Expr::Field(field) = referenced_expression else {
                return false;
            };
            let syn::Expr::Path(receiver) = field.base.as_ref() else {
                return false;
            };
            receiver
                .path
                .is_ident(constants_str::CODE_STYLE_SELF_VALUE_IDENTIFIER)
                && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
        };
        let is_formatter = |expression: &syn::Expr| matches!(expression, syn::Expr::Path(formatter) if formatter.path.is_ident(constants_str::CODE_STYLE_FMT_ARGUMENT_IDENTIFIER));
        let is_display_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DISPLAY_TRAIT_IDENTIFIER
            })
        });
        let is_forwarding = item_impl.items.len() == constants_usize::ONE
            && item_impl.items.first().is_some_and(|item| {
                let syn::ImplItem::Fn(function) = item else {
                    return false;
                };
                function.sig.ident == constants_str::CODE_STYLE_FMT_FN_IDENTIFIER
                    && function.block.stmts.len() == constants_usize::ONE
                    && function.block.stmts.first().is_some_and(|statement| {
                        let syn::Stmt::Expr(expression, None) = statement else {
                            return false;
                        };
                        if let syn::Expr::MethodCall(call) = expression {
                            return is_inner_field(call.receiver.as_ref())
                                && call.method == constants_str::CODE_STYLE_FMT_FN_IDENTIFIER
                                && call.args.len() == constants_usize::ONE
                                && call.args.first().is_some_and(is_formatter);
                        }
                        if let syn::Expr::Call(call) = expression {
                            let syn::Expr::Path(function_path) = call.func.as_ref() else {
                                return false;
                            };
                            return function_path.path.segments.last().is_some_and(|segment| {
                                segment.ident == constants_str::CODE_STYLE_FMT_FN_IDENTIFIER
                            }) && function_path.path.segments.iter().any(|segment| {
                                segment.ident == constants_str::CODE_STYLE_DISPLAY_TRAIT_IDENTIFIER
                            }) && call.args.len() == constants_usize::TWO
                                && call.args.first().is_some_and(is_inner_field)
                                && call
                                    .args
                                    .iter()
                                    .nth(constants_usize::ONE)
                                    .is_some_and(is_formatter);
                        }
                        let syn::Expr::Macro(macro_expression) = expression else {
                            return false;
                        };
                        if !macro_expression.mac.path.is_ident(constants_str::WRITE_ALT) {
                            return false;
                        }
                        let parser = syn::punctuated::Punctuated::<
                            syn::Expr,
                            syn::Token![,],
                        >::parse_terminated;
                        let Ok(arguments) =
                            syn::parse::Parser::parse2(parser, macro_expression.mac.tokens.clone())
                        else {
                            return false;
                        };
                        arguments.len() == constants_usize::THREE
                            && arguments.first().is_some_and(is_formatter)
                            && arguments
                                .iter()
                                .nth(constants_usize::TWO)
                                .is_some_and(is_inner_field)
                            && arguments
                                .iter()
                                .nth(constants_usize::ONE)
                                .is_some_and(|argument| {
                                    matches!(argument, syn::Expr::Lit(syn::ExprLit {
                                        lit: syn::Lit::Str(value),
                                        ..
                                    }) if value.value().as_bytes() == b"{}")
                                })
                    })
            });
        if is_display_impl && is_forwarding {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_DISPLAY.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ForwardingIntoIteratorVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingIntoIteratorVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let is_into_iterator_impl = item_impl.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_ITERATOR_TRAIT_IDENTIFIER
            })
        });
        let forwards_inner = item_impl.items.iter().any(|item| {
            let syn::ImplItem::Fn(function) = item else {
                return false;
            };
            function.sig.ident == constants_str::CODE_STYLE_INTO_ITERATOR_FN_IDENTIFIER
                && function.block.stmts.len() == constants_usize::ONE
                && function.block.stmts.first().is_some_and(|statement| {
                    let syn::Stmt::Expr(syn::Expr::MethodCall(call), None) = statement else {
                        return false;
                    };
                    let syn::Expr::Field(field) = call.receiver.as_ref() else {
                        return false;
                    };
                    let syn::Expr::Path(receiver) = field.base.as_ref() else {
                        return false;
                    };
                    call.method == constants_str::CODE_STYLE_INTO_ITERATOR_FN_IDENTIFIER
                        && receiver
                            .path
                            .is_ident(constants_str::CODE_STYLE_SELF_VALUE_IDENTIFIER)
                        && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
                })
        });
        if is_into_iterator_impl && forwards_inner {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_INTO_ITERATOR.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PassthroughIntoInnerFromVisitor {
    errors: crate::types::DiagnosticMessages,
    inner_types: std::collections::BTreeMap<String, syn::Type>,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PassthroughIntoMethodVisitor {
    errors: crate::types::DiagnosticMessages,
    tuple_struct_names: std::collections::BTreeSet<String>,
}
impl<'ast> syn::visit::Visit<'ast> for PassthroughIntoMethodVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let wrapper_name = if item_impl.trait_.is_none() {
            let syn::Type::Path(self_type) = item_impl.self_ty.as_ref() else {
                return syn::visit::visit_item_impl(self, item_impl);
            };
            self_type
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
        } else {
            None
        };
        let is_tuple_wrapper = wrapper_name
            .as_ref()
            .is_some_and(|name| self.tuple_struct_names.contains(name));
        let has_passthrough_into_method = is_tuple_wrapper
            && item_impl.items.iter().any(|item| {
                let syn::ImplItem::Fn(function) = item else {
                    return false;
                };
                let consumes_self = function.sig.inputs.first().is_some_and(|input| {
                    matches!(input, syn::FnArg::Receiver(receiver) if matches!(receiver.kind, syn::ReceiverKind::Value))
                });
                consumes_self
                    && function.sig.ident == constants_str::INTO_MODEL
                    && function.block.stmts.len() == constants_usize::ONE
                    && function.block.stmts.first().is_some_and(|statement| {
                        let syn::Stmt::Expr(syn::Expr::Field(field), None) = statement else {
                            return false;
                        };
                        let syn::Expr::Path(receiver) = field.base.as_ref() else {
                            return false;
                        };
                        receiver
                            .path
                            .is_ident(constants_str::CODE_STYLE_SELF_VALUE_IDENTIFIER)
                            && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
                    })
            });
        if has_passthrough_into_method {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_INTO_MODEL.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if matches!(&item_struct.fields, syn::Fields::Unnamed(fields) if fields.unnamed.len() == constants_usize::ONE)
        {
            let _inserted = self
                .tuple_struct_names
                .insert(item_struct.ident.to_string());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
impl<'ast> syn::visit::Visit<'ast> for PassthroughIntoInnerFromVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let source_wrapper_name = item_impl.trait_.as_ref().and_then(|(path, _)| {
            let segment = path.segments.last()?;
            if segment.ident != constants_str::CODE_STYLE_FROM_TRAIT_IDENTIFIER {
                return None;
            }
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };
            let syn::GenericArgument::Type(syn::Type::Path(source)) = arguments.args.first()?
            else {
                return None;
            };
            source
                .path
                .segments
                .last()
                .map(|value| value.ident.to_string())
        });
        let targets_inner = source_wrapper_name
            .as_ref()
            .and_then(|name| self.inner_types.get(name))
            .is_some_and(|inner| inner == item_impl.self_ty.as_ref());
        let forwards_inner = item_impl.items.iter().any(|item| {
            let syn::ImplItem::Fn(function) = item else {
                return false;
            };
            let parameter_name = function.sig.inputs.first().and_then(|argument| {
                let syn::FnArg::Typed(typed_argument) = argument else {
                    return None;
                };
                let syn::Pat::Ident(identifier) = typed_argument.pat.as_ref() else {
                    return None;
                };
                Some(&identifier.ident)
            });
            function.sig.ident == constants_str::CODE_STYLE_FROM_FN_IDENTIFIER
                && function.block.stmts.len() == constants_usize::ONE
                && function.block.stmts.first().is_some_and(|statement| {
                    let syn::Stmt::Expr(syn::Expr::Field(field), None) = statement else {
                        return false;
                    };
                    let syn::Expr::Path(receiver) = field.base.as_ref() else {
                        return false;
                    };
                    parameter_name.is_some_and(|name| receiver.path.is_ident(name))
                        && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
                })
        });
        if targets_inner && forwards_inner {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_PASSTHROUGH_INTO_INNER_FROM.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &item_struct.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
        {
            let _previous = self
                .inner_types
                .insert(item_struct.ident.to_string(), field.ty.clone());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PassthroughFromVisitor {
    errors: crate::types::DiagnosticMessages,
    inner_types: std::collections::BTreeMap<String, syn::Type>,
}
impl<'ast> syn::visit::Visit<'ast> for PassthroughFromVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let from_type = item_impl.trait_.as_ref().and_then(|(path, _)| {
            let segment = path.segments.last()?;
            if segment.ident != constants_str::CODE_STYLE_FROM_TRAIT_IDENTIFIER {
                return None;
            }
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };
            let argument = arguments.args.first()?;
            let syn::GenericArgument::Type(value) = argument else {
                return None;
            };
            Some(value)
        });
        let wrapped_type_name = if let syn::Type::Path(path) = item_impl.self_ty.as_ref() {
            path.path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
        } else {
            None
        };
        let wraps_from_type = wrapped_type_name
            .as_ref()
            .and_then(|name| self.inner_types.get(name))
            .zip(from_type)
            .is_some_and(|(inner_type, source_type)| inner_type == source_type);
        let is_passthrough = item_impl.items.len() == constants_usize::ONE
            && item_impl.items.first().is_some_and(|item| {
                let syn::ImplItem::Fn(function) = item else {
                    return false;
                };
                function.sig.ident == constants_str::CODE_STYLE_FROM_FN_IDENTIFIER
                    && function.block.stmts.len() == constants_usize::ONE
                    && function.block.stmts.first().is_some_and(|statement| {
                        let syn::Stmt::Expr(expression, None) = statement else {
                            return false;
                        };
                        let syn::Expr::Call(call) = expression else {
                            return false;
                        };
                        let syn::Expr::Path(constructor) = call.func.as_ref() else {
                            return false;
                        };
                        constructor
                            .path
                            .is_ident(constants_str::CODE_STYLE_SELF_CONSTRUCTOR_IDENTIFIER)
                            && call.args.len() == constants_usize::ONE
                            && call.args.first().is_some_and(|argument| {
                                let syn::Expr::Path(value) = argument else {
                                    return false;
                                };
                                value
                                    .path
                                    .is_ident(constants_str::CODE_STYLE_VALUE_IDENTIFIER)
                            })
                    })
            });
        if wraps_from_type && is_passthrough {
            self.errors
                .push(constants_str::CODE_STYLE_MANUAL_PASSTHROUGH_FROM.to_owned());
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &item_struct.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
        {
            let _previous = self
                .inner_types
                .insert(item_struct.ident.to_string(), field.ty.clone());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct TestStringLiteralVisitor {
    values: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for TestStringLiteralVisitor {
    fn visit_expr_lit(&mut self, expr_lit: &'ast syn::ExprLit) {
        if let syn::Lit::Str(literal_string) = &expr_lit.lit {
            self.values.push(literal_string.value());
        }
        syn::visit::visit_expr_lit(self, expr_lit);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        let mut streams = vec![r#macro.tokens.clone()];
        while let Some(stream) = streams.pop() {
            stream.into_iter().for_each(|token| match token {
                proc_macro2::TokenTree::Group(group) => streams.push(group.stream()),
                proc_macro2::TokenTree::Literal(literal) => {
                    if let Ok(value) = syn::parse_str::<syn::LitStr>(literal.to_string().as_str()) {
                        self.values.push(value.value());
                    }
                }
                proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
            });
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct ProductionStringLiteralVisitor {
    values: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ProductionStringLiteralVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if attribute.path().is_ident(constants_str::PATH_ALT_5) {
            return;
        }
        syn::visit::visit_attribute(self, attribute);
    }
    fn visit_expr_lit(&mut self, expr_lit: &'ast syn::ExprLit) {
        if let syn::Lit::Str(literal_string) = &expr_lit.lit {
            self.values.push(literal_string.value());
        }
        syn::visit::visit_expr_lit(self, expr_lit);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "alignment order required by optimal_memory_layout takes precedence over alphabetical field order"
)]
pub(super) struct StringConstantDeclarationVisitor {
    errors: crate::types::DiagnosticMessages,
    allow_generated_string_constants: crate::types::AnalyzerBool,
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct ConstantInitializerStringLiteralVisitor {
    found: crate::types::AnalyzerBool,
}
impl ConstantInitializerStringLiteralVisitor {
    fn contains(expr: &syn::Expr) -> crate::types::AnalyzerBool {
        let mut visitor = Self::default();
        syn::visit::Visit::visit_expr(&mut visitor, expr);
        visitor.found
    }
    fn static_type_is_string_constant(syn_type_ref: crate::types::SynTypeRef<'_>) -> bool {
        match syn_type_ref.get() {
            syn::Type::Array(array) => Self::static_type_is_string_constant(
                crate::types::SynTypeRef::from(array.elem.as_ref()),
            ),
            syn::Type::Group(group) => Self::static_type_is_string_constant(
                crate::types::SynTypeRef::from(group.elem.as_ref()),
            ),
            syn::Type::Paren(paren) => Self::static_type_is_string_constant(
                crate::types::SynTypeRef::from(paren.elem.as_ref()),
            ),
            syn::Type::Path(path) => path.path.segments.last().is_some_and(|segment| {
                matches!(
                    segment.ident.to_string().as_str(),
                    constants_str::STR_ALT | constants_str::STRING
                )
            }),
            syn::Type::Reference(reference) => Self::static_type_is_string_constant(
                crate::types::SynTypeRef::from(reference.elem.as_ref()),
            ),
            syn::Type::Slice(slice) => Self::static_type_is_string_constant(
                crate::types::SynTypeRef::from(slice.elem.as_ref()),
            ),
            syn::Type::Tuple(tuple) => tuple.elems.iter().any(|element| {
                Self::static_type_is_string_constant(crate::types::SynTypeRef::from(element))
            }),
            syn::Type::FnPtr(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Verbatim(_)
            | _ => false,
        }
    }
}
#[allow(
    unused_variables,
    reason = "no-op visitor hooks preserve repository type-based parameter names"
)]
impl<'ast> syn::visit::Visit<'ast> for ConstantInitializerStringLiteralVisitor {
    fn visit_expr_closure(&mut self, expr_closure: &'ast syn::ExprClosure) {
        let _: &syn::ExprClosure = std::hint::black_box(expr_closure);
    }
    fn visit_expr_lit(&mut self, expr_lit: &'ast syn::ExprLit) {
        if matches!(expr_lit.lit, syn::Lit::Str(_)) {
            self.found.set_true();
        }
    }
    fn visit_item(&mut self, item: &'ast syn::Item) {
        let _: &syn::Item = std::hint::black_box(item);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        let _: &syn::Macro = std::hint::black_box(r#macro);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StringConstantDeclarationVisitor {
    fn visit_expr_const(&mut self, expr_const: &'ast syn::ExprConst) {
        let mut literal_visitor = TestStringLiteralVisitor {
            values: crate::types::SourceTextList::default(),
        };
        syn::visit::Visit::visit_block(&mut literal_visitor, &expr_const.block);
        if !literal_visitor.values.is_empty() {
            self.errors.push(constants_str::VALUE_FEDD2A2E.to_owned());
        }
        syn::visit::visit_expr_const(self, expr_const);
    }
    fn visit_impl_item_const(&mut self, impl_item_const: &'ast syn::ImplItemConst) {
        if crate::code_style::type_stores_string_text(crate::types::SynTypeRef::from(
            &impl_item_const.ty,
        ))
        .get()
            || ConstantInitializerStringLiteralVisitor::contains(&impl_item_const.expr).get()
        {
            self.errors.push(format!(
                "associated string constant `{}` must be declared in constants_str",
                impl_item_const.ident
            ));
        }
        syn::visit::visit_impl_item_const(self, impl_item_const);
    }
    fn visit_impl_item_fn(&mut self, impl_item_fn: &'ast syn::ImplItemFn) {
        if impl_item_fn.sig.constness.is_some() {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: crate::types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, &impl_item_fn.block);
            if !literal_visitor.values.is_empty() {
                self.errors.push(format!(
                    "const method `{}` contains string literals",
                    impl_item_fn.sig.ident
                ));
            }
        }
        syn::visit::visit_impl_item_fn(self, impl_item_fn);
    }
    fn visit_item_const(&mut self, item_const: &'ast syn::ItemConst) {
        if crate::code_style::type_stores_string_text(crate::types::SynTypeRef::from(
            item_const.ty.as_ref(),
        ))
        .get()
            || ConstantInitializerStringLiteralVisitor::contains(item_const.expr.as_ref()).get()
        {
            self.errors.push(format!(
                "string constant `{}` must be declared in constants_str",
                item_const.ident
            ));
        }
        syn::visit::visit_item_const(self, item_const);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if item_fn.sig.constness.is_some() {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: crate::types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, &item_fn.block);
            if !literal_visitor.values.is_empty() {
                self.errors.push(format!(
                    "const function `{}` contains string literals",
                    item_fn.sig.ident
                ));
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_static(&mut self, item_static: &'ast syn::ItemStatic) {
        if ConstantInitializerStringLiteralVisitor::static_type_is_string_constant(
            crate::types::SynTypeRef::from(item_static.ty.as_ref()),
        ) || ConstantInitializerStringLiteralVisitor::contains(item_static.expr.as_ref()).get()
        {
            self.errors.push(format!(
                "string static `{}` must be declared in constants_str",
                item_static.ident
            ));
        }
        syn::visit::visit_item_static(self, item_static);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        fn group_contains_str(group: &proc_macro2::Group) -> bool {
            group.stream().into_iter().any(|token| match token {
                proc_macro2::TokenTree::Group(nested) => group_contains_str(&nested),
                proc_macro2::TokenTree::Ident(ident) => ident == constants_str::STR_ALT,
                proc_macro2::TokenTree::Literal(_) | proc_macro2::TokenTree::Punct(_) => false,
            })
        }
        fn contains(token_stream: proc_macro2::TokenStream) -> bool {
            let token_trees = token_stream.into_iter().collect::<Vec<_>>();
            token_trees.iter().enumerate().any(|(index, token)| {
                if let proc_macro2::TokenTree::Group(group) = token
                    && contains(group.stream())
                {
                    return true;
                }
                if !matches!(
                    token,
                    proc_macro2::TokenTree::Ident(ident)
                        if ident == constants_str::VALUE_F75C6596 || ident == constants_str::STATIC
                ) {
                    return false;
                }
                if matches!(
                    index.checked_sub(constants_usize::ONE).and_then(|previous| token_trees.get(previous)),
                    Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '\''
                ) {
                    return false;
                }
                if matches!(
                    token_trees.iter().skip(index).nth(constants_usize::ONE),
                    Some(proc_macro2::TokenTree::Ident(ident)) if ident == constants_str::VALUE_0F1E18BB
                ) {
                    return false;
                }
                token_trees
                    .iter()
                    .skip(index)
                    .skip(constants_usize::ONE)
                    .try_fold(false, |stores_string, following| match following {
                        proc_macro2::TokenTree::Group(group) => {
                            Ok(stores_string || group_contains_str(group))
                        }
                        proc_macro2::TokenTree::Ident(ident) => Ok(stores_string || ident == constants_str::STR_ALT),
                        proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '=' => {
                            Err(stores_string)
                        }
                        proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ';' => {
                            Err(false)
                        }
                        proc_macro2::TokenTree::Literal(_) | proc_macro2::TokenTree::Punct(_) => {
                            Ok(stores_string)
                        }
                    })
                    .unwrap_or_else(|found| found)
            })
        }
        if r#macro.path.segments.last().is_some_and(|segment| {
            segment.ident == constants_str::SHARED_VALUES_DEFINE_STR_CONSTANTS
        }) {
            self.errors.push(constants_str::VALUE_23159C36.to_owned());
        }
        if !self.allow_generated_string_constants.get() && contains(r#macro.tokens.clone()) {
            self.errors.push(constants_str::VALUE_BA372BD2.to_owned());
        }
        syn::visit::visit_macro(self, r#macro);
    }
    fn visit_trait_item_const(&mut self, trait_item_const: &'ast syn::TraitItemConst) {
        if crate::code_style::type_stores_string_text(crate::types::SynTypeRef::from(
            &trait_item_const.ty,
        ))
        .get()
            || trait_item_const
                .default
                .as_ref()
                .is_some_and(|(_, expression)| {
                    ConstantInitializerStringLiteralVisitor::contains(expression).get()
                })
        {
            self.errors.push(format!(
                "trait string constant `{}` must be declared in constants_str",
                trait_item_const.ident
            ));
        }
        syn::visit::visit_trait_item_const(self, trait_item_const);
    }
    fn visit_trait_item_fn(&mut self, trait_item_fn: &'ast syn::TraitItemFn) {
        if trait_item_fn.sig.constness.is_some()
            && let Some(block) = &trait_item_fn.default
        {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: crate::types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, block);
            if !literal_visitor.values.is_empty() {
                self.errors.push(format!(
                    "const trait method `{}` contains string literals",
                    trait_item_fn.sig.ident
                ));
            }
        }
        syn::visit::visit_trait_item_fn(self, trait_item_fn);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct StringConstantVisitor {
    errors: crate::types::DiagnosticMessages,
}

#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Default,
)]
pub(super) struct TracingMessageLiteralVisitor {
    values: crate::types::SourceTextList,
}

impl<'ast> syn::visit::Visit<'ast> for TracingMessageLiteralVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        let is_tracing_event = r#macro.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::CONFIG_TRACING_TRACE
                    | constants_str::CONFIG_TRACING_DEBUG
                    | constants_str::CONFIG_TRACING_INFO
                    | constants_str::CONFIG_TRACING_WARN
                    | constants_str::CONFIG_TRACING_ERROR
                    | constants_str::EVENT
            )
        });
        if is_tracing_event {
            let mut streams = vec![r#macro.tokens.clone()];
            while let Some(stream) = streams.pop() {
                stream.into_iter().for_each(|token| match token {
                    proc_macro2::TokenTree::Group(group) => streams.push(group.stream()),
                    proc_macro2::TokenTree::Literal(literal) => {
                        if let Ok(value) = syn::parse_str::<syn::LitStr>(&literal.to_string()) {
                            self.values.push(value.value());
                        }
                    }
                    proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
                });
            }
        }
        syn::visit::visit_macro(self, r#macro);
    }
}

#[allow(
    unused_variables,
    reason = "the no-op visitor hook preserves the repository type-based parameter name"
)]
impl<'ast> syn::visit::Visit<'ast> for StringConstantVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        let _: &syn::Attribute = std::hint::black_box(attribute);
    }
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if matches!(
            expr_call.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::COMPILE_ERROR_TOKEN_STREAM
                })
        ) {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: crate::types::SourceTextList::default(),
            };
            expr_call.args.iter().for_each(|arg| {
                syn::visit::Visit::visit_expr(&mut literal_visitor, arg);
            });
            if !literal_visitor.values.is_empty() {
                self.errors.push(
                    constants_str::COMPILE_ERROR_TOKEN_STREAM_CALL_CONTAINS_STRING_LITERALS
                        .to_owned(),
                );
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_expr_lit(&mut self, expr_lit: &'ast syn::ExprLit) {
        if let syn::Lit::Str(value) = &expr_lit.lit {
            let start = value.span().start();
            let end = value.span().end();
            self.errors.push(format!(
                "{}:{}-{}:{}: string literal {:?} must be declared in constants_str",
                start.line,
                start.column,
                end.line,
                end.column,
                value.value()
            ));
        }
        syn::visit::visit_expr_lit(self, expr_lit);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        let macro_name = r#macro
            .path
            .segments
            .last()
            .map_or_else(String::new, |segment| segment.ident.to_string());
        if matches!(
            macro_name.as_str(),
            constants_str::SHARED_VALUES_ASSERT
                | constants_str::SHARED_VALUES_ASSERT_EQ
                | constants_str::SHARED_VALUES_ASSERT_NE
                | constants_str::SHARED_VALUES_DEBUG_ASSERT
                | constants_str::SHARED_VALUES_DEBUG_ASSERT_EQ
                | constants_str::SHARED_VALUES_DEBUG_ASSERT_NE
        ) {
            let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
            if let Ok(expressions) = syn::parse::Parser::parse2(parser, r#macro.tokens.clone()) {
                let mut visitor = Self::new(crate::types::DiagnosticMessages::default());
                let required_expression_count = if matches!(
                    macro_name.as_str(),
                    constants_str::SHARED_VALUES_ASSERT | constants_str::SHARED_VALUES_DEBUG_ASSERT
                ) {
                    constants_usize::ONE
                } else {
                    constants_usize::TWO
                };
                expressions
                    .iter()
                    .take(required_expression_count)
                    .for_each(|expression| {
                        syn::visit::Visit::visit_expr(&mut visitor, expression);
                    });
                self.errors.extend(visitor.errors);
            }
            syn::visit::visit_macro(self, r#macro);
            return;
        }
        let is_syntax_boundary = r#macro.path.segments.last().is_some_and(|segment| {
            constants_str::CODE_STYLE_STRING_LITERAL_MACRO_BOUNDARIES
                .contains(&segment.ident.to_string().as_str())
        });
        if !is_syntax_boundary {
            let mut streams = vec![r#macro.tokens.clone()];
            let mut values = Vec::new();
            while let Some(stream) = streams.pop() {
                stream.into_iter().for_each(|token| match token {
                    proc_macro2::TokenTree::Group(group) => streams.push(group.stream()),
                    proc_macro2::TokenTree::Literal(literal) => {
                        if let Ok(value) = syn::parse_str::<syn::LitStr>(&literal.to_string()) {
                            let start = literal.span().start();
                            let end = literal.span().end();
                            values.push((
                                value.value(),
                                start.line,
                                start.column,
                                end.line,
                                end.column,
                            ));
                        }
                    }
                    proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
                });
            }
            while let Some((value, start_line, start_column, end_line, end_column)) = values.pop() {
                self.errors.push(format!(
                    "{start_line}:{start_column}-{end_line}:{end_column}: macro `{macro_name}` string literal {value:?} must be declared in constants_str"
                ));
            }
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct TestNameVisitor {
    errors: crate::types::DiagnosticMessages,
    module_names: crate::types::SourceTextList,
    root_test_found: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for TestNameVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let is_test = |attrs: &[syn::Attribute]| {
            attrs.iter().any(|attr| {
                attr.path()
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == constants_str::TEST_ATTRIBUTE_NAME)
            })
        };
        if is_test(item_fn.attrs.as_slice()) {
            if !item_fn
                .sig
                .ident
                .to_string()
                .starts_with(constants_str::TEST_NAME_PREFIX)
            {
                self.errors.push(format!(
                    "test function `{}` must start with `test_`",
                    item_fn.sig.ident
                ));
            }
            match self.module_names.last() {
                Some(module_name) if module_name == constants_str::TEST_TESTS => {
                    let error = format!("test module `{module_name}` must be named `tests`");
                    if !self.errors.contains(&error) {
                        self.errors.push(error);
                    }
                }
                Some(module_name)
                    if module_name != constants_str::TESTS_ALT
                        && !module_name.starts_with(constants_str::TEST_NAME_PREFIX) =>
                {
                    let error = format!(
                        "test module `{module_name}` must be named `tests` or start with `test_`"
                    );
                    if !self.errors.contains(&error) {
                        self.errors.push(error);
                    }
                }
                None => self.root_test_found = crate::types::AnalyzerBool::from(true),
                Some(_) => {}
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        self.module_names.push(item_mod.ident.to_string());
        syn::visit::visit_item_mod(self, item_mod);
        drop(self.module_names.pop());
    }
}

#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the lexical source scanner is grouped with the source visitors it supports"
)]
pub(super) fn rust_comment_position(source: &str) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes.get(index) == Some(&b'/')
            && matches!(
                bytes.get(index.saturating_add(constants_usize::ONE)),
                Some(b'/' | b'*')
            )
        {
            return Some(index);
        }
        let prefix_len = match bytes.get(index..index.saturating_add(constants_usize::TWO)) {
            Some([b'b' | b'c', b'"']) => constants_usize::ONE,
            _ => 0usize,
        };
        if bytes.get(index.saturating_add(prefix_len)) == Some(&b'"') {
            index = index
                .saturating_add(prefix_len)
                .saturating_add(constants_usize::ONE);
            while index < bytes.len() {
                match bytes.get(index) {
                    Some(b'\\') => index = index.saturating_add(constants_usize::TWO),
                    Some(b'"') => {
                        index = index.saturating_add(constants_usize::ONE);
                        break;
                    }
                    Some(_) | None => index = index.saturating_add(constants_usize::ONE),
                }
            }
            continue;
        }
        let raw_prefix_len = match bytes.get(index..) {
            Some([b'b' | b'c', b'r', ..]) => constants_usize::TWO,
            Some([b'r', ..]) => constants_usize::ONE,
            _ => 0usize,
        };
        if raw_prefix_len != 0usize {
            let mut opening_quote = index.saturating_add(raw_prefix_len);
            while bytes.get(opening_quote) == Some(&b'#') {
                opening_quote = opening_quote.saturating_add(constants_usize::ONE);
            }
            if bytes.get(opening_quote) == Some(&b'"') {
                let hash_count = opening_quote
                    .saturating_sub(index)
                    .saturating_sub(raw_prefix_len);
                index = opening_quote.saturating_add(constants_usize::ONE);
                while index < bytes.len() {
                    if bytes.get(index) == Some(&b'"')
                        && bytes
                            .get(
                                index.saturating_add(constants_usize::ONE)
                                    ..index
                                        .saturating_add(constants_usize::ONE)
                                        .saturating_add(hash_count),
                            )
                            .is_some_and(|hashes| hashes.iter().all(|byte| *byte == b'#'))
                    {
                        index = index
                            .saturating_add(constants_usize::ONE)
                            .saturating_add(hash_count);
                        break;
                    }
                    index = index.saturating_add(constants_usize::ONE);
                }
                continue;
            }
        }
        let char_prefix_len = usize::from(bytes.get(index) == Some(&b'b'));
        let apostrophe = index.saturating_add(char_prefix_len);
        if bytes.get(apostrophe) == Some(&b'\'') {
            let content = apostrophe.saturating_add(constants_usize::ONE);
            let char_end = if bytes.get(content) == Some(&b'\\') {
                match bytes.get(content.saturating_add(constants_usize::ONE)) {
                    Some(b'x') => content.saturating_add(4usize),
                    Some(b'u')
                        if bytes.get(content.saturating_add(constants_usize::TWO))
                            == Some(&b'{') =>
                    {
                        bytes
                            .get(content.saturating_add(3usize)..)
                            .and_then(|tail| tail.iter().position(|byte| *byte == b'}'))
                            .map_or(content, |offset| {
                                content.saturating_add(offset).saturating_add(4usize)
                            })
                    }
                    Some(_) => content.saturating_add(constants_usize::TWO),
                    None => content,
                }
            } else {
                source
                    .get(content..)
                    .and_then(|tail| tail.chars().next())
                    .map_or(content, |character| {
                        content.saturating_add(character.len_utf8())
                    })
            };
            if bytes.get(char_end) == Some(&b'\'') {
                index = char_end.saturating_add(constants_usize::ONE);
                continue;
            }
        }
        index = index.saturating_add(constants_usize::ONE);
    }
    None
}
