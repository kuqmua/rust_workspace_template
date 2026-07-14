const SNAKE_IDENT_MAX_LEN: usize = 1_048_576;
#[cfg(test)]
#[allow(dead_code)] // dev dependencies are exercised by integration tests, not proc-macro unit code
fn dependency_markers<Value>(
    _: Option<Value>,
    _: Option<serde_json::Value>,
    _: Option<utoipa::openapi::OpenApi>,
) where
    Value: serde::Serialize,
{
}
#[derive(Debug, Default)]
struct NewtypeAttrs {
    options: workspace_macro_helpers::StdUniqueOptionSet<NewtypeOption>,
    to_err_string_mode: Option<ToErrStringMode>,
}
struct BoundedStringAttrs {
    description: Option<SynExpr>,
    max: Option<SynExpr>,
    min: Option<SynExpr>,
    options: workspace_macro_helpers::StdUniqueOptionSet<BoundedStringOption>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BoundedStringOption {
    Chars,
    NulFree,
    Serde,
    Trim,
    Utoipa,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NewtypeOption {
    AsRef,
    AsRefInner,
    AsRefOwned,
    AsRefStr,
    AsRefTarget,
    AsSlice,
    DebugTransparent,
    DerefInner,
    DerefMutInner,
    DerefMutTarget,
    DerefTarget,
    Display,
    From,
    Getter,
    IntoInner,
    IntoInnerFrom,
    IntoVec,
    Secret,
    ToTokens,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToErrStringMode {
    AsRefStr,
    Debug,
    Display,
}
struct ProcMacro2GeneratedTs(proc_macro2::TokenStream);
impl From<ProcMacro2GeneratedTs> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2GeneratedTs) -> Self {
        value.0
    }
}
impl From<ProcMacro2GeneratedTs> for proc_macro::TokenStream {
    fn from(value: ProcMacro2GeneratedTs) -> Self {
        proc_macro2::TokenStream::from(value).into()
    }
}
impl quote::ToTokens for ProcMacro2GeneratedTs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
struct NewtypeBool(bool);
impl NewtypeBool {
    const fn get(&self) -> bool {
        self.0
    }
}
struct SnakeIdent(String);
#[derive(Debug)]
struct SnakeIdentLen(usize);
impl From<usize> for SnakeIdentLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct SnakeIdentTryFromStringEr {
    len: SnakeIdentLen,
}
impl std::fmt::Display for SnakeIdentTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "snake ident length {} exceeds maximum {SNAKE_IDENT_MAX_LEN}",
            self.len.0
        )
    }
}
impl TryFrom<String> for SnakeIdent {
    type Error = SnakeIdentTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SNAKE_IDENT_MAX_LEN {
            return Err(SnakeIdentTryFromStringEr {
                len: SnakeIdentLen::from(value.len()),
            });
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for SnakeIdent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for SnakeIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl quote::ToTokens for SnakeIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(Clone, Copy)]
struct SynAttrsRef<'syn_lt>(&'syn_lt [syn::Attribute]);
impl<'syn_lt> From<&'syn_lt [syn::Attribute]> for SynAttrsRef<'syn_lt> {
    fn from(value: &'syn_lt [syn::Attribute]) -> Self {
        Self(value)
    }
}
impl AsRef<[syn::Attribute]> for SynAttrsRef<'_> {
    fn as_ref(&self) -> &[syn::Attribute] {
        self.0
    }
}
#[derive(Clone, Copy)]
struct SynDeriveInputRef<'syn_lt>(&'syn_lt syn::DeriveInput);
impl<'syn_lt> From<&'syn_lt syn::DeriveInput> for SynDeriveInputRef<'syn_lt> {
    fn from(value: &'syn_lt syn::DeriveInput) -> Self {
        Self(value)
    }
}
impl AsRef<syn::DeriveInput> for SynDeriveInputRef<'_> {
    fn as_ref(&self) -> &syn::DeriveInput {
        self.0
    }
}
#[derive(Clone, Copy)]
struct SynIdentRef<'syn_lt>(&'syn_lt syn::Ident);
impl<'syn_lt> From<&'syn_lt syn::Ident> for SynIdentRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynIdentRef<'_> {
    fn as_ref(&self) -> &syn::Ident {
        self.0
    }
}
struct SynExpr(syn::Expr);
impl From<syn::Expr> for SynExpr {
    fn from(value: syn::Expr) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Expr> for SynExpr {
    fn as_ref(&self) -> &syn::Expr {
        &self.0
    }
}
impl quote::ToTokens for SynExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
#[derive(Clone, Copy)]
struct SynParseNestedMetaRef<'syn_lt>(&'syn_lt syn::meta::ParseNestedMeta<'syn_lt>);
impl<'syn_lt> From<&'syn_lt syn::meta::ParseNestedMeta<'syn_lt>>
    for SynParseNestedMetaRef<'syn_lt>
{
    fn from(value: &'syn_lt syn::meta::ParseNestedMeta<'syn_lt>) -> Self {
        Self(value)
    }
}
impl<'syn_lt> AsRef<syn::meta::ParseNestedMeta<'syn_lt>> for SynParseNestedMetaRef<'syn_lt> {
    fn as_ref(&self) -> &syn::meta::ParseNestedMeta<'syn_lt> {
        self.0
    }
}
#[derive(Clone, Copy)]
struct SynTypeRef<'syn_lt>(&'syn_lt syn::Type);
impl<'syn_lt> From<&'syn_lt syn::Type> for SynTypeRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Type) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Type> for SynTypeRef<'_> {
    fn as_ref(&self) -> &syn::Type {
        self.0
    }
}
impl NewtypeAttrs {
    fn contains(&self, option: NewtypeOption) -> NewtypeBool {
        NewtypeBool(self.options.contains(option).get())
    }
    fn set_to_err_string_mode(
        &mut self,
        mode: ToErrStringMode,
        meta: SynParseNestedMetaRef<'_>,
    ) -> syn::Result<()> {
        if self.to_err_string_mode.replace(mode).is_some() {
            return Err(meta
                .as_ref()
                .error("only one to_err_string mode can be selected"));
        }
        Ok(())
    }
    fn try_insert(
        &mut self,
        option: NewtypeOption,
        meta: SynParseNestedMetaRef<'_>,
    ) -> syn::Result<()> {
        self.options
            .try_insert_with(option, || meta.as_ref().error("duplicate newtype option"))
    }
}
#[proc_macro_derive(Newtype, attributes(newtype))]
pub fn newtype(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_ts) {
        Ok(v) => v,
        Err(er) => return er.into_compile_error().into(),
    };
    match gen_newtype_ts(SynDeriveInputRef::from(&input)) {
        Ok(v) => v.into(),
        Err(er) => er.into_compile_error().into(),
    }
}
#[proc_macro_derive(EnumFromStr)]
pub fn enum_from_str(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_ts) {
        Ok(v) => v,
        Err(er) => return er.into_compile_error().into(),
    };
    match gen_enum_from_str_ts(SynDeriveInputRef::from(&input)) {
        Ok(v) => v.into(),
        Err(er) => er.into_compile_error().into(),
    }
}
#[proc_macro_derive(BoundedString, attributes(bounded_string))]
pub fn bounded_string(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_ts) {
        Ok(v) => v,
        Err(er) => return er.into_compile_error().into(),
    };
    match gen_bounded_string_ts(SynDeriveInputRef::from(&input)) {
        Ok(v) => v.into(),
        Err(er) => er.into_compile_error().into(),
    }
}
#[allow(clippy::single_call_fn)] // keeps top-level derive flow separate from token generation details
fn gen_newtype_ts(input: SynDeriveInputRef<'_>) -> syn::Result<ProcMacro2GeneratedTs> {
    let input_ref = input.as_ref();
    let attrs = parse_newtype_attrs(SynAttrsRef::from(input_ref.attrs.as_slice()))?;
    validate_newtype_attrs(&attrs, input)?;
    let inner_ty = tuple_struct_one_field_ty(input)?;
    validate_newtype_inner_ty_attrs(&attrs, inner_ty)?;
    let inner_ty_ref = inner_ty.as_ref();
    let ident = &input_ref.ident;
    let (impl_generics, ty_generics, where_clause) = input_ref.generics.split_for_impl();
    let debug_transparent_ts = attrs
        .contains(NewtypeOption::DebugTransparent).get()
        .then(|| {
        let mut debug_generics = input_ref.generics.clone();
        debug_generics
            .make_where_clause()
            .predicates
            .push(syn::parse_quote! { #inner_ty_ref: std::fmt::Debug });
        let (debug_impl_generics, debug_ty_generics, debug_where_clause) =
            debug_generics.split_for_impl();
        quote::quote! {
            impl #debug_impl_generics std::fmt::Debug for #ident #debug_ty_generics #debug_where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Debug::fmt(&self.0, f)
                }
            }
        }
    });
    let display_ts = attrs.contains(NewtypeOption::Display).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::fmt::Display for #ident #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }
    });
    let as_ref_str_ts = attrs.contains(NewtypeOption::AsRefStr).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics AsRef<str> for #ident #ty_generics #where_clause {
                fn as_ref(&self) -> &str {
                    AsRef::<str>::as_ref(&self.0)
                }
            }
        }
    });
    let as_ref_ts = attrs.contains(NewtypeOption::AsRef).get().then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub const fn as_ref(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
        }
    });
    let as_ref_inner_ts = if attrs.contains(NewtypeOption::AsRefInner).get() {
        let syn::Type::Reference(inner_ref_ty) = inner_ty_ref else {
            return Err(syn::Error::new_spanned(
                inner_ty_ref,
                "#[newtype(as_ref_inner)] requires a shared reference inner type",
            ));
        };
        let target_ty = &inner_ref_ty.elem;
        Some(quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics AsRef<#target_ty> for #ident #ty_generics #where_clause {
                fn as_ref(&self) -> &#target_ty {
                    self.0
                }
            }
        })
    } else {
        None
    };
    let as_ref_owned_ts = attrs.contains(NewtypeOption::AsRefOwned).get().then(|| {
        quote::quote! {
            impl #impl_generics AsRef<#inner_ty_ref> for #ident #ty_generics #where_clause {
                fn as_ref(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
        }
    });
    let as_ref_target_ts = attrs.contains(NewtypeOption::AsRefTarget).get().then(|| {
        quote::quote! {
            impl #impl_generics AsRef<<#inner_ty_ref as std::ops::Deref>::Target> for #ident #ty_generics #where_clause
            where
                #inner_ty_ref: std::ops::Deref,
            {
                fn as_ref(&self) -> &<#inner_ty_ref as std::ops::Deref>::Target {
                    std::ops::Deref::deref(&self.0)
                }
            }
        }
    });
    let as_slice_ts = attrs.contains(NewtypeOption::AsSlice).get().then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn as_slice(&self) -> &<#inner_ty_ref as std::ops::Deref>::Target
                where
                    #inner_ty_ref: std::ops::Deref,
                {
                    std::ops::Deref::deref(&self.0)
                }
            }
        }
    });
    let deref_inner_ts = attrs.contains(NewtypeOption::DerefInner).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = #inner_ty_ref;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    });
    let deref_target_ts = attrs.contains(NewtypeOption::DerefTarget).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = <#inner_ty_ref as std::ops::Deref>::Target;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    });
    let deref_mut_inner_ts = attrs.contains(NewtypeOption::DerefMutInner).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::ops::DerefMut for #ident #ty_generics #where_clause {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        }
    });
    let deref_mut_target_ts = attrs
        .contains(NewtypeOption::DerefMutTarget)
        .get()
        .then(|| {
            quote::quote! {
                #[allow(single_use_lifetimes)]
                impl #impl_generics std::ops::DerefMut for #ident #ty_generics #where_clause {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }
            }
        });
    let from_ts = attrs.contains(NewtypeOption::From).get().then(|| {
        quote::quote! {
            impl #impl_generics From<#inner_ty_ref> for #ident #ty_generics #where_clause {
                fn from(value: #inner_ty_ref) -> Self {
                    Self(value)
                }
            }
        }
    });
    let getter_ts = attrs.contains(NewtypeOption::Getter).get().then(|| {
        let trait_ident = quote::format_ident!("Get{ident}");
        let fn_name = ident_to_snake(SynIdentRef::from(ident));
        let fn_ident = quote::format_ident!("get_{}", fn_name.as_ref());
        quote::quote! {
            pub trait #trait_ident {
                fn #fn_ident(&self) -> &#inner_ty_ref;
            }
            impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
                fn #fn_ident(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
            impl #impl_generics #trait_ident for &#ident #ty_generics #where_clause {
                fn #fn_ident(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
        }
    });
    let into_inner_ts = attrs.contains(NewtypeOption::IntoInner).get().then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn into_inner(self) -> #inner_ty_ref {
                    self.0
                }
            }
        }
    });
    let into_inner_from_ts = attrs.contains(NewtypeOption::IntoInnerFrom).get().then(|| {
        quote::quote! {
            impl #impl_generics From<#ident #ty_generics> for #inner_ty_ref #where_clause {
                fn from(value: #ident #ty_generics) -> Self {
                    value.0
                }
            }
        }
    });
    let into_vec_ts = attrs.contains(NewtypeOption::IntoVec).get().then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn into_vec(self) -> #inner_ty_ref {
                    self.0
                }
            }
        }
    });
    let to_tokens_ts = attrs.contains(NewtypeOption::ToTokens).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics quote::ToTokens for #ident #ty_generics #where_clause {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    quote::ToTokens::to_tokens(&self.0, tokens);
                }
            }
        }
    });
    let to_err_string_ts = gen_to_err_string_ts(&attrs, SynIdentRef::from(ident));
    Ok(ProcMacro2GeneratedTs(quote::quote! {
        #debug_transparent_ts
        #display_ts
        #as_ref_str_ts
        #as_ref_ts
        #as_ref_inner_ts
        #as_ref_owned_ts
        #as_ref_target_ts
        #as_slice_ts
        #deref_inner_ts
        #deref_target_ts
        #deref_mut_inner_ts
        #deref_mut_target_ts
        #from_ts
        #getter_ts
        #into_inner_ts
        #into_inner_from_ts
        #into_vec_ts
        #to_tokens_ts
        #to_err_string_ts
    }))
}
#[allow(clippy::single_call_fn)] // checked String wrapper generation is separate from forwarding newtype impls
fn gen_bounded_string_ts(input: SynDeriveInputRef<'_>) -> syn::Result<ProcMacro2GeneratedTs> {
    let input_ref = input.as_ref();
    let attrs = parse_bounded_string_attrs(SynAttrsRef::from(input_ref.attrs.as_slice()))?;
    let inner_ty = tuple_struct_one_field_ty(input)?;
    if !type_path_ends_with_string_ident(inner_ty).get() {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            "BoundedString supports only String tuple structs",
        ));
    }
    if !input_ref.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input_ref.generics,
            "BoundedString does not support generics",
        ));
    }
    let ident = &input_ref.ident;
    let vis = &input_ref.vis;
    let er_ident = quote::format_ident!("{ident}TryFromStringEr");
    let BoundedStringAttrs {
        description,
        max: max_option,
        min,
        options,
    } = attrs;
    let max = max_option.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "BoundedString requires #[bounded_string(max = ...)]",
        )
    })?;
    let chars = options.contains(BoundedStringOption::Chars).get();
    let nul_free = options.contains(BoundedStringOption::NulFree).get();
    let serde = options.contains(BoundedStringOption::Serde).get();
    let trim = options.contains(BoundedStringOption::Trim).get();
    let utoipa = options.contains(BoundedStringOption::Utoipa).get();
    if utoipa && !chars {
        return Err(syn::Error::new_spanned(
            input_ref,
            "BoundedString utoipa requires chars so OpenAPI length semantics match runtime",
        ));
    }
    let min_ts =
        min.unwrap_or_else(|| SynExpr::from(syn::Expr::Verbatim(quote::quote! { 0usize })));
    let normalize_ts = trim.then(|| quote::quote! { let value = value.trim().to_owned(); });
    let len_ts = if chars {
        quote::quote! { value.chars().count() }
    } else {
        quote::quote! { value.len() }
    };
    let nul_check_ts = nul_free.then(|| {
        quote::quote! {
            if value.contains('\0') {
                return Err(Self::Error::ContainsNul);
            }
        }
    });
    let serde_ts = serde.then(|| {
        quote::quote! {
            impl serde::Serialize for #ident {
                fn serialize<Serializer>(&self, serializer: Serializer) -> Result<Serializer::Ok, Serializer::Error>
                where
                    Serializer: serde::Serializer,
                {
                    serde::Serialize::serialize(&self.0, serializer)
                }
            }
            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
                where
                    Deserializer: serde::Deserializer<'de>,
                {
                    let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                    Self::try_from(value).map_err(serde::de::Error::custom)
                }
            }
        }
    });
    let utoipa_ts = utoipa.then(|| {
        quote::quote! {
            impl<'schema_lt> utoipa::ToSchema<'schema_lt> for #ident {
                fn schema() -> (
                    &'schema_lt str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        stringify!(#ident),
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::SchemaType::String)
                            .min_length(Some(#min_ts))
                            .max_length(Some(#max))
                            .build()
                            .into(),
                    )
                }
            }
        }
    });
    let description_ts = description.map_or_else(
        || {
            let value = ident_to_snake(SynIdentRef::from(ident))
                .as_ref()
                .replace('_', " ");
            quote::quote! {#value}
        },
        |value| quote::quote! {#value},
    );
    Ok(ProcMacro2GeneratedTs(quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #vis enum #er_ident {
            InvalidBounds { min: usize, max: usize },
            TooShort { len: usize, min: usize },
            TooLong { len: usize, max: usize },
            ContainsNul,
        }
        impl std::fmt::Display for #er_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::InvalidBounds { min, max } => {
                        write!(f, "{} minimum length {min} exceeds maximum {max}", #description_ts)
                    }
                    Self::TooShort { len, min } => {
                        write!(f, "{} length {len} is below minimum {min}", #description_ts)
                    }
                    Self::TooLong { len, max } => {
                        write!(f, "{} length {len} exceeds maximum {max}", #description_ts)
                    }
                    Self::ContainsNul => write!(f, "{} contains a NUL character", #description_ts),
                }
            }
        }
        impl From<#er_ident> for #ident {
            fn from(value: #er_ident) -> Self {
                Self(value.to_string())
            }
        }
        impl TryFrom<String> for #ident {
            type Error = #er_ident;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                #normalize_ts
                if #min_ts > #max {
                    return Err(Self::Error::InvalidBounds { min: #min_ts, max: #max });
                }
                #nul_check_ts
                let len = #len_ts;
                if len < #min_ts {
                    return Err(Self::Error::TooShort { len, min: #min_ts });
                }
                if len > #max {
                    return Err(Self::Error::TooLong {
                        len,
                        max: #max,
                    });
                }
                Ok(Self(value))
            }
        }
        #serde_ts
        #utoipa_ts
    }))
}
#[allow(clippy::single_call_fn)] // keeps enum parsing derive independent from newtype tuple-struct generation
fn gen_enum_from_str_ts(input: SynDeriveInputRef<'_>) -> syn::Result<ProcMacro2GeneratedTs> {
    let input_ref = input.as_ref();
    let ident = &input_ref.ident;
    let data_enum = match &input_ref.data {
        syn::Data::Enum(v) => v,
        syn::Data::Struct(_) | syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input_ref,
                "EnumFromStr supports only enums",
            ));
        }
    };
    let variants = data_enum
        .variants
        .iter()
        .map(|variant| {
            if !matches!(variant.fields, syn::Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    variant,
                    "EnumFromStr supports only unit variants",
                ));
            }
            Ok((
                &variant.ident,
                ident_to_snake(SynIdentRef::from(&variant.ident)),
            ))
        })
        .collect::<syn::Result<Vec<(&syn::Ident, SnakeIdent)>>>()?;
    let allowed_values = variants
        .iter()
        .map(|(_, name)| name.as_ref())
        .collect::<Vec<&str>>()
        .join(", ");
    let arms = variants.iter().map(|(variant_ident, name)| {
        let name_ref = name.as_ref();
        quote::quote! {
            v if v.eq_ignore_ascii_case(#name_ref) => Ok(Self::#variant_ident),
        }
    });
    Ok(ProcMacro2GeneratedTs(quote::quote! {
        impl std::str::FromStr for #ident {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#arms)*
                    _ => Err(format!("Unknown value: {s}. Allowed values: {allowed_values}", allowed_values = #allowed_values)),
                }
            }
        }
    }))
}
#[allow(clippy::single_call_fn)] // required checked-string options are parsed together for focused diagnostics
fn parse_bounded_string_attrs(attrs: SynAttrsRef<'_>) -> syn::Result<BoundedStringAttrs> {
    let parsed = attrs
        .as_ref()
        .iter()
        .filter(|attr| attr.path().is_ident("bounded_string"))
        .try_fold(
            BoundedStringAttrs {
                description: None,
                max: None,
                min: None,
                options: workspace_macro_helpers::StdUniqueOptionSet::default(),
            },
            |mut parsed, attr| {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("max") {
                        parsed.max = Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident("min") {
                        parsed.min = Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident("description") {
                        parsed.description =
                            Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident("chars") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Chars, || {
                                meta.error("duplicate bounded_string option")
                            });
                    }
                    if meta.path.is_ident("nul_free") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::NulFree, || {
                                meta.error("duplicate bounded_string option")
                            });
                    }
                    if meta.path.is_ident("serde") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Serde, || {
                                meta.error("duplicate bounded_string option")
                            });
                    }
                    if meta.path.is_ident("trim") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Trim, || {
                                meta.error("duplicate bounded_string option")
                            });
                    }
                    if meta.path.is_ident("utoipa") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Utoipa, || {
                                meta.error("duplicate bounded_string option")
                            });
                    }
                    Err(meta.error("unknown bounded_string option"))
                })?;
                Ok::<BoundedStringAttrs, syn::Error>(parsed)
            },
        )?;
    if parsed.max.is_none() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "BoundedString requires #[bounded_string(max = ...)]",
        ));
    }
    Ok(parsed)
}
#[allow(clippy::single_call_fn)] // attr parsing is intentionally isolated from code generation
fn parse_newtype_attrs(attrs: SynAttrsRef<'_>) -> syn::Result<NewtypeAttrs> {
    attrs
        .as_ref()
        .iter()
        .filter(|attr| attr.path().is_ident("newtype"))
        .try_fold(NewtypeAttrs::default(), |mut acc, attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("as_ref_str") {
                    return acc
                        .try_insert(NewtypeOption::AsRefStr, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("as_ref") {
                    return acc
                        .try_insert(NewtypeOption::AsRef, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("as_ref_inner") {
                    return acc.try_insert(
                        NewtypeOption::AsRefInner,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("as_ref_owned") {
                    return acc.try_insert(
                        NewtypeOption::AsRefOwned,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("as_ref_target") {
                    return acc.try_insert(
                        NewtypeOption::AsRefTarget,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("as_slice") {
                    return acc
                        .try_insert(NewtypeOption::AsSlice, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("deref") || meta.path.is_ident("deref_target") {
                    return acc.try_insert(
                        NewtypeOption::DerefTarget,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("deref_inner") {
                    return acc.try_insert(
                        NewtypeOption::DerefInner,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("deref_mut_inner") {
                    return acc.try_insert(
                        NewtypeOption::DerefMutInner,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("deref_mut_target") {
                    return acc.try_insert(
                        NewtypeOption::DerefMutTarget,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("debug_transparent") {
                    return acc.try_insert(
                        NewtypeOption::DebugTransparent,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("display") {
                    return acc
                        .try_insert(NewtypeOption::Display, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("from") || meta.path.is_ident("from_inner") {
                    return acc.try_insert(NewtypeOption::From, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("getter") {
                    return acc
                        .try_insert(NewtypeOption::Getter, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("into_inner") {
                    return acc
                        .try_insert(NewtypeOption::IntoInner, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("into_inner_from") {
                    return acc.try_insert(
                        NewtypeOption::IntoInnerFrom,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("into_vec") {
                    return acc
                        .try_insert(NewtypeOption::IntoVec, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("secret") {
                    return acc
                        .try_insert(NewtypeOption::Secret, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("to_tokens") {
                    return acc
                        .try_insert(NewtypeOption::ToTokens, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("to_err_string")
                    || meta.path.is_ident("to_err_string_display")
                {
                    return acc.set_to_err_string_mode(
                        ToErrStringMode::Display,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("to_err_string_as_ref_str") {
                    return acc.set_to_err_string_mode(
                        ToErrStringMode::AsRefStr,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("to_err_string_debug") {
                    return acc.set_to_err_string_mode(
                        ToErrStringMode::Debug,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                Err(meta.error("unknown newtype option"))
            })?;
            Ok(acc)
        })
}
#[allow(clippy::single_call_fn)] // validation stays named so proc-macro diagnostics are not mixed into generation
fn validate_newtype_attrs(attrs: &NewtypeAttrs, input: SynDeriveInputRef<'_>) -> syn::Result<()> {
    if attrs.options.is_empty().get() && attrs.to_err_string_mode.is_none() {
        return Err(syn::Error::new_spanned(
            input.as_ref(),
            "Newtype requires at least one #[newtype(...)] option",
        ));
    }
    if attrs.contains(NewtypeOption::DerefInner).get()
        && attrs.contains(NewtypeOption::DerefTarget).get()
    {
        return Err(syn::Error::new_spanned(
            input.as_ref(),
            "deref_inner and deref_target cannot be combined",
        ));
    }
    if attrs.contains(NewtypeOption::DerefMutInner).get()
        && !attrs.contains(NewtypeOption::DerefInner).get()
    {
        return Err(syn::Error::new_spanned(
            input.as_ref(),
            "deref_mut_inner requires deref_inner",
        ));
    }
    if attrs.contains(NewtypeOption::DerefMutTarget).get()
        && !attrs.contains(NewtypeOption::DerefTarget).get()
    {
        return Err(syn::Error::new_spanned(
            input.as_ref(),
            "deref_mut_target requires deref_target",
        ));
    }
    if attrs.contains(NewtypeOption::Secret).get()
        && (attrs.contains(NewtypeOption::DebugTransparent).get()
            || attrs.contains(NewtypeOption::Display).get()
            || attrs.contains(NewtypeOption::ToTokens).get()
            || attrs.to_err_string_mode.is_some())
    {
        return Err(syn::Error::new_spanned(
            input.as_ref(),
            "secret cannot be combined with formatting, token, or error-string forwarding",
        ));
    }
    Ok(())
}
#[allow(clippy::single_call_fn)] // string wrapper policy belongs to newtype validation before From impl generation
fn validate_newtype_inner_ty_attrs(
    attrs: &NewtypeAttrs,
    inner_ty: SynTypeRef<'_>,
) -> syn::Result<()> {
    if attrs.contains(NewtypeOption::AsRefInner).get()
        && !matches!(
            inner_ty.as_ref(),
            syn::Type::Reference(syn::TypeReference {
                mutability: None,
                ..
            })
        )
    {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            "#[newtype(as_ref_inner)] requires a shared reference inner type",
        ));
    }
    if attrs.contains(NewtypeOption::AsRefOwned).get()
        && matches!(inner_ty.as_ref(), syn::Type::Reference(_))
    {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            "#[newtype(as_ref_owned)] does not support reference inner types; use as_ref_inner",
        ));
    }
    if attrs.contains(NewtypeOption::From).get() && type_path_ends_with_string_ident(inner_ty).get()
    {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            "#[newtype(from_inner)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead",
        ));
    }
    Ok(())
}
#[allow(clippy::single_call_fn)] // tuple field extraction is separate to keep derive input validation explicit
fn tuple_struct_one_field_ty(input: SynDeriveInputRef<'_>) -> syn::Result<SynTypeRef<'_>> {
    let input_ref = input.0;
    let shape =
        workspace_macro_helpers::SynStructShapeRef::try_from(input_ref).map_err(|_error| {
            syn::Error::new_spanned(input_ref, "Newtype supports only tuple structs")
        })?;
    let unnamed = match shape {
        workspace_macro_helpers::SynStructShapeRef::Tuple(v) => &v.get().unnamed,
        workspace_macro_helpers::SynStructShapeRef::Named(_)
        | workspace_macro_helpers::SynStructShapeRef::Unit => {
            return Err(syn::Error::new_spanned(
                input_ref,
                "Newtype supports only tuple structs",
            ));
        }
    };
    if unnamed.len() != 1 {
        return Err(syn::Error::new_spanned(
            input_ref,
            "Newtype supports only one-field tuple structs",
        ));
    }
    unnamed
        .first()
        .map(|field| SynTypeRef::from(&field.ty))
        .ok_or_else(|| syn::Error::new_spanned(input_ref, "Newtype field not found"))
}
#[allow(clippy::single_call_fn)] // newtype validation only needs terminal path ident matching for concrete String wrappers
fn type_path_ends_with_string_ident(ty: SynTypeRef<'_>) -> NewtypeBool {
    NewtypeBool(match ty.as_ref() {
        syn::Type::Path(v) if v.qself.is_none() => v
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "String"),
        syn::Type::Path(_) | _ => false,
    })
}
#[allow(clippy::single_call_fn)] // proc-macro generated getter names need local snake_case conversion without adding another dependency
fn ident_to_snake(ident: SynIdentRef<'_>) -> SnakeIdent {
    let (out, _) = ident.as_ref().to_string().chars().fold(
        (String::new(), false),
        |(mut out, mut prev_lowercase), ch| {
            if ch.is_uppercase() {
                if prev_lowercase && !out.is_empty() {
                    out.push('_');
                }
                ch.to_lowercase().for_each(|lower| out.push(lower));
                prev_lowercase = false;
            } else {
                out.push(ch);
                prev_lowercase = true;
            }
            (out, prev_lowercase)
        },
    );
    SnakeIdent::try_from(out).expect("2e7a9c4f")
}
#[allow(clippy::single_call_fn)] // ToErrString code generation has distinct modes from base newtype impls
fn gen_to_err_string_ts(
    attrs: &NewtypeAttrs,
    ident: SynIdentRef<'_>,
) -> Option<ProcMacro2GeneratedTs> {
    let ident_ref = ident.as_ref();
    attrs.to_err_string_mode.map(|mode| match mode {
        ToErrStringMode::AsRefStr => {
            ProcMacro2GeneratedTs(quote::quote! {
                impl to_err_string::ToErrString for #ident_ref {
                    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
                        to_err_string::ToErrStringValue::try_from(AsRef::<str>::as_ref(&self.0).to_owned()).unwrap_or_else(to_err_string::ToErrStringValue::from)
                    }
                }
            })
        }
        ToErrStringMode::Debug => {
            ProcMacro2GeneratedTs(quote::quote! {
                impl to_err_string::ToErrString for #ident_ref {
                    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
                        to_err_string::ToErrStringValue::try_from(format!("{:?}", self.0)).unwrap_or_else(to_err_string::ToErrStringValue::from)
                    }
                }
            })
        }
        ToErrStringMode::Display => {
            ProcMacro2GeneratedTs(quote::quote! {
                impl to_err_string::ToErrString for #ident_ref {
                    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
                        to_err_string::ToErrStringValue::try_from(self.0.to_string()).unwrap_or_else(to_err_string::ToErrStringValue::from)
                    }
                }
            })
        }
    })
}
#[cfg(test)]
mod tests {
    #[test]
    fn bounded_string_missing_max_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(BoundedString)]
            #[bounded_string(min = 1)]
            struct Value(String);
        };
        let result = super::gen_bounded_string_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "29f8ddc2");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "BoundedString requires #[bounded_string(max = ...)]"
            );
        }
    }
    #[test]
    fn bounded_string_utoipa_byte_length_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(BoundedString)]
            #[bounded_string(max = 4, utoipa)]
            struct Value(String);
        };
        let result = super::gen_bounded_string_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "da6f2151");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "BoundedString utoipa requires chars so OpenAPI length semantics match runtime"
            );
        }
    }
    #[test]
    fn duplicate_options_preserve_attribute_diagnostic() {
        let bounded_input = syn::parse_quote! {
            #[derive(BoundedString)]
            #[bounded_string(max = 4, trim, trim)]
            struct BoundedValue(String);
        };
        let bounded_result =
            super::gen_bounded_string_ts(super::SynDeriveInputRef::from(&bounded_input));
        if let Err(er) = bounded_result {
            assert_eq!(er.to_string(), "duplicate bounded_string option");
        } else {
            panic!("d03ced5c");
        }
        let newtype_input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(display, display)]
            struct Value(String);
        };
        let newtype_result = super::gen_newtype_ts(super::SynDeriveInputRef::from(&newtype_input));
        if let Err(er) = newtype_result {
            assert_eq!(er.to_string(), "duplicate newtype option");
        } else {
            panic!("bb438633");
        }
    }
    #[test]
    fn newtype_as_ref_owned_reference_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(as_ref_owned)]
            struct Value<'value_lt>(&'value_lt u16);
        };
        let result = super::gen_newtype_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "d73a920b");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "#[newtype(as_ref_owned)] does not support reference inner types; use as_ref_inner"
            );
        }
    }
    #[test]
    fn newtype_as_ref_inner_non_reference_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(as_ref_inner)]
            struct Value(u16);
        };
        let result = super::gen_newtype_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "33c7e891");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "#[newtype(as_ref_inner)] requires a shared reference inner type"
            );
        }
    }
    #[test]
    fn newtype_from_string_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(from_inner)]
            struct Name(String);
        };
        let result = super::gen_newtype_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "f9b7c2a1");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "#[newtype(from_inner)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead"
            );
        }
    }
    #[test]
    fn newtype_secret_formatting_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(debug_transparent, secret)]
            struct SecretValue(Vec<u8>);
        };
        let result = super::gen_newtype_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "46d9f064");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "secret cannot be combined with formatting, token, or error-string forwarding"
            );
        }
    }
}
