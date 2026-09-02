#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::wildcard_enum_match_arm,
    reason = "policy visitors stay grouped with their invariant, repository policy requires iterator methods, and syn non-exhaustive enums require fallback handling"
)]

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct AwaitVisitor {
    found: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for AwaitVisitor {
    fn visit_expr_await(&mut self, expr_await: &'ast syn::ExprAwait) {
        self.found.set_true();
        syn::visit::visit_expr_await(self, expr_await);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct LockAcrossAwaitVisitor {
    violations: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for LockAcrossAwaitVisitor {
    fn visit_block(&mut self, block: &'ast syn::Block) {
        let mut active_guards = std::collections::BTreeSet::<String>::new();
        block.stmts.iter().for_each(|statement| {
            let mut await_visitor = AwaitVisitor::default();
            syn::visit::Visit::visit_stmt(&mut await_visitor, statement);
            if await_visitor.get_found().get() && !active_guards.is_empty() {
                self.violations.push(format!(
                    "lock guards held across await: {}",
                    active_guards
                        .iter()
                        .map(String::as_str)
                        .collect::<Vec<&str>>()
                        .join(", ")
                ));
            }
            if let syn::Stmt::Local(local) = statement
                && local
                    .init
                    .as_ref()
                    .is_some_and(|initializer| expression_acquires_lock(initializer.expr.as_ref()))
                && let syn::Pat::Ident(identifier) = &local.pat
            {
                let _inserted = active_guards.insert(identifier.ident.to_string());
            }
            (|| {
                let syn::Stmt::Expr(syn::Expr::Call(call), _) = statement else {
                    return None;
                };
                let syn::Expr::Path(function) = call.func.as_ref() else {
                    return None;
                };
                if !matches!(
                    function
                        .path
                        .segments
                        .last()
                        .map(|segment| segment.ident.to_string())
                        .as_deref(),
                    Some(constants_str::VALUE_D90EE9CC)
                ) {
                    return None;
                }
                let syn::Expr::Path(argument) = call.args.first()? else {
                    return None;
                };
                (argument.path.segments.len() == constants_usize::ONE)
                    .then(|| {
                        argument.path.segments.first().map(|segment| {
                            crate::types::SourceText::try_from(segment.ident.to_string())
                                .expect(constants_str::DIAGNOSTIC_D4F6BDCE)
                        })
                    })
                    .flatten()
            })()
            .into_iter()
            .for_each(|identifier| {
                let _removed = active_guards.remove(identifier.as_ref());
            });
        });
        syn::visit::visit_block(self, block);
    }
}

fn expression_acquires_lock(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Await(await_expression) => {
            expression_acquires_lock(await_expression.base.as_ref())
        }
        syn::Expr::MethodCall(call) => {
            call.args.is_empty()
                && matches!(
                    call.method.to_string().as_str(),
                    constants_str::VALUE_0C030586
                        | constants_str::VALUE_DB488AC5
                        | constants_str::PG_CRUD_READ_PERMISSION_ACTION
                        | constants_str::VALUE_35D47C1A
                        | constants_str::WRITE_ALT
                        | constants_str::VALUE_FC58C841
                )
        }
        syn::Expr::Paren(paren) => expression_acquires_lock(paren.expr.as_ref()),
        syn::Expr::Try(try_expression) => expression_acquires_lock(try_expression.expr.as_ref()),
        _ => false,
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct LeakApiVisitor {
    violations: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for LeakApiVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = expr_call.func.as_ref() {
            let path = function
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<String>>()
                .join(constants_str::PATH_SEPARATOR);
            if [
                constants_str::VALUE_C26EBF7F,
                constants_str::VALUE_5188A49C,
                constants_str::VALUE_9C055078,
                constants_str::VALUE_FEE41E56,
                constants_str::VALUE_FA94FFC8,
                constants_str::VALUE_2E8E6C33,
                constants_str::VALUE_30F0E257,
                constants_str::VALUE_86C84494,
                constants_str::VALUE_58CAC57E,
                constants_str::VALUE_36F221B5,
                constants_str::VALUE_AF4FFF7C,
                constants_str::VALUE_6D10B254,
            ]
            .contains(&path.as_str())
            {
                self.violations.push(path);
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_type_path(&mut self, type_path: &'ast syn::TypePath) {
        if type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::VALUE_6462221C)
        {
            self.violations
                .push(constants_str::VALUE_6462221C.to_owned());
        }
        syn::visit::visit_type_path(self, type_path);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct SpawnConsumptionVisitor {
    consumed: crate::types::SourceTextBTreeSet,
}
impl SpawnConsumptionVisitor {
    fn record_path(&mut self, expr: &syn::Expr) {
        if let syn::Expr::Path(path) = expr
            && path.path.segments.len() == constants_usize::ONE
            && let Some(segment) = path.path.segments.first()
        {
            let _inserted = self.consumed.insert(segment.ident.to_string());
        }
    }
    fn record_macro_tokens(&mut self, token_stream: proc_macro2::TokenStream) {
        let trees = token_stream
            .into_iter()
            .collect::<Vec<proc_macro2::TokenTree>>();
        trees.iter().for_each(|token| {
            if let proc_macro2::TokenTree::Group(group) = token {
                self.record_macro_tokens(group.stream());
            }
        });
        trees.windows(3usize).for_each(|window| {
            if let [
                proc_macro2::TokenTree::Ident(identifier),
                proc_macro2::TokenTree::Punct(dot),
                proc_macro2::TokenTree::Ident(operation),
            ] = window
                && dot.as_char() == '.'
                && matches!(
                    operation.to_string().as_str(),
                    constants_str::VALUE_3A53DB8A | constants_str::VALUE_1AEFE47E
                )
            {
                let _inserted = self.consumed.insert(identifier.to_string());
            }
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SpawnConsumptionVisitor {
    fn visit_expr_await(&mut self, expr_await: &'ast syn::ExprAwait) {
        self.record_path(expr_await.base.as_ref());
        syn::visit::visit_expr_await(self, expr_await);
    }
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        let is_drop = matches!(
            expr_call.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| segment.ident == constants_str::VALUE_D90EE9CC)
        );
        if !is_drop {
            expr_call
                .args
                .iter()
                .for_each(|argument| self.record_path(argument));
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::VALUE_3A53DB8A {
            self.record_path(expr_method_call.receiver.as_ref());
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_field_value(&mut self, field_value: &'ast syn::FieldValue) {
        self.record_path(&field_value.expr);
        syn::visit::visit_field_value(self, field_value);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        self.record_macro_tokens(r#macro.tokens.clone());
        syn::visit::visit_macro(self, r#macro);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct SpawnLifecycleVisitor {
    violations: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for SpawnLifecycleVisitor {
    fn visit_block(&mut self, block: &'ast syn::Block) {
        let mut pending = std::collections::BTreeSet::<String>::new();
        block.stmts.iter().for_each(|statement| {
            let mut consumption = SpawnConsumptionVisitor::default();
            syn::visit::Visit::visit_stmt(&mut consumption, statement);
            consumption.consumed.into_iter().for_each(|identifier| {
                let _removed = pending.remove(identifier.as_str());
            });
            if let syn::Stmt::Local(local) = statement
                && local.init.as_ref().is_some_and(|initializer| {
                    crate::code_style::unowned_spawn_expr(initializer.expr.as_ref())
                })
                && let syn::Pat::Ident(identifier) = &local.pat
                && !identifier.ident.to_string().starts_with('_')
            {
                let _inserted = pending.insert(identifier.ident.to_string());
            }
        });
        pending.into_iter().for_each(|identifier| {
            self.violations.push(format!(
                "spawned task `{identifier}` is retained but never awaited, aborted, or transferred to an owner"
            ));
        });
        syn::visit::visit_block(self, block);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct RouteLiteralVisitor {
    violations: crate::types::DiagnosticMessages,
}
impl RouteLiteralVisitor {
    fn inspect_literal(&mut self, lit_str: &syn::LitStr) {
        let value = lit_str.value();
        if !value.starts_with('/') || value.starts_with(constants_str::VALUE_A2C23396) {
            return;
        }
        if value == constants_str::VALUE_702ACF7C
            || value.starts_with(constants_str::VALUE_4D3A663E)
        {
            self.violations.push(format!(
                "route `{value}` must not use the removed `/api` prefix"
            ));
        }
        value
            .split('/')
            .filter(|segment| !segment.is_empty())
            .filter(|segment| !(segment.starts_with('{') && segment.ends_with('}')))
            .filter(|segment| {
                !segment.split('.').all(|part| {
                    !part.is_empty()
                        && part.bytes().all(|byte| {
                            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
                        })
                })
            })
            .for_each(|segment| {
                self.violations.push(format!(
                    "route segment `{segment}` in `{value}` is not snake_case"
                ));
            });
    }
    fn inspect_tokens(&mut self, token_stream: proc_macro2::TokenStream) {
        token_stream.into_iter().for_each(|token| match token {
            proc_macro2::TokenTree::Group(group) => self.inspect_tokens(group.stream()),
            proc_macro2::TokenTree::Literal(literal) => {
                if let Ok(parsed) = syn::parse_str::<syn::LitStr>(literal.to_string().as_str()) {
                    self.inspect_literal(&parsed);
                }
            }
            proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Punct(_) => {}
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for RouteLiteralVisitor {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if attribute.path().segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                constants_str::VALUE_BDE31E29 | constants_str::VALUE_2466624A
            )
        }) && let syn::Meta::List(list) = &attribute.meta
        {
            self.inspect_tokens(list.tokens.clone());
        }
        syn::visit::visit_attribute(self, attribute);
    }
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if matches!(
            expr_method_call.method.to_string().as_str(),
            constants_str::VALUE_8A84E406
                | constants_str::VALUE_75EF2E32
                | constants_str::VALUE_84BBA14A
        ) && let Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(literal),
            ..
        })) = expr_method_call.args.first()
        {
            self.inspect_literal(literal);
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct SelectMacroVisitor {
    count: crate::types::AnalyzerCount,
    unsafe_operations: crate::types::DiagnosticMessages,
}
impl SelectMacroVisitor {
    fn inspect_sensitive_tokens(&mut self, token_stream: proc_macro2::TokenStream) {
        token_stream.into_iter().for_each(|token| match token {
            proc_macro2::TokenTree::Group(group) => {
                self.inspect_sensitive_tokens(group.stream());
            }
            proc_macro2::TokenTree::Ident(identifier)
                if matches!(
                    identifier.to_string().as_str(),
                    constants_str::VALUE_574C97CF
                        | constants_str::VALUE_EB83DC1A
                        | constants_str::VALUE_8882BF3F
                        | constants_str::VALUE_6DF24C37
                        | constants_str::VALUE_27CE1D1B
                        | constants_str::VALUE_86F7474B
                ) =>
            {
                self.unsafe_operations.push(format!(
                    "tokio::select contains cancellation-sensitive `{identifier}`"
                ));
            }
            proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Literal(_)
            | proc_macro2::TokenTree::Punct(_) => {}
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SelectMacroVisitor {
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        let is_select = r#macro
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::SELECT_ALT_3);
        if is_select {
            self.count.saturating_inc();
            self.inspect_sensitive_tokens(r#macro.tokens.clone());
        }
        syn::visit::visit_macro(self, r#macro);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct ExpressionPathVisitor {
    paths: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ExpressionPathVisitor {
    fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
        self.paths.push(
            crate::code_style::path_to_string(crate::types::SynPathRef::from(&expr_path.path))
                .as_ref()
                .to_owned(),
        );
        syn::visit::visit_expr_path(self, expr_path);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct IgnoredMapErrBindingVisitor {
    entries: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for IgnoredMapErrBindingVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::CODE_STYLE_MAP_ERR
            && let Some(syn::Expr::Closure(closure)) = expr_method_call.args.first()
        {
            let ignored_inputs = closure
                .inputs
                .iter()
                .filter_map(|input| match input {
                    syn::Pat::Wild(_) => Some(constants_str::UNDERSCORE.to_owned()),
                    syn::Pat::Ident(identifier)
                        if identifier.ident.to_string().starts_with('_') =>
                    {
                        Some(identifier.ident.to_string())
                    }
                    _ => None,
                })
                .collect::<Vec<String>>();
            if !ignored_inputs.is_empty() {
                let mut path_visitor = ExpressionPathVisitor::default();
                syn::visit::Visit::visit_expr(&mut path_visitor, closure.body.as_ref());
                path_visitor.paths.sort();
                self.entries.push(format!(
                    "line {}: {} => {}",
                    syn::spanned::Spanned::span(expr_method_call).start().line,
                    ignored_inputs.join(","),
                    path_visitor.paths.join(",")
                ));
            }
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct RawVecTupleWrapperVisitor {
    identifiers: crate::types::SourceTextList,
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct FromVecImplVisitor {
    targets: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for FromVecImplVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        if let Some((trait_path, _)) = &item_impl.trait_
            && let Some(trait_segment) = trait_path.segments.last()
            && trait_segment.ident == constants_str::FROM_ALT_3
            && let syn::PathArguments::AngleBracketed(arguments) = &trait_segment.arguments
            && let Some(syn::GenericArgument::Type(syn::Type::Path(value_type))) =
                arguments.args.first()
            && value_type
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::VEC)
        {
            self.targets.push(format!(
                "line {}",
                syn::spanned::Spanned::span(item_impl).start().line
            ));
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}
impl<'ast> syn::visit::Visit<'ast> for RawVecTupleWrapperVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &item_struct.fields
            && fields.unnamed.len() == constants_usize::ONE
            && let Some(field) = fields.unnamed.first()
            && let syn::Type::Path(path) = &field.ty
            && path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == constants_str::VEC)
        {
            self.identifiers.push(item_struct.ident.to_string());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct UsizeMaxExprVisitor {
    count: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UsizeMaxExprVisitor {
    fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
        let mut segments = expr_path.path.segments.iter();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_USIZE)
            && segments
                .next()
                .is_some_and(|segment| segment.ident == constants_str::VALUE_2D9C014A)
            && segments.next().is_none()
        {
            self.count.saturating_inc();
        }
        syn::visit::visit_expr_path(self, expr_path);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        if crate::code_style::attrs_contain_test_only_cfg(crate::types::SynAttributeListRef::from(
            item_mod.attrs.as_slice(),
        ))
        .get()
        {
            return;
        }
        syn::visit::visit_item_mod(self, item_mod);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct SharedDispatchVisitor {
    arc_types: crate::types::AnalyzerCount,
    lock_types: crate::types::AnalyzerCount,
    trait_objects: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SharedDispatchVisitor {
    fn visit_type_path(&mut self, type_path: &'ast syn::TypePath) {
        if let Some(segment) = type_path.path.segments.last() {
            match segment.ident.to_string().as_str() {
                constants_str::ARC => self.arc_types.saturating_inc(),
                constants_str::MUTEX | constants_str::VALUE_02DF7EC2 => {
                    self.lock_types.saturating_inc();
                }
                _ => {}
            }
        }
        syn::visit::visit_type_path(self, type_path);
    }
    fn visit_type_trait_object(&mut self, type_trait_object: &'ast syn::TypeTraitObject) {
        self.trait_objects.saturating_inc();
        syn::visit::visit_type_trait_object(self, type_trait_object);
    }
}

#[derive(proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct PublicApiVisitor {
    entries: crate::types::SourceTextList,
    lines: crate::types::SourceTextList,
}
impl PublicApiVisitor {
    fn source(&self, span: proc_macro2::Span) -> crate::types::SourceText {
        let start = span.start().line.saturating_sub(constants_usize::ONE);
        let end = span.end().line;
        let normalized = self
            .lines
            .get(start..end)
            .map(|lines| lines.join(constants_str::NEWLINE))
            .expect(constants_str::DIAGNOSTIC_C9D73E55)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(constants_str::SPACE);
        crate::types::SourceText::try_from(normalized).expect(constants_str::DIAGNOSTIC_31F04BB7)
    }
    fn field_type(&self, field: &syn::Field) -> crate::types::SourceText {
        let source = self.source(syn::spanned::Spanned::span(field));
        let separator = [':', ' '].into_iter().collect::<String>();
        let field_type = source
            .as_ref()
            .rsplit_once(separator.as_str())
            .map(|(_field, field_type)| field_type.trim().trim_end_matches(',').to_owned())
            .expect(constants_str::DIAGNOSTIC_5AF91E82);
        crate::types::SourceText::try_from(field_type).expect(constants_str::DIAGNOSTIC_3E2D89EF)
    }
    fn record(&mut self, span: proc_macro2::Span, bool: bool) {
        let start = span.start().line.saturating_sub(constants_usize::ONE);
        let end = span.end().line;
        let source = self
            .lines
            .get(start..end)
            .map(|lines| lines.join(constants_str::NEWLINE))
            .expect(constants_str::DIAGNOSTIC_3E180ABF);
        let relevant = if bool {
            source
                .split_once('{')
                .map_or(source.as_str(), |(signature, _body)| signature)
        } else {
            source.as_str()
        };
        self.entries.push(
            relevant
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(constants_str::SPACE),
        );
    }
    fn record_contract_struct_api(&mut self, item_struct: &syn::ItemStruct) {
        let Some(attribute) = item_struct
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident(constants_str::VALUE_21E85007))
        else {
            return;
        };
        let mut constructor = false;
        let mut into_parts = false;
        attribute
            .parse_nested_meta(|metadata| {
                if metadata.path.is_ident(constants_str::NEW) {
                    constructor = true;
                }
                if metadata.path.is_ident(constants_str::VALUE_1E3D0F4B) {
                    into_parts = true;
                }
                Ok(())
            })
            .expect(constants_str::DIAGNOSTIC_D932A5F1);
        let syn::Fields::Named(fields) = &item_struct.fields else {
            return;
        };
        let identifiers = fields
            .named
            .iter()
            .filter_map(|field| field.ident.as_ref())
            .collect::<Vec<_>>();
        let types = fields
            .named
            .iter()
            .map(|field| self.field_type(field))
            .collect::<Vec<_>>();
        if constructor {
            let parameters = identifiers
                .iter()
                .zip(types.iter())
                .map(|(identifier, field_type)| format!("{identifier}: {}", field_type.as_ref()))
                .collect::<Vec<_>>()
                .join(constants_str::TEXT_ALT_6);
            self.entries.push(format!(
                "#[must_use] pub const fn new({parameters}) -> Self"
            ));
        }
        if into_parts {
            self.entries.push(format!(
                "#[must_use] pub fn into_parts(self) -> ({})",
                types
                    .iter()
                    .map(AsRef::as_ref)
                    .collect::<Vec<&str>>()
                    .join(", ")
            ));
        }
        fields.named.iter().for_each(|field| {
            let Some(identifier) = field.ident.as_ref() else {
                return;
            };
            let wrapped_field_type = self.field_type(field);
            let field_type = wrapped_field_type.as_ref();
            field
                .attrs
                .iter()
                .filter(|field_attribute| {
                    field_attribute.path().is_ident(constants_str::VALUE_21E85007)
                })
                .for_each(|field_attribute| {
                    field_attribute
                        .parse_nested_meta(|metadata| {
                            let signature = if metadata.path.is_ident(constants_str::VALUE_D106CCB1) {
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> &{field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::VALUE_6F5A6034) {
                                format!(
                                    "#[must_use] pub const fn {identifier}(self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::VALUE_8972F0EE) {
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::VALUE_6B847A0E) {
                                format!(
                                    "#[must_use] pub fn into_{identifier}(self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident(constants_str::VALUE_ECA7C4E3) {
                                let inner_type = field_type
                                    .strip_prefix(constants_str::VALUE_7E0FC0D7)
                                    .and_then(|value| value.strip_suffix('>'))
                                    .expect(constants_str::DIAGNOSTIC_7C4E2A91);
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> Option<&{inner_type}>"
                                )
                            } else if metadata.path.is_ident(constants_str::VALUE_03FDB065) {
                                let _parsed_element_type =
                                    metadata.value()?.parse::<syn::Type>()?;
                                let attribute_source = self
                                    .source(syn::spanned::Spanned::span(field_attribute));
                                let element_type = attribute_source
                                    .as_ref()
                                    .split_once('=')
                                    .map(|(_prefix, value)| {
                                        value.trim().trim_end_matches([')', ']'])
                                    })
                                    .expect(constants_str::DIAGNOSTIC_9BA9415C);
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> &[{element_type}]"
                                )
                            } else {
                                String::new()
                            };
                            if !signature.is_empty() {
                                self.entries.push(signature);
                            }
                            Ok(())
                        })
                        .expect(constants_str::DIAGNOSTIC_206ADBF7);
                });
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicApiVisitor {
    fn visit_impl_item_fn(&mut self, impl_item_fn: &'ast syn::ImplItemFn) {
        if matches!(impl_item_fn.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(impl_item_fn), true);
        }
        syn::visit::visit_impl_item_fn(self, impl_item_fn);
    }
    fn visit_item_const(&mut self, item_const: &'ast syn::ItemConst) {
        if matches!(item_const.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(item_const), true);
        }
        syn::visit::visit_item_const(self, item_const);
    }
    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        if matches!(item_enum.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(item_enum), false);
            if item_enum.attrs.iter().any(|attribute| {
                crate::code_style::derive_attr_has_terminal(
                    crate::types::SynAttributeRef::from(attribute),
                    crate::types::SourceTextRef::from(constants_str::VALUE_4529EB51),
                )
                .get()
            }) {
                self.entries.push(format!(
                    "pub const COUNT: usize = {}usize;",
                    item_enum.variants.len()
                ));
                self.entries
                    .push(String::from(constants_str::VALUE_5F528A82));
            }
        }
        syn::visit::visit_item_enum(self, item_enum);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if matches!(item_fn.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(item_fn), true);
        }
        syn::visit::visit_item_fn(self, item_fn);
    }
    fn visit_item_static(&mut self, item_static: &'ast syn::ItemStatic) {
        if matches!(item_static.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(item_static), true);
        }
        syn::visit::visit_item_static(self, item_static);
    }
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if matches!(item_struct.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(item_struct), false);
            self.record_contract_struct_api(item_struct);
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        if matches!(item_trait.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(item_trait), false);
        }
        syn::visit::visit_item_trait(self, item_trait);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct StructErrorVisitor {
    identifiers: crate::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for StructErrorVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        if item_struct.attrs.iter().any(|attribute| {
            crate::code_style::derive_attr_has_terminal(
                crate::types::SynAttributeRef::from(attribute),
                crate::types::SourceTextRef::from(constants_str::ERROR),
            )
            .get()
        }) {
            self.identifiers.push(item_struct.ident.to_string());
        }
        syn::visit::visit_item_struct(self, item_struct);
    }
}

#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Default,
)]
struct LoopAllocationVisitor {
    depth: crate::types::AnalyzerCount,
    entries: crate::types::DiagnosticMessages,
}
impl LoopAllocationVisitor {
    fn record(&mut self, source_text_ref: crate::types::SourceTextRef<'_>) {
        if self.depth.get() != constants_usize::ZERO {
            self.entries.push(source_text_ref.as_ref().to_owned());
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for LoopAllocationVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = expr_call.func.as_ref() {
            let path =
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&function.path));
            if [
                constants_str::VALUE_EDB966EE,
                constants_str::VALUE_13D4D62E,
                constants_str::VALUE_6D0C4109,
                constants_str::VALUE_7879C268,
                constants_str::VALUE_FA4D593C,
                constants_str::VALUE_F36B8CD3,
                constants_str::VALUE_AA9C75B0,
                constants_str::VALUE_B07BDC6E,
                constants_str::VALUE_16359A6F,
                constants_str::VALUE_C0ED6D49,
                constants_str::VALUE_58AFC68F,
                constants_str::VALUE_568F63F0,
            ]
            .contains(&path.as_ref())
            {
                self.record(crate::types::SourceTextRef::from(path.as_ref()));
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_expr_loop(&mut self, expr_loop: &'ast syn::ExprLoop) {
        self.depth.saturating_inc();
        syn::visit::visit_expr_loop(self, expr_loop);
        self.depth.saturating_dec();
    }
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if matches!(
            expr_method_call.method.to_string().as_str(),
            constants_str::VALUE_B5D61DC8
                | constants_str::VALUE_81824C90
                | constants_str::VALUE_E132B7C0
                | constants_str::VALUE_C5E9F49A
        ) {
            self.record(crate::types::SourceTextRef::from(
                expr_method_call.method.to_string().as_str(),
            ));
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_expr_while(&mut self, expr_while: &'ast syn::ExprWhile) {
        self.depth.saturating_inc();
        syn::visit::visit_expr_while(self, expr_while);
        self.depth.saturating_dec();
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        if let Some(segment) = r#macro.path.segments.last()
            && matches!(
                segment.ident.to_string().as_str(),
                constants_str::SHARED_VALUES_FORMAT | constants_str::VALUE_38A4FDFC
            )
        {
            let operation = segment.ident.to_string();
            self.record(crate::types::SourceTextRef::from(operation.as_str()));
        }
        syn::visit::visit_macro(self, r#macro);
    }
}

#[test]
fn test_lock_guards_are_not_held_across_await() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    LockAcrossAwaitVisitor::default(),
                );
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}: {violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "ce73f4a1 {violations:#?}");
    });
}

#[test]
fn test_allocations_inside_loops_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::VALUE_B7558033,
            (constants_usize::ONE, constants_str::VALUE_F3EA9A31),
        ),
        (
            constants_str::VALUE_BC495D5D,
            (constants_usize::ONE, constants_str::VALUE_BAC5F80E),
        ),
        (
            constants_str::VALUE_E841E205,
            (constants_usize::ONE, constants_str::VALUE_870DAE5B),
        ),
        (
            constants_str::VALUE_BE04A453,
            (constants_usize::ONE, constants_str::VALUE_D1CA6996),
        ),
        (
            constants_str::VALUE_94FCEDB7,
            (constants_usize::ONE, constants_str::VALUE_855EA4C0),
        ),
    ]);
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    LoopAllocationVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .entries
                    .into_iter()
                    .map(move |entry| format!("{path}:{entry}"))
            })
            .fold(
                std::collections::BTreeMap::<String, usize>::new(),
                |mut counts, entry| {
                    let _count = counts
                        .entry(entry)
                        .and_modify(|count| *count = count.saturating_add(constants_usize::ONE))
                        .or_insert(constants_usize::ONE);
                    counts
                },
            );
        let expected = reviewed
            .iter()
            .map(|(entry, (count, reason))| {
                assert!(!reason.is_empty(), "418fe0af");
                ((*entry).to_owned(), *count)
            })
            .collect::<std::collections::BTreeMap<String, usize>>();
        assert_eq!(
            observed, expected,
            "418fe0af loop allocation inventory changed"
        );
    });
}

#[test]
fn test_struct_error_exceptions_match_reviewed_snapshot() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    StructErrorVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .get_identifiers()
                    .clone()
                    .into_iter()
                    .map(move |identifier| format!("{path}:{identifier}"))
            })
            .collect::<Vec<String>>();
        entries.sort();
        let mut current_snapshot = String::from(constants_str::VALUE_C746CC87);
        entries.into_iter().for_each(|entry| {
            current_snapshot.push_str(entry.as_str());
            current_snapshot.push('\n');
        });
        let snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::STRUCT_ERROR_SNAPSHOT_PATH);
        if std::env::var_os(constants_str::UPDATE_CODE_STYLE_SNAPSHOTS).is_some() {
            std::fs::write(snapshot_path.as_path(), current_snapshot.as_bytes())
                .expect(constants_str::DIAGNOSTIC_65E1D4F0);
        }
        let expected_snapshot =
            std::fs::read_to_string(snapshot_path).expect(constants_str::DIAGNOSTIC_BA047D32);
        assert_eq!(
            current_snapshot, expected_snapshot,
            "731ffc35 struct error inventory changed"
        );
    });
}

#[test]
fn test_contract_public_api_matches_reviewed_snapshot() {
    let reviewed = [
        (constants_str::VALUE_A766D43E, constants_str::VALUE_FE05288C),
        (constants_str::VALUE_C34A5FE6, constants_str::VALUE_8733430F),
        (constants_str::VALUE_0C5CC511, constants_str::VALUE_CF3D8D33),
    ];
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut current_snapshot = String::from(constants_str::VALUE_1F3A1C37);
        reviewed.iter().for_each(|(directory_suffix, reason)| {
            assert!(!reason.is_empty(), "505a0cf7");
            let mut entries = snapshot
                .rs_files()
                .iter()
                .filter(|source_file| {
                    source_file
                        .path()
                        .as_ref()
                        .to_string_lossy()
                        .contains(directory_suffix)
                })
                .flat_map(|source_file| {
                    let visitor = crate::code_style::visit_syn_file(
                        crate::types::SynFileRef::from(source_file.ast().as_ref()),
                        PublicApiVisitor {
                            entries: crate::types::SourceTextList::default(),
                            lines: crate::types::SourceTextList::from(
                                source_file
                                    .content()
                                    .as_ref()
                                    .lines()
                                    .map(str::to_owned)
                                    .collect::<Vec<String>>(),
                            ),
                        },
                    );
                    visitor.entries
                })
                .collect::<Vec<String>>();
            entries.sort();
            current_snapshot.push_str(format!("\n[{directory_suffix}] # {reason}\n").as_str());
            entries.into_iter().for_each(|entry| {
                current_snapshot.push_str(entry.as_str());
                current_snapshot.push('\n');
            });
        });
        let snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::CONTRACT_PUBLIC_API_SNAPSHOT_PATH);
        if std::env::var_os(constants_str::UPDATE_CODE_STYLE_SNAPSHOTS).is_some()
            || std::env::var_os(constants_str::UPDATE_CONTRACT_PUBLIC_API_SNAPSHOT).is_some()
        {
            std::fs::write(snapshot_path.as_path(), current_snapshot.as_bytes())
                .expect(constants_str::DIAGNOSTIC_E2C6B190);
        }
        let expected_snapshot =
            std::fs::read_to_string(snapshot_path).expect(constants_str::DIAGNOSTIC_FD9130E7);
        assert_eq!(
            current_snapshot, expected_snapshot,
            "505a0cf7 contract public API snapshot changed"
        );
    });
}

#[test]
fn test_arc_lock_and_trait_object_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::CODE_STYLE_COMMON_ROUTES_OWNER,
            (
                3,
                0,
                2,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_MACRO_HELPERS_OWNER,
            (
                0,
                0,
                83,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_NOTIFICATION_SERVICE_OWNER,
            (
                2,
                0,
                1,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_COMMON_OWNER,
            (
                0,
                0,
                12,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_MACRO_COMMON_OWNER,
            (
                0,
                0,
                140,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_PG_TYPES_COMMON_OWNER,
            (
                0,
                0,
                1,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_WHERE_FILTERS_OWNER,
            (
                0,
                0,
                4,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_WHERE_FILTERS_GENERATE_SRC_OWNER,
            (
                0,
                0,
                30,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_SERVER_OWNER,
            (
                2,
                0,
                1,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_SERVER_ADMIN_OWNER,
            (
                3,
                0,
                1,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_SERVER_RUNTIME_CORE_OWNER,
            (
                4,
                3,
                0,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::CODE_STYLE_SERVER_RUNTIME_HTTP_OWNER,
            (
                1,
                1,
                4,
                constants_str::CODE_STYLE_SHARED_DISPATCH_OWNER_REASON,
            ),
        ),
        (
            constants_str::VALUE_20A65589,
            (
                constants_usize::ZERO,
                constants_usize::ZERO,
                3usize,
                constants_str::VALUE_F75AB320,
            ),
        ),
        (
            constants_str::VALUE_823EE954,
            (0, 0, 4, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_D11679FC,
            (0, 0, 2, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_794839A7,
            (0, 0, 2, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_642AA8AC,
            (0, 0, 4, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_31BDEFD7,
            (0, 0, 2, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_95F11308,
            (0, 0, 3, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_BDEB5C57,
            (0, 0, 1, constants_str::VALUE_40349028),
        ),
        (
            constants_str::VALUE_8F0CF86A,
            (0, 0, 4, constants_str::VALUE_E98F8E33),
        ),
        (
            constants_str::VALUE_427B03A1,
            (0, 0, 5, constants_str::VALUE_D4BDC80F),
        ),
        (
            constants_str::VALUE_30B1AC8C,
            (0, 0, 1, constants_str::VALUE_DC7573CC),
        ),
        (
            constants_str::VALUE_8CD81F6A,
            (0, 0, 10, constants_str::VALUE_AC5E426F),
        ),
        (
            constants_str::VALUE_9C3011F7,
            (1, 0, 0, constants_str::VALUE_60EE0A5C),
        ),
        (
            constants_str::VALUE_315FCF0A,
            (1, 0, 0, constants_str::VALUE_60EE0A5C),
        ),
        (
            constants_str::VALUE_778833C1,
            (1, 0, 0, constants_str::VALUE_60EE0A5C),
        ),
        (
            constants_str::VALUE_EAC3A6DC,
            (0, 0, 3, constants_str::VALUE_0CD339A2),
        ),
        (
            constants_str::VALUE_7FE2AF02,
            (0, 0, 179, constants_str::VALUE_7FA1ACFA),
        ),
        (
            constants_str::VALUE_D405F3E1,
            (0, 0, 177, constants_str::VALUE_FB2CE6C2),
        ),
        (
            constants_str::VALUE_9D0FC67D,
            (0, 0, 1, constants_str::VALUE_E7909B41),
        ),
        (
            constants_str::VALUE_206B48D7,
            (0, 0, 1, constants_str::VALUE_F5E028C2),
        ),
    ]);
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed_by_owner = std::collections::BTreeMap::new();
        let mut violations = Vec::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .for_each(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    SharedDispatchVisitor::default(),
                );
                let observed = (
                    visitor.arc_types.get(),
                    visitor.lock_types.get(),
                    visitor.trait_objects.get(),
                );
                if observed
                    == (
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                        constants_usize::ZERO,
                    )
                {
                    return;
                }
                let path = source_file.path().as_ref().display().to_string();
                let reviewed_entry = reviewed
                    .iter()
                    .filter(|(suffix, (_arc, _lock, _traits, reason))| {
                        (path.ends_with(**suffix)
                            || crate::code_style::declared_child_matches(path.as_str(), suffix))
                            && !reason.is_empty()
                    })
                    .max_by_key(|(suffix, _expected)| suffix.len());
                match reviewed_entry {
                    Some((suffix, _expected)) => {
                        let entry = observed_by_owner.entry(*suffix).or_insert((0, 0, 0));
                        entry.0 += observed.0;
                        entry.1 += observed.1;
                        entry.2 += observed.2;
                    }
                    None => violations.push(format!(
                        "{path}: unreviewed Arc/lock/trait-object usage {observed:?}"
                    )),
                }
            });
        reviewed.iter().for_each(|(suffix, expected)| {
            let observed = observed_by_owner.get(suffix).copied().unwrap_or((0, 0, 0));
            if observed != (expected.0, expected.1, expected.2) {
                violations.push(format!(
                    "shared-dispatch inventory changed for {suffix}: expected={:?}, observed={observed:?}",
                    (expected.0, expected.1, expected.2)
                ));
            }
        });
        assert!(violations.is_empty(), "66b91e7a {violations:#?}");
    });
}

#[test]
fn test_ignored_map_err_bindings_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::CODE_STYLE_COMMON_ROUTES_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_CONFIG_LIB_OWNER,
            (4usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_EXTERNAL_SERVICE_EMULATORS_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_FILE_STORAGE_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_FRONTEND_CONTRACT_VALIDATION_OWNER,
            (2usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_MACRO_HELPERS_OWNER,
            (3usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_NOTIFICATION_SERVICE_OWNER,
            (2usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_COMMON_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_PG_CRUD_WHERE_FILTERS_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_SERVER_ADMIN_OWNER,
            (110usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_SERVER_ADMIN_CONTRACT_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_SERVER_ADMIN_CORE_OWNER,
            (1usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_SERVER_ADMIN_FRONTEND_OWNER,
            (11usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_SERVER_RUNTIME_HTTP_OWNER,
            (11usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_WORKSPACE_SCAFFOLD_OWNER,
            (16usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::CODE_STYLE_WORKSPACE_TEST_RUNNER_OWNER,
            (2usize, constants_str::CODE_STYLE_MAP_ERR_OWNER_REASON),
        ),
        (
            constants_str::VALUE_AC7A6F68,
            (2usize, constants_str::VALUE_3995FF01),
        ),
        (
            constants_str::VALUE_769125D7,
            (constants_usize::ONE, constants_str::VALUE_EB67E2C6),
        ),
        (
            constants_str::VALUE_46FA1B05,
            (2usize, constants_str::VALUE_C98C08E2),
        ),
        (
            constants_str::VALUE_96B90C9B,
            (constants_usize::ONE, constants_str::VALUE_9111728C),
        ),
        (
            constants_str::VALUE_E5C6D18E,
            (8usize, constants_str::VALUE_9111728C),
        ),
        (
            constants_str::VALUE_11F5A276,
            (2usize, constants_str::VALUE_C1819A84),
        ),
        (
            constants_str::VALUE_2CBAA4F4,
            (constants_usize::ONE, constants_str::VALUE_53588272),
        ),
        (
            constants_str::VALUE_D191EE7F,
            (2usize, constants_str::VALUE_53588272),
        ),
        (
            constants_str::VALUE_B29D07A8,
            (constants_usize::ONE, constants_str::VALUE_099B4392),
        ),
        (
            constants_str::VALUE_A1750307,
            (constants_usize::ONE, constants_str::VALUE_099B4392),
        ),
        (
            constants_str::VALUE_3930BC5E,
            (constants_usize::ONE, constants_str::VALUE_7B6389D8),
        ),
        (
            constants_str::VALUE_E24F0FD4,
            (constants_usize::ONE, constants_str::VALUE_01371493),
        ),
        (
            constants_str::VALUE_4C6F4532,
            (8usize, constants_str::VALUE_80247FE1),
        ),
        (
            constants_str::VALUE_DC454021,
            (4usize, constants_str::VALUE_FD41C49E),
        ),
        (
            constants_str::VALUE_DC37304F,
            (constants_usize::ONE, constants_str::VALUE_FD41C49E),
        ),
        (
            constants_str::VALUE_939FFBC6,
            (6usize, constants_str::VALUE_FD41C49E),
        ),
        (
            constants_str::VALUE_F3169686,
            (5usize, constants_str::VALUE_FAE4D1C8),
        ),
        (
            constants_str::VALUE_27AB06E9,
            (2usize, constants_str::VALUE_B1E73CDD),
        ),
        (
            constants_str::VALUE_9E7DB142,
            (10usize, constants_str::VALUE_0B70A676),
        ),
        (
            constants_str::VALUE_BEBEC57E,
            (constants_usize::ONE, constants_str::VALUE_9CA4EAEB),
        ),
    ]);
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed_by_owner = std::collections::BTreeMap::new();
        let mut violations = Vec::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = crate::code_style::visit_syn_file(
                crate::types::SynFileRef::from(source_file.ast().as_ref()),
                IgnoredMapErrBindingVisitor::default(),
            );
            if visitor.entries.is_empty() {
                return;
            }
            let path = source_file.path().as_ref().display().to_string();
            let reviewed_entry = reviewed
                .iter()
                .filter(|(suffix, (_count, reason))| {
                    (path.ends_with(**suffix)
                        || crate::code_style::declared_child_matches(path.as_str(), suffix))
                        && !reason.is_empty()
                })
                .max_by_key(|(suffix, _expected)| suffix.len());
            match reviewed_entry {
                Some((suffix, _expected)) => {
                    *observed_by_owner.entry(*suffix).or_insert(0usize) += visitor.entries.len();
                }
                None => violations.push(format!(
                    "{path}: unreviewed ignored map_err bindings: count={}",
                    visitor.entries.len()
                )),
            }
        });
        reviewed.iter().for_each(|(suffix, (expected, _reason))| {
            let observed = observed_by_owner.get(suffix).copied().unwrap_or(0usize);
            if observed != *expected {
                violations.push(format!(
                    "ignored map_err inventory changed for {suffix}: expected count={expected}; observed count={observed}"
                ));
            }
        });
        assert!(violations.is_empty(), "bb0dbc1f {violations:#?}");
    });
}

#[test]
fn test_raw_vec_tuple_wrappers_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (constants_str::VALUE_7630EBEC, constants_str::VALUE_C2C65C68),
        (constants_str::VALUE_86D03626, constants_str::VALUE_E3D9A7E6),
        (constants_str::VALUE_6C761A40, constants_str::VALUE_BC91BCEF),
        (constants_str::VALUE_F68E036F, constants_str::VALUE_D17C5423),
        (constants_str::VALUE_090096ED, constants_str::VALUE_07FFA47C),
        (constants_str::VALUE_94E2B4FA, constants_str::VALUE_63229E70),
        (constants_str::VALUE_D9B93146, constants_str::VALUE_FC3332AB),
        (constants_str::VALUE_6F5D2E20, constants_str::VALUE_0901EA34),
        (constants_str::VALUE_9DFC7A97, constants_str::VALUE_FDB078C8),
        (constants_str::VALUE_0525E2BF, constants_str::VALUE_FBBB4FDC),
        (constants_str::VALUE_CAE88716, constants_str::VALUE_FBBB4FDC),
        (constants_str::VALUE_D51ADF29, constants_str::VALUE_16D85132),
        (constants_str::VALUE_975B0C21, constants_str::VALUE_FBBB4FDC),
        (constants_str::VALUE_AA7EE094, constants_str::VALUE_16D85132),
        (constants_str::VALUE_5879251A, constants_str::VALUE_FBBB4FDC),
        (constants_str::VALUE_51CC135E, constants_str::VALUE_16D85132),
        (constants_str::VALUE_B1A7F284, constants_str::VALUE_16D85132),
        (constants_str::VALUE_8C2154B5, constants_str::VALUE_55AE895C),
        (constants_str::VALUE_7314D06D, constants_str::VALUE_EF8D5AF4),
        (constants_str::VALUE_9AE03CB2, constants_str::VALUE_986BBD24),
        (constants_str::VALUE_6BF051A2, constants_str::VALUE_A5952628),
        (constants_str::VALUE_C7F27415, constants_str::VALUE_C3214518),
        (constants_str::VALUE_A417488B, constants_str::VALUE_D39882F4),
        (constants_str::VALUE_919ACACB, constants_str::VALUE_1FBF1A7A),
        (constants_str::VALUE_9DB8F65B, constants_str::VALUE_7C37CACC),
        (constants_str::VALUE_671231A3, constants_str::VALUE_D82FE516),
        (constants_str::VALUE_DEB830DD, constants_str::VALUE_DCAEE23B),
        (constants_str::VALUE_DD337AC0, constants_str::VALUE_C9221A63),
        (constants_str::VALUE_06C235F4, constants_str::VALUE_211A1405),
        (constants_str::VALUE_2316F647, constants_str::VALUE_0EFD8ED8),
        (constants_str::VALUE_5D687FEA, constants_str::VALUE_0C7973A9),
        (constants_str::VALUE_7E7B2B37, constants_str::VALUE_0F84F758),
        (constants_str::VALUE_CB780650, constants_str::VALUE_8FEB779E),
        (constants_str::VALUE_1D2594F2, constants_str::VALUE_5D972838),
        (constants_str::VALUE_A48AAE67, constants_str::VALUE_2DCAD87D),
        (constants_str::VALUE_B9937202, constants_str::VALUE_13C920C3),
        (constants_str::VALUE_2941B657, constants_str::VALUE_13C920C3),
        (constants_str::VALUE_FAB1545F, constants_str::VALUE_64EA6158),
        (constants_str::VALUE_9FB992E8, constants_str::VALUE_3F51E18F),
        (constants_str::VALUE_D200D86F, constants_str::VALUE_352F4313),
        (constants_str::VALUE_0F3C7A91, constants_str::VALUE_352F4313),
        (constants_str::VALUE_413BDF99, constants_str::VALUE_28A55761),
        (constants_str::VALUE_EA3B0668, constants_str::VALUE_82F6C375),
    ]);
    reviewed
        .values()
        .for_each(|reason| assert!(!reason.is_empty(), "f8c9471a"));
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    RawVecTupleWrapperVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .get_identifiers()
                    .clone()
                    .into_iter()
                    .map(move |identifier| format!("{path}:{identifier}"))
            })
            .collect::<std::collections::BTreeSet<String>>();
        let expected = reviewed
            .keys()
            .map(|entry| (*entry).to_owned())
            .collect::<std::collections::BTreeSet<String>>();
        assert_eq!(
            observed, expected,
            "f8c9471a raw Vec tuple wrapper inventory changed"
        );
    });
}

#[test]
fn test_from_vec_implementations_are_forbidden() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    FromVecImplVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .targets
                    .into_iter()
                    .map(move |target| format!("{path}:{target}"))
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "6cc64ce8 {violations:#?}");
    });
}

#[test]
fn test_raw_vec_tuple_wrapper_visitor_detects_qualified_and_nested_types() {
    let file: syn::File = syn::parse_quote! {
        struct Qualified(std::vec::Vec<u8>);
        struct Named {
            values: Vec<u8>,
        }
        mod nested {
            struct Nested(Vec<u8>);
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&file),
        RawVecTupleWrapperVisitor::default(),
    );
    assert_eq!(visitor.get_identifiers().len(), 2usize);
}

#[test]
fn test_usize_max_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            constants_str::VALUE_08DBA674,
            (constants_usize::ONE, constants_str::VALUE_CEE3C893),
        ),
        (
            constants_str::VALUE_2483AEA6,
            (3usize, constants_str::VALUE_245849CA),
        ),
        (
            constants_str::VALUE_7FE2AF02,
            (3usize, constants_str::VALUE_211A1405),
        ),
        (
            constants_str::PROCESS_ARGUMENTS_PATH,
            (constants_usize::ONE, constants_str::VALUE_28A0F9A4),
        ),
        (
            constants_str::PROCESS_COMMANDS_PATH,
            (constants_usize::ONE, constants_str::VALUE_28A0F9A4),
        ),
        (
            constants_str::CHILD_DIAGNOSTIC_PATH,
            (
                constants_usize::ONE,
                constants_str::DIAGNOSTIC_BUFFER_MAX_REASON,
            ),
        ),
        (
            constants_str::STD_COLLECTIONS_CHILD_PROCESS_MAP_PATH,
            (
                constants_usize::ONE,
                constants_str::RUNTIME_LIMITED_STORAGE_MAX_REASON,
            ),
        ),
        (
            constants_str::CHILD_PROCESS_REPORTS_PATH,
            (
                constants_usize::ONE,
                constants_str::DIAGNOSTIC_BUFFER_MAX_REASON,
            ),
        ),
        (
            constants_str::LEASE_REGISTRY_INNER_PATH,
            (2usize, constants_str::VALUE_E55D8523),
        ),
        (
            constants_str::LEASE_IDS_PATH,
            (constants_usize::ONE, constants_str::VALUE_E55D8523),
        ),
        (
            constants_str::SINGLE_FLIGHT_INNER_PATH,
            (constants_usize::ONE, constants_str::VALUE_845FE7CB),
        ),
        (
            constants_str::DISK_CACHE_EVICTION_PLAN_PATH,
            (
                constants_usize::ONE,
                constants_str::RUNTIME_LIMITED_STORAGE_MAX_REASON,
            ),
        ),
        (
            constants_str::ENV_KEYS_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::INIT_ENTRIES_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::RUN_COMMANDS_PATH,
            (
                constants_usize::ONE,
                constants_str::FAILURE_SENTINEL_MAX_REASON,
            ),
        ),
        (
            constants_str::COMMAND_TEXTS_PATH,
            (constants_usize::ONE, constants_str::VALUE_28A0F9A4),
        ),
        (
            constants_str::ROUTE_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::ROUTE_COVERAGE_DESCRIPTORS_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::ACTION_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::ROUTE_METADATA_LIST_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::ROUTE_TEST_CATEGORIES_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::ROUTE_SCHEMA_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::FIELD_CONTRACTS_PATH,
            (
                constants_usize::ONE,
                constants_str::COMPILE_TIME_CATALOG_MAX_REASON,
            ),
        ),
        (
            constants_str::ROUTE_CONTRACT_MISMATCHES_PATH,
            (
                constants_usize::ONE,
                constants_str::RUNTIME_LIMITED_STORAGE_MAX_REASON,
            ),
        ),
        (
            constants_str::SERVICE_CATALOG_ENTRIES_PATH,
            (constants_usize::ONE, constants_str::VALUE_0BF03626),
        ),
    ]);
    reviewed
        .values()
        .for_each(|(_count, reason)| assert!(!reason.is_empty(), "cfc5175f"));
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed = std::collections::BTreeMap::<&str, usize>::new();
        let mut violations = Vec::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !crate::code_style::is_test_source_path(crate::types::PathRef::from(
                    std::borrow::Borrow::<std::path::Path>::borrow(source_file.path()),
                ))
                .get()
            })
            .for_each(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    UsizeMaxExprVisitor::default(),
                );
                let count = visitor.count.get();
                if count == constants_usize::ZERO {
                    return;
                }
                let path = source_file.path().as_ref().display().to_string();
                let reviewed_owner = reviewed
                    .keys()
                    .filter(|suffix| {
                        path.ends_with(**suffix)
                            || crate::code_style::declared_child_matches(path.as_str(), suffix)
                    })
                    .max_by_key(|suffix| suffix.len());
                if let Some(owner) = reviewed_owner {
                    let _observed_count_entry = observed
                        .entry(owner)
                        .and_modify(|observed_count| {
                            *observed_count = observed_count.saturating_add(count);
                        })
                        .or_insert(count);
                } else {
                    violations.push(format!("unreviewed usize::MAX owner: {path}"));
                }
            });
        let expected = reviewed
            .iter()
            .map(|(path, (count, _reason))| (*path, *count))
            .collect::<std::collections::BTreeMap<&str, usize>>();
        assert_eq!(observed, expected, "cfc5175f usize::MAX inventory changed");
        assert!(violations.is_empty(), "cfc5175f {violations:#?}");
    });
}

