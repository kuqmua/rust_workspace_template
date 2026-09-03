#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct RuntimePanicExpectUnwrapVisitor {
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimePanicExpectUnwrapVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if expr_method_call.method == constants_str::CODE_STYLE_EXPECT_METHOD_NAME {
            self.errors.push(constants_str::EXPECT_CALL.to_owned());
        }
        if expr_method_call.method == constants_str::UNWRAP {
            self.errors.push(constants_str::UNWRAP_CALL.to_owned());
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_macro(&mut self, r#macro: &'ast syn::Macro) {
        let is_panic =
            r#macro.path.segments.last().is_some_and(|segment| {
                segment.ident == constants_str::CODE_STYLE_PANIC_METHOD_NAME
            });
        if is_panic {
            self.errors.push(constants_str::PANIC_CALL.to_owned());
        }
        syn::visit::visit_macro(self, r#macro);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct RuntimeMutexVisitor {
    found_count: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeMutexVisitor {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_type_path(&mut self, type_path: &'ast syn::TypePath) {
        if crate::code_style::path_has_segment(
            crate::types::SynPathRef::from(&type_path.path),
            crate::types::SourceTextRef::from(constants_str::MUTEX),
        )
        .get()
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_type_path(self, type_path);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "runtime analysis keeps declaration order aligned with generated layout or processing flow"
)]
pub(super) struct RuntimeArcVisitor {
    errors: crate::types::DiagnosticMessages,
    allow_arc_value_usage: crate::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeArcVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(expr_call))
            .is_some_and(|path| {
                crate::code_style::path_ends_with(
                    path,
                    crate::types::StaticStrSliceRef::from(
                        [constants_str::ARC, constants_str::NEW].as_slice(),
                    ),
                )
                .get()
            })
            && !self.allow_arc_value_usage.get()
        {
            self.errors.push(
                constants_str::ARC_PATH_NEW_OUTSIDE_APPROVED_CROSS_THREAD_STATE_CONSTRUCTION
                    .to_owned(),
            );
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_item_type(&mut self, item_type: &'ast syn::ItemType) {
        let contains_arc = match item_type.ty.as_ref() {
            syn::Type::Path(path) => crate::code_style::path_has_segment(
                crate::types::SynPathRef::from(&path.path),
                crate::types::SourceTextRef::from(constants_str::ARC),
            )
            .get(),
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
            | _ => false,
        };
        if contains_arc {
            let name = item_type.ident.to_string();
            if !name.contains(constants_str::SHARED) && !name.contains(constants_str::DYNARC) {
                self.errors.push(format!(
                    "Arc type alias `{name}` must be explicitly named as shared cross-thread state"
                ));
            }
        }
        syn::visit::visit_item_type(self, item_type);
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct AsyncBlockingCallVisitor {
    async_fn_depth: crate::types::AnalyzerCount,
    errors: crate::types::DiagnosticMessages,
}
impl<'ast> syn::visit::Visit<'ast> for AsyncBlockingCallVisitor {
    fn visit_expr_async(&mut self, expr_async: &'ast syn::ExprAsync) {
        self.async_fn_depth.saturating_inc();
        syn::visit::visit_expr_async(self, expr_async);
        self.async_fn_depth.saturating_dec();
    }
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if self.async_fn_depth.get() != 0
            && crate::code_style::expr_call_path(crate::types::SynExprCallRef::from(expr_call))
                .is_some_and(|path| {
                    let path_text = crate::code_style::path_to_string(path);
                    crate::code_style::path_ends_with(
                        path,
                        crate::types::StaticStrSliceRef::from(
                            [
                                constants_str::FUTURES,
                                constants_str::EXECUTOR,
                                constants_str::BLOCK_ON,
                            ]
                            .as_slice(),
                        ),
                    )
                    .get()
                        || crate::code_style::path_ends_with(
                            path,
                            crate::types::StaticStrSliceRef::from(
                                [
                                    constants_str::TOKIO,
                                    constants_str::TASK,
                                    constants_str::BLOCK_IN_PLACE,
                                ]
                                .as_slice(),
                            ),
                        )
                        .get()
                        || crate::code_style::path_ends_with(
                            path,
                            crate::types::StaticStrSliceRef::from(
                                [
                                    constants_str::STD,
                                    constants_str::THREAD,
                                    constants_str::SLEEP,
                                ]
                                .as_slice(),
                            ),
                        )
                        .get()
                        || constants_str::BLOCKING_STD_FS_CALLS.contains(&path_text.as_ref())
                        || constants_str::BLOCKING_STD_NET_CALLS.contains(&path_text.as_ref())
                })
        {
            self.errors
                .push(constants_str::BLOCKING_CALL_INSIDE_ASYNC_FUNCTION.to_owned());
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
    fn visit_expr_closure(&mut self, expr_closure: &'ast syn::ExprClosure) {
        let is_async = expr_closure.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_expr_closure(self, expr_closure);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        let method = expr_method_call.method.to_string();
        if self.async_fn_depth.get() != 0
            && matches!(
                method.as_str(),
                constants_str::BLOCK_ON
                    | constants_str::BLOCK_IN_PLACE
                    | constants_str::BLOCKING_RECV
                    | constants_str::BLOCKING_SEND
            )
        {
            self.errors.push(format!(
                ".{}() blocking method call inside async function",
                expr_method_call.method
            ));
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_impl_item_fn(&mut self, impl_item_fn: &'ast syn::ImplItemFn) {
        let is_async = impl_item_fn.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_impl_item_fn(self, impl_item_fn);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if crate::code_style::has_test_only_cfg_attr(crate::types::SynItemRef::from(item)).get() {
            return;
        }
        syn::visit::visit_item(self, item);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let is_async = item_fn.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, item_fn);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_trait_item_fn(&mut self, trait_item_fn: &'ast syn::TraitItemFn) {
        let is_async = trait_item_fn.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_trait_item_fn(self, trait_item_fn);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
}
#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct UnitTestExternalServiceVisitor {
    errors: crate::types::DiagnosticMessages,
    test_depth: crate::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnitTestExternalServiceVisitor {
    fn visit_expr_method_call(&mut self, expr_method_call: &'ast syn::ExprMethodCall) {
        if self.test_depth.get() != 0
            && matches!(
                expr_method_call.method.to_string().as_str(),
                constants_str::CONNECT | constants_str::VALUE_3DFFA238
            )
        {
            self.errors.push(format!(
                "unit tests must not connect to an external service with `.{}()`",
                expr_method_call.method
            ));
        }
        syn::visit::visit_expr_method_call(self, expr_method_call);
    }
    fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
        let path = crate::types::SynPathRef::from(&expr_path.path);
        let path_text = crate::code_style::path_to_string(path);
        let is_external_service_client = [
            [
                constants_str::REQWEST,
                constants_str::CLIENT,
                constants_str::NEW,
            ]
            .as_slice(),
            [
                constants_str::STD,
                constants_str::NET,
                constants_str::TCPSTREAM,
                constants_str::CONNECT,
            ]
            .as_slice(),
            [
                constants_str::STD,
                constants_str::NET,
                constants_str::TCPLISTENER,
                constants_str::BIND,
            ]
            .as_slice(),
            [
                constants_str::STD,
                constants_str::NET,
                constants_str::UDPSOCKET,
                constants_str::BIND,
            ]
            .as_slice(),
            [
                constants_str::TOKIO,
                constants_str::NET,
                constants_str::TCPSTREAM,
                constants_str::CONNECT,
            ]
            .as_slice(),
            [
                constants_str::TOKIO,
                constants_str::NET,
                constants_str::TCPLISTENER,
                constants_str::BIND,
            ]
            .as_slice(),
            [
                constants_str::TOKIO,
                constants_str::NET,
                constants_str::UDPSOCKET,
                constants_str::BIND,
            ]
            .as_slice(),
        ]
        .into_iter()
        .any(|segments| {
            crate::code_style::path_ends_with(path, crate::types::StaticStrSliceRef::from(segments))
                .get()
        }) || [
            constants_str::VALUE_364F9D39,
            constants_str::VALUE_BDB563EC,
            constants_str::VALUE_FE4D84FC,
            constants_str::VALUE_2FCCA7C7,
        ]
        .contains(&path_text.as_ref());
        if self.test_depth.get() != 0 && is_external_service_client {
            self.errors.push(format!(
                "unit tests must not depend on external service client `{}`",
                crate::code_style::path_to_string(crate::types::SynPathRef::from(&expr_path.path))
                    .as_ref()
            ));
        }
        syn::visit::visit_expr_path(self, expr_path);
    }
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if item_fn.attrs.iter().any(|attribute| {
            attribute.path().is_ident(constants_str::VALUE_5F0AF516)
                && matches!(
                    &attribute.meta,
                    syn::Meta::NameValue(name_value)
                        if matches!(
                            &name_value.value,
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(reason),
                                ..
                            }) if !reason.value().trim().is_empty()
                        )
                )
        }) {
            return;
        }
        let is_test = self.test_depth.get() != 0
            || crate::code_style::item_fn_is_unit_test(crate::types::SynItemFnRef::from(item_fn))
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
        let is_test = self.test_depth.get() != 0
            || item_mod.attrs.iter().any(|attr| {
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
