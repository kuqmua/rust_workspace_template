#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    clippy::single_call_fn,
    clippy::wildcard_enum_match_arm,
    reason = "policy visitors stay grouped with their invariant, repository policy requires iterator methods, and syn non-exhaustive enums require fallback handling"
)]

#[derive(Default)]
struct AwaitVisitor {
    found: super::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for AwaitVisitor {
    fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
        self.found.set_true();
        syn::visit::visit_expr_await(self, i);
    }
}

#[derive(Default)]
struct LockAcrossAwaitVisitor {
    violations: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for LockAcrossAwaitVisitor {
    fn visit_block(&mut self, i: &'ast syn::Block) {
        let mut active_guards = std::collections::BTreeSet::<String>::new();
        i.stmts.iter().for_each(|statement| {
            let mut await_visitor = AwaitVisitor::default();
            syn::visit::Visit::visit_stmt(&mut await_visitor, statement);
            if await_visitor.found.get() && !active_guards.is_empty() {
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
                && local_initializer_acquires_lock(local)
                && let syn::Pat::Ident(identifier) = &local.pat
            {
                let _inserted = active_guards.insert(identifier.ident.to_string());
            }
            dropped_identifier(statement)
                .into_iter()
                .for_each(|identifier| {
                    let _removed = active_guards.remove(identifier.as_ref());
                });
        });
        syn::visit::visit_block(self, i);
    }
}

fn local_initializer_acquires_lock(local: &syn::Local) -> bool {
    local
        .init
        .as_ref()
        .is_some_and(|initializer| expression_acquires_lock(initializer.expr.as_ref()))
}

fn expression_acquires_lock(expression: &syn::Expr) -> bool {
    match expression {
        syn::Expr::Await(await_expression) => {
            expression_acquires_lock(await_expression.base.as_ref())
        }
        syn::Expr::MethodCall(call) => {
            call.args.is_empty()
                && matches!(
                    call.method.to_string().as_str(),
                    "lock" | "lock_owned" | "read" | "read_owned" | "write" | "write_owned"
                )
        }
        syn::Expr::Paren(paren) => expression_acquires_lock(paren.expr.as_ref()),
        syn::Expr::Try(try_expression) => expression_acquires_lock(try_expression.expr.as_ref()),
        _ => false,
    }
}

fn dropped_identifier(statement: &syn::Stmt) -> Option<super::types::SourceText> {
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
        Some("drop")
    ) {
        return None;
    }
    let syn::Expr::Path(argument) = call.args.first()? else {
        return None;
    };
    (argument.path.segments.len() == 1usize)
        .then(|| {
            argument.path.segments.first().map(|segment| {
                super::types::SourceText::try_from(segment.ident.to_string()).expect("d4f6bdce")
            })
        })
        .flatten()
}

#[derive(Default)]
struct LeakApiVisitor {
    violations: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for LeakApiVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = i.func.as_ref() {
            let path = function
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<String>>()
                .join("::");
            if [
                "Box::leak",
                "std::boxed::Box::leak",
                "std::mem::forget",
                "core::mem::forget",
                "Arc::into_raw",
                "std::sync::Arc::into_raw",
                "Box::into_raw",
                "std::boxed::Box::into_raw",
                "Arc::from_raw",
                "std::sync::Arc::from_raw",
                "Box::from_raw",
                "std::boxed::Box::from_raw",
            ]
            .contains(&path.as_str())
            {
                self.violations.push(path);
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if i.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "ManuallyDrop")
        {
            self.violations.push("ManuallyDrop".to_owned());
        }
        syn::visit::visit_type_path(self, i);
    }
}

#[derive(Default)]
struct SpawnConsumptionVisitor {
    consumed: super::types::StdSourceTextSet,
}
impl SpawnConsumptionVisitor {
    fn record_path(&mut self, expression: &syn::Expr) {
        if let syn::Expr::Path(path) = expression
            && path.path.segments.len() == 1usize
            && let Some(segment) = path.path.segments.first()
        {
            let _inserted = self.consumed.insert(segment.ident.to_string());
        }
    }
    fn record_macro_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        let trees = tokens.into_iter().collect::<Vec<proc_macro2::TokenTree>>();
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
                && matches!(operation.to_string().as_str(), "abort" | "await")
            {
                let _inserted = self.consumed.insert(identifier.to_string());
            }
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for SpawnConsumptionVisitor {
    fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
        self.record_path(i.base.as_ref());
        syn::visit::visit_expr_await(self, i);
    }
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        let is_drop = matches!(
            i.func.as_ref(),
            syn::Expr::Path(path)
                if path.path.segments.last().is_some_and(|segment| segment.ident == "drop")
        );
        if !is_drop {
            i.args
                .iter()
                .for_each(|argument| self.record_path(argument));
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == "abort" {
            self.record_path(i.receiver.as_ref());
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_field_value(&mut self, i: &'ast syn::FieldValue) {
        self.record_path(&i.expr);
        syn::visit::visit_field_value(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        self.record_macro_tokens(i.tokens.clone());
        syn::visit::visit_macro(self, i);
    }
}

#[derive(Default)]
struct SpawnLifecycleVisitor {
    violations: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for SpawnLifecycleVisitor {
    fn visit_block(&mut self, i: &'ast syn::Block) {
        let mut pending = std::collections::BTreeSet::<String>::new();
        i.stmts.iter().for_each(|statement| {
            let mut consumption = SpawnConsumptionVisitor::default();
            syn::visit::Visit::visit_stmt(&mut consumption, statement);
            consumption.consumed.into_iter().for_each(|identifier| {
                let _removed = pending.remove(identifier.as_str());
            });
            if let syn::Stmt::Local(local) = statement
                && local
                    .init
                    .as_ref()
                    .is_some_and(|initializer| super::unowned_spawn_expr(initializer.expr.as_ref()))
                && let syn::Pat::Ident(identifier) = &local.pat
                && !identifier.ident.to_string().starts_with('_')
            {
                let _inserted = pending.insert(identifier.ident.to_string());
            }
        });
        pending.into_iter().for_each(|identifier| {
            self.violations.push(format!(
                "spawn handle `{identifier}` is retained but never awaited, aborted, or transferred to an owner"
            ));
        });
        syn::visit::visit_block(self, i);
    }
}

#[derive(Default)]
struct RouteLiteralVisitor {
    violations: super::types::DiagnosticMsgs,
}
impl RouteLiteralVisitor {
    fn inspect_literal(&mut self, literal: &syn::LitStr) {
        let value = literal.value();
        if !value.starts_with('/') || value.starts_with("//") {
            return;
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
    fn inspect_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        tokens.into_iter().for_each(|token| match token {
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
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if i.path().segments.last().is_some_and(|segment| {
            matches!(segment.ident.to_string().as_str(), "typed_route" | "strum")
        }) && let syn::Meta::List(list) = &i.meta
        {
            self.inspect_tokens(list.tokens.clone());
        }
        syn::visit::visit_attribute(self, i);
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if matches!(
            i.method.to_string().as_str(),
            "route" | "nest" | "route_service"
        ) && let Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(literal),
            ..
        })) = i.args.first()
        {
            self.inspect_literal(literal);
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

#[derive(Default)]
struct SelectMacroVisitor {
    count: super::types::AnalyzerCount,
    unsafe_operations: super::types::DiagnosticMsgs,
}
impl SelectMacroVisitor {
    fn inspect_sensitive_tokens(&mut self, tokens: proc_macro2::TokenStream) {
        tokens.into_iter().for_each(|token| match token {
            proc_macro2::TokenTree::Group(group) => {
                self.inspect_sensitive_tokens(group.stream());
            }
            proc_macro2::TokenTree::Ident(identifier)
                if matches!(
                    identifier.to_string().as_str(),
                    "read_exact"
                        | "read_to_end"
                        | "read_to_string"
                        | "reserve"
                        | "send"
                        | "write_all"
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
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        let is_select = i
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "select");
        if is_select {
            self.count.saturating_inc();
            self.inspect_sensitive_tokens(i.tokens.clone());
        }
        syn::visit::visit_macro(self, i);
    }
}

#[derive(Default)]
struct ExpressionPathVisitor {
    paths: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for ExpressionPathVisitor {
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        self.paths.push(
            super::path_to_string(super::types::SynPathRef::from(&i.path))
                .as_ref()
                .to_owned(),
        );
        syn::visit::visit_expr_path(self, i);
    }
}

#[derive(Default)]
struct IgnoredMapErrBindingVisitor {
    entries: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for IgnoredMapErrBindingVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == "map_err"
            && let Some(syn::Expr::Closure(closure)) = i.args.first()
        {
            let ignored_inputs = closure
                .inputs
                .iter()
                .filter_map(|input| match input {
                    syn::Pat::Wild(_) => Some("_".to_owned()),
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
                    syn::spanned::Spanned::span(i).start().line,
                    ignored_inputs.join(","),
                    path_visitor.paths.join(",")
                ));
            }
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

#[derive(Default)]
struct RawVecTupleWrapperVisitor {
    identifiers: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for RawVecTupleWrapperVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if let syn::Fields::Unnamed(fields) = &i.fields
            && fields.unnamed.len() == 1usize
            && let Some(field) = fields.unnamed.first()
            && let syn::Type::Path(path) = &field.ty
            && path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Vec")
        {
            self.identifiers.push(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(Default)]
struct UsizeMaxExprVisitor {
    count: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UsizeMaxExprVisitor {
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        let mut segments = i.path.segments.iter();
        if segments
            .next()
            .is_some_and(|segment| segment.ident == "usize")
            && segments
                .next()
                .is_some_and(|segment| segment.ident == "MAX")
            && segments.next().is_none()
        {
            self.count.saturating_inc();
        }
        syn::visit::visit_expr_path(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        if super::attrs_contain_test_only_cfg(super::types::SynAttributeListRef::from(
            i.attrs.as_slice(),
        ))
        .get()
        {
            return;
        }
        syn::visit::visit_item_mod(self, i);
    }
}

#[derive(Default)]
struct SharedDispatchVisitor {
    arc_types: super::types::AnalyzerCount,
    lock_types: super::types::AnalyzerCount,
    trait_objects: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for SharedDispatchVisitor {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if let Some(segment) = i.path.segments.last() {
            match segment.ident.to_string().as_str() {
                "Arc" => self.arc_types.saturating_inc(),
                "Mutex" | "RwLock" => self.lock_types.saturating_inc(),
                _ => {}
            }
        }
        syn::visit::visit_type_path(self, i);
    }
    fn visit_type_trait_object(&mut self, i: &'ast syn::TypeTraitObject) {
        self.trait_objects.saturating_inc();
        syn::visit::visit_type_trait_object(self, i);
    }
}

struct PublicApiVisitor {
    entries: super::types::SourceTextList,
    lines: super::types::SourceTextList,
}
impl PublicApiVisitor {
    fn source(&self, span: proc_macro2::Span) -> super::types::SourceText {
        let start = span.start().line.saturating_sub(1usize);
        let end = span.end().line;
        let normalized = self
            .lines
            .get(start..end)
            .map(|lines| lines.join("\n"))
            .expect("c9d73e55")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        super::types::SourceText::try_from(normalized).expect("31f04bb7")
    }
    fn field_type(&self, field: &syn::Field) -> super::types::SourceText {
        let source = self.source(syn::spanned::Spanned::span(field));
        let field_type = source
            .as_ref()
            .split_once(':')
            .map(|(_field, field_type)| field_type.trim().trim_end_matches(',').to_owned())
            .expect("5af91e82");
        super::types::SourceText::try_from(field_type).expect("3e2d89ef")
    }
    fn record(&mut self, span: proc_macro2::Span, signature_only: bool) {
        let start = span.start().line.saturating_sub(1usize);
        let end = span.end().line;
        let source = self
            .lines
            .get(start..end)
            .map(|lines| lines.join("\n"))
            .expect("3e180abf");
        let relevant = if signature_only {
            source
                .split_once('{')
                .map_or(source.as_str(), |(signature, _body)| signature)
        } else {
            source.as_str()
        };
        self.entries
            .push(relevant.split_whitespace().collect::<Vec<&str>>().join(" "));
    }
    fn record_contract_struct_api(&mut self, item: &syn::ItemStruct) {
        let Some(attribute) = item
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("contract_struct_api"))
        else {
            return;
        };
        let mut constructor = false;
        let mut into_parts = false;
        attribute
            .parse_nested_meta(|metadata| {
                if metadata.path.is_ident("new") {
                    constructor = true;
                }
                if metadata.path.is_ident("into_parts") {
                    into_parts = true;
                }
                Ok(())
            })
            .expect("d932a5f1");
        let syn::Fields::Named(fields) = &item.fields else {
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
                .join(", ");
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
                    field_attribute.path().is_ident("contract_struct_api")
                })
                .for_each(|field_attribute| {
                    field_attribute
                        .parse_nested_meta(|metadata| {
                            let signature = if metadata.path.is_ident("borrow") {
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> &{field_type}"
                                )
                            } else if metadata.path.is_ident("copy") {
                                format!(
                                    "#[must_use] pub const fn {identifier}(self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident("copy_ref") {
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident("into") {
                                format!(
                                    "#[must_use] pub fn into_{identifier}(self) -> {field_type}"
                                )
                            } else if metadata.path.is_ident("option_borrow") {
                                let inner_type = field_type
                                    .strip_prefix("Option<")
                                    .and_then(|value| value.strip_suffix('>'))
                                    .expect("9ba9415c");
                                format!(
                                    "#[must_use] pub const fn {identifier}(&self) -> Option<&{inner_type}>"
                                )
                            } else if metadata.path.is_ident("slice") {
                                let parsed_element_type =
                                    metadata.value()?.parse::<syn::Type>()?;
                                let wrapped_element_type = self
                                    .source(syn::spanned::Spanned::span(&parsed_element_type));
                                let element_type = wrapped_element_type.as_ref();
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
                        .expect("206adbf7");
                });
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for PublicApiVisitor {
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_impl_item_fn(self, i);
    }
    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_item_const(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), false);
            if i.attrs.iter().any(|attribute| {
                super::derive_attr_has_terminal(
                    super::types::SynAttributeRef::from(attribute),
                    super::types::SourceTextRef::from("UnitEnumIndex"),
                )
                .get()
            }) {
                self.entries.push(format!(
                    "pub const COUNT: usize = {}usize;",
                    i.variants.len()
                ));
                self.entries.push(String::from(
                    "#[must_use] pub const fn index(self) -> usize",
                ));
            }
        }
        syn::visit::visit_item_enum(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), true);
        }
        syn::visit::visit_item_static(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), false);
            self.record_contract_struct_api(i);
        }
        syn::visit::visit_item_struct(self, i);
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        if matches!(i.vis, syn::Visibility::Public(_)) {
            self.record(syn::spanned::Spanned::span(i), false);
        }
        syn::visit::visit_item_trait(self, i);
    }
}

#[derive(Default)]
struct StructErrorVisitor {
    identifiers: super::types::SourceTextList,
}
impl<'ast> syn::visit::Visit<'ast> for StructErrorVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if i.attrs.iter().any(|attribute| {
            super::derive_attr_has_terminal(
                super::types::SynAttributeRef::from(attribute),
                super::types::SourceTextRef::from("Error"),
            )
            .get()
        }) {
            self.identifiers.push(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}

#[derive(Default)]
struct LoopAllocationVisitor {
    depth: super::types::AnalyzerCount,
    entries: super::types::DiagnosticMsgs,
}
impl LoopAllocationVisitor {
    fn record(&mut self, operation: super::types::SourceTextRef<'_>) {
        if self.depth.get() != 0usize {
            self.entries.push(operation.as_ref().to_owned());
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for LoopAllocationVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = i.func.as_ref() {
            let path = super::path_to_string(super::types::SynPathRef::from(&function.path));
            if [
                "Box::new",
                "String::from",
                "String::new",
                "String::with_capacity",
                "Vec::new",
                "Vec::with_capacity",
                "std::boxed::Box::new",
                "std::string::String::from",
                "std::string::String::new",
                "std::string::String::with_capacity",
                "std::vec::Vec::new",
                "std::vec::Vec::with_capacity",
            ]
            .contains(&path.as_ref())
            {
                self.record(super::types::SourceTextRef::from(path.as_ref()));
            }
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        self.depth.saturating_inc();
        syn::visit::visit_expr_loop(self, i);
        self.depth.saturating_dec();
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if matches!(
            i.method.to_string().as_str(),
            "clone" | "collect" | "to_owned" | "to_string"
        ) {
            self.record(super::types::SourceTextRef::from(
                i.method.to_string().as_str(),
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        self.depth.saturating_inc();
        syn::visit::visit_expr_while(self, i);
        self.depth.saturating_dec();
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if let Some(segment) = i.path.segments.last()
            && matches!(segment.ident.to_string().as_str(), "format" | "vec")
        {
            let operation = segment.ident.to_string();
            self.record(super::types::SourceTextRef::from(operation.as_str()));
        }
        syn::visit::visit_macro(self, i);
    }
}

#[test]
fn lock_guards_are_not_held_across_await() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    LockAcrossAwaitVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "ce73f4a1 {violations:#?}");
    });
}

#[test]
fn allocations_inside_loops_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "../file_storage/src/lib.rs:clone",
            (
                1usize,
                "multipart chunk assembly must retain owned buffers until the completed file is committed",
            ),
        ),
        (
            "../frontend_contract_validation/src/json_snapshot.rs:String::from",
            (
                1usize,
                "the bounded JSON parser materializes one owned map key per parsed object field",
            ),
        ),
        (
            "../frontend_contract_validation/src/openapi_validation.rs:to_owned",
            (
                1usize,
                "OpenAPI validation records independently owned operation identifiers",
            ),
        ),
        (
            "../frontend_contract_macros/src/lib.rs:to_string",
            (
                1usize,
                "compile-time route generation materializes variant identifiers outside runtime hot paths",
            ),
        ),
        (
            "../macro_clippy_check_common/src/lib.rs:String::from",
            (
                1usize,
                "compile-time lint inspection owns diagnostic source fragments",
            ),
        ),
        (
            "../server_runtime_http/src/lib.rs:to_string",
            (
                2usize,
                "bounded request parsing materializes validated protocol values that outlive input buffers",
            ),
        ),
        (
            "../str_constants_macros/src/lib.rs:collect",
            (
                1usize,
                "compile-time constant generation collects tokens outside runtime hot paths",
            ),
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
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
                        .and_modify(|count| *count = count.saturating_add(1usize))
                        .or_insert(1usize);
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
fn struct_error_exceptions_match_reviewed_snapshot() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut entries = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    StructErrorVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .identifiers
                    .into_iter()
                    .map(move |identifier| format!("{path}:{identifier}"))
            })
            .collect::<Vec<String>>();
        entries.sort();
        let mut current_snapshot = String::from(
            "# GENERATED REVIEWED SINGLE-CASE AND TRANSPARENT ERROR STRUCTS; DO NOT EDIT\n",
        );
        entries.into_iter().for_each(|entry| {
            current_snapshot.push_str(entry.as_str());
            current_snapshot.push('\n');
        });
        let snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(str_constants::STRUCT_ERROR_SNAPSHOT_PATH);
        if std::env::var_os(str_constants::UPDATE_CODE_STYLE_SNAPSHOTS).is_some() {
            std::fs::write(snapshot_path.as_path(), current_snapshot.as_bytes()).expect("65e1d4f0");
        }
        let expected_snapshot = std::fs::read_to_string(snapshot_path).expect("ba047d32");
        assert_eq!(
            current_snapshot, expected_snapshot,
            "731ffc35 struct error inventory changed"
        );
    });
}

#[test]
fn contract_public_api_matches_reviewed_snapshot() {
    let reviewed = [
        ("common_routes/src", "common route contract"),
        ("frontend_contract/src", "generic frontend route contract"),
        ("server_admin_contract/src", "administrator wire contract"),
    ];
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut current_snapshot =
            String::from("# GENERATED CONTRACT PUBLIC API SNAPSHOT; DO NOT EDIT\n");
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
                    let visitor = super::visit_syn_file(
                        super::types::SynFileRef::from(source_file.ast().as_ref()),
                        PublicApiVisitor {
                            entries: super::types::SourceTextList::default(),
                            lines: super::types::SourceTextList::from(
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
            .join(str_constants::CONTRACT_PUBLIC_API_SNAPSHOT_PATH);
        if std::env::var_os(str_constants::UPDATE_CODE_STYLE_SNAPSHOTS).is_some()
            || std::env::var_os(str_constants::UPDATE_CONTRACT_PUBLIC_API_SNAPSHOT).is_some()
        {
            std::fs::write(snapshot_path.as_path(), current_snapshot.as_bytes()).expect("e2c6b190");
        }
        let expected_snapshot = std::fs::read_to_string(snapshot_path).expect("fd9130e7");
        assert_eq!(
            current_snapshot, expected_snapshot,
            "505a0cf7 contract public API snapshot changed"
        );
    });
}

#[test]
fn arc_lock_and_trait_object_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "location_lib/location/src/lib.rs",
            (
                0usize,
                0usize,
                3usize,
                "location formatting accepts heterogeneous display values",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_to_err_string_token_stream.rs",
            (
                0,
                0,
                4,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_pub_type_alias_token_stream.rs",
            (
                0,
                0,
                2,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_if_write_is_err_token_stream.rs",
            (
                0,
                0,
                2,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_try_from_token_stream.rs",
            (
                0,
                0,
                4,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_default_token_stream.rs",
            (
                0,
                0,
                2,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_from_token_stream.rs",
            (
                0,
                0,
                3,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/src/generate_new_or_try_new.rs",
            (
                0,
                0,
                70,
                "the generator composes heterogeneous token fragments without exposing generics",
            ),
        ),
        (
            "macros_helpers/src/pagination_start_end_initialization_token_stream.rs",
            (
                0,
                0,
                1,
                "token generation accepts a heterogeneous ToTokens input",
            ),
        ),
        (
            "macros_helpers/src/generate_impl_display_token_stream.rs",
            (
                0,
                0,
                4,
                "token generation accepts heterogeneous ToTokens inputs",
            ),
        ),
        (
            "macros_helpers/generate_derive_token_stream_builder/src/lib.rs",
            (
                0,
                0,
                5,
                "the derive builder composes heterogeneous token fragments",
            ),
        ),
        (
            "optml/src/lib.rs",
            (
                0,
                0,
                1,
                "the proc-macro forwards a heterogeneous ToTokens input",
            ),
        ),
        (
            "naming/naming_macros/src/lib.rs",
            (
                0,
                0,
                10,
                "naming generators format heterogeneous token values",
            ),
        ),
        (
            "common_routes/src/lib.rs",
            (
                4,
                0,
                3,
                "route state is shared across threads behind its parameter trait",
            ),
        ),
        (
            "server/src/main.rs",
            (
                3,
                0,
                1,
                "the server shares application state across worker threads",
            ),
        ),
        (
            "server_runtime_http/src/health.rs",
            (1, 0, 0, "health state is shared across request tasks"),
        ),
        (
            "server_runtime_http/src/limits.rs",
            (1, 0, 0, "the semaphore is shared across request tasks"),
        ),
        (
            "server_runtime_http/src/lib.rs",
            (
                2,
                0,
                6,
                "runtime middleware shares state and erases heterogeneous service errors",
            ),
        ),
        (
            "server_runtime_core/src/resource_budget.rs",
            (
                1,
                0,
                0,
                "the resource budget semaphore is shared across tasks",
            ),
        ),
        (
            "server_runtime_core/src/single_flight.rs",
            (
                1,
                1,
                0,
                "single-flight ownership requires shared synchronized state",
            ),
        ),
        (
            "server_runtime_http/src/bounded_read.rs",
            (1, 0, 0, "the read limiter is shared across request tasks"),
        ),
        (
            "server_runtime_core/src/history.rs",
            (
                1,
                1,
                0,
                "run history is shared and asynchronously synchronized",
            ),
        ),
        (
            "server_runtime_core/src/lease_registry.rs",
            (
                1,
                1,
                0,
                "lease state is shared and asynchronously synchronized",
            ),
        ),
        (
            "server_runtime_http/src/metrics_layer.rs",
            (
                1,
                1,
                1,
                "the bounded metrics cache is shared across request threads",
            ),
        ),
        (
            "pg_crud/where_filters/src/lib.rs",
            (
                0,
                0,
                4,
                "query fragments use dynamic dispatch over heterogeneous SQL parts",
            ),
        ),
        (
            "pg_crud/where_filters/generate_where_filters_src/src/source.rs",
            (
                0,
                0,
                31,
                "the generator composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/filters.rs",
            (
                0,
                0,
                2,
                "filter generation accepts heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/lib.rs",
            (
                0,
                0,
                96,
                "CRUD generation composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/pg_type_test_cases.rs",
            (
                0,
                0,
                51,
                "generated fixtures compose heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_macros_common/src/token_stream_helpers.rs",
            (
                0,
                0,
                11,
                "token helpers accept heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/query_fragment.rs",
            (
                0,
                0,
                1,
                "query fragments require heterogeneous SQL part dispatch",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/lib.rs",
            (
                0,
                0,
                11,
                "CRUD contracts operate on heterogeneous query parts",
            ),
        ),
        (
            "pg_crud/pg_table/generate_pg_table_src/src/source.rs",
            (
                0,
                0,
                179,
                "table generation composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_types/generate_pg_types_src/src/source.rs",
            (
                0,
                0,
                177,
                "type generation composes heterogeneous token fragments",
            ),
        ),
        (
            "pg_crud/pg_types/pg_types_common/src/lib.rs",
            (0, 0, 1, "PostgreSQL query parts use dynamic dispatch"),
        ),
        (
            "workspace_test_runner/src/main.rs",
            (0, 0, 1, "the runner prints heterogeneous command errors"),
        ),
        (
            "server_admin/src/generated_tables.rs",
            (1, 0, 1, "generated handlers share erased application state"),
        ),
        (
            "server_admin/src/lib.rs",
            (
                1,
                0,
                0,
                "administrator state is shared across request tasks",
            ),
        ),
        (
            "server_admin/src/generated_auth.rs",
            (
                0,
                0,
                1,
                "generated authentication accepts heterogeneous service implementations",
            ),
        ),
        (
            "server_admin/src/auth.rs",
            (
                1,
                0,
                0,
                "authentication state is shared across request tasks",
            ),
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        let mut violations = Vec::new();
        snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .for_each(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    SharedDispatchVisitor::default(),
                );
                let observed = (
                    visitor.arc_types.get(),
                    visitor.lock_types.get(),
                    visitor.trait_objects.get(),
                );
                if observed == (0usize, 0usize, 0usize) {
                    return;
                }
                let path = source_file.path().as_ref().display().to_string();
                let reviewed_entry =
                    reviewed
                        .iter()
                        .find(|(suffix, (_arc, _lock, _traits, reason))| {
                            path.ends_with(**suffix) && !reason.is_empty()
                        });
                match reviewed_entry {
                    Some((suffix, (arc, lock, traits, _reason)))
                        if observed == (*arc, *lock, *traits) =>
                    {
                        let _inserted = matched.insert((*suffix).to_owned());
                    }
                    Some((suffix, expected)) => violations.push(format!(
                        "{path}: shared-dispatch inventory changed for {suffix}: expected={:?}, observed={observed:?}",
                        (expected.0, expected.1, expected.2)
                    )),
                    None => violations.push(format!(
                        "{path}: unreviewed Arc/lock/trait-object usage {observed:?}"
                    )),
                }
            });
        if matched.len() != reviewed.len() {
            violations.push(format!(
                "stale Arc/lock/trait-object inventory: matched={matched:#?}"
            ));
        }
        assert!(violations.is_empty(), "66b91e7a {violations:#?}");
    });
}

#[test]
fn ignored_map_err_bindings_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "external_service_emulators/src/lib.rs",
            (
                1usize,
                "the emulator maps channel closure to its domain error",
            ),
        ),
        (
            "route_validators/src/hdr_val.rs",
            (
                2usize,
                "header parse details are intentionally mapped to validation errors",
            ),
        ),
        (
            "macros_helpers/src/test_database.rs",
            (
                1usize,
                "the test database helper maps setup failure to its fixture error",
            ),
        ),
        (
            "macros_helpers/src/write_string_into_file.rs",
            (
                1usize,
                "the file helper maps conversion failure to its domain error",
            ),
        ),
        (
            "notification_service/src/main.rs",
            (
                2usize,
                "service bootstrap classifies configuration failures",
            ),
        ),
        (
            "file_storage/src/lib.rs",
            (
                1usize,
                "storage input failure is classified at the boundary",
            ),
        ),
        (
            "common_routes/src/lib.rs",
            (
                1usize,
                "health component capacity maps to the established public contract error",
            ),
        ),
        (
            "server_runtime_http/src/outbound_url.rs",
            (
                1usize,
                "URL parse details are intentionally hidden by the domain error",
            ),
        ),
        (
            "server_runtime_http/src/wire_token.rs",
            (
                1usize,
                "wire token part failures map to a stable public category",
            ),
        ),
        (
            "server_runtime_http/src/origin.rs",
            (1usize, "origin parsing maps to a stable validation error"),
        ),
        (
            "server_runtime_http/src/secure_cookie.rs",
            (1usize, "cookie header details are intentionally redacted"),
        ),
        (
            "server_runtime_http/src/multipart.rs",
            (
                1usize,
                "multipart path validation exposes a stable domain error",
            ),
        ),
        (
            "server_runtime_http/src/lib.rs",
            (
                1usize,
                "timeout details map to the public shutdown timeout variant",
            ),
        ),
        (
            "server_runtime_http/src/bounded_read.rs",
            (1usize, "closed limiter state maps to a stable read error"),
        ),
        (
            "server_runtime_http/src/child_process.rs",
            (
                1usize,
                "elapsed timeout details map to the child timeout variant",
            ),
        ),
        (
            "server_runtime_http/src/http_header_policy.rs",
            (
                3usize,
                "header construction errors are intentionally classified",
            ),
        ),
        (
            "server_runtime_core/src/exclusive_run.rs",
            (1usize, "the atomic compare failure maps to already active"),
        ),
        (
            "pg_crud/pg_crud_common/src/read_query_plan.rs",
            (
                2usize,
                "query plan validation maps to stable contract errors",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/cursor.rs",
            (
                9usize,
                "cursor parsing maps low-level failures to wire categories",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/bounded_btree_map.rs",
            (
                1usize,
                "the compatibility wrapper maps the shared capacity error to its existing public error",
            ),
        ),
        (
            "pg_crud/where_filters/src/lib.rs",
            (
                1usize,
                "the exact-length compatibility wrapper preserves its location-aware public error",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/date_sql_filter.rs",
            (
                2usize,
                "date filter parsing maps to contract validation errors",
            ),
        ),
        (
            "pg_crud/pg_crud_common/src/advisory_lock.rs",
            (
                3usize,
                "advisory lock conversion maps to its bounded domain error",
            ),
        ),
        (
            "server_admin_contract/src/lib.rs",
            (
                1usize,
                "administrator collections preserve their stable public capacity error",
            ),
        ),
        (
            "pg_crud/pg_table/src/lib.rs",
            (
                2usize,
                "table validation maps generated failures to a public category",
            ),
        ),
        (
            "config_lib/src/lib.rs",
            (4usize, "configuration parsing exposes stable field errors"),
        ),
        (
            "workspace_test_runner/src/main.rs",
            (1usize, "runner input conversion maps to a command error"),
        ),
        (
            "workspace_test_runner/src/execution.rs",
            (1usize, "summary initialization maps to the runner error"),
        ),
        (
            "frontend_contract_validation/src/json_snapshot.rs",
            (
                2usize,
                "serialization details map to snapshot contract errors",
            ),
        ),
        (
            "workspace_scaffold/src/main.rs",
            (9usize, "catalog parsing maps to stable scaffold errors"),
        ),
        (
            "newtype/src/lib.rs",
            (1usize, "invalid derive input maps to the macro diagnostic"),
        ),
        (
            "server_admin_core/src/lib.rs",
            (
                4usize,
                "domain conversion failures map to administrator validation errors",
            ),
        ),
        (
            "server_admin/src/generated_tables.rs",
            (
                1usize,
                "generated table conformance maps to its public error",
            ),
        ),
        (
            "server_admin/src/repository/users.rs",
            (
                12usize,
                "repository row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/settings.rs",
            (
                8usize,
                "settings row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/data_tables.rs",
            (
                21usize,
                "data table parsing maps to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/audit.rs",
            (
                10usize,
                "audit row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/rate_limits.rs",
            (
                4usize,
                "rate-limit row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/sessions.rs",
            (
                5usize,
                "session row conversions map to typed repository errors",
            ),
        ),
        (
            "server_admin/src/repository/cleanup.rs",
            (
                1usize,
                "cleanup conversion maps to a typed repository error",
            ),
        ),
        (
            "server_admin/src/auth/audit.rs",
            (3usize, "audit request validation maps to stable API errors"),
        ),
        (
            "server_admin/src/auth/html.rs",
            (
                9usize,
                "HTML form parsing maps details to stable API errors",
            ),
        ),
        (
            "server_admin/src/auth/session.rs",
            (1usize, "system clock failure maps to the session category"),
        ),
        (
            "server_admin/src/auth/handlers.rs",
            (
                15usize,
                "handler input failures map to stable API categories",
            ),
        ),
        (
            "server_admin/src/auth.rs",
            (
                12usize,
                "authentication failures map to stable and redacted API categories",
            ),
        ),
        (
            "server_admin_frontend/src/app/http/fetch.rs",
            (
                5usize,
                "browser fetch failures map to serializable UI error categories",
            ),
        ),
        (
            "server_admin_frontend/src/app/http/mutation.rs",
            (
                9usize,
                "browser mutation failures map to serializable UI error categories",
            ),
        ),
        (
            "server_admin_frontend/src/app/http/url.rs",
            (
                2usize,
                "browser URL failures map to a stable UI query error",
            ),
        ),
        (
            "server_admin_frontend/src/app/loader.rs",
            (
                1usize,
                "browser page loading failures map to a stable query error",
            ),
        ),
        (
            "server_admin_frontend/src/app/query/location.rs",
            (
                11usize,
                "browser query parsing failures map to stable UI error categories",
            ),
        ),
        (
            "server_admin_frontend/src/app/query/page.rs",
            (
                1usize,
                "browser page-location failures map to a stable UI fetch error",
            ),
        ),
    ]);
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut matched = std::collections::BTreeSet::new();
        let mut violations = Vec::new();
        snapshot.rs_files().iter().for_each(|source_file| {
            let visitor = super::visit_syn_file(
                super::types::SynFileRef::from(source_file.ast().as_ref()),
                IgnoredMapErrBindingVisitor::default(),
            );
            if visitor.entries.is_empty() {
                return;
            }
            let path = source_file.path().as_ref().display().to_string();
            let reviewed_entry = reviewed.iter().find(|(suffix, (_count, reason))| {
                path.ends_with(**suffix) && !reason.is_empty()
            });
            match reviewed_entry {
                Some((suffix, (count, _reason)))
                    if *count == visitor.entries.len() => {
                    let _inserted = matched.insert((*suffix).to_owned());
                }
                Some((suffix, (count, _reason))) => {
                    violations.push(format!(
                        "{path}: ignored map_err inventory changed for {suffix}: expected count={count}; observed count={}",
                        visitor.entries.len()
                    ));
                }
                None => violations.push(format!(
                    "{path}: unreviewed ignored map_err bindings: count={}",
                    visitor.entries.len()
                )),
            }
        });
        if matched.len() != reviewed.len() {
            violations.push(format!(
                "stale ignored map_err inventory: matched={matched:#?}"
            ));
        }
        assert!(violations.is_empty(), "bb0dbc1f {violations:#?}");
    });
}

#[test]
fn raw_vec_tuple_wrappers_match_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "../bounded_types/src/lib.rs:BoundedVec",
            "the shared bounded vector is the reviewed owner of raw Vec storage",
        ),
        (
            "../common_routes/src/lib.rs:HealthComponents",
            "infallible fixed-size array conversions require raw storage; Vec conversion and serde delegate to bounded_types",
        ),
        (
            "../development_data_bootstrap/src/lib.rs:DevelopmentIdentitySpecs",
            "the bootstrap catalog owns validated development identities assembled in process",
        ),
        (
            "../frontend_contract_macros/src/lib.rs:SynRouteRegistrySchemas",
            "the proc-macro compiler owns a compile-time syntax collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/batch_validation.rs:BatchInvalidItems",
            "batch validation owns its bounded invalid-item accumulator",
        ),
        (
            "../pg_crud/pg_crud_common/src/bounded_unique_vec.rs:BoundedUniqueVec",
            "the compatibility collection enforces both length and uniqueness invariants",
        ),
        (
            "../pg_crud/pg_crud_common/src/bounded_vec.rs:BoundedVec",
            "the compatibility wrapper delegates validation and serde to bounded_types",
        ),
        (
            "../pg_crud/pg_crud_common/src/cardinality.rs:DuplicateCandidates",
            "cardinality analysis owns an internal duplicate candidate collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/date_sql_filter.rs:ChronoUtcDateTimes",
            "the SQL bind plan owns an operational collection assembled from validated filters",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbColumnContractSnapshots",
            "schema conformance owns an internal deterministic snapshot collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbColumnSnapshots",
            "schema conformance owns an internal deterministic snapshot collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbColumnSpecs",
            "schema conformance owns an internal static specification collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbKeyContractSnapshots",
            "schema conformance owns an internal deterministic snapshot collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbKeySpecs",
            "schema conformance owns an internal static specification collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbObjectSnapshots",
            "schema conformance owns an internal deterministic snapshot collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbObjectSpecs",
            "schema conformance owns an internal static specification collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbDefaultSpecs",
            "schema conformance owns an internal static specification collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbSchemaTexts",
            "schema conformance owns an internal deterministic text collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/db_schema_conformance.rs:DbStaticSchemaTexts",
            "schema conformance owns an internal static text collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/filter_bind_plan.rs:FilterBindPlan",
            "the query planner owns an internal ordered bind collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/lib.rs:AllEnumVariants",
            "the enum helper owns a compile-time-complete variant collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/lib.rs:NotEmptyUniqueVec",
            "the collection enforces non-empty and uniqueness invariants together",
        ),
        (
            "../pg_crud/pg_crud_common/src/list_total.rs:ListItems",
            "list-total planning owns an operational result collection",
        ),
        (
            "../pg_crud/pg_crud_common/src/operational_invariants.rs:PgSqlIdentifiers",
            "the invariant checker owns validated SQL identifier wrappers",
        ),
        (
            "../pg_crud/pg_crud_common/src/order_preserving_deduplication.rs:OrderPreservingValues",
            "the deduplication helper owns its ordered working collection",
        ),
        (
            "../pg_crud/pg_crud_macros_common/src/lib.rs:ParseTokenStreamStrings",
            "the proc-macro compiler owns a compile-time token rendering collection",
        ),
        (
            "../pg_crud/pg_crud_macros_common/src/lib.rs:ProcMacro2GeneratedRustTokenStreamVec",
            "the proc-macro compiler owns generated token streams",
        ),
        (
            "../pg_crud/pg_table/generate_pg_table_src/src/source.rs:TableTestNames",
            "the source generator owns compile-time generated test names",
        ),
        (
            "../pg_crud/pg_types/generate_pg_types_src/src/source.rs:GeneratePgTypeRecords",
            "the source generator owns compile-time catalog records",
        ),
        (
            "../pg_crud/pg_types/generate_pg_types_src/src/source.rs:GeneratePgTypes",
            "the source generator owns compile-time generated types",
        ),
        (
            "../pg_crud/where_filters/src/lib.rs:BoundedVec",
            "the exact-length compatibility wrapper delegates validation and serde to bounded_types",
        ),
        (
            "../pg_crud/where_filters/src/lib.rs:PgTypeNotEmptyUniqueVec",
            "the generated filter collection enforces non-empty and uniqueness invariants",
        ),
        (
            "../server_admin/src/auth.rs:JsonwebtokenAdminDecodingKeys",
            "validated configuration determines the runtime key collection",
        ),
        (
            "../server_runtime_http/src/bounded_read.rs:BoundedBytes",
            "the byte limit is supplied dynamically and enforced by the bounded reader",
        ),
        (
            "../server_runtime_http/src/cors.rs:HttpCorsAllowOriginHeaderValues",
            "the parser enforces its byte and item limits before construction",
        ),
        (
            "../server_runtime_http/src/multipart.rs:MultipartBytesParts",
            "the multipart budget is supplied dynamically and enforced while parsing",
        ),
        (
            "../server_runtime_http/src/multipart.rs:MultipartTextParts",
            "the multipart budget is supplied dynamically and enforced while parsing",
        ),
        (
            "../str_constants_macros/src/lib.rs:ConstantParts",
            "the proc-macro compiler owns compile-time constant fragments",
        ),
        (
            "../str_constants_macros/src/lib.rs:Constants",
            "the proc-macro compiler owns compile-time constant declarations",
        ),
        (
            "../str_constants_macros/src/lib.rs:Fragments",
            "the proc-macro compiler owns compile-time string fragments",
        ),
        (
            "../workspace_macro_helpers/src/lib.rs:ProcMacro2MacroTokens",
            "the shared proc-macro helper owns compile-time tokens",
        ),
        (
            "../workspace_macro_helpers/src/lib.rs:ProcMacro2TopLevelCommaParts",
            "the shared proc-macro helper owns compile-time token parts",
        ),
    ]);
    reviewed
        .values()
        .for_each(|reason| assert!(!reason.is_empty(), "f8c9471a"));
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    RawVecTupleWrapperVisitor::default(),
                );
                let path = source_file.path().as_ref().display().to_string();
                visitor
                    .identifiers
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
fn raw_vec_tuple_wrapper_visitor_detects_qualified_and_nested_types() {
    let file: syn::File = syn::parse_quote! {
        struct Qualified(std::vec::Vec<u8>);
        struct Named {
            values: Vec<u8>,
        }
        mod nested {
            struct Nested(Vec<u8>);
        }
    };
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&file),
        RawVecTupleWrapperVisitor::default(),
    );
    assert_eq!(visitor.identifiers.len(), 2usize);
}

#[test]
fn usize_max_usage_matches_reviewed_inventory() {
    let reviewed = std::collections::BTreeMap::from([
        (
            "../bounded_types/src/lib.rs",
            (
                4usize,
                "the shared type provides its explicitly unbounded specialization, overflow boundary, and schema handling",
            ),
        ),
        (
            "../file_storage/src/lib.rs",
            (
                1usize,
                "the process-owned path catalog is assembled from already bounded storage paths",
            ),
        ),
        (
            "../frontend_contract/src/lib.rs",
            (
                3usize,
                "compile-time generated frontend catalogs have no wire-controlled cardinality",
            ),
        ),
        (
            "../frontend_contract/src/route.rs",
            (
                3usize,
                "compile-time generated route catalogs have no wire-controlled cardinality",
            ),
        ),
        (
            "../frontend_contract/src/route_coverage.rs",
            (
                1usize,
                "the compile-time route test category catalog has no wire-controlled cardinality",
            ),
        ),
        (
            "../frontend_contract_validation/src/route_contract_validation.rs",
            (
                1usize,
                "validation mismatches are bounded by the already finite route catalog",
            ),
        ),
        (
            "../initialize_environment_files/src/main.rs",
            (
                4usize,
                "the local workspace initializer catalogs are bounded by files in the checked-out workspace",
            ),
        ),
        (
            "../pg_crud/pg_table/generate_pg_table_src/src/source.rs",
            (
                3usize,
                "the proc-macro source generator operates on finite compile-time schema declarations",
            ),
        ),
        (
            "../prepare_postgresql_databases/src/lib.rs",
            (
                2usize,
                "the local process command catalog is derived from finite workspace configuration",
            ),
        ),
        (
            "../server_runtime_core/src/lease_registry.rs",
            (
                3usize,
                "the runtime-configured lease maximum is enforced at mutation sites",
            ),
        ),
        (
            "../server_runtime_core/src/single_flight.rs",
            (
                1usize,
                "the runtime-configured single-flight maximum is enforced before insertion",
            ),
        ),
        (
            "../server_runtime_http/src/child_process.rs",
            (
                3usize,
                "runtime-configured process and diagnostic limits are enforced while collecting",
            ),
        ),
        (
            "../workspace_scaffold/src/main.rs",
            (
                1usize,
                "the local service catalog is bounded by the checked-out workspace",
            ),
        ),
        (
            "../workspace_test_runner/src/execution.rs",
            (
                2usize,
                "runner command text is derived from the finite workspace test plan",
            ),
        ),
    ]);
    reviewed
        .values()
        .for_each(|(_count, reason)| assert!(!reason.is_empty(), "cfc5175f"));
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let observed = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| {
                !source_file
                    .path()
                    .as_ref()
                    .components()
                    .any(|component| component.as_os_str() == "tests")
            })
            .filter_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    UsizeMaxExprVisitor::default(),
                );
                let count = visitor.count.get();
                (count != 0usize)
                    .then(|| (source_file.path().as_ref().display().to_string(), count))
            })
            .collect::<std::collections::BTreeMap<String, usize>>();
        let expected = reviewed
            .iter()
            .map(|(path, (count, _reason))| ((*path).to_owned(), *count))
            .collect::<std::collections::BTreeMap<String, usize>>();
        assert_eq!(observed, expected, "cfc5175f usize::MAX inventory changed");
    });
}

#[test]
fn usize_max_expression_visitor_skips_test_modules() {
    let file: syn::File = syn::parse_quote! {
        const PRODUCTION_MAXIMUM: usize = usize::MAX;
        #[cfg(test)]
        mod tests {
            const TEST_MAXIMUM: usize = usize::MAX;
        }
    };
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&file),
        UsizeMaxExprVisitor::default(),
    );
    assert_eq!(visitor.count.get(), 1usize);
}

#[test]
fn select_sites_match_reviewed_cancellation_inventory() {
    let reviewed = [
        (
            "server/src/main.rs",
            1usize,
            "the shutdown signal races two cancellation-safe signal receivers",
        ),
        (
            "server_runtime_http/src/lib.rs",
            1usize,
            "the pinned server future is resumed after the shutdown notification branch",
        ),
        (
            "server_runtime_http/src/lifecycle.rs",
            1usize,
            "the interval tick and oneshot shutdown receiver are cancellation-safe",
        ),
    ];
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let mut observed = std::collections::BTreeMap::<String, usize>::new();
        let mut violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    SelectMacroVisitor::default(),
                );
                if visitor.count.get() != 0usize {
                    let _previous = observed.insert(
                        source_file.path().as_ref().display().to_string(),
                        visitor.count.get(),
                    );
                }
                visitor.unsafe_operations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
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
fn select_policy_rejects_cancellation_sensitive_operations() {
    let ast = syn::parse_file(
        "async fn invalid(reader: &mut Reader, sender: &Sender) {
            tokio::select! {
                value = reader.read_exact(&mut [0u8; 8]) => drop(value),
                value = sender.send(Message) => drop(value),
            }
        }",
    )
    .expect("714c620f");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        SelectMacroVisitor::default(),
    );
    assert_eq!(visitor.unsafe_operations.len(), 2usize, "c4267f0a");
}

#[test]
fn architectural_boundaries_reject_upward_dependencies() {
    let boundaries = [
        (
            "frontend_contract",
            "the generic frontend contract must not depend on service, application, database, or runtime crates",
        ),
        (
            "server_admin_contract",
            "the administrator contract may depend downward on generic contracts and values, but not on runtime implementations",
        ),
        (
            "server_observability",
            "observability must not depend on HTTP, application, or route crates",
        ),
        (
            "server_runtime_core",
            "the runtime core must not depend on HTTP, application, or route crates",
        ),
        (
            "server_runtime_http",
            "the HTTP runtime must not depend on application or route crates",
        ),
    ];
    super::snapshot::with_codebase_snapshot(|snapshot| {
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
                .expect("010e6a3f");
            let observed = package
                .dependencies
                .iter()
                .filter(|dependency| workspace_names.as_ref().contains(dependency.name.as_str()))
                .filter(|dependency| match *package_name {
                    "frontend_contract" => {
                        dependency.name == "app_state"
                            || dependency.name.starts_with("notification_service")
                            || dependency.name.starts_with("pg_")
                            || (dependency.name.starts_with("server")
                                && dependency.name != "server_runtime_macros")
                    }
                    "server_admin_contract" => {
                        dependency.name == "app_state"
                            || dependency.name == "server"
                            || dependency.name == "server_admin"
                            || dependency.name == "server_app_state"
                            || dependency.name.ends_with("_runtime")
                            || dependency.name.starts_with("notification_service")
                            || dependency.name.starts_with("pg_")
                    }
                    "server_observability" | "server_runtime_core" | "server_runtime_http" => {
                        dependency.name == "app_state"
                            || dependency.name == "server"
                            || dependency.name.starts_with("server_admin")
                            || dependency.name.starts_with("notification_service")
                            || dependency.name.ends_with("_contract")
                            || dependency.name.starts_with("pg_")
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
fn lock_across_await_policy_requires_explicit_drop() {
    let invalid = syn::parse_file(
        "async fn invalid(lock: &tokio::sync::Mutex<u8>) {
            let guard = lock.lock().await;
            operation().await;
            drop(guard);
        }",
    )
    .expect("b57df6a3");
    let valid = syn::parse_file(
        "async fn valid(lock: &tokio::sync::Mutex<u8>) {
            let guard = lock.lock().await;
            drop(guard);
            operation().await;
        }",
    )
    .expect("a62f1ce9");
    let invalid_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&invalid),
        LockAcrossAwaitVisitor::default(),
    );
    let valid_visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&valid),
        LockAcrossAwaitVisitor::default(),
    );
    assert_eq!(invalid_visitor.violations.len(), 1usize, "bbfce72c");
    assert!(valid_visitor.violations.is_empty(), "4b732bd1");
}

#[test]
fn production_code_does_not_use_explicit_leak_apis() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .filter(|source_file| !source_file.path().as_ref().starts_with("tests"))
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    LeakApiVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "8522eda3 {violations:#?}");
    });
}