#[test]
fn test_usize_max_expression_visitor_skips_test_modules() {
    let file: syn::File = syn::parse_quote! {
        const PRODUCTION_MAXIMUM: usize = usize::MAX;
        #[cfg(test)]
        mod tests {
            const TEST_MAXIMUM: usize = usize::MAX;
        }
    };
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&file),
        UsizeMaxExprVisitor::default(),
    );
    assert_eq!(visitor.count.get(), constants_usize::ONE);
}

#[test]
fn test_select_sites_match_reviewed_cancellation_inventory() {
    let reviewed = [
        (
            constants_str::SERVE_WITH_GRACEFUL_SHUTDOWN_PATH,
            constants_usize::ONE,
            constants_str::SERVE_WITH_GRACEFUL_SHUTDOWN_SELECT_REASON,
        ),
        (
            constants_str::SPAWN_INTERVAL_TASK_PATH,
            constants_usize::ONE,
            constants_str::VALUE_5337167F,
        ),
        (
            constants_str::WAIT_FOR_SERVICE_SHUTDOWN_SIGNAL_PATH,
            constants_usize::ONE,
            constants_str::VALUE_C8647B8D,
        ),
    ];
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed = std::collections::BTreeMap::<String, usize>::new();
        let mut violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    SelectMacroVisitor::default(),
                );
                if visitor.count.get() != constants_usize::ZERO {
                    let path = source_file.path().as_ref().display().to_string();
                    let reviewed_owner = reviewed
                        .iter()
                        .map(|(reviewed_path, _count, _reason)| *reviewed_path)
                        .filter(|reviewed_path| {
                            path.ends_with(reviewed_path)
                                || crate::code_style::declared_child_matches(
                                    path.as_str(),
                                    reviewed_path,
                                )
                        })
                        .max_by_key(|reviewed_path| reviewed_path.len());
                    if let Some(owner_suffix) = reviewed_owner {
                        let owner_path = format!("../{owner_suffix}");
                        let _observed_entry = observed
                            .entry(owner_path)
                            .and_modify(|count| {
                                *count = count.saturating_add(visitor.count.get());
                            })
                            .or_insert_with(|| visitor.count.get());
                    } else {
                        return std::iter::once(format!(
                            "{}: unreviewed select owner",
                            source_file.path().as_ref().display()
                        ))
                        .chain(visitor.unsafe_operations.into_iter().map(|violation| {
                            format!("{}: {violation}", source_file.path().as_ref().display())
                        }))
                        .collect::<Vec<String>>();
                    }
                }
                visitor
                    .unsafe_operations
                    .into_iter()
                    .map(|violation| {
                        format!("{}: {violation}", source_file.path().as_ref().display())
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        reviewed
            .iter()
            .filter(|(_path, _count, reason)| reason.is_empty())
            .for_each(|(path, _count, _reason)| {
                violations.push(format!("select inventory `{path}` has no reason"));
            });
        let expected = reviewed
            .iter()
            .map(|(path, count, _reason)| (format!("../{path}"), *count))
            .collect::<std::collections::BTreeMap<String, usize>>();
        if observed != expected {
            violations.push(format!(
                "select inventory changed: expected={expected:#?}, observed={observed:#?}"
            ));
        }
        assert!(violations.is_empty(), "29fa10f5 {violations:#?}");
    });
}

