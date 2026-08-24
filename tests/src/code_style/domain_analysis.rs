#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct StringWrapperNameVisitor {
    pub names: super::types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperNameVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::item_struct_is_single_string_wrapper(super::types::SynItemStructRef::from(i))
            .get()
        {
            let _: bool = self.names.insert(i.ident.to_string());
        }
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct StringWrapperFromVisitor<'names_lt> {
    pub ers: super::types::DiagnosticMsgs,
    pub len_checked_function_names: &'names_lt super::types::StdSourceTextSet,
    pub string_wrapper_names: &'names_lt super::types::StdSourceTextSet,
    pub try_from_string_len_checked_names: super::types::StdSourceTextSet,
    pub try_from_string_names: super::types::StdSourceTextSet,
}
impl StringWrapperFromVisitor<'_> {
    fn check_bounded_string_attr(&mut self, item: super::types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        if !super::item_struct_is_single_string_wrapper(item).get() {
            return;
        }
        let has_derive = item_ref.attrs.iter().any(|attr| {
            super::attr_has_bounded_string_derive(super::types::SynAttributeRef::from(attr)).get()
        });
        let has_max_bound = item_ref.attrs.iter().any(|attr| {
            super::attr_has_bounded_string_max_bound(super::types::SynAttributeRef::from(attr))
                .get()
        });
        if has_derive && has_max_bound {
            let identifier = item_ref.ident.to_string();
            let _: bool = self.try_from_string_names.insert(identifier.clone());
            let _: bool = self.try_from_string_len_checked_names.insert(identifier);
        }
    }
    fn check_from_impl(&mut self, item: super::types::SynItemImplRef<'_>) {
        if !super::item_impl_is_from_string(item).get() {
            return;
        }
        let identifier = super::item_impl_self_ty_identifier(item).map_or_else(
            || String::from(str_constants::NON_PATH_TARGET),
            String::from,
        );
        self.ers.push(format!(
            "`{identifier}` implements `From<String>`; implement `TryFrom<String>` instead"
        ));
    }
    fn check_newtype_attr(&mut self, item: super::types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        if !super::item_struct_is_single_string_wrapper(item).get() {
            return;
        }
        if item_ref.attrs.iter().any(|attr| {
            super::attr_has_newtype_from_option(super::types::SynAttributeRef::from(attr)).get()
        }) {
            self.ers.push(format!(
                        "string wrapper `{}` derives `newtype::FromInner`; derive `newtype::TryFrom` with a length check instead",
                        item_ref.ident
                    ));
        }
        let has_try_from = item_ref.attrs.iter().any(|attr| {
            attr.path().is_ident(str_constants::DERIVE)
                && attr.meta.require_list().is_ok_and(|list| {
                    list.tokens
                        .to_string()
                        .contains(str_constants::NEWTYPE_TRY_FROM_DERIVE_NAME)
                })
        });
        let mut has_len_check = false;
        item_ref
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident(str_constants::NEWTYPE_TRY_FROM))
            .for_each(|attr| {
                drop(attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident(str_constants::NEWTYPE_TRY_FROM_ERROR) {
                        let _error_type = meta.value()?.parse::<syn::Type>()?;
                        return Ok(());
                    }
                    if meta
                        .path
                        .is_ident(str_constants::NEWTYPE_TRY_FROM_VALIDATOR)
                    {
                        let expr = meta.value()?.parse::<syn::Expr>()?;
                        let mut visitor = LenMethodCallVisitor {
                            found: super::types::AnalyzerBool::default(),
                        };
                        syn::visit::Visit::visit_expr(&mut visitor, &expr);
                        let path_is_len_checked = match &expr {
                            syn::Expr::Path(path) => {
                                let full_path = super::path_to_string(
                                    super::types::SynPathRef::from(&path.path),
                                );
                                self.len_checked_function_names.contains(full_path.as_ref())
                                    || path.path.segments.last().is_some_and(|segment| {
                                        self.len_checked_function_names
                                            .contains(segment.ident.to_string().as_str())
                                    })
                            }
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
                            | _ => false,
                        };
                        if visitor.found.get() || path_is_len_checked {
                            has_len_check = true;
                        }
                    }
                    Ok(())
                }));
            });
        if has_try_from {
            let identifier = item_ref.ident.to_string();
            let _: bool = self.try_from_string_names.insert(identifier.clone());
            if has_len_check {
                let _: bool = self.try_from_string_len_checked_names.insert(identifier);
            }
        }
    }
    fn check_try_from_impl(&mut self, item: super::types::SynItemImplRef<'_>) {
        if !super::item_impl_is_try_from_string(item).get() {
            return;
        }
        let Some(identifier) = super::item_impl_self_ty_identifier(item) else {
            return;
        };
        if !self.string_wrapper_names.contains(identifier.as_ref()) {
            return;
        }
        let _: bool = self
            .try_from_string_names
            .insert(String::from(identifier.clone()));
        if super::item_impl_contains_len_call(item).get() {
            let _: bool = self
                .try_from_string_len_checked_names
                .insert(String::from(identifier));
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct LenCheckedFunctionNameVisitor {
    pub names: super::types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for LenCheckedFunctionNameVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        let mut visitor = LenMethodCallVisitor {
            found: super::types::AnalyzerBool::default(),
        };
        syn::visit::Visit::visit_block(&mut visitor, &i.block);
        if visitor.found.get() {
            let _: bool = self.names.insert(i.sig.ident.to_string());
        }
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let Some(type_name) =
            super::item_impl_self_ty_identifier(super::types::SynItemImplRef::from(i))
        else {
            syn::visit::visit_item_impl(self, i);
            return;
        };
        i.items.iter().for_each(|item| {
            let syn::ImplItem::Fn(method) = item else {
                return;
            };
            let mut visitor = LenMethodCallVisitor {
                found: super::types::AnalyzerBool::default(),
            };
            syn::visit::Visit::visit_block(&mut visitor, &method.block);
            if visitor.found.get() {
                let _: bool =
                    self.names
                        .insert(format!("{}::{}", type_name.as_ref(), method.sig.ident));
            }
        });
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for StringWrapperFromVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        self.check_from_impl(super::types::SynItemImplRef::from(i));
        self.check_try_from_impl(super::types::SynItemImplRef::from(i));
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.check_bounded_string_attr(super::types::SynItemStructRef::from(i));
        self.check_newtype_attr(super::types::SynItemStructRef::from(i));
        syn::visit::visit_item_struct(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct LenMethodCallVisitor {
    pub found: super::types::AnalyzerBool,
}
impl<'ast> syn::visit::Visit<'ast> for LenMethodCallVisitor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        if i.method == str_constants::LEN {
            self.found.set_true();
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct PublicTupleWrapperFieldVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DirectDeserializeTupleWrapperVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DeserializeConversionCallVisitor {
    pub found: super::types::AnalyzerBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ManualDeserializeTupleWrapperVisitor<'names> {
    pub ers: super::types::DiagnosticMsgs,
    pub names: &'names super::types::StdSourceTextSet,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct TupleWrapperConversionCollector {
    pub converted_names: super::types::StdSourceTextSet,
    pub from_inner_names: super::types::StdSourceTextSet,
    pub from_names: super::types::StdSourceTextSet,
    pub inner_types: std::collections::BTreeMap<String, syn::Type>,
    pub names: super::types::StdSourceTextSet,
    pub try_from_inner_names: super::types::StdSourceTextSet,
    pub try_from_names: super::types::StdSourceTextSet,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DirectTupleWrapperConstructorVisitor<'names> {
    pub current_wrapper_name: Option<String>,
    pub ers: super::types::DiagnosticMsgs,
    pub inside_conversion_impl: super::types::AnalyzerBool,
    pub names: &'names super::types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for PublicTupleWrapperFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::item_struct_is_single_field_tuple_wrapper(super::types::SynItemStructRef::from(i))
            .get()
            && super::item_struct_single_field_is_non_private(super::types::SynItemStructRef::from(
                i,
            ))
            .get()
        {
            self.ers.push(format!(
                "tuple wrapper `{}` exposes its inner field; make the field private and initialize through From/TryFrom",
                i.ident
                    ));
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectDeserializeTupleWrapperVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::item_struct_is_single_field_tuple_wrapper(super::types::SynItemStructRef::from(i))
            .get()
            && super::item_struct_derives_deserialize(super::types::SynItemStructRef::from(i)).get()
            && !super::item_struct_deserialize_uses_conversion(
                super::types::SynItemStructRef::from(i),
            )
            .get()
        {
            let start = syn::spanned::Spanned::span(i).start();
            self.ers.push(format!(
                "tuple wrapper `{}` derives Deserialize directly at {}:{}; this lets serde construct the inner field without using the wrapper's required conversion path and can bypass validation or other construction invariants. Deserialize a raw value through `#[serde(from = \"RawType\")]` or `#[serde(try_from = \"RawType\")]` so construction finishes in From/TryFrom",
                i.ident, start.line, start.column
            ));
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DeserializeConversionCallVisitor {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = i.func.as_ref()
            && path.path.segments.last().is_some_and(|segment| {
                segment.ident == str_constants::FROM_ALT_4
                    || segment.ident == str_constants::NEWTYPE_TRY_FROM
            })
        {
            self.found.set_true();
        }
        syn::visit::visit_expr_call(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for ManualDeserializeTupleWrapperVisitor<'_> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let is_deserialize_impl = i.trait_.as_ref().is_some_and(|(path, _)| {
            path.segments.last().is_some_and(|segment| {
                segment.ident == str_constants::CODE_STYLE_DESERIALIZE_DERIVE_NAME
            })
        });
        let Some(name) = super::item_impl_self_ty_identifier(super::types::SynItemImplRef::from(i))
        else {
            syn::visit::visit_item_impl(self, i);
            return;
        };
        if is_deserialize_impl && self.names.contains(name.as_ref()) {
            let mut visitor = DeserializeConversionCallVisitor {
                found: super::types::AnalyzerBool::default(),
            };
            syn::visit::visit_item_impl(&mut visitor, i);
            if !visitor.found.get() {
                let start = syn::spanned::Spanned::span(i).start();
                self.ers.push(format!(
                    "tuple wrapper `{}` implements Deserialize without an explicit From/TryFrom call at {}:{}; returning the wrapper through another construction path can bypass validation or other invariants. Deserialize into a raw type, then finish with `Self::from(raw)` or `Self::try_from(raw)` and map conversion errors with `serde::de::Error::custom`",
                    name.as_ref(), start.line, start.column
                ));
            }
        }
        syn::visit::visit_item_impl(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for TupleWrapperConversionCollector {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let item_ref = super::types::SynItemImplRef::from(i);
        let Some(name) = super::item_impl_self_ty_identifier(item_ref) else {
            syn::visit::visit_item_impl(self, i);
            return;
        };
        if super::item_impl_is_from(item_ref).get() {
            let _: bool = self.from_names.insert(name.as_ref().to_owned());
            if let Some(inner_type) = self.inner_types.get(name.as_ref())
                && super::item_impl_input_type_is(item_ref, inner_type).get()
            {
                let _: bool = self.from_inner_names.insert(name.as_ref().to_owned());
            }
        }
        if super::item_impl_is_try_from(item_ref).get() {
            let _: bool = self.try_from_names.insert(name.as_ref().to_owned());
            if let Some(inner_type) = self.inner_types.get(name.as_ref())
                && super::item_impl_input_type_is(item_ref, inner_type).get()
            {
                let _: bool = self.try_from_inner_names.insert(name.as_ref().to_owned());
            }
        }
        if super::item_impl_is_from_or_try_from(item_ref).get() {
            let _: bool = self.converted_names.insert(name.as_ref().to_owned());
        }
        syn::visit::visit_item_impl(self, i);
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::item_struct_is_single_field_tuple_wrapper(super::types::SynItemStructRef::from(i))
            .get()
        {
            let name = i.ident.to_string();
            let _: bool = self.names.insert(name.clone());
            if let syn::Fields::Unnamed(fields) = &i.fields
                && fields.unnamed.len() == usize_constants::ONE
                && let Some(field) = fields.unnamed.first()
            {
                drop(self.inner_types.insert(name.clone(), field.ty.clone()));
            }
            let item_ref = super::types::SynItemStructRef::from(i);
            if super::item_struct_derives_from_inner(item_ref).get() {
                let _: bool = self.from_names.insert(name.clone());
                let _: bool = self.from_inner_names.insert(name.clone());
            }
            if super::item_struct_derives_try_from(item_ref).get() {
                let _: bool = self.try_from_names.insert(name.clone());
                let _: bool = self.try_from_inner_names.insert(name.clone());
            }
            if super::item_struct_derives_conversion(item_ref).get() {
                let _: bool = self.converted_names.insert(name);
            }
        }
        syn::visit::visit_item_struct(self, i);
    }
}
impl<'ast> syn::visit::Visit<'ast> for DirectTupleWrapperConstructorVisitor<'_> {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if !self.inside_conversion_impl.get()
            && let syn::Expr::Path(path) = i.func.as_ref()
            && let Some(segment) = path.path.segments.last()
            && (self.names.contains(segment.ident.to_string().as_str())
                || (segment.ident == str_constants::SELF && self.current_wrapper_name.is_some()))
        {
            let span = syn::spanned::Spanned::span(i.func.as_ref());
            let start = span.start();
            let end = span.end();
            let wrapper_name = self
                .current_wrapper_name
                .as_deref()
                .filter(|_| segment.ident == str_constants::SELF)
                .map_or_else(|| segment.ident.to_string(), str::to_owned);
            self.ers.push(format!(
                "tuple wrapper `{}` is initialized directly at {}:{}-{}:{}; use From/TryFrom",
                wrapper_name, start.line, start.column, end.line, end.column
            ));
        }
        syn::visit::visit_expr_call(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let previous = self.inside_conversion_impl;
        let previous_wrapper_name = self.current_wrapper_name.take();
        self.inside_conversion_impl =
            super::item_impl_is_from_or_try_from(super::types::SynItemImplRef::from(i));
        self.current_wrapper_name =
            super::item_impl_self_ty_identifier(super::types::SynItemImplRef::from(i))
                .map(|name| name.as_ref().to_owned())
                .filter(|name| self.names.contains(name));
        syn::visit::visit_item_impl(self, i);
        self.inside_conversion_impl = previous;
        self.current_wrapper_name = previous_wrapper_name;
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DeclaredDomainTypeVisitor {
    pub names: super::types::StdSourceTextSet,
}
impl<'ast> syn::visit::Visit<'ast> for DeclaredDomainTypeVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
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
        if super::path_ends_with(
            super::types::SynPathRef::from(&i.path),
            super::types::StaticStrSliceRef::from(
                [
                    str_constants::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
                    str_constants::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
                ]
                .as_slice(),
            ),
        )
        .get()
        {
            super::collect_generate_pg_types_domain_names(
                super::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if super::config_lib_domain_type_macro_path(super::types::SynPathRef::from(&i.path)).get() {
            super::collect_first_macro_identifier_domain_name(
                super::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if super::path_ends_with(
            super::types::SynPathRef::from(&i.path),
            super::types::StaticStrSliceRef::from(
                [str_constants::API_OPERATION_ERROR_MACRO_IDENTIFIER].as_slice(),
            ),
        )
        .get()
        {
            super::collect_first_macro_identifier_domain_name(
                super::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if super::path_ends_with(
            super::types::SynPathRef::from(&i.path),
            super::types::StaticStrSliceRef::from([str_constants::BOOL_ENUM_TO_TOKENS].as_slice()),
        )
        .get()
        {
            super::collect_first_macro_identifier_domain_name(
                super::types::SourceTextRef::from(i.tokens.to_string().as_str()),
                &mut self.names,
            );
        }
        if super::path_ends_with(
            super::types::SynPathRef::from(&i.path),
            super::types::StaticStrSliceRef::from(
                [
                    str_constants::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
                    str_constants::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
                ]
                .as_slice(),
            ),
        )
        .get()
        {
            let _: bool = self
                .names
                .insert(String::from(str_constants::DTOKENSTREAMBUILDER));
        }
        syn::visit::visit_macro(self, i);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct DomainTypePolicyVisitor<'types> {
    pub closure_body_scan_depth: super::types::AnalyzerCount,
    pub ers: super::types::DiagnosticMsgs,
    pub generic_scopes: Vec<super::types::StdSourceTextSet>,
    pub repo_crates: super::types::StdStdSourceTextSetRef<'types>,
    pub repo_types: super::types::StdStdSourceTextSetRef<'types>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AnalyzerStateRawContainerFieldVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct HelperRawTextReturnVisitor {
    pub ers: super::types::DiagnosticMsgs,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct ExternalLeafWrapperNameVisitor<'types> {
    pub ers: super::types::DiagnosticMsgs,
    pub repo_crates: super::types::StdStdSourceTextSetRef<'types>,
}
impl DomainTypePolicyVisitor<'_> {
    fn check_fields(
        &mut self,
        fields: super::types::SynFieldsRef<'_>,
        ctx: super::types::SourceTextRef<'_>,
        allow_single_newtype_raw: super::types::AnalyzerBool,
    ) {
        let fields_ref = fields.as_ref();
        if allow_single_newtype_raw.get()
            && matches!(fields_ref, syn::Fields::Unnamed(unnamed_fields) if unnamed_fields.unnamed.len() == 1)
        {
            return;
        }
        fields_ref
            .iter()
            .for_each(|field| self.check_ty(super::types::SynTypeRef::from(&field.ty), ctx));
    }
    fn check_path_arguments(
        &mut self,
        arguments: super::types::SynPathArgumentsRef<'_>,
        ctx: super::types::SourceTextRef<'_>,
    ) {
        match arguments.as_ref() {
            syn::PathArguments::AngleBracketed(args) => {
                args.args
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::GenericArgument::Type(ty) => Some(ty),
                        syn::GenericArgument::AssocConst(_)
                        | syn::GenericArgument::AssocType(_)
                        | syn::GenericArgument::Constraint(_)
                        | syn::GenericArgument::Const(_)
                        | syn::GenericArgument::Lifetime(_)
                        | _ => None,
                    })
                    .for_each(|ty| self.check_ty(super::types::SynTypeRef::from(ty), ctx));
            }
            syn::PathArguments::Parenthesized(args) => {
                args.inputs
                    .iter()
                    .for_each(|arg| self.check_ty(super::types::SynTypeRef::from(&arg.ty), ctx));
                match &args.output {
                    syn::ReturnType::Default => {}
                    syn::ReturnType::Type(_, ty) => {
                        self.check_ty(super::types::SynTypeRef::from(&**ty), ctx);
                    }
                }
            }
            syn::PathArguments::None => {}
        }
    }
    fn check_sig(
        &mut self,
        sig: super::types::SynSignatureRef<'_>,
        ctx: super::types::SourceTextRef<'_>,
    ) {
        let sig_ref = sig.as_ref();
        self.push_generics(super::types::SynGenericsRef::from(&sig_ref.generics));
        sig_ref
            .inputs
            .iter()
            .filter_map(|input| match input {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat_ty) => Some(pat_ty),
            })
            .for_each(|pat_ty| {
                self.check_ty(
                    super::types::SynTypeRef::from(&*pat_ty.ty),
                    super::types::SourceTextRef::from(
                        format!("{} parameter", ctx.as_ref()).as_str(),
                    ),
                );
            });
        match &sig_ref.output {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, ty) => {
                self.check_ty(
                    super::types::SynTypeRef::from(&**ty),
                    super::types::SourceTextRef::from(
                        format!("{} return type", ctx.as_ref()).as_str(),
                    ),
                );
            }
        }
        self.pop_generics();
    }
    fn check_ty(&mut self, ty: super::types::SynTypeRef<'_>, ctx: super::types::SourceTextRef<'_>) {
        match ty.as_ref() {
            syn::Type::Array(ty_array) => {
                self.check_ty(super::types::SynTypeRef::from(&*ty_array.elem), ctx);
            }
            syn::Type::Group(ty_group) => {
                self.check_ty(super::types::SynTypeRef::from(&*ty_group.elem), ctx);
            }
            syn::Type::Paren(ty_paren) => {
                self.check_ty(super::types::SynTypeRef::from(&*ty_paren.elem), ctx);
            }
            syn::Type::Path(ty_path) => {
                self.check_ty_path(super::types::SynTypePathRef::from(ty_path), ctx);
            }
            syn::Type::Reference(ty_reference) => {
                self.check_ty(super::types::SynTypeRef::from(&*ty_reference.elem), ctx);
            }
            syn::Type::Slice(ty_slice) => {
                self.check_ty(super::types::SynTypeRef::from(&*ty_slice.elem), ctx);
            }
            syn::Type::Tuple(ty_tuple) => {
                ty_tuple
                    .elems
                    .iter()
                    .for_each(|elem| self.check_ty(super::types::SynTypeRef::from(elem), ctx));
            }
            syn::Type::FnPtr(_)
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
    fn check_ty_path(
        &mut self,
        ty_path: super::types::SynTypePathRef<'_>,
        ctx: super::types::SourceTextRef<'_>,
    ) {
        let ty_path_ref = ty_path.as_ref();
        if let Some(qself) = &ty_path_ref.qself {
            self.check_ty(super::types::SynTypeRef::from(&*qself.ty), ctx);
            ty_path_ref.path.segments.iter().for_each(|segment| {
                self.check_path_arguments(
                    super::types::SynPathArgumentsRef::from(&segment.arguments),
                    ctx,
                );
            });
            return;
        }
        let Some(segment) = ty_path_ref.path.segments.last() else {
            return;
        };
        let identifier = segment.ident.to_string();
        if super::path_first_segment_is_self(super::types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            self.check_path_arguments(
                super::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
            );
            return;
        }
        if super::is_structural_generic_container(super::types::SourceTextRef::from(
            identifier.as_str(),
        ))
        .get()
        {
            self.check_path_arguments(
                super::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
            );
            return;
        }
        if self
            .is_allowed_type_identifier(super::types::SourceTextRef::from(identifier.as_str()))
            .get()
        {
            self.check_path_arguments(
                super::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
            );
            return;
        }
        if self
            .path_starts_with_allowed_type_identifier(super::types::SynPathRef::from(
                &ty_path_ref.path,
            ))
            .get()
        {
            ty_path_ref.path.segments.iter().for_each(|path_segment| {
                self.check_path_arguments(
                    super::types::SynPathArgumentsRef::from(&path_segment.arguments),
                    ctx,
                );
            });
            return;
        }
        if self
            .path_starts_with_repo_crate(super::types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            ty_path_ref.path.segments.iter().for_each(|path_segment| {
                self.check_path_arguments(
                    super::types::SynPathArgumentsRef::from(&path_segment.arguments),
                    ctx,
                );
            });
            return;
        }
        if self
            .path_starts_with_external_crate(super::types::SynPathRef::from(&ty_path_ref.path))
            .get()
        {
            self.ers.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                ctx.as_ref(),
                super::path_to_string(super::types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
            self.check_path_arguments(
                super::types::SynPathArgumentsRef::from(&segment.arguments),
                ctx,
            );
            return;
        }
        self.ers.push(format!(
                "{} uses `{}`; use a repository domain wrapper type and initialize it with From/TryFrom instead of exposing raw external or primitive types",
                ctx.as_ref(),
                super::path_to_string(super::types::SynPathRef::from(&ty_path_ref.path)).as_ref()
            ));
        self.check_path_arguments(
            super::types::SynPathArgumentsRef::from(&segment.arguments),
            ctx,
        );
    }
    fn closure_body_scan_is_active(&self) -> super::types::AnalyzerBool {
        super::types::AnalyzerBool::from(self.closure_body_scan_depth.get() > 0)
    }
    fn is_allowed_type_identifier(
        &self,
        identifier: super::types::SourceTextRef<'_>,
    ) -> super::types::AnalyzerBool {
        let identifier_ref = identifier.as_ref();
        super::types::AnalyzerBool::from(
            identifier_ref == str_constants::SELF
                || self.repo_types.as_ref().contains(identifier_ref)
                || self
                    .generic_scopes
                    .iter()
                    .rev()
                    .any(|scope| scope.contains(identifier_ref)),
        )
    }
    fn path_starts_with_allowed_type_identifier(
        &self,
        path: super::types::SynPathRef<'_>,
    ) -> super::types::AnalyzerBool {
        let path_ref = path.as_ref();
        super::types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    self.is_allowed_type_identifier(super::types::SourceTextRef::from(
                        segment.ident.to_string().as_str(),
                    ))
                    .get()
                }),
        )
    }
    fn path_starts_with_external_crate(
        &self,
        path: super::types::SynPathRef<'_>,
    ) -> super::types::AnalyzerBool {
        let path_ref = path.as_ref();
        super::types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    let identifier = segment.ident.to_string();
                    identifier != str_constants::CRATE
                        && identifier != str_constants::SELF_ALT
                        && identifier != str_constants::SUPER
                        && !self.repo_crates.as_ref().contains(&identifier)
                        && !self
                            .is_allowed_type_identifier(super::types::SourceTextRef::from(
                                identifier.as_str(),
                            ))
                            .get()
                }),
        )
    }
    fn path_starts_with_repo_crate(
        &self,
        path: super::types::SynPathRef<'_>,
    ) -> super::types::AnalyzerBool {
        let path_ref = path.as_ref();
        super::types::AnalyzerBool::from(
            path_ref.segments.len() > 1
                && path_ref.segments.first().is_some_and(|segment| {
                    let identifier = segment.ident.to_string();
                    self.repo_crates.as_ref().contains(&identifier)
                }),
        )
    }
    fn pop_generics(&mut self) {
        let popped = self.generic_scopes.pop();
        assert!(popped.is_some(), "1cb23b63");
    }
    fn push_generics(&mut self, generics: super::types::SynGenericsRef<'_>) {
        let mut names = std::collections::BTreeSet::new();
        names.extend(
            generics
                .as_ref()
                .params
                .iter()
                .filter_map(|param| match param {
                    syn::GenericParam::Type(type_param) => Some(type_param.ident.to_string()),
                    syn::GenericParam::Const(_) | syn::GenericParam::Lifetime(_) => None,
                }),
        );
        self.generic_scopes
            .push(super::types::StdSourceTextSet::from(names));
    }
    fn scan_block_for_closure_inputs(&mut self, block: super::types::SynBlockRef<'_>) {
        self.closure_body_scan_depth.saturating_inc();
        syn::visit::visit_block(self, block.as_ref());
        self.closure_body_scan_depth.saturating_dec();
    }
}
impl<'ast> syn::visit::Visit<'ast> for DomainTypePolicyVisitor<'_> {
    fn visit_expr_closure(&mut self, i: &'ast syn::ExprClosure) {
        i.inputs.iter().for_each(|input| {
            if let syn::Pat::Type(pat_ty) = input {
                self.check_ty(
                    super::types::SynTypeRef::from(&*pat_ty.ty),
                    super::types::SourceTextRef::from(str_constants::CLOSURE_PARAMETER),
                );
            }
        });
        syn::visit::visit_expr_closure(self, i);
    }
    fn visit_item(&mut self, i: &'ast syn::Item) {
        if super::has_test_only_cfg_attr(super::types::SynItemRef::from(i)).get() {
            return;
        }
        if self.closure_body_scan_is_active().get() {
            return;
        }
        syn::visit::visit_item(self, i);
    }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        if super::identifier_is_diagnostic_try_from_string_error(
            super::types::SynIdentifierRef::from(&i.ident),
        )
        .get()
        {
            return;
        }
        self.push_generics(super::types::SynGenericsRef::from(&i.generics));
        i.variants.iter().for_each(|variant| {
            self.check_fields(
                super::types::SynFieldsRef::from(&variant.fields),
                super::types::SourceTextRef::from(format!("enum `{}` variant", i.ident).as_str()),
                super::types::AnalyzerBool::default(),
            );
        });
        self.pop_generics();
    }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if super::item_fn_is_proc_macro(super::types::SynItemFnRef::from(i)).get() {
            return;
        }
        self.check_sig(
            super::types::SynSignatureRef::from(&i.sig),
            super::types::SourceTextRef::from(format!("function `{}`", i.sig.ident).as_str()),
        );
        self.scan_block_for_closure_inputs(super::types::SynBlockRef::from(&*i.block));
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if i.trait_.is_some() {
            return;
        }
        self.push_generics(super::types::SynGenericsRef::from(&i.generics));
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !super::attrs_contain_test_only_cfg(
                        super::types::SynAttributeListRef::from(item_fn.attrs.as_slice()),
                    )
                    .get() =>
                {
                    if super::method_is_explicit_wrapper_accessor(
                        super::types::SynIdentifierRef::from(&item_fn.sig.ident),
                    )
                    .get()
                        || super::method_is_private_newtype_validator(item_fn).get()
                    {
                        None
                    } else {
                        Some(item_fn)
                    }
                }
                syn::ImplItem::Const(_)
                | syn::ImplItem::Macro(_)
                | syn::ImplItem::Type(_)
                | syn::ImplItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.check_sig(
                    super::types::SynSignatureRef::from(&item_fn.sig),
                    super::types::SourceTextRef::from(
                        format!("method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !super::attrs_contain_test_only_cfg(
                        super::types::SynAttributeListRef::from(item_fn.attrs.as_slice()),
                    )
                    .get() =>
                {
                    Some(item_fn)
                }
                syn::ImplItem::Const(_)
                | syn::ImplItem::Macro(_)
                | syn::ImplItem::Type(_)
                | syn::ImplItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.scan_block_for_closure_inputs(super::types::SynBlockRef::from(&item_fn.block));
            });
        self.pop_generics();
    }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.push_generics(super::types::SynGenericsRef::from(&i.generics));
        self.check_fields(
            super::types::SynFieldsRef::from(&i.fields),
            super::types::SourceTextRef::from(format!("struct `{}` field", i.ident).as_str()),
            super::types::AnalyzerBool::from(true),
        );
        self.pop_generics();
    }
    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.push_generics(super::types::SynGenericsRef::from(&i.generics));
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::TraitItem::Fn(item_fn)
                    if !super::attrs_contain_test_only_cfg(
                        super::types::SynAttributeListRef::from(item_fn.attrs.as_slice()),
                    )
                    .get() =>
                {
                    Some(item_fn)
                }
                syn::TraitItem::Const(_)
                | syn::TraitItem::Macro(_)
                | syn::TraitItem::Type(_)
                | syn::TraitItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.check_sig(
                    super::types::SynSignatureRef::from(&item_fn.sig),
                    super::types::SourceTextRef::from(
                        format!("trait method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
        self.pop_generics();
    }
}
impl AnalyzerStateRawContainerFieldVisitor {
    fn check_fields(&mut self, item: super::types::SynItemStructRef<'_>) {
        let item_ref = item.as_ref();
        item_ref.fields.iter().for_each(|field| {
            if let Some((raw_ty, wrapper_ty)) =
                super::analyzer_state_raw_container_ty(super::types::SynTypeRef::from(&field.ty))
            {
                let field_name = field
                    .ident
                    .as_ref()
                    .map_or_else(|| String::from(str_constants::TUPLE), ToString::to_string);
                self.ers.push(format!(
                    "struct `{}` field `{}` uses `{}`; use `{}`",
                    item_ref.ident,
                    field_name,
                    raw_ty.get(),
                    wrapper_ty.get()
                ));
            }
        });
    }
}
impl<'ast> syn::visit::Visit<'ast> for AnalyzerStateRawContainerFieldVisitor {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::item_struct_is_single_field_tuple_wrapper(super::types::SynItemStructRef::from(i))
            .get()
        {
            return;
        }
        self.check_fields(super::types::SynItemStructRef::from(i));
        syn::visit::visit_item_struct(self, i);
    }
}
impl HelperRawTextReturnVisitor {
    fn check_sig(
        &mut self,
        sig: super::types::SynSignatureRef<'_>,
        ctx: super::types::SourceTextRef<'_>,
    ) {
        let syn::ReturnType::Type(_, ty) = &sig.as_ref().output else {
            return;
        };
        if let Some((raw_ty, wrapper_ty)) =
            super::raw_text_return_ty(super::types::SynTypeRef::from(&**ty))
        {
            self.ers.push(format!(
                "{} return type uses `{}`; use `{}`",
                ctx.as_ref(),
                raw_ty.get(),
                wrapper_ty.get()
            ));
        }
    }
}
impl<'ast> syn::visit::Visit<'ast> for HelperRawTextReturnVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        if super::item_fn_is_proc_macro(super::types::SynItemFnRef::from(i)).get() {
            return;
        }
        self.check_sig(
            super::types::SynSignatureRef::from(&i.sig),
            super::types::SourceTextRef::from(format!("function `{}`", i.sig.ident).as_str()),
        );
        syn::visit::visit_item_fn(self, i);
    }
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        if i.trait_.is_some() {
            return;
        }
        i.items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(item_fn)
                    if !super::method_is_explicit_wrapper_accessor(
                        super::types::SynIdentifierRef::from(&item_fn.sig.ident),
                    )
                    .get() =>
                {
                    Some(item_fn)
                }
                syn::ImplItem::Const(_)
                | syn::ImplItem::Macro(_)
                | syn::ImplItem::Type(_)
                | syn::ImplItem::Verbatim(_)
                | _ => None,
            })
            .for_each(|item_fn| {
                self.check_sig(
                    super::types::SynSignatureRef::from(&item_fn.sig),
                    super::types::SourceTextRef::from(
                        format!("method `{}`", item_fn.sig.ident).as_str(),
                    ),
                );
            });
    }
}
impl<'ast> syn::visit::Visit<'ast> for ExternalLeafWrapperNameVisitor<'_> {
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        if super::attrs_contain_test_only_cfg(super::types::SynAttributeListRef::from(
            i.attrs.as_slice(),
        ))
        .get()
        {
            return;
        }
        let syn::Fields::Unnamed(fields) = &i.fields else {
            syn::visit::visit_item_struct(self, i);
            return;
        };
        if fields.unnamed.len() != 1 {
            syn::visit::visit_item_struct(self, i);
            return;
        }
        let Some(field) = fields.unnamed.first() else {
            syn::visit::visit_item_struct(self, i);
            return;
        };
        self.check_external_leaf_wrapper_name(
            super::types::SynItemStructRef::from(i),
            super::types::SynTypeRef::from(&field.ty),
        );
        syn::visit::visit_item_struct(self, i);
    }
}
impl ExternalLeafWrapperNameVisitor<'_> {
    fn check_external_leaf_wrapper_name(
        &mut self,
        item: super::types::SynItemStructRef<'_>,
        ty: super::types::SynTypeRef<'_>,
    ) {
        let Some(first_segment) = self.external_root_segment(ty) else {
            return;
        };
        let first_segment_ref = first_segment.get();
        let item_ref = item.as_ref();
        let expected_prefix = super::identifier_to_upper_camel_fragment(
            super::types::SynIdentifierRef::from(&first_segment_ref.ident),
        );
        let identifier = item_ref.ident.to_string();
        if super::is_external_leaf_wrapper_name_exception(super::types::SourceTextRef::from(
            identifier.as_str(),
        ))
        .get()
        {
            return;
        }
        if identifier.starts_with(expected_prefix.as_ref()) {
            return;
        }
        self.ers.push(format!(
            "tuple wrapper `{}` wraps external crate `{}`; rename it so it starts with `{}`",
            item_ref.ident,
            first_segment_ref.ident,
            expected_prefix.as_ref()
        ));
    }
    fn external_root_segment<'ty_lt>(
        &self,
        ty: super::types::SynTypeRef<'ty_lt>,
    ) -> Option<super::types::SynPathSegmentRef<'ty_lt>> {
        match ty.get() {
            syn::Type::Array(ty_array) => {
                self.external_root_segment(super::types::SynTypeRef::from(&*ty_array.elem))
            }
            syn::Type::Group(ty_group) => {
                self.external_root_segment(super::types::SynTypeRef::from(&*ty_group.elem))
            }
            syn::Type::Paren(ty_paren) => {
                self.external_root_segment(super::types::SynTypeRef::from(&*ty_paren.elem))
            }
            syn::Type::Path(ty_path) => {
                self.external_root_segment_from_path(super::types::SynTypePathRef::from(ty_path))
            }
            syn::Type::Reference(ty_reference) => {
                self.external_root_segment(super::types::SynTypeRef::from(&*ty_reference.elem))
            }
            syn::Type::Slice(ty_slice) => {
                self.external_root_segment(super::types::SynTypeRef::from(&*ty_slice.elem))
            }
            syn::Type::Tuple(ty_tuple) => ty_tuple
                .elems
                .iter()
                .find_map(|elem| self.external_root_segment(super::types::SynTypeRef::from(elem))),
            syn::Type::FnPtr(_)
            | syn::Type::ImplTrait(_)
            | syn::Type::Infer(_)
            | syn::Type::Macro(_)
            | syn::Type::Never(_)
            | syn::Type::Ptr(_)
            | syn::Type::TraitObject(_)
            | syn::Type::Verbatim(_)
            | _ => None,
        }
    }
    fn external_root_segment_from_arguments<'args_lt>(
        &self,
        arguments: super::types::SynPathArgumentsRef<'args_lt>,
    ) -> Option<super::types::SynPathSegmentRef<'args_lt>> {
        match arguments.get() {
            syn::PathArguments::AngleBracketed(args) => {
                args.args.iter().find_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => {
                        self.external_root_segment(super::types::SynTypeRef::from(ty))
                    }
                    syn::GenericArgument::AssocConst(_)
                    | syn::GenericArgument::AssocType(_)
                    | syn::GenericArgument::Constraint(_)
                    | syn::GenericArgument::Const(_)
                    | syn::GenericArgument::Lifetime(_)
                    | _ => None,
                })
            }
            syn::PathArguments::Parenthesized(args) => args
                .inputs
                .iter()
                .find_map(|arg| self.external_root_segment(super::types::SynTypeRef::from(&arg.ty)))
                .or_else(|| match &args.output {
                    syn::ReturnType::Default => None,
                    syn::ReturnType::Type(_, ty) => {
                        self.external_root_segment(super::types::SynTypeRef::from(&**ty))
                    }
                }),
            syn::PathArguments::None => None,
        }
    }
    fn external_root_segment_from_path<'path_lt>(
        &self,
        ty_path: super::types::SynTypePathRef<'path_lt>,
    ) -> Option<super::types::SynPathSegmentRef<'path_lt>> {
        let ty_path_ref = ty_path.get();
        if let Some(qself) = &ty_path_ref.qself {
            return self.external_root_segment(super::types::SynTypeRef::from(&*qself.ty));
        }
        let first_segment = ty_path_ref.path.segments.first()?;
        let first_identifier = first_segment.ident.to_string();
        if first_identifier == str_constants::CRATE
            || first_identifier == str_constants::SELF_ALT
            || first_identifier == str_constants::SUPER
            || self.repo_crates.as_ref().contains(&first_identifier)
        {
            return ty_path_ref.path.segments.iter().find_map(|segment| {
                self.external_root_segment_from_arguments(super::types::SynPathArgumentsRef::from(
                    &segment.arguments,
                ))
            });
        }
        if ty_path_ref.path.segments.len() > 1 {
            return Some(super::types::SynPathSegmentRef::from(first_segment));
        }
        ty_path_ref.path.segments.iter().find_map(|segment| {
            self.external_root_segment_from_arguments(super::types::SynPathArgumentsRef::from(
                &segment.arguments,
            ))
        })
    }
}
