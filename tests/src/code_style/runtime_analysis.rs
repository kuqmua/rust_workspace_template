#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct RuntimePanicExpectUnwrapVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimePanicExpectUnwrapVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == constants_str::CODE_STYLE_EXPECT_METHOD_NAME {
            self.ers.push(constants_str::EXPECT_CALL.to_owned());
        }
        if i.method == constants_str::UNWRAP {
            self.ers.push(constants_str::UNWRAP_CALL.to_owned());
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        if i.path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == constants_str::CODE_STYLE_PANIC_METHOD_NAME)
        {
            self.ers.push(constants_str::PANIC_CALL.to_owned());
        }
        syn::visit::visit_macro(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct RuntimeMutexVisitor {
    pub found_count: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeMutexVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if super::path_has_segment(
            super::types::SynPathRef::from(&i.path),
            super::types::SourceTextRef::from(constants_str::MUTEX),
        )
        .get()
        {
            self.found_count.saturating_inc();
        }
        syn::visit::visit_type_path(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub(super) struct RuntimeArcVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub allow_arc_value_usage: super::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for RuntimeArcVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if super::expr_call_path(super::types::SynExprCallRef::from(i)).is_some_and(|path| {
            super::path_ends_with(
                path,
                super::types::StaticStrSliceRef::from(
                    [constants_str::ARC, constants_str::NEW].as_slice(),
                ),
            )
            .get()
        }) && !self.allow_arc_value_usage.get()
        {
            self.ers.push(
                constants_str::ARC_PATH_NEW_OUTSIDE_APPROVED_CROSS_THREAD_STATE_CONSTRUCTION
                    .to_owned(),
            );
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        if super::type_contains_segment(
            super::types::SynTypeRef::from(&*i.ty),
            super::types::SourceTextRef::from(constants_str::ARC),
        )
        .get()
        {
            let name = i.ident.to_string();
            if !name.contains(constants_str::SHARED) && !name.contains(constants_str::DYNARC) {
                self.ers.push(format!(
                    "Arc type alias `{name}` must be explicitly named as shared cross-thread state"
                ));
            }
        }
        syn::visit::visit_item_type(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AsyncBlockingCallVisitor {
    pub async_fn_depth: super::types::AnalyzerCount,
    pub ers: super::types::DiagnosticMsgs,
}
impl<'ast> syn::visit::Visit<'ast> for AsyncBlockingCallVisitor {
    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        self.async_fn_depth.saturating_inc();
        syn::visit::visit_expr_async(self, i);
        self.async_fn_depth.saturating_dec();
    }
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if self.async_fn_depth.get() != 0
            && super::expr_call_path(super::types::SynExprCallRef::from(i))
                .is_some_and(|path| super::path_is_blocking_async_call(path).get())
        {
            self.ers
                .push(constants_str::BLOCKING_CALL_INSIDE_ASYNC_FUNCTION.to_owned());
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_expr_closure(&mut self, i: &'ast syn::ExprClosure) {
        let is_async = i.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_expr_closure(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if self.async_fn_depth.get() != 0
            && super::method_is_blocking_async_call(super::types::SourceTextRef::from(
                i.method.to_string().as_str(),
            ))
            .get()
        {
            self.ers.push(format!(
                ".{}() blocking method call inside async function",
                i.method
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_impl_item_fn(&mut self, i: &'ast syn::ImplItemFn) {
        let is_async = i.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_impl_item_fn(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let is_async = i.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
    fn visit_trait_item_fn(&mut self, i: &'ast syn::TraitItemFn) {
        let is_async = i.sig.asyncness.is_some();
        if is_async {
            self.async_fn_depth.saturating_inc();
        }
        syn::visit::visit_trait_item_fn(self, i);
        if is_async {
            self.async_fn_depth.saturating_dec();
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct UnitTestExternalServiceVisitor {
    pub ers: super::types::DiagnosticMsgs,
    pub test_depth: super::types::AnalyzerCount,
}
impl<'ast> syn::visit::Visit<'ast> for UnitTestExternalServiceVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if self.test_depth.get() != 0
            && matches!(
                i.method.to_string().as_str(),
                constants_str::CONNECT | constants_str::VALUE_3DFFA238
            )
        {
            self.ers.push(format!(
                "unit tests must not connect to an external service with `.{}()`",
                i.method
            ));
        }
        syn::visit::visit_expr_method_call(self, i);
    }
    fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
        if self.test_depth.get() != 0
            && super::path_is_external_service_client(super::types::SynPathRef::from(&i.path)).get()
        {
            self.ers.push(format!(
                "unit tests must not depend on external service client `{}`",
                super::path_to_string(super::types::SynPathRef::from(&i.path)).as_ref()
            ));
        }
        syn::visit::visit_expr_path(self, i);
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if i.attrs.iter().any(|attribute| {
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
            || super::item_fn_is_unit_test(super::types::SynItemFnRef::from(i)).get();
        if is_test {
            self.test_depth.saturating_inc();
        }
        syn::visit::visit_item_fn(self, i);
        if is_test {
            self.test_depth.saturating_dec();
        }
    }
    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let is_test = self.test_depth.get() != 0
            || i.attrs.iter().any(|attr| {
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
