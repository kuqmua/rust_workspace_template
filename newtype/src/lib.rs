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
    try_from: Option<NewtypeTryFromAttrs>,
}
#[derive(Debug)]
struct NewtypeTryFromAttrs {
    error: Option<SynType>,
    validator: SynExpr,
}
struct BoundedStringAttrs {
    description: Option<SynExpr>,
    max: Option<SynExpr>,
    min: Option<SynExpr>,
    options: workspace_macro_helpers::StdUniqueOptionSet<BoundedStringOption>,
    validator: Option<SynExpr>,
}
struct WireEnumAttrs {
    error_message: SynExpr,
    ref_type: SynIdentifier,
}
impl syn::parse::Parse for WireEnumAttrs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut error_message = None;
        let mut ref_type = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == str_constants::WIRE_ENUM_REF_TYPE {
                ref_type = Some(SynIdentifier(input.parse::<syn::Ident>()?));
            } else if name == str_constants::WIRE_ENUM_ERROR_MESSAGE {
                error_message = Some(SynExpr(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    str_constants::WIRE_ENUM_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            error_message: error_message
                .ok_or_else(|| input.error(str_constants::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
            ref_type: ref_type
                .ok_or_else(|| input.error(str_constants::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
        })
    }
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
    AsMut,
    AsRef,
    AsRefInner,
    AsRefOwned,
    AsRefStr,
    AsRefTarget,
    AsSlice,
    DebugRedacted,
    DebugTransparent,
    DerefInner,
    DerefMutInner,
    DerefMutTarget,
    DerefTarget,
    Display,
    ErrorTransparent,
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
struct ProcMacroInputTokenStream(proc_macro::TokenStream);
impl From<proc_macro::TokenStream> for ProcMacroInputTokenStream {
    fn from(value: proc_macro::TokenStream) -> Self {
        Self(value)
    }
}
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
struct SynIdentifier(syn::Ident);
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
#[derive(Debug)]
struct SynType(syn::Type);
#[derive(Debug)]
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
impl NewtypeAttrs {
    fn contains(&self, option: NewtypeOption) -> NewtypeBool {
        NewtypeBool(self.options.contains(option).get())
    }
}
fn derive_newtype_option(
    input_token_stream: ProcMacroInputTokenStream,
    option: NewtypeOption,
    to_err_string_mode: Option<ToErrStringMode>,
) -> ProcMacro2GeneratedTokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream.0) {
        Ok(v) => v,
        Err(error) => return ProcMacro2GeneratedTokenStream(error.into_compile_error()),
    };
    let mut attrs = NewtypeAttrs {
        options: workspace_macro_helpers::StdUniqueOptionSet::default(),
        to_err_string_mode,
        try_from: None,
    };
    if let Err(error) = attrs.options.try_insert_with(option, || {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::DUPLICATE_NEWTYPE_OPTION,
        )
    }) {
        return ProcMacro2GeneratedTokenStream(error.into_compile_error());
    }
    match generate_newtype_token_stream_with_attrs(SynDeriveInputRef::from(&input), &attrs) {
        Ok(v) => v,
        Err(error) => ProcMacro2GeneratedTokenStream(error.into_compile_error()),
    }
}
#[allow(clippy::single_call_fn)] // keeps TryFrom attribute parsing separate from its proc-macro entry point
fn derive_newtype_try_from(
    input_token_stream: ProcMacroInputTokenStream,
) -> ProcMacro2GeneratedTokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream.0) {
        Ok(v) => v,
        Err(error) => return ProcMacro2GeneratedTokenStream(error.into_compile_error()),
    };
    let mut error_opt = None;
    let mut validator_opt = None;
    let parse_result = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident(str_constants::NEWTYPE_TRY_FROM))
        .try_for_each(|attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(str_constants::NEWTYPE_TRY_FROM_ERROR) {
                    if error_opt.is_some() {
                        return Err(meta.error(str_constants::NEWTYPE_TRY_FROM_ERROR_DUPLICATE));
                    }
                    error_opt = Some(SynType(meta.value()?.parse::<syn::Type>()?));
                    return Ok(());
                }
                if meta
                    .path
                    .is_ident(str_constants::NEWTYPE_TRY_FROM_VALIDATOR)
                {
                    if validator_opt.is_some() {
                        return Err(meta.error(str_constants::NEWTYPE_TRY_FROM_VALIDATOR_DUPLICATE));
                    }
                    validator_opt = Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                    return Ok(());
                }
                Err(meta.error(str_constants::NEWTYPE_TRY_FROM_UNKNOWN_OPTION))
            })
        });
    if let Err(error) = parse_result {
        return ProcMacro2GeneratedTokenStream(error.into_compile_error());
    }
    let Some(validator) = validator_opt else {
        return ProcMacro2GeneratedTokenStream(
            syn::Error::new_spanned(&input, str_constants::NEWTYPE_TRY_FROM_VALIDATOR_REQUIRED)
                .into_compile_error(),
        );
    };
    let attrs = NewtypeAttrs {
        options: workspace_macro_helpers::StdUniqueOptionSet::default(),
        to_err_string_mode: None,
        try_from: Some(NewtypeTryFromAttrs {
            error: error_opt,
            validator,
        }),
    };
    match generate_newtype_token_stream_with_attrs(SynDeriveInputRef::from(&input), &attrs) {
        Ok(v) => v,
        Err(error) => ProcMacro2GeneratedTokenStream(error.into_compile_error()),
    }
}
#[proc_macro_derive(AsMut)]
pub fn as_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsMut,
        None,
    )
    .into()
}
#[proc_macro_derive(AsRef)]
pub fn as_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsRef,
        None,
    )
    .into()
}
#[proc_macro_derive(AsRefInner)]
pub fn as_ref_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsRefInner,
        None,
    )
    .into()
}
#[proc_macro_derive(AsRefOwned)]
pub fn as_ref_owned(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsRefOwned,
        None,
    )
    .into()
}
#[proc_macro_derive(AsRefStr)]
pub fn as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsRefStr,
        None,
    )
    .into()
}
#[proc_macro_derive(AsRefTarget)]
pub fn as_ref_target(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsRefTarget,
        None,
    )
    .into()
}
#[proc_macro_derive(AsSlice)]
pub fn as_slice(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::AsSlice,
        None,
    )
    .into()
}
#[proc_macro_derive(DebugRedacted)]
pub fn debug_redacted(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::DebugRedacted,
        None,
    )
    .into()
}
#[proc_macro_derive(DebugTransparent)]
pub fn debug_transparent(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::DebugTransparent,
        None,
    )
    .into()
}
#[proc_macro_derive(DerefInner)]
pub fn deref_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::DerefInner,
        None,
    )
    .into()
}
#[proc_macro_derive(DerefMutInner)]
pub fn deref_mut_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::DerefMutInner,
        None,
    )
    .into()
}
#[proc_macro_derive(DerefMutTarget)]
pub fn deref_mut_target(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::DerefMutTarget,
        None,
    )
    .into()
}
#[proc_macro_derive(DerefTarget)]
pub fn deref_target(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::DerefTarget,
        None,
    )
    .into()
}
#[proc_macro_derive(Display)]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::Display,
        None,
    )
    .into()
}
#[proc_macro_derive(ErrorTransparent)]
pub fn error_transparent(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::ErrorTransparent,
        None,
    )
    .into()
}
#[proc_macro_derive(FromInner)]
pub fn from_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::From,
        None,
    )
    .into()
}
#[proc_macro_derive(Getter)]
pub fn getter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::Getter,
        None,
    )
    .into()
}
#[proc_macro_derive(IntoInner)]
pub fn into_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::IntoInner,
        None,
    )
    .into()
}
#[proc_macro_derive(IntoInnerFrom)]
pub fn into_inner_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::IntoInnerFrom,
        None,
    )
    .into()
}
#[proc_macro_derive(IntoVec)]
pub fn into_vec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::IntoVec,
        None,
    )
    .into()
}
#[proc_macro_derive(ToTokens)]
pub fn to_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::ToTokens,
        None,
    )
    .into()
}
#[proc_macro_derive(ToErrString)]
pub fn to_err_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::Secret,
        Some(ToErrStringMode::Display),
    )
    .into()
}
#[proc_macro_derive(ToErrStringAsRefStr)]
pub fn to_err_string_as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::Secret,
        Some(ToErrStringMode::AsRefStr),
    )
    .into()
}
#[proc_macro_derive(ToErrStringDebug)]
pub fn to_err_string_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_option(
        ProcMacroInputTokenStream::from(input),
        NewtypeOption::Secret,
        Some(ToErrStringMode::Debug),
    )
    .into()
}
#[proc_macro_derive(TryFrom, attributes(try_from))]
pub fn try_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_newtype_try_from(ProcMacroInputTokenStream::from(input)).into()
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
#[proc_macro_derive(WireEnum, attributes(wire_enum, wire))]
pub fn wire_enum(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse::<syn::DeriveInput>(input_token_stream) {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let Some(attribute) = input
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(str_constants::WIRE_ENUM))
    else {
        return syn::Error::new_spanned(input.ident, str_constants::WIRE_ENUM_REQUIRES_ATTRIBUTE)
            .to_compile_error()
            .into();
    };
    let attrs = match attribute.parse_args::<WireEnumAttrs>() {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let syn::Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(
            &input.ident,
            str_constants::WIRE_ENUM_SUPPORTS_UNIT_VARIANTS,
        )
        .to_compile_error()
        .into();
    };
    let parsed_variants_result = data_enum.variants.iter().try_fold(
        (
            std::collections::BTreeSet::new(),
            Vec::<(&syn::Ident, syn::LitStr)>::new(),
        ),
        |(mut unique_values, mut parsed), variant| {
            if !matches!(variant.fields, syn::Fields::Unit) {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    str_constants::WIRE_ENUM_SUPPORTS_UNIT_VARIANTS,
                ));
            }
            let wire_attribute = variant
                .attrs
                .iter()
                .find(|candidate| candidate.path().is_ident(str_constants::WIRE_ENUM_WIRE))
                .ok_or_else(|| {
                    syn::Error::new_spanned(
                        &variant.ident,
                        str_constants::WIRE_ENUM_VARIANT_REQUIRES_WIRE,
                    )
                })?;
            let value = wire_attribute.parse_args::<syn::LitStr>()?;
            if !unique_values.insert(value.value()) {
                return Err(syn::Error::new_spanned(
                    value,
                    str_constants::WIRE_ENUM_DUPLICATE_VALUE,
                ));
            }
            parsed.push((&variant.ident, value));
            Ok((unique_values, parsed))
        },
    );
    let (_, parsed_variants) = match parsed_variants_result {
        Ok(value) => value,
        Err(error) => return error.to_compile_error().into(),
    };
    let identifiers = parsed_variants
        .iter()
        .map(|(identifier, _value)| *identifier)
        .collect::<Vec<_>>();
    let values = parsed_variants
        .iter()
        .map(|(_identifier, value)| value)
        .collect::<Vec<_>>();
    let identifier = &input.ident;
    let error_identifier = quote::format_ident!("{}TryFromStrError", identifier);
    let error_message = attrs.error_message.0;
    let ref_type = attrs.ref_type.0;
    let variant_count = identifiers.len();
    quote::quote! {
        impl #identifier {
            pub const ALL: [Self; #variant_count] = [#(Self::#identifiers),*];
            #[must_use]
            pub fn as_str(self) -> #ref_type<'static> {
                match self {
                    #(Self::#identifiers => #ref_type::from(#values)),*
                }
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct #error_identifier;
        impl std::fmt::Display for #error_identifier {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(#error_message)
            }
        }
        impl std::error::Error for #error_identifier {}
        impl TryFrom<&str> for #identifier {
            type Error = #error_identifier;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    #(#values => Ok(Self::#identifiers),)*
                    _value => Err(#error_identifier),
                }
            }
        }
        impl serde::Serialize for #identifier {
            fn serialize<Serializer>(
                &self,
                serializer: Serializer,
            ) -> Result<Serializer::Ok, Serializer::Error>
            where
                Serializer: serde::Serializer,
            {
                serializer.serialize_str(self.as_str().as_ref())
            }
        }
    }
    .into()
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
fn generate_newtype_token_stream_with_attrs(
    input: SynDeriveInputRef<'_>,
    attrs: &NewtypeAttrs,
) -> syn::Result<ProcMacro2GeneratedTokenStream> {
    let input_ref = input.as_ref();
    let inner_ty = tuple_struct_one_field_ty(input)?;
    validate_newtype_inner_ty_attrs(attrs, inner_ty)?;
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
    let debug_redacted_token_stream =
        attrs.contains(NewtypeOption::DebugRedacted).get().then(|| {
            let redacted = str_constants::REDACTED_ALT_3;
            quote::quote! {
                #[allow(single_use_lifetimes)]
                impl #impl_generics std::fmt::Debug for #identifier #ty_generics #where_clause {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_tuple(stringify!(#identifier))
                            .field(&#redacted)
                            .finish()
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
    let error_transparent_token_stream = attrs
        .contains(NewtypeOption::ErrorTransparent)
        .get()
        .then(|| {
            quote::quote! {
                #[allow(single_use_lifetimes)]
                impl #impl_generics std::error::Error for #identifier #ty_generics #where_clause {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(&self.0)
                    }
                }
            }
        });
    let as_mut_token_stream = if attrs.contains(NewtypeOption::AsMut).get() {
        let syn::Type::Reference(inner_ref_ty) = inner_ty_ref else {
            return Err(syn::Error::new_spanned(
                inner_ty_ref,
                str_constants::NEWTYPE_AS_MUT_REQUIRES_MUTABLE_REFERENCE_INNER_TYPE,
            ));
        };
        if inner_ref_ty.mutability.is_none() {
            return Err(syn::Error::new_spanned(
                inner_ty_ref,
                str_constants::NEWTYPE_AS_MUT_REQUIRES_MUTABLE_REFERENCE_INNER_TYPE,
            ));
        }
        let target_ty = &inner_ref_ty.elem;
        Some(quote::quote! {
            #[allow(single_use_lifetimes)]
            impl #impl_generics AsMut<#target_ty> for #identifier #ty_generics #where_clause {
                fn as_mut(&mut self) -> &mut #target_ty {
                    self.0
                }
            }
        })
    } else {
        None
    };
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
                str_constants::MACRO_DIAGNOSTICS_AS_REF_INNER_SHARED_REF_ERROR,
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
    let try_from_token_stream = attrs.try_from.as_ref().map(|try_from| {
        let inferred_error = syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path::from(quote::format_ident!("{identifier}Error")),
        });
        let error = try_from
            .error
            .as_ref()
            .map_or(&inferred_error, |value| &value.0);
        let validator = &try_from.validator;
        quote::quote! {
            impl #impl_generics TryFrom<#inner_ty_ref> for #identifier #ty_generics #where_clause {
                type Error = #error;
                fn try_from(value: #inner_ty_ref) -> Result<Self, Self::Error> {
                    (#validator)(&value)?;
                    Ok(Self(value))
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
        generate_to_err_string_token_stream(attrs, SynIdentifierRef::from(identifier));
    Ok(ProcMacro2GeneratedTokenStream(quote::quote! {
        #debug_transparent_token_stream
        #debug_redacted_token_stream
        #display_token_stream
        #error_transparent_token_stream
        #as_mut_token_stream
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
        #try_from_token_stream
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
            str_constants::BOUNDEDSTRING_SUPPORTS_ONLY_STRING_TUPLE_STRUCTS,
        ));
    }
    if !input_ref.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input_ref.generics,
            str_constants::BOUNDEDSTRING_DOES_NOT_SUPPORT_GENERICS,
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
        validator,
    } = attrs;
    let max = max_option.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::MACRO_DIAGNOSTICS_BOUNDED_STRING_MAX_ERROR,
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
            str_constants::BOUNDEDSTRING_UTOIPA_REQUIRES_CHARS_SO_OPENAPI_LENGTH_SEMANTICS_MATCH_RUNTIME,
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
    let validator_check_token_stream = validator.map(|validator_expression| {
        quote::quote! {
                if !(#validator_expression)(&value) {
                return Err(Self::Error::InvalidValue);
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
                .replace('_', str_constants::SPACE);
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
            InvalidValue,
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
                    Self::InvalidValue => write!(f, "{} has an invalid value", #description_token_stream),
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
                #validator_check_token_stream
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
                str_constants::ENUMFROMSTR_SUPPORTS_ONLY_ENUMS,
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
                    str_constants::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS,
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
        .join(str_constants::TEXT_ALT_6);
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
        .filter(|attr| attr.path().is_ident(str_constants::BOUNDED_STRING))
        .try_fold(
            BoundedStringAttrs {
                description: None,
                max: None,
                min: None,
                options: workspace_macro_helpers::StdUniqueOptionSet::default(),
                validator: None,
            },
            |mut parsed, attr| {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident(str_constants::MAX) {
                        parsed.max = Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::MIN) {
                        parsed.min = Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::DESCRIPTION) {
                        parsed.description =
                            Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::NEWTYPE_TRY_FROM_VALIDATOR) {
                        if parsed.validator.is_some() {
                            return Err(meta.error(
                                str_constants::NEWTYPE_TRY_FROM_VALIDATOR_DUPLICATE,
                            ));
                        }
                        parsed.validator =
                            Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                        return Ok(());
                    }
                    if meta.path.is_ident(str_constants::CHARS) {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Chars, || {
                                meta.error(str_constants::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident(str_constants::NUL_FREE) {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::NulFree, || {
                                meta.error(str_constants::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident(str_constants::SERDE) {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Serde, || {
                                meta.error(str_constants::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident(str_constants::TRIM) {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Trim, || {
                                meta.error(str_constants::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    if meta.path.is_ident(str_constants::UTOIPA) {
                        return parsed
                            .options
                            .try_insert_with(BoundedStringOption::Utoipa, || {
                                meta.error(str_constants::MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR)
                            });
                    }
                    Err(meta.error(str_constants::UNKNOWN_BOUNDED_STRING_OPTION))
                })?;
                Ok::<BoundedStringAttrs, syn::Error>(parsed)
            },
        )?;
    if parsed.max.is_none() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            str_constants::MACRO_DIAGNOSTICS_BOUNDED_STRING_MAX_ERROR,
        ));
    }
    Ok(parsed)
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
            str_constants::MACRO_DIAGNOSTICS_AS_REF_INNER_SHARED_REF_ERROR,
        ));
    }
    if attrs.contains(NewtypeOption::AsRefOwned).get()
        && matches!(inner_ty.as_ref(), syn::Type::Reference(_))
    {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            str_constants::NEWTYPE_AS_REF_OWNED_DOES_NOT_SUPPORT_REFERENCE_INNER_TYPES_USE_AS,
        ));
    }
    if attrs.contains(NewtypeOption::From).get()
        && type_path_ends_with_string_identifier(inner_ty).get()
    {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            str_constants::NEWTYPE_FROM_INNER_CANNOT_BE_USED_FOR_STRING_WRAPPERS_IMPLEMENT_TRYFROM_STRING,
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
                str_constants::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR,
            )
        })?;
    let unnamed = match shape {
        workspace_macro_helpers::SynStructShapeRef::Tuple(v) => &v.get().unnamed,
        workspace_macro_helpers::SynStructShapeRef::Named(_)
        | workspace_macro_helpers::SynStructShapeRef::Unit => {
            return Err(syn::Error::new_spanned(
                input_ref,
                str_constants::MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR,
            ));
        }
    };
    if unnamed.len() != 1 {
        return Err(syn::Error::new_spanned(
            input_ref,
            str_constants::NEWTYPE_SUPPORTS_ONLY_ONE_FIELD_TUPLE_STRUCTS,
        ));
    }
    unnamed
        .first()
        .map(|field| SynTypeRef::from(&field.ty))
        .ok_or_else(|| syn::Error::new_spanned(input_ref, str_constants::NEWTYPE_FIELD_NOT_FOUND))
}
#[allow(clippy::single_call_fn)] // newtype validation only needs terminal path identifier matching for concrete String wrappers
fn type_path_ends_with_string_identifier(ty: SynTypeRef<'_>) -> NewtypeBool {
    NewtypeBool(match ty.as_ref() {
        syn::Type::Path(v) if v.qself.is_none() => v
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == str_constants::STRING),
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
    }
}
