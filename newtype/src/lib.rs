const SNAKE_IDENT_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Default)]
struct NewtypeAttrs {
    options: std::collections::BTreeSet<NewtypeOption>,
    to_err_string_mode: Option<ToErrStringMode>,
}
struct BoundedStringAttrs {
    description: SynLitStr,
    max: SynExpr,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NewtypeOption {
    AsRef,
    AsRefStr,
    AsSlice,
    Deref,
    Display,
    From,
    Getter,
    IntoInner,
    IntoVec,
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
impl std::fmt::Debug for SnakeIdentLen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
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
struct SynLitStr(syn::LitStr);
impl From<syn::LitStr> for SynLitStr {
    fn from(value: syn::LitStr) -> Self {
        Self(value)
    }
}
impl AsRef<syn::LitStr> for SynLitStr {
    fn as_ref(&self) -> &syn::LitStr {
        &self.0
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
        NewtypeBool(self.options.contains(&option))
    }
    fn insert(&mut self, option: NewtypeOption) {
        let _: bool = self.options.insert(option);
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
    let display_ts = attrs.contains(NewtypeOption::Display).get().then(|| {
        quote::quote! {
            impl #impl_generics std::fmt::Display for #ident #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }
    });
    let as_ref_str_ts = attrs.contains(NewtypeOption::AsRefStr).get().then(|| {
        quote::quote! {
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
    let deref_ts = attrs.contains(NewtypeOption::Deref).get().then(|| {
        quote::quote! {
            impl #impl_generics std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = <#inner_ty_ref as std::ops::Deref>::Target;
                fn deref(&self) -> &Self::Target {
                    &self.0
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
    let to_err_string_ts = gen_to_err_string_ts(&attrs, SynIdentRef::from(ident));
    Ok(ProcMacro2GeneratedTs(quote::quote! {
        #display_ts
        #as_ref_str_ts
        #as_ref_ts
        #as_slice_ts
        #deref_ts
        #from_ts
        #getter_ts
        #into_inner_ts
        #into_vec_ts
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
    let er_ident = quote::format_ident!("{ident}TryFromStringEr");
    let max = attrs.max;
    let description = attrs.description.as_ref().value();
    Ok(ProcMacro2GeneratedTs(quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #er_ident {
            TooLong { len: usize, max: usize },
        }
        impl std::fmt::Display for #er_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::TooLong { len, max } => {
                        write!(f, "{} length {len} exceeds maximum {max}", #description)
                    }
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
                if value.len() > #max {
                    return Err(Self::Error::TooLong {
                        len: value.len(),
                        max: #max,
                    });
                }
                Ok(Self(value))
            }
        }
        impl AsRef<str> for #ident {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.0.as_str())
            }
        }
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
    let (max, description) = attrs
        .as_ref()
        .iter()
        .filter(|attr| attr.path().is_ident("bounded_string"))
        .try_fold((None, None), |(mut max, mut description), attr| {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("max") {
                    max = Some(SynExpr::from(meta.value()?.parse::<syn::Expr>()?));
                    return Ok(());
                }
                if meta.path.is_ident("description") {
                    description = Some(SynLitStr::from(meta.value()?.parse::<syn::LitStr>()?));
                    return Ok(());
                }
                Err(meta.error("unknown bounded_string option"))
            })?;
            Ok::<(Option<SynExpr>, Option<SynLitStr>), syn::Error>((max, description))
        })?;
    let parsed_max = max.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "BoundedString requires #[bounded_string(max = ...)]",
        )
    })?;
    let parsed_description = description.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "BoundedString requires #[bounded_string(description = \"...\")]",
        )
    })?;
    Ok(BoundedStringAttrs {
        description: parsed_description,
        max: parsed_max,
    })
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
                    acc.insert(NewtypeOption::AsRefStr);
                    return Ok(());
                }
                if meta.path.is_ident("as_ref") {
                    acc.insert(NewtypeOption::AsRef);
                    return Ok(());
                }
                if meta.path.is_ident("as_slice") {
                    acc.insert(NewtypeOption::AsSlice);
                    return Ok(());
                }
                if meta.path.is_ident("deref") {
                    acc.insert(NewtypeOption::Deref);
                    return Ok(());
                }
                if meta.path.is_ident("display") {
                    acc.insert(NewtypeOption::Display);
                    return Ok(());
                }
                if meta.path.is_ident("from") {
                    acc.insert(NewtypeOption::From);
                    return Ok(());
                }
                if meta.path.is_ident("getter") {
                    acc.insert(NewtypeOption::Getter);
                    return Ok(());
                }
                if meta.path.is_ident("into_inner") {
                    acc.insert(NewtypeOption::IntoInner);
                    return Ok(());
                }
                if meta.path.is_ident("into_vec") {
                    acc.insert(NewtypeOption::IntoVec);
                    return Ok(());
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
    if attrs.options.is_empty() && attrs.to_err_string_mode.is_none() {
        return Err(syn::Error::new_spanned(
            input.as_ref(),
            "Newtype requires at least one #[newtype(...)] option",
        ));
    }
    Ok(())
}
#[allow(clippy::single_call_fn)] // string wrapper policy belongs to newtype validation before From impl generation
fn validate_newtype_inner_ty_attrs(
    attrs: &NewtypeAttrs,
    inner_ty: SynTypeRef<'_>,
) -> syn::Result<()> {
    if attrs.contains(NewtypeOption::From).get() && type_path_ends_with_string_ident(inner_ty).get()
    {
        return Err(syn::Error::new_spanned(
            inner_ty.as_ref(),
            "#[newtype(from)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead",
        ));
    }
    Ok(())
}
#[allow(clippy::single_call_fn)] // tuple field extraction is separate to keep derive input validation explicit
fn tuple_struct_one_field_ty(input: SynDeriveInputRef<'_>) -> syn::Result<SynTypeRef<'_>> {
    let input_ref = input.0;
    let data_struct = match &input_ref.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input_ref,
                "Newtype supports only tuple structs",
            ));
        }
    };
    let unnamed = match &data_struct.fields {
        syn::Fields::Unnamed(v) => &v.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => {
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
    fn newtype_from_string_returns_compile_error() {
        let input = syn::parse_quote! {
            #[derive(Newtype)]
            #[newtype(from)]
            struct Name(String);
        };
        let result = super::gen_newtype_ts(super::SynDeriveInputRef::from(&input));
        assert!(result.is_err(), "f9b7c2a1");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "#[newtype(from)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead"
            );
        }
    }
}