#[test]
fn test_select_policy_rejects_cancellation_sensitive_operations() {
    let ast =
        syn::parse_file(constants_str::VALUE_F6958372).expect(constants_str::DIAGNOSTIC_714C620F);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        SelectMacroVisitor::default(),
    );
    assert_eq!(visitor.unsafe_operations.len(), 2usize, "c4267f0a");
}

#[test]
fn test_architectural_boundaries_reject_upward_dependencies() {
    let boundaries = [
        (constants_str::VALUE_9A26B6D6, constants_str::VALUE_D54C0026),
        (constants_str::VALUE_5906FF0B, constants_str::VALUE_64313A40),
        (constants_str::VALUE_B29A11B9, constants_str::VALUE_FB301D46),
        (constants_str::VALUE_E1717E8B, constants_str::VALUE_72104B4E),
        (constants_str::VALUE_B4F499E2, constants_str::VALUE_2773E6CE),
    ];
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let workspace_names = snapshot.workspace_crate_names();
        let mut violations = Vec::new();
        boundaries.iter().for_each(|(package_name, reason)| {
            if reason.is_empty() {
                violations.push(format!(
                    "architecture boundary `{package_name}` has no reason"
                ));
            }
            let package = snapshot
                .workspace_metadata()
                .get()
                .packages
                .iter()
                .find(|package| package.name == *package_name)
                .expect(constants_str::DIAGNOSTIC_010E6A3F);
            let observed = package
                .dependencies
                .iter()
                .filter(|dependency| workspace_names.as_ref().contains(dependency.name.as_str()))
                .filter(|dependency| match *package_name {
                    constants_str::VALUE_9A26B6D6 => {
                        dependency.name == constants_str::VALUE_CA3132B2
                            || dependency.name.starts_with(constants_str::VALUE_AAADEE66)
                            || dependency.name.starts_with(constants_str::VALUE_D11DB134)
                            || (dependency.name.starts_with(constants_str::VALUE_B3EACD33)
                                && dependency.name != constants_str::VALUE_EC36C4C9)
                    }
                    constants_str::VALUE_5906FF0B => {
                        dependency.name == constants_str::VALUE_CA3132B2
                            || dependency.name == constants_str::VALUE_B3EACD33
                            || dependency.name == constants_str::VALUE_6D090579
                            || dependency.name == constants_str::VALUE_D0393EDD
                            || dependency.name.ends_with(constants_str::VALUE_93CEFD0B)
                            || dependency.name.starts_with(constants_str::VALUE_AAADEE66)
                            || dependency.name.starts_with(constants_str::VALUE_D11DB134)
                    }
                    constants_str::VALUE_B29A11B9
                    | constants_str::VALUE_E1717E8B
                    | constants_str::VALUE_B4F499E2 => {
                        dependency.name == constants_str::VALUE_CA3132B2
                            || dependency.name == constants_str::VALUE_B3EACD33
                            || dependency.name.starts_with(constants_str::VALUE_6D090579)
                            || dependency.name.starts_with(constants_str::VALUE_AAADEE66)
                            || dependency.name.ends_with(constants_str::VALUE_14AD1127)
                            || dependency.name.starts_with(constants_str::VALUE_D11DB134)
                    }
                    _ => true,
                })
                .map(|dependency| dependency.name.clone())
                .collect::<std::collections::BTreeSet<String>>();
            if !observed.is_empty() {
                violations.push(format!(
                    "{package_name} has upward workspace dependencies: {observed:?}"
                ));
            }
        });
        assert!(violations.is_empty(), "2fdc155b {violations:#?}");
    });
}