#[test]
fn retained_spawn_handles_are_supervised() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    SpawnLifecycleVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "aa16974d {violations:#?}");
    });
}

#[test]
fn spawn_lifecycle_policy_rejects_unconsumed_handles() {
    let ast = syn::parse_file(
        "async fn tasks() {
            let forgotten = tokio::spawn(async {});
            let awaited = tokio::spawn(async {});
            awaited.await;
            let transferred = tokio::spawn(async {});
            supervise(transferred);
        }",
    )
    .expect("834138af");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        SpawnLifecycleVisitor::default(),
    );
    assert_eq!(
        visitor.violations.as_slice(),
        [
            "spawn handle `forgotten` is retained but never awaited, aborted, or transferred to an owner"
        ],
        "a1680c46"
    );
}

#[test]
fn route_path_segments_use_snake_case() {
    super::snapshot::with_codebase_snapshot(|snapshot| {
        let violations = snapshot
            .rs_files()
            .iter()
            .flat_map(|source_file| {
                let visitor = super::visit_syn_file(
                    super::types::SynFileRef::from(source_file.ast().as_ref()),
                    RouteLiteralVisitor::default(),
                );
                visitor.violations.into_iter().map(|violation| {
                    format!("{}: {violation}", source_file.path().as_ref().display())
                })
            })
            .collect::<Vec<String>>();
        assert!(violations.is_empty(), "ebde2ab8 {violations:#?}");
    });
}

#[test]
fn route_path_policy_rejects_kebab_case() {
    let ast = syn::parse_file(
        "#[typed_route(path = \"/admin/swagger_ui/{user_id}\")]
         struct Valid;
         #[typed_route(path = \"/admin/swagger-ui\")]
         struct Invalid;",
    )
    .expect("9aa037dc");
    let visitor = super::visit_syn_file(
        super::types::SynFileRef::from(&ast),
        RouteLiteralVisitor::default(),
    );
    assert_eq!(visitor.violations.len(), 1usize, "d15287e9");
}
