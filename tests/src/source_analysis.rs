#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DbgVisitor {
    pub found: super::types::AnalyzerBool,
}

#[derive(Default, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct CustomTypeNameVisitor {
    pub names: super::types::SourceTextList,
}

impl<'ast> syn::visit::Visit<'ast> for CustomTypeNameVisitor {
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.names.push(i.ident.to_string());
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.names.push(i.ident.to_string());
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.names.push(i.ident.to_string());
        syn::visit::visit_item_trait(self, i);
    }

    fn visit_item_trait_alias(&mut self, i: &'ast syn::ItemTraitAlias) {
        self.names.push(i.ident.to_string());
        syn::visit::visit_item_trait_alias(self, i);
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        self.names.push(i.ident.to_string());
        syn::visit::visit_item_type(self, i);
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        self.names.push(i.ident.to_string());
        syn::visit::visit_item_union(self, i);
    }
}

#[derive(Default, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct FreeFnNameVisitor {
    pub names: super::types::SourceTextList,
}

impl<'ast> syn::visit::Visit<'ast> for FreeFnNameVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.names.push(i.sig.ident.to_string());
        syn::visit::visit_item_fn(self, i);
    }
}

#[derive(Default, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct OptimalMemoryLayoutVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl OptimalMemoryLayoutVisitor {
    fn check_attrs(&mut self, identifier: &syn::Ident, attrs: &[syn::Attribute], kind: &str) {
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
            self.ers.push(format!(
                "{kind} `{identifier}` must derive `optimal_memory_layout::OptimalMemoryLayout`"
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for OptimalMemoryLayoutVisitor {
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.check_attrs(&i.ident, &i.attrs, constants_str::ENUM);
        syn::visit::visit_item_enum(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.check_attrs(&i.ident, &i.attrs, constants_str::STRUCT);
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DbgVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path
            .segments
            .last()
            .is_some_and(|v_4b8e1c7a| v_4b8e1c7a.ident == constants_str::DBG)
        {
            self.found.set_true();
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct TodoUnimplVisitor {
    pub todo_found: super::types::AnalyzerCount,
    pub unimplemented_found: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for TodoUnimplVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if let Some(last_segment) = i.path.segments.last() {
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
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct UnwrapVisitor {
    pub found_count: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnwrapVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::UNWRAP && i.args.is_empty() {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ForLoopVisitor {
    pub found_count: super::types::AnalyzerCount,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct SourceDroppingMapErrVisitor {
    pub found_count: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SourceDroppingMapErrVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::CODE_STYLE_MAP_ERR
            && i.args.first().is_some_and(|argument| {
                matches!(argument, syn::Expr::Closure(closure) if closure.inputs.iter().any(|input| matches!(input, syn::Pat::Wild(_))))
            })
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct NumericAsCastVisitor {
    pub found_count: super::types::AnalyzerCount,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct SerdeJsonValueFieldVisitor {
    pub violations: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueFieldVisitor {
    fn visit_field(&mut self, i: &'ast syn::Field) {
        let mut type_visitor = SerdeJsonValueTypeVisitor::default();
        syn::visit::Visit::visit_type(&mut type_visitor, &i.ty);
        if type_visitor.found.get() {
            self.violations
                .push(constants_str::CODE_STYLE_UNNAMED_ITEM.to_owned());
        }
        syn::visit::visit_field(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(&syn::Item::Struct(
            i.clone(),
        )))
        .get()
            || i.ident == constants_str::CODE_STYLE_SERDE_JSON_ADMIN_AUDIT_DETAILS
        {
            return;
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct SerdeJsonValueTypeVisitor {
    pub found: super::types::AnalyzerBool,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct PublicStructFieldVisitor {
    pub violations: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct CrateVisibleStructFieldVisitor {
    pub violations: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for CrateVisibleStructFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        i.fields
            .iter()
            .enumerate()
            .filter(|(_, field)| {
                matches!(
                    &field.vis,
                    syn::Visibility::Restricted(visibility)
                        if visibility
                            .path
                            .is_ident(constants_str::CODE_STYLE_CRATE_VISIBILITY_PATH)
                )
            })
            .for_each(|(index, field)| {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| index.to_string(), ToString::to_string);
                self.violations.push(format!("{}::{field_name}", i.ident));
            });
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicStructFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(&syn::Item::Struct(
            i.clone(),
        )))
        .get()
        {
            return;
        }
        i.fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !matches!(field.vis, syn::Visibility::Inherited))
            .for_each(|(index, field)| {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| index.to_string(), ToString::to_string);
                self.violations.push(format!("{}::{field_name}", i.ident));
            });
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for SerdeJsonValueTypeVisitor {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        let mut segments = i.path.segments.iter().rev();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_VALUE)
            && segments
                .next()
                .is_some_and(|segment| segment.ident == constants_str::SERDE_JSON)
        {
            self.found.set_true();
        }
        syn::visit::visit_type_path(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for NumericAsCastVisitor {
    fn visit_expr_cast(&mut self, i: &'ast syn::ExprCast) {
        let numeric = if let syn::Type::Path(path) = i.ty.as_ref() {
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
        syn::visit::visit_expr_cast(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForLoopVisitor {
    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        self.found_count.saturating_inc();
        syn::visit::visit_expr_for_loop(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct IncludeAssetMacroVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for IncludeAssetMacroVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if let Some(segment) = i.path.segments.last()
            && (segment.ident == constants_str::INCLUDE_STR
                || segment.ident == constants_str::INCLUDE_BYTES)
        {
            self.ers.push(format!("contains {}!()", segment.ident));
        }
        syn::visit::visit_macro(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DirectPathCallVisitor {
    pub calls: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct UnboundedReadVisitor {
    pub calls: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for UnboundedReadVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let Some(path) = super::expr_call_path(super::types::SynExprCallRef::from(i)) {
            let call = super::path_to_string(path);
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
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::PG_CRUD_PG_TEXT {
            self.calls.push(format!(".{}()", i.method));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectPathCallVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let Some(path) = super::expr_call_path(super::types::SynExprCallRef::from(i)) {
            self.calls
                .push(super::path_to_string(path).as_ref().to_owned());
        }
        syn::visit::visit_expr_call(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct LostSpawnVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for LostSpawnVisitor {
    fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
        let discarded = match i {
            syn::Stmt::Expr(expression, _) => super::unowned_spawn_expr(expression),
            syn::Stmt::Local(local) => local.init.as_ref().is_some_and(|init| {
                super::unowned_spawn_expr(init.expr.as_ref())
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
            self.ers.push(
                constants_str::SPAWN_RESULT_IS_DISCARDED_RETAIN_AND_SUPERVISE_TASK.to_owned(),
            );
        }
        syn::visit::visit_stmt(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct TestNondeterminismVisitor {
    pub calls: super::types::DiagnosticMsgs,
    pub test_depth: super::types::AnalyzerCount,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct SensitiveTextDebugDeriveVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct SensitiveErrorFormatVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct GeneratedRandomnessVisitor {
    pub calls: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct StaticStateVisitor {
    pub identifiers: super::types::SourceTextList,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct PrintMacroVisitor {
    pub calls: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ProductionLinePrintMacroVisitor {
    pub calls: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DoubleUnderscoreNamingVisitor {
    pub identifiers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ShortFunctionNamingVisitor {
    pub identifiers: super::types::DiagnosticMsgs,
}
impl ShortFunctionNamingVisitor {
    fn check_identifier(&mut self, identifier: &syn::Ident) {
        let identifier_text = identifier.to_string();
        if identifier_text.starts_with(constants_str::WORKSPACE_SHORT_MAKE_PREFIX) {
            self.identifiers.push(identifier_text);
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct PublicLogicVisitor {
    pub found: super::types::AnalyzerBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct OwnedTestVisitor {
    pub found: super::types::AnalyzerBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AllowReasonVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub lines: super::types::SourceTextList,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DiagnosticIdVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub ids: super::types::SourceTextList,
}
impl DiagnosticIdVisitor {
    pub(super) fn record(
        &mut self,
        kind: super::types::SourceTextRef<'_>,
        value: super::types::SourceTextRef<'_>,
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
                self.ers.push(format!(
                    "expect message diagnostic ID must be followed by at least two context words: {value:?}",
                    value = value.as_ref(),
                ));
            } else {
                self.ids.push(prefix.to_owned());
            }
        } else {
            self.ers.push(format!(
                "{kind} message must start with a unique eight-character lowercase hexadecimal diagnostic ID: {value:?}",
                kind = kind.as_ref(),
                value = value.as_ref(),
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for DiagnosticIdVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::CODE_STYLE_EXPECT_METHOD_NAME {
            match i.args.first() {
                Some(syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) if i.args.len() == constants_usize::ONE => self.record(
                    super::types::SourceTextRef::from(constants_str::CODE_STYLE_EXPECT_METHOD_NAME),
                    super::types::SourceTextRef::from(lit_str.value().as_str()),
                ),
                Some(_) | None => self.ers.push(constants_str::VALUE_3C063239.to_owned()),
            }
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if super::macro_path_is_quote(super::types::SynPathRef::from(&i.path)).get() {
            super::scan_generated_diagnostic_tokens(&i.tokens, self);
        }
        if i.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_PANIC_METHOD_NAME)
        {
            match i.tokens.clone().into_iter().next() {
                Some(proc_macro2::TokenTree::Literal(literal)) => {
                    match syn::parse_str::<syn::LitStr>(literal.to_string().as_str()) {
                        Ok(lit_str) => {
                            let value = lit_str.value();
                            if !super::panic_uses_dynamic_diagnostic_id(
                                super::types::SourceTextRef::from(value.as_str()),
                            )
                            .get()
                            {
                                self.record(
                                    super::types::SourceTextRef::from(
                                        constants_str::CODE_STYLE_PANIC_METHOD_NAME,
                                    ),
                                    super::types::SourceTextRef::from(value.as_str()),
                                );
                            }
                        }
                        Err(_error) => self.ers.push(constants_str::VALUE_CCFFF72E.to_owned()),
                    }
                }
                Some(_) | None => self.ers.push(constants_str::VALUE_CCFFF72E.to_owned()),
            }
        }
        syn::visit::visit_macro(self, i);
    }
}
#[allow(clippy::needless_for_each)] // repository source policy requires iterator methods instead of for loops
impl<'ast> syn::visit::Visit<'ast> for SensitiveTextDebugDeriveVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::sensitive_text_wrapper_identifier(super::types::SourceTextRef::from(
            i.ident.to_string().as_str(),
        ))
        .get()
            && i.fields
                .iter()
                .any(|field| super::type_contains_sensitive_text_or_bytes(&field.ty))
        {
            [
                constants_str::VALUE_1A03BD2F,
                constants_str::VALUE_34E108C0,
                constants_str::VALUE_4A177217,
                constants_str::VALUE_00857394,
            ]
            .into_iter()
            .for_each(|derive_name| {
                if i.attrs.iter().any(|attr| {
                    super::derive_attr_has_terminal(
                        super::types::SynAttributeRef::from(attr),
                        super::types::SourceTextRef::from(derive_name),
                    )
                    .get()
                }) {
                    self.ers.push(format!(
                        "sensitive text wrapper `{}` derives `{derive_name}` without redaction",
                        i.ident
                    ));
                }
            });
        }
        syn::visit::visit_item_struct(self, i);
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
                super::sensitive_text_wrapper_identifier(super::types::SourceTextRef::from(
                    identifier.to_string().as_str(),
                ))
                .get()
                .then(|| format!("{{{identifier}"))
            });
            let tuple_placeholder = field
                .ident
                .is_none()
                .then(|| format!("{{{index}"))
                .filter(|_| super::type_contains_sensitive_text_or_bytes(&field.ty));
            [named_placeholder, tuple_placeholder]
                .into_iter()
                .flatten()
                .for_each(|field_placeholder| {
                    if templates
                        .iter()
                        .any(|template| template.contains(field_placeholder.as_str()))
                    {
                        self.ers.push(format!(
                            "error formatter exposes sensitive field placeholder `{field_placeholder}`"
                        ));
                    }
                });
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SensitiveErrorFormatVisitor {
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        i.variants
            .iter()
            .for_each(|variant| self.inspect_fields(&variant.attrs, &variant.fields));
        syn::visit::visit_item_enum(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.inspect_fields(&i.attrs, &i.fields);
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for GeneratedRandomnessVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if super::macro_path_is_quote(super::types::SynPathRef::from(&i.path)).get() {
            let compact = i
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
        syn::visit::visit_macro(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StaticStateVisitor {
    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        self.identifiers.push(i.ident.to_string());
        syn::visit::visit_item_static(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for PrintMacroVisitor {
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::SHARED_VALUES_PRINT
                    | constants_str::SHARED_VALUES_PRINTLN
                    | constants_str::SHARED_VALUES_EPRINT
                    | constants_str::SHARED_VALUES_EPRINTLN
            )
        }) {
            self.calls.push(
                super::path_to_string(super::types::SynPathRef::from(&i.path))
                    .as_ref()
                    .to_owned(),
            );
        }
        syn::visit::visit_macro(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ProductionLinePrintMacroVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::TEST_ALT_3)
                || super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(attr)).get()
        }) {
            return;
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::SHARED_VALUES_PRINTLN | constants_str::SHARED_VALUES_EPRINTLN
            )
        }) {
            self.calls.push(
                super::path_to_string(super::types::SynPathRef::from(&i.path))
                    .as_ref()
                    .to_owned(),
            );
        }
        syn::visit::visit_macro(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DoubleUnderscoreNamingVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let identifier = i.sig.ident.to_string();
        if identifier.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE) {
            self.identifiers.push(identifier);
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let identifier = i.ident.to_string();
        if identifier.contains(constants_str::WORKSPACE_SCAFFOLD_DOUBLE_UNDERSCORE) {
            self.identifiers.push(identifier);
        }
        syn::visit::visit_item_mod(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ShortFunctionNamingVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.check_identifier(&i.sig.ident);
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        i.items.iter().for_each(|item| {
            if let syn::ImplItem::Fn(function) = item {
                self.check_identifier(&function.sig.ident);
            }
        });
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicLogicVisitor {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.found.set_true();
        }
        syn::visit::visit_impl_item_fn(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.found.set_true();
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        if matches!(i.vis, syn::Visibility::Public(_))
            && i.items.iter().any(|item| {
                matches!(
                    item,
                    syn::TraitItem::Fn(function) if function.default.is_some()
                )
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_item_trait(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for OwnedTestVisitor {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if i.path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::TEST_ALT_3)
        {
            self.found.set_true();
        }
        syn::visit::visit_attribute(self, i);
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        if i.ident == constants_str::TESTS_ALT
            && i.attrs.iter().any(|attribute| {
                super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(attribute)).get()
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_item_mod(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for AllowReasonVisitor {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        let is_lint_suppression = i.path().segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::VALUE_41008373 | constants_str::CODE_STYLE_EXPECT_METHOD_NAME
            )
        });
        if is_lint_suppression {
            let has_reason_argument = match &i.meta {
                syn::Meta::List(list) => list
                    .tokens
                    .to_string()
                    .contains(constants_str::VALUE_45F4C964),
                syn::Meta::Path(_) | syn::Meta::NameValue(_) => false,
            };
            let span = syn::spanned::Spanned::span(i);
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
                let path = i
                    .path()
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<String>>()
                    .join(constants_str::PATH_SEPARATOR);
                let attribute = match &i.meta {
                    syn::Meta::List(list) => format!("#[{path}({})]", list.tokens),
                    syn::Meta::NameValue(_) => format!("#[{path} = value]"),
                    syn::Meta::Path(_) => format!("#[{path}]"),
                };
                self.ers.push(format!(
                    "line {start_line}: lint suppression `{attribute}` requires an explicit reason"
                ));
            }
        }
        syn::visit::visit_attribute(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for TestNondeterminismVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if self.test_depth.get() != 0
            && let Some(path) = super::expr_call_path(super::types::SynExprCallRef::from(i))
        {
            let text = super::path_to_string(path);
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
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        if self.test_depth.get() != 0 {
            let text = super::path_to_string(super::types::SynPathRef::from(&i.path));
            if matches!(
                text.as_ref(),
                constants_str::RAND_PATH_RNGS_PATH_OS_RNG | constants_str::RAND_CORE_PATH_OS_RNG
            ) {
                self.calls.push(text.as_ref().to_owned());
            }
        }
        syn::visit::visit_expr_path(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let is_test = super::item_fn_is_unit_test(super::types::SynItemFnRef::from(i)).get();
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let is_test = i.attrs.iter().any(|attr| {
            super::attr_is_test_only_cfg(super::types::SynAttributeRef::from(attr)).get()
        });
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_mod(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "alignment order required by optimal_memory_layout takes precedence over alphabetical field order"
)]
pub(super) struct UseImportVisitor {
    pub public_use_roots: super::types::SourceTextList,
    pub allow_leptos_prelude_import: super::types::AnalyzerBool,
    pub found_non_public_use_import: super::types::AnalyzerBool,
    pub found_use_rename: super::types::AnalyzerBool,
}
impl UseImportVisitor {
    fn use_tree_contains_rename(
        use_tree: super::types::SynUseTreeRef<'_>,
    ) -> super::types::AnalyzerBool {
        super::types::AnalyzerBool::from(match use_tree.as_ref() {
            syn::UseTree::Path(use_path) => {
                Self::use_tree_contains_rename(super::types::SynUseTreeRef::from(&*use_path.tree))
                    .get()
            }
            syn::UseTree::Name(_) | syn::UseTree::Glob(_) => false,
            syn::UseTree::Rename(_) => true,
            syn::UseTree::Group(use_group) => use_group.items.iter().any(|item| {
                Self::use_tree_contains_rename(super::types::SynUseTreeRef::from(item)).get()
            }),
        })
    }
}
impl<'ast> syn::visit::Visit<'ast> for UseImportVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
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
                self.public_use_roots
                    .push(String::from(constants_str::ASTERISK));
            }
        } else {
            let is_leptos_prelude_import = if let syn::UseTree::Path(leptos) = &i.tree
                && let syn::UseTree::Path(prelude) = leptos.tree.as_ref()
            {
                leptos.ident == constants_str::CODE_STYLE_LEPTOS_CRATE
                    && prelude.ident == constants_str::CODE_STYLE_PRELUDE_MODULE
                    && matches!(prelude.tree.as_ref(), syn::UseTree::Group(_))
            } else {
                false
            };
            let is_allowed_leptos_import =
                self.allow_leptos_prelude_import.get() && is_leptos_prelude_import;
            if !is_allowed_leptos_import {
                self.found_non_public_use_import.set_true();
            }
        }
        if Self::use_tree_contains_rename(super::types::SynUseTreeRef::from(&i.tree)).get() {
            self.found_use_rename.set_true();
        }
        syn::visit::visit_item_use(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct TypeAliasVisitor {
    pub ers: super::types::DiagnosticMsgs,
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
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct EmptyEnumVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl EmptyEnumVisitor {
    fn check(&mut self, item: &syn::ItemEnum) {
        if item.variants.is_empty() {
            self.ers.push(format!(
                "enum `{}` has no variants; use an inhabited domain type or return the concrete type from infallible functions",
                item.ident
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for EmptyEnumVisitor {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if let syn::Meta::List(meta) = &i.meta
            && let Ok(item) = syn::parse2::<syn::ItemEnum>(meta.tokens.clone())
        {
            self.check(&item);
        }
        syn::visit::visit_attribute(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.check(i);
        syn::visit::visit_item_enum(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct InfallibleResultVisitor {
    pub ers: super::types::DiagnosticMsgs,
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
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
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
        if let syn::ReturnType::Type(_, ty) = &i.sig.output
            && result_error(ty).is_some_and(Self::type_is_infallible)
        {
            self.ers.push(format!(
                "function `{}` returns `Result` with `Infallible`; return the concrete success type",
                i.sig.ident
            ));
        }
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let mut fields = i.fields.iter();
        if let Some(field) = fields.next()
            && fields.next().is_none()
            && Self::type_is_infallible(&field.ty)
        {
            self.ers.push(format!(
                "struct `{}` wraps `Infallible`; remove the wrapper and return the concrete success type",
                i.ident
            ));
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ConstantAliasVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for ConstantAliasVisitor {
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        let local_constant_name = i.ident.to_string();
        if local_constant_name == constants_str::UNDERSCORE {
            return;
        }
        if let syn::Expr::Path(expression_path) = i.expr.as_ref()
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
            self.ers.push(format!(
                "`{local_constant_name}` aliases `{}`; use the source constant directly",
                super::path_to_string(super::types::SynPathRef::from(&expression_path.path))
                    .as_ref()
            ));
        }
        syn::visit::visit_item_const(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ForwardingDerefVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub inner_types: std::collections::BTreeMap<String, syn::Type>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ForwardingBorrowVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingBorrowVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_borrow_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_BORROW_TRAIT_IDENTIFIER
            })
        });
        let forwards_inner = i.items.iter().any(|item| {
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
            self.ers
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_BORROW.to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingDerefVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_deref_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DEREF_TRAIT_IDENTIFIER
            })
        });
        let wrapped_type_name = if let syn::Type::Path(path) = i.self_ty.as_ref() {
            path.path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
        } else {
            None
        };
        let target_type = i.items.iter().find_map(|item| {
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
        let forwards_inner = i.items.iter().any(|item| {
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
            self.ers
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_DEREF.to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &i.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
        {
            let _previous = self
                .inner_types
                .insert(i.ident.to_string(), field.ty.clone());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ForwardingDisplayVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ManualErrorImplVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ManualNotImplVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ConstDisplayImplVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for ConstDisplayImplVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_display_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DISPLAY_TRAIT_IDENTIFIER
            })
        });
        let writes_constant = i.items.len() == constants_usize::ONE
            && i.items.first().is_some_and(|item| {
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
            let start = syn::spanned::Spanned::span(i).start();
            self.ers.push(format!(
                "constant Display implementation at {}:{}; derive newtype::DisplayConst instead",
                start.line, start.column
            ));
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualNotImplVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_not_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_NOT_TRAIT_IDENTIFIER
            })
        });
        if is_not_impl {
            let start = syn::spanned::Spanned::span(i).start();
            self.ers.push(format!(
                "manual Not implementation at {}:{}; derive newtype::NotInner instead",
                start.line, start.column
            ));
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualErrorImplVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_error_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_ERROR_TRAIT_IDENTIFIER
            })
        });
        if is_error_impl {
            let start = syn::spanned::Spanned::span(i).start();
            self.ers.push(format!(
                "manual Error implementation at {}:{}; derive thiserror::Error instead",
                start.line, start.column
            ));
        }
        syn::visit::visit_item_impl(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct JsonCallVisitor {
    pub found: super::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for JsonCallVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if matches!(
            i.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_AXUM_JSON_IDENTIFIER
                })
        ) {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct JsonIntoResponseErrorVisitor<'names_lt> {
    pub ers: super::types::DiagnosticMsgs,
    pub thiserror_enum_names: &'names_lt super::types::SourceTextBTreeSet,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct TupleResponseVisitor {
    pub found: super::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for TupleResponseVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if i.args
            .iter()
            .any(|argument| matches!(argument, syn::Expr::Tuple(_)))
            && matches!(
                i.func.as_ref(),
                syn::Expr::Path(path)
                    if path.path.segments.last().is_some_and(|segment| {
                        segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_METHOD_IDENTIFIER
                    })
            )
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, i);
    }

    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::CODE_STYLE_INTO_RESPONSE_METHOD_IDENTIFIER
            && matches!(i.receiver.as_ref(), syn::Expr::Tuple(_))
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for JsonIntoResponseErrorVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_into_response = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_TRAIT_IDENTIFIER
            })
        });
        if is_into_response {
            let mut json_visitor = JsonCallVisitor::default();
            syn::visit::Visit::visit_item_impl(&mut json_visitor, i);
            let mut tuple_visitor = TupleResponseVisitor::default();
            syn::visit::Visit::visit_item_impl(&mut tuple_visitor, i);
            if json_visitor.found.get() || tuple_visitor.found.get() {
                let name =
                    super::item_impl_self_ty_identifier(super::types::SynItemImplRef::from(i))
                        .map_or_else(
                            || String::from(constants_str::NON_PATH_TARGET),
                            String::from,
                        );
                if !self.thiserror_enum_names.contains(name.as_str()) {
                    self.ers.push(format!(
                        "JSON API error response type `{name}` must be an enum deriving thiserror::Error"
                    ));
                }
            }
        }
        syn::visit::visit_item_impl(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct ThiserrorEnumVisitor {
    pub location_names: super::types::SourceTextBTreeSet,
    pub names: super::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for ThiserrorEnumVisitor {
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        let derives_thiserror = i.attrs.iter().any(|attr| {
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
            let _: bool = self.names.insert(i.ident.to_string());
            let derives_location =
                i.attrs.iter().any(|attr| {
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
                || i.variants.iter().any(|variant| {
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
                let _: bool = self.location_names.insert(i.ident.to_string());
            }
        }
        syn::visit::visit_item_enum(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ApiErrorLocationVisitor<'names_lt> {
    pub ers: super::types::DiagnosticMsgs,
    pub thiserror_location_enum_names: &'names_lt super::types::SourceTextBTreeSet,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct IntoResponseTypeVisitor {
    pub names: super::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for IntoResponseTypeVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_into_response = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_TRAIT_IDENTIFIER
            })
        });
        if is_into_response
            && let Some(name) =
                super::item_impl_self_ty_identifier(super::types::SynItemImplRef::from(i))
        {
            let _: bool = self.names.insert(String::from(name));
        }
        syn::visit::visit_item_impl(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ApiErrorSourceVisitor<'names_lt> {
    pub api_error_names: &'names_lt super::types::SourceTextBTreeSet,
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct RouteOperationErrorVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub names: super::types::SourceTextBTreeSet,
    pub operations: super::types::SourceTextBTreeSet,
    pub registered: super::types::SourceTextBTreeSet,
}
impl<'ast> syn::visit::Visit<'ast> for RouteOperationErrorVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let route_error_name = i.attrs.iter().find_map(|attr| {
            attr.path().segments.last().and_then(|segment| {
                let path = (segment.ident == constants_str::CODE_STYLE_ROUTE_ERROR_IDENTIFIER)
                    .then(|| attr.parse_args::<syn::Path>().ok())
                    .flatten()?;
                path.segments.last().map(|value| value.ident.to_string())
            })
        });
        let is_route_operation = route_error_name.is_some()
            || i.attrs.iter().any(|attr| {
                attr.path().segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_ROUTE_OPENAPI_IDENTIFIER
                        || segment.ident == constants_str::CODE_STYLE_ROUTE_OPERATION_IDENTIFIER
                })
            });
        if is_route_operation {
            let _: bool = self.operations.insert(i.sig.ident.to_string());
            let error_name = route_error_name.or_else(|| match &i.sig.output {
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
                Some(name) => self.ers.push(format!(
                    "route operation `{}` reuses error type `{name}`",
                    i.sig.ident
                )),
                None => {}
            }
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        i.attrs
            .iter()
            .filter(|attr| {
                attr.path().segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::CODE_STYLE_ENDPOINT_REGISTRY_IDENTIFIER
                })
            })
            .filter_map(|attr| match &attr.meta {
                syn::Meta::List(value) => Some(value.tokens.clone()),
                syn::Meta::NameValue(_) | syn::Meta::Path(_) => None,
            })
            .flat_map(proc_macro2::TokenStream::into_iter)
            .filter_map(|token| match token {
                proc_macro2::TokenTree::Group(group)
                    if group.delimiter() == proc_macro2::Delimiter::Parenthesis =>
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
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ApiErrorSourceVisitor<'_> {
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if self.api_error_names.contains(i.ident.to_string().as_str()) {
            i.variants.iter().for_each(|variant| {
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
                        self.ers.push(format!(
                            "API response error `{}::{}` source must be wrapped in ObservedError",
                            i.ident, variant.ident
                        ));
                    }
                });
            });
        }
        syn::visit::visit_item_enum(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ApiErrorLocationVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_into_response = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_RESPONSE_TRAIT_IDENTIFIER
            })
        });
        if is_into_response {
            let name = super::item_impl_self_ty_identifier(super::types::SynItemImplRef::from(i))
                .map_or_else(
                    || String::from(constants_str::NON_PATH_TARGET),
                    String::from,
                );
            if self.thiserror_location_enum_names.contains(name.as_str()) {
                self.ers.push(format!(
                    "API response error enum `{name}` must keep source location in HttpErrorDiagnostic"
                ));
            }
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingDisplayVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_display_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_DISPLAY_TRAIT_IDENTIFIER
            })
        });
        let is_forwarding = i.items.len() == constants_usize::ONE
            && i.items.first().is_some_and(|item| {
                let syn::ImplItem::Fn(function) = item else {
                    return false;
                };
                function.sig.ident == constants_str::CODE_STYLE_FMT_FN_IDENTIFIER
                    && function.block.stmts.len() == constants_usize::ONE
                    && function.block.stmts.first().is_some_and(|statement| {
                        let syn::Stmt::Expr(expression, None) = statement else {
                            return false;
                        };
                        let syn::Expr::MethodCall(call) = expression else {
                            return false;
                        };
                        let syn::Expr::Field(field) = call.receiver.as_ref() else {
                            return false;
                        };
                        let syn::Expr::Path(receiver) = field.base.as_ref() else {
                            return false;
                        };
                        receiver
                            .path
                            .is_ident(constants_str::CODE_STYLE_SELF_VALUE_IDENTIFIER)
                            && matches!(&field.member, syn::Member::Unnamed(index) if index.index == constants_u32::ZERO)
                            && call.method == constants_str::CODE_STYLE_FMT_FN_IDENTIFIER
                            && call.args.len() == constants_usize::ONE
                            && call.args.first().is_some_and(|argument| {
                                let syn::Expr::Path(formatter) = argument else {
                                    return false;
                                };
                                formatter
                                    .path
                                    .is_ident(constants_str::CODE_STYLE_FMT_ARGUMENT_IDENTIFIER)
                            })
                    })
            });
        if is_display_impl && is_forwarding {
            self.ers
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_DISPLAY.to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ForwardingIntoIteratorVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for ForwardingIntoIteratorVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_into_iterator_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_INTO_ITERATOR_TRAIT_IDENTIFIER
            })
        });
        let forwards_inner = i.items.iter().any(|item| {
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
            self.ers
                .push(constants_str::CODE_STYLE_MANUAL_FORWARDING_INTO_ITERATOR.to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct PassthroughIntoInnerFromVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub inner_types: std::collections::BTreeMap<String, syn::Type>,
}
impl<'ast> syn::visit::Visit<'ast> for PassthroughIntoInnerFromVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let source_wrapper_name = i.trait_.as_ref().and_then(|(path, _)| {
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
            .is_some_and(|inner| inner == i.self_ty.as_ref());
        let forwards_inner = i.items.iter().any(|item| {
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
            self.ers
                .push(constants_str::CODE_STYLE_MANUAL_PASSTHROUGH_INTO_INNER_FROM.to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &i.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
        {
            let _previous = self
                .inner_types
                .insert(i.ident.to_string(), field.ty.clone());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct PassthroughFromVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub inner_types: std::collections::BTreeMap<String, syn::Type>,
}
impl<'ast> syn::visit::Visit<'ast> for PassthroughFromVisitor {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let from_type = i.trait_.as_ref().and_then(|(path, _)| {
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
        let wrapped_type_name = if let syn::Type::Path(path) = i.self_ty.as_ref() {
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
        let is_passthrough = i.items.len() == constants_usize::ONE
            && i.items.first().is_some_and(|item| {
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
            self.ers
                .push(constants_str::CODE_STYLE_MANUAL_PASSTHROUGH_FROM.to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &i.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
        {
            let _previous = self
                .inner_types
                .insert(i.ident.to_string(), field.ty.clone());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct TestStringLiteralVisitor {
    pub values: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for TestStringLiteralVisitor {
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if let syn::Lit::Str(literal_string) = &i.lit {
            self.values.push(literal_string.value());
        }
        syn::visit::visit_expr_lit(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let mut streams = vec![i.tokens.clone()];
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
        syn::visit::visit_macro(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ProductionStringLiteralVisitor {
    pub values: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ProductionStringLiteralVisitor {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if i.path().is_ident(constants_str::PATH_ALT_5) {
            return;
        }
        syn::visit::visit_attribute(self, i);
    }
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if let syn::Lit::Str(literal_string) = &i.lit {
            self.values.push(literal_string.value());
        }
        syn::visit::visit_expr_lit(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "alignment order required by optimal_memory_layout takes precedence over alphabetical field order"
)]
pub(super) struct StringConstantDeclarationVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub allow_generated_string_constants: super::types::AnalyzerBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Default)]
pub(super) struct ConstantInitializerStringLiteralVisitor {
    pub found: super::types::AnalyzerBool,
}
impl ConstantInitializerStringLiteralVisitor {
    fn contains(expr: &syn::Expr) -> super::types::AnalyzerBool {
        let mut visitor = Self::default();
        syn::visit::Visit::visit_expr(&mut visitor, expr);
        visitor.found
    }
}
impl<'ast> syn::visit::Visit<'ast> for ConstantInitializerStringLiteralVisitor {
    fn visit_expr_closure(&mut self, _i: &'ast syn::ExprClosure) {}
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if matches!(i.lit, syn::Lit::Str(_)) {
            self.found.set_true();
        }
    }
    fn visit_item(&mut self, _i: &'ast syn::Item) {}
    fn visit_macro(&mut self, _i: &'ast syn::Macro) {}
}
impl<'ast> syn::visit::Visit<'ast> for StringConstantDeclarationVisitor {
    fn visit_expr_const(&mut self, i: &'ast syn::ExprConst) {
        let mut literal_visitor = TestStringLiteralVisitor {
            values: super::types::SourceTextList::default(),
        };
        syn::visit::Visit::visit_block(&mut literal_visitor, &i.block);
        if !literal_visitor.values.is_empty() {
            self.ers.push(constants_str::VALUE_FEDD2A2E.to_owned());
        }
        syn::visit::visit_expr_const(self, i);
    }
    fn visit_impl_item_const(&mut self, i: &'ast syn::ImplItemConst) {
        if super::type_stores_string_text(super::types::SynTypeRef::from(&i.ty)).get()
            || ConstantInitializerStringLiteralVisitor::contains(&i.expr).get()
        {
            self.ers.push(format!(
                "associated string constant `{}` must be declared in constants_str",
                i.ident
            ));
        }
        syn::visit::visit_impl_item_const(self, i);
    }
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if i.sig.constness.is_some() {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, &i.block);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "const method `{}` contains string literals",
                    i.sig.ident
                ));
            }
        }
        syn::visit::visit_impl_item_fn(self, i);
    }
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        if super::type_stores_string_text(super::types::SynTypeRef::from(i.ty.as_ref())).get()
            || ConstantInitializerStringLiteralVisitor::contains(i.expr.as_ref()).get()
        {
            self.ers.push(format!(
                "string constant `{}` must be declared in constants_str",
                i.ident
            ));
        }
        syn::visit::visit_item_const(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.sig.constness.is_some() {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, &i.block);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "const function `{}` contains string literals",
                    i.sig.ident
                ));
            }
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        if super::type_stores_string_text(super::types::SynTypeRef::from(i.ty.as_ref())).get()
            || ConstantInitializerStringLiteralVisitor::contains(i.expr.as_ref()).get()
        {
            self.ers.push(format!(
                "string static `{}` must be declared in constants_str",
                i.ident
            ));
        }
        syn::visit::visit_item_static(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        fn group_contains_str(group: &proc_macro2::Group) -> bool {
            group.stream().into_iter().any(|token| match token {
                proc_macro2::TokenTree::Group(nested) => group_contains_str(&nested),
                proc_macro2::TokenTree::Ident(ident) => ident == constants_str::STR_ALT,
                proc_macro2::TokenTree::Literal(_) | proc_macro2::TokenTree::Punct(_) => false,
            })
        }
        fn contains(tokens: proc_macro2::TokenStream) -> bool {
            let token_trees = tokens.into_iter().collect::<Vec<_>>();
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
        if i.path.segments.last().is_some_and(|segment| {
            segment.ident == constants_str::SHARED_VALUES_DEFINE_STR_CONSTANTS
        }) {
            self.ers.push(constants_str::VALUE_23159C36.to_owned());
        }
        if !self.allow_generated_string_constants.get() && contains(i.tokens.clone()) {
            self.ers.push(constants_str::VALUE_BA372BD2.to_owned());
        }
        syn::visit::visit_macro(self, i);
    }
    fn visit_trait_item_const(&mut self, i: &'ast syn::TraitItemConst) {
        if super::type_stores_string_text(super::types::SynTypeRef::from(&i.ty)).get()
            || i.default.as_ref().is_some_and(|(_, expression)| {
                ConstantInitializerStringLiteralVisitor::contains(expression).get()
            })
        {
            self.ers.push(format!(
                "trait string constant `{}` must be declared in constants_str",
                i.ident
            ));
        }
        syn::visit::visit_trait_item_const(self, i);
    }
    fn visit_trait_item_fn(&mut self, i: &'ast syn::TraitItemFn) {
        if i.sig.constness.is_some()
            && let Some(block) = &i.default
        {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            };
            syn::visit::Visit::visit_block(&mut literal_visitor, block);
            if !literal_visitor.values.is_empty() {
                self.ers.push(format!(
                    "const trait method `{}` contains string literals",
                    i.sig.ident
                ));
            }
        }
        syn::visit::visit_trait_item_fn(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct StringConstantVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for StringConstantVisitor {
    fn visit_attribute(&mut self, _i: &'ast syn::Attribute) {}
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if matches!(
            i.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| {
                    segment.ident == constants_str::COMPILE_ERROR_TOKEN_STREAM
                })
        ) {
            let mut literal_visitor = TestStringLiteralVisitor {
                values: super::types::SourceTextList::default(),
            };
            i.args.iter().for_each(|arg| {
                syn::visit::Visit::visit_expr(&mut literal_visitor, arg);
            });
            if !literal_visitor.values.is_empty() {
                self.ers.push(
                    constants_str::COMPILE_ERROR_TOKEN_STREAM_CALL_CONTAINS_STRING_LITERALS
                        .to_owned(),
                );
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
        if let syn::Lit::Str(value) = &i.lit {
            let start = value.span().start();
            let end = value.span().end();
            self.ers.push(format!(
                "{}:{}-{}:{}: string literal {:?} must be declared in constants_str",
                start.line,
                start.column,
                end.line,
                end.column,
                value.value()
            ));
        }
        syn::visit::visit_expr_lit(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::CODE_STYLE_EXPECT_METHOD_NAME {
            syn::visit::Visit::visit_expr(self, i.receiver.as_ref());
            return;
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let is_syntax_boundary = i.path.segments.last().is_some_and(|segment| {
            constants_str::CODE_STYLE_STRING_LITERAL_MACRO_BOUNDARIES
                .contains(&segment.ident.to_string().as_str())
        });
        if !is_syntax_boundary {
            let mut streams = vec![i.tokens.clone()];
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
            let macro_name = i
                .path
                .segments
                .last()
                .map_or_else(String::new, |segment| segment.ident.to_string());
            while let Some((value, start_line, start_column, end_line, end_column)) = values.pop() {
                self.ers.push(format!(
                    "{start_line}:{start_column}-{end_line}:{end_column}: macro `{macro_name}` string literal {value:?} must be declared in constants_str"
                ));
            }
        }
        syn::visit::visit_macro(self, i);
    }
}