#[test]
fn test_lock_across_await_policy_requires_explicit_drop() {
    let invalid =
        syn::parse_file(constants_str::VALUE_6F786FC4).expect(constants_str::DIAGNOSTIC_B57DF6A3);
    let valid =
        syn::parse_file(constants_str::VALUE_D481790B).expect(constants_str::DIAGNOSTIC_A62F1CE9);
    let invalid_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&invalid),
        LockAcrossAwaitVisitor::default(),
    );
    let valid_visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&valid),
        LockAcrossAwaitVisitor::default(),
    );
    assert_eq!(
        invalid_visitor.get_violations().len(),
        constants_usize::ONE,
        "bbfce72c"
    );
    assert!(valid_visitor.get_violations().is_empty(), "4b732bd1");
}

#[test]
fn test_production_code_does_not_use_explicit_leak_apis() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .starts_with(constants_str::TESTS_CODE_STYLE)
            })
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    LeakApiVisitor::default(),
                );
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}: {violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "8522eda3 {violations:#?}");
    });
}

#[test]
fn test_retained_spawn_tasks_are_supervised() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    SpawnLifecycleVisitor::default(),
                );
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}: {violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "aa16974d {violations:#?}");
    });
}

#[test]
fn test_spawn_lifecycle_policy_rejects_unconsumed_tasks() {
    let ast =
        syn::parse_file(constants_str::VALUE_9F18A090).expect(constants_str::DIAGNOSTIC_834138AF);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        SpawnLifecycleVisitor::default(),
    );
    assert_eq!(
        visitor.get_violations().as_slice(),
        [constants_str::VALUE_B20423DF],
        "a1680c46"
    );
}

