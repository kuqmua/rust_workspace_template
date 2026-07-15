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
struct ProcMacro2GeneratedTokenStream(proc_macro2::TokenStream);
impl From<ProcMacro2GeneratedTokenStream> for proc_macro2::TokenStream {
    fn from(value: ProcMacro2GeneratedTokenStream) -> Self {
        value.0
    }
}
impl From<ProcMacro2GeneratedTokenStream> for proc_macro::TokenStream {
    fn from(value: ProcMacro2GeneratedTokenStream) -> Self {
        proc_macro2::TokenStream::from(value).into()
    }
}
impl quote::ToTokens for ProcMacro2GeneratedTokenStream {
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
struct SnakeIdentifier(String);
#[derive(Debug)]
struct SnakeIdentifierifierLen(usize);
impl From<usize> for SnakeIdentifierifierLen {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct SnakeIdentifierifierTryFromStringError {
    len: SnakeIdentifierifierLen,
}
impl std::fmt::Display for SnakeIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "snake identifier length {} exceeds maximum {SNAKE_IDENT_MAX_LEN}",
            self.len.0
        )
    }
}
impl TryFrom<String> for SnakeIdentifier {
    type Error = SnakeIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SNAKE_IDENT_MAX_LEN {
            return Err(SnakeIdentifierifierTryFromStringError {
                len: SnakeIdentifierifierLen::from(value.len()),
            });
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for SnakeIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for SnakeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl quote::ToTokens for SnakeIdentifier {
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
struct SynIdentifierRef<'syn_lt>(&'syn_lt syn::Ident);
impl<'syn_lt> From<&'syn_lt syn::Ident> for SynIdentifierRef<'syn_lt> {
    fn from(value: &'syn_lt syn::Ident) -> Self {
        Self(value)
    }
}
impl AsRef<syn::Ident> for SynIdentifierRef<'_> {
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
pub fn newtype(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(v) => v,
        Err(error) => return error.into_compile_error().into(),
    };
    match generate_newtype_token_stream(SynDeriveInputRef::from(&input)) {
        Ok(v) => v.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
#[proc_macro_derive(EnumFromStr)]
pub fn enum_from_str(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(v) => v,
        Err(error) => return error.into_compile_error().into(),
    };
    match generate_enum_from_str_token_stream(SynDeriveInputRef::from(&input)) {
        Ok(v) => v.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
#[proc_macro_derive(BoundedString, attributes(bounded_string))]
pub fn bounded_string(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(v) => v,
        Err(error) => return error.into_compile_error().into(),
    };
    match generate_bounded_string_token_stream(SynDeriveInputRef::from(&input)) {
        Ok(v) => v.into(),
        Err(error) => error.into_compile_error().into(),
    }
}
#[allow(clippy::single_call_fn)] // keeps top-level derive flow separate from token generation details
fn generate_newtype_token_stream(
    input: SynDeriveInputRef<'_>,
) -> syn::Result<ProcMacro2GeneratedTokenStream> {
    let input_ref = input.as_ref();
    let attrs = parse_newtype_attrs(SynAttrsRef::from(input_ref.attrs.as_slice()))?;
    validate_newtype_attrs(&attrs, input)?;
    let inner_ty = tuple_struct_one_field_ty(input)?;
    validate_newtype_inner_ty_attrs(&attrs, inner_ty)?;
    let inner_ty_ref = inner_ty.as_ref();
    let identifier = &input_ref.ident;
    let (impl_generics, ty_generics, where_clause) = input_ref.generics.split_for_impl();
    let debug_transparent_token_stream = attrs
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
            impl #debug_impl_generics std::fmt::Debug for #identifier #debug_ty_generics #debug_where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Debug::fmt(&self.0, f)
                }
            }
        }
    });
    let display_token_stream = attrs.contains(NewtypeOption::Display).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::fmt::Display for #identifier #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }
    });
    let as_ref_str_token_stream = attrs.contains(NewtypeOption::AsRefStr).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics AsRef<str> for #identifier #ty_generics #where_clause {
                fn as_ref(&self) -> &str {
                    AsRef::<str>::as_ref(&self.0)
                }
            }
        }
    });
    let as_ref_token_stream = attrs.contains(NewtypeOption::AsRef).get().then(|| {
        quote::quote! {
            impl #impl_generics #identifier #ty_generics #where_clause {
                #[must_use]
                pub const fn as_ref(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
        }
    });
    let as_ref_inner_token_stream = if attrs.contains(NewtypeOption::AsRefInner).get() {
        let syn::Type::Reference(inner_ref_ty) = inner_ty_ref else {
            return Err(syn::Error::new_spanned(
                inner_ty_ref,
                contract_constants::macro_diagnostics::AS_REF_INNER_SHARED_REF_ERROR,
            ));
        };
        let target_ty = &inner_ref_ty.elem;
        Some(quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics AsRef<#target_ty> for #identifier #ty_generics #where_clause {
                fn as_ref(&self) -> &#target_ty {
                    self.0
                }
            }
        })
    } else {
        None
    };
    let as_ref_owned_token_stream = attrs.contains(NewtypeOption::AsRefOwned).get().then(|| {
        quote::quote! {
            impl #impl_generics AsRef<#inner_ty_ref> for #identifier #ty_generics #where_clause {
                fn as_ref(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
        }
    });
    let as_ref_target_token_stream = attrs.contains(NewtypeOption::AsRefTarget).get().then(|| {
        quote::quote! {
            impl #impl_generics AsRef<<#inner_ty_ref as std::ops::Deref>::Target> for #identifier #ty_generics #where_clause
            where
                #inner_ty_ref: std::ops::Deref,
            {
                fn as_ref(&self) -> &<#inner_ty_ref as std::ops::Deref>::Target {
                    std::ops::Deref::deref(&self.0)
                }
            }
        }
    });
    let as_slice_token_stream = attrs.contains(NewtypeOption::AsSlice).get().then(|| {
        quote::quote! {
            impl #impl_generics #identifier #ty_generics #where_clause {
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
    let deref_inner_token_stream = attrs.contains(NewtypeOption::DerefInner).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::ops::Deref for #identifier #ty_generics #where_clause {
                type Target = #inner_ty_ref;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    });
    let deref_target_token_stream = attrs.contains(NewtypeOption::DerefTarget).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics std::ops::Deref for #identifier #ty_generics #where_clause {
                type Target = <#inner_ty_ref as std::ops::Deref>::Target;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    });
    let deref_mut_inner_token_stream =
        attrs.contains(NewtypeOption::DerefMutInner).get().then(|| {
            quote::quote! {
                #[allow(single_use_lifetimes)]
                impl #impl_generics std::ops::DerefMut for #identifier #ty_generics #where_clause {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }
            }
        });
    let deref_mut_target_token_stream = attrs.contains(NewtypeOption::DerefMutTarget).get().then(
        || {
            quote::quote! {
                #[allow(single_use_lifetimes)]
                impl #impl_generics std::ops::DerefMut for #identifier #ty_generics #where_clause {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }
            }
        },
    );
    let from_token_stream = attrs.contains(NewtypeOption::From).get().then(|| {
        quote::quote! {
            impl #impl_generics From<#inner_ty_ref> for #identifier #ty_generics #where_clause {
                fn from(value: #inner_ty_ref) -> Self {
                    Self(value)
                }
            }
        }
    });
    let getter_token_stream = attrs.contains(NewtypeOption::Getter).get().then(|| {
        let trait_identifier = quote::format_ident!("Get{identifier}");
        let fn_name = identifier_to_snake(SynIdentifierRef::from(identifier));
        let fn_identifier = quote::format_ident!("get_{}", fn_name.as_ref());
        quote::quote! {
            pub trait #trait_identifier {
                fn #fn_identifier(&self) -> &#inner_ty_ref;
            }
            impl #impl_generics #trait_identifier for #identifier #ty_generics #where_clause {
                fn #fn_identifier(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
            impl #impl_generics #trait_identifier for &#identifier #ty_generics #where_clause {
                fn #fn_identifier(&self) -> &#inner_ty_ref {
                    &self.0
                }
            }
        }
    });
    let into_inner_token_stream = attrs.contains(NewtypeOption::IntoInner).get().then(|| {
        quote::quote! {
            impl #impl_generics #identifier #ty_generics #where_clause {
                #[must_use]
                pub fn into_inner(self) -> #inner_ty_ref {
                    self.0
                }
            }
        }
    });
    let into_inner_from_token_stream =
        attrs.contains(NewtypeOption::IntoInnerFrom).get().then(|| {
            quote::quote! {
                impl #impl_generics From<#identifier #ty_generics> for #inner_ty_ref #where_clause {
                    fn from(value: #identifier #ty_generics) -> Self {
                        value.0
                    }
                }
            }
        });
    let into_vec_token_stream = attrs.contains(NewtypeOption::IntoVec).get().then(|| {
        quote::quote! {
            impl #impl_generics #identifier #ty_generics #where_clause {
                #[must_use]
                pub fn into_vec(self) -> #inner_ty_ref {
                    self.0
                }
            }
        }
    });
    let to_tokens_token_stream = attrs.contains(NewtypeOption::ToTokens).get().then(|| {
        quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics quote::ToTokens for #identifier #ty_generics #where_clause {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    quote::ToTokens::to_tokens(&self.0, tokens);
                }
            }
        }
    });
    let to_err_string_token_stream =
        generate_to_err_string_token_stream(&attrs, SynIdentifierRef::from(identifier));
    Ok(ProcMacro2GeneratedTokenStream(quote::quote! {
        #debug_transparent_token_stream
        #display_token_stream
        #as_ref_str_token_stream
        #as_ref_token_stream
        #as_ref_inner_token_stream
        #as_ref_owned_token_stream
        #as_ref_target_token_stream
        #as_slice_token_stream
        #deref_inner_token_stream
        #deref_target_token_stream
        #deref_mut_inner_token_stream
        #deref_mut_target_token_stream
        #from_token_stream
        #getter_token_stream
        #into_inner_token_stream
        #into_inner_from_token_stream
        #into_vec_token_stream
        #to_tokens_token_stream
        #to_err_string_token_stream
    }))
}
#[allow(clippy::single_call_fn)] // checked String wrapper generation is separate from forwarding newtype impls
fn generate_bounded_string_token_stream(
    input: SynDeriveInputRef<'_>,
) -> syn::Result<ProcMacro2GeneratedTokenStream> {
    let input_ref = input.as_ref();
    let attrs = parse_bounded_string_attrs(SynAttrsRef::from(input_ref.attrs.as_slice()))?;
    let inner_ty = tuple_struct_one_field_ty(input)?;
    if !type_path_ends_with_string_identifier(inner_ty).get() {
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
    let identifier = &input_ref.ident;
    let vis = &input_ref.vis;
    let error_identifier = quote::format_ident!("{identifier}TryFromStringError");
    let BoundedStringAttrs {
        description,
        max: max_option,
        min,
        options,
    } = attrs;
    let max = max_option.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            contract_constants::macro_diagnostics::BOUNDED_STRING_MAX_ERROR,
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
    let min_token_stream =
        min.unwrap_or_else(|| SynExpr::from(syn::Expr::Verbatim(quote::quote! { 0usize })));
    let normalize_token_stream =
        trim.then(|| quote::quote! { let value = value.trim().to_owned(); });
    let len_token_stream = if chars {
        quote::quote! { value.chars().count() }
    } else {
        quote::quote! { value.len() }
    };
    let nul_check_token_stream = nul_free.then(|| {
        quote::quote! {
            if value.contains('\0') {
                return Err(Self::Error::ContainsNul);
            }
        }
    });
    let serde_token_stream = serde.then(|| {
        quote::quote! {
            impl serde::Serialize for #identifier {
                fn serialize<Serializer>(&self, serializer: Serializer) -> Result<Serializer::Ok, Serializer::Error>
                where
                    Serializer: serde::Serializer,
                {
                    serde::Serialize::serialize(&self.0, serializer)
                }
            }
            impl<'de> serde::Deserialize<'de> for #identifier {
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
    let utoipa_token_stream = utoipa.then(|| {
        quote::quote! {
            impl<'schema_lt> utoipa::ToSchema<'schema_lt> for #identifier {
                fn schema() -> (
                    &'schema_lt str,
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                ) {
                    (
                        stringify!(#identifier),
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::SchemaType::String)
                            .min_length(Some(#min_token_stream))
                            .max_length(Some(#max))
                            .build()
                            .into(),
                    )
                }
            }
        }
    });
    let description_token_stream = description.map_or_else(
        || {
            let value = identifier_to_snake(SynIdentifierRef::from(identifier))
                .as_ref()
                .replace('_', " ");
            quote::quote! {#value}
        },
        |value| quote::quote! {#value},
    );
    Ok(ProcMacro2GeneratedTokenStream(quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #vis enum #error_identifier {
            InvalidBounds { min: usize, max: usize },
            TooShort { len: usize, min: usize },
            TooLong { len: usize, max: usize },
            ContainsNul,
        }
        impl std::fmt::Display for #error_identifier {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::InvalidBounds { min, max } => {
                        write!(f, "{} minimum length {min} exceeds maximum {max}", #description_token_stream)
                    }
                    Self::TooShort { len, min } => {
                        write!(f, "{} length {len} is below minimum {min}", #description_token_stream)
                    }
                    Self::TooLong { len, max } => {
                        write!(f, "{} length {len} exceeds maximum {max}", #description_token_stream)
                    }
                    Self::ContainsNul => write!(f, "{} contains a NUL character", #description_token_stream),
                }
            }
        }
        impl From<#error_identifier> for #identifier {
            fn from(value: #error_identifier) -> Self {
                Self(value.to_string())
            }
        }
        impl TryFrom<String> for #identifier {
            type Error = #error_identifier;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                #normalize_token_stream
                if #min_token_stream > #max {
                    return Err(Self::Error::InvalidBounds { min: #min_token_stream, max: #max });
                }
                #nul_check_token_stream
                let len = #len_token_stream;
                if len < #min_token_stream {
                    return Err(Self::Error::TooShort { len, min: #min_token_stream });
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
        #serde_token_stream
        #utoipa_token_stream
    }))
}
#[allow(clippy::single_call_fn)] // keeps enum parsing derive independent from newtype tuple-struct generation
fn generate_enum_from_str_token_stream(
    input: SynDeriveInputRef<'_>,
) -> syn::Result<ProcMacro2GeneratedTokenStream> {
    let input_ref = input.as_ref();
    let identifier = &input_ref.ident;
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
                identifier_to_snake(SynIdentifierRef::from(&variant.ident)),
            ))
        })
        .collect::<syn::Result<Vec<(&syn::Ident, SnakeIdentifier)>>>()?;
    let allowed_values = variants
        .iter()
        .map(|(_, name)| name.as_ref())
        .collect::<Vec<&str>>()
        .join(", ");
    let arms = variants.iter().map(|(variant_identifier, name)| {
        let name_ref = name.as_ref();
        quote::quote! {
            v if v.eq_ignore_ascii_case(#name_ref) => Ok(Self::#variant_identifier),
        }
    });
    Ok(ProcMacro2GeneratedTokenStream(quote::quote! {
        impl std::str::FromStr for #identifier {
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
                                meta.error(contract_constants::macro_diagnostics::DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident("nul_free") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::NulFree, || {
                                meta.error(contract_constants::macro_diagnostics::DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident("serde") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Serde, || {
                                meta.error(contract_constants::macro_diagnostics::DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident("trim") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Trim, || {
                                meta.error(contract_constants::macro_diagnostics::DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident("utoipa") {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Utoipa, || {
                                meta.error(contract_constants::macro_diagnostics::DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
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
            contract_constants::macro_diagnostics::BOUNDED_STRING_MAX_ERROR,
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
        .try_fold(NewtypeAttrs::default(), |mut accumulator, attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("as_ref_str") {
                    return accumulator
                        .try_insert(NewtypeOption::AsRefStr, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("as_ref") {
                    return accumulator
                        .try_insert(NewtypeOption::AsRef, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("as_ref_inner") {
                    return accumulator.try_insert(
                        NewtypeOption::AsRefInner,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("as_ref_owned") {
                    return accumulator.try_insert(
                        NewtypeOption::AsRefOwned,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("as_ref_target") {
                    return accumulator.try_insert(
                        NewtypeOption::AsRefTarget,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("as_slice") {
                    return accumulator
                        .try_insert(NewtypeOption::AsSlice, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("deref") || meta.path.is_ident("deref_target") {
                    return accumulator.try_insert(
                        NewtypeOption::DerefTarget,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("deref_inner") {
                    return accumulator.try_insert(
                        NewtypeOption::DerefInner,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("deref_mut_inner") {
                    return accumulator.try_insert(
                        NewtypeOption::DerefMutInner,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("deref_mut_target") {
                    return accumulator.try_insert(
                        NewtypeOption::DerefMutTarget,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("debug_transparent") {
                    return accumulator.try_insert(
                        NewtypeOption::DebugTransparent,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("display") {
                    return accumulator
                        .try_insert(NewtypeOption::Display, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("from") || meta.path.is_ident("from_inner") {
                    return accumulator
                        .try_insert(NewtypeOption::From, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("getter") {
                    return accumulator
                        .try_insert(NewtypeOption::Getter, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("into_inner") {
                    return accumulator
                        .try_insert(NewtypeOption::IntoInner, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("into_inner_from") {
                    return accumulator.try_insert(
                        NewtypeOption::IntoInnerFrom,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("into_vec") {
                    return accumulator
                        .try_insert(NewtypeOption::IntoVec, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("secret") {
                    return accumulator
                        .try_insert(NewtypeOption::Secret, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("to_tokens") {
                    return accumulator
                        .try_insert(NewtypeOption::ToTokens, SynParseNestedMetaRef::from(&meta));
                }
                if meta.path.is_ident("to_err_string")
                    || meta.path.is_ident("to_err_string_display")
                {
                    return accumulator.set_to_err_string_mode(
                        ToErrStringMode::Display,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("to_err_string_as_ref_str") {
                    return accumulator.set_to_err_string_mode(
                        ToErrStringMode::AsRefStr,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                if meta.path.is_ident("to_err_string_debug") {
                    return accumulator.set_to_err_string_mode(
                        ToErrStringMode::Debug,
                        SynParseNestedMetaRef::from(&meta),
                    );
                }
                Err(meta.error("unknown newtype option"))
            })?;
            Ok(accumulator)
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
            contract_constants::macro_diagnostics::AS_REF_INNER_SHARED_REF_ERROR,
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
    if attrs.contains(NewtypeOption::From).get()
        && type_path_ends_with_string_identifier(inner_ty).get()
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
            syn::Error::new_spanned(
                input_ref,
                contract_constants::macro_diagnostics::TUPLE_STRUCT_ERROR,
            )
        })?;
    let unnamed = match shape {
        workspace_macro_helpers::SynStructShapeRef::Tuple(v) => &v.get().unnamed,
        workspace_macro_helpers::SynStructShapeRef::Named(_)
        | workspace_macro_helpers::SynStructShapeRef::Unit => {
            return Err(syn::Error::new_spanned(
                input_ref,
                contract_constants::macro_diagnostics::TUPLE_STRUCT_ERROR,
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
#[allow(clippy::single_call_fn)] // newtype validation only needs terminal path identifier matching for concrete String wrappers
fn type_path_ends_with_string_identifier(ty: SynTypeRef<'_>) -> NewtypeBool {
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
fn identifier_to_snake(identifier: SynIdentifierRef<'_>) -> SnakeIdentifier {
    let (out, _) = identifier.as_ref().to_string().chars().fold(
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
    SnakeIdentifier::try_from(out).expect("2e7a9c4f")
}
#[allow(clippy::single_call_fn)] // ToErrString code generation has distinct modes from base newtype impls
fn generate_to_err_string_token_stream(
    attrs: &NewtypeAttrs,
    identifier: SynIdentifierRef<'_>,
) -> Option<ProcMacro2GeneratedTokenStream> {
    let ident_ref = identifier.as_ref();
    attrs.to_err_string_mode.map(|mode| match mode {
        ToErrStringMode::AsRefStr => {
            ProcMacro2GeneratedTokenStream(quote::quote! {
                impl to_err_string::ToErrString for #ident_ref {
                    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
                        to_err_string::ToErrStringValue::try_from(AsRef::<str>::as_ref(&self.0).to_owned()).unwrap_or_else(to_err_string::ToErrStringValue::from)
                    }
                }
            })
        }
        ToErrStringMode::Debug => {
            ProcMacro2GeneratedTokenStream(quote::quote! {
                impl to_err_string::ToErrString for #ident_ref {
                    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
                        to_err_string::ToErrStringValue::try_from(format!("{:?}", self.0)).unwrap_or_else(to_err_string::ToErrStringValue::from)
                    }
                }
            })
        }
        ToErrStringMode::Display => {
            ProcMacro2GeneratedTokenStream(quote::quote! {
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
        let result =
            super::generate_bounded_string_token_stream(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "29f8ddc2");
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
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
        let result =
            super::generate_bounded_string_token_stream(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "da6f2151");
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
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
        let bounded_result = super::generate_bounded_string_token_stream(
            super::SynDeriveInputRef::from(&bounded_input),
        );
        if let Err(error) = bounded_result {
            assert_eq!(error.to_string(), "duplicate bounded_string option");
        } else {
            panic!("d03ced5c");
        }
        let newtype_input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(display, display)]
            struct Value(String);
        };
        let newtype_result =
            super::generate_newtype_token_stream(super::SynDeriveInputRef::from(&newtype_input));
        if let Err(error) = newtype_result {
            assert_eq!(error.to_string(), "duplicate newtype option");
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
        let result = super::generate_newtype_token_stream(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "d73a920b");
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
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
        let result = super::generate_newtype_token_stream(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "33c7e891");
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
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
        let result = super::generate_newtype_token_stream(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "f9b7c2a1");
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
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
        let result = super::generate_newtype_token_stream(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "46d9f064");
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                "secret cannot be combined with formatting, token, or error-string forwarding"
            );
        }
    }
}