#[test]
fn test_route_path_segments_use_snake_case() {
    super::test_code_style_snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = crate::code_style::visit_syn_file(
                    crate::types::SynFileRef::from(source_file.ast().as_ref()),
                    RouteLiteralVisitor::default(),
                );
                visitor
                    .get_violations()
                    .clone()
                    .into_iter()
                    .map(|violation| {
                        format!("{}: {violation}", source_file.path().as_ref().display())
                    })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "ebde2ab8 {violations:#?}");
    });
}

#[test]
fn test_route_path_policy_rejects_kebab_case() {
    let ast =
        syn::parse_file(constants_str::VALUE_72E2834F).expect(constants_str::DIAGNOSTIC_9AA037DC);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        RouteLiteralVisitor::default(),
    );
    assert_eq!(
        visitor.get_violations().len(),
        constants_usize::ONE,
        "d15287e9"
    );
}

#[test]
fn test_route_path_policy_rejects_api_prefix() {
    let ast =
        syn::parse_file(constants_str::VALUE_D7270E5B).expect(constants_str::DIAGNOSTIC_3EAA623D);
    let visitor = crate::code_style::visit_syn_file(
        crate::types::SynFileRef::from(&ast),
        RouteLiteralVisitor::default(),
    );
    assert_eq!(
        visitor.get_violations().len(),
        constants_usize::ONE,
        "5caaea72"
    );
}
