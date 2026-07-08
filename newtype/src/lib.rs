#[derive(Debug, Default)]
struct NewtypeAttrs {
    options: std::collections::BTreeSet<NewtypeOption>,
    to_err_string_mode: Option<ToErrStringMode>,
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
impl NewtypeAttrs {
    fn contains(&self, option: NewtypeOption) -> bool {
        self.options.contains(&option)
    }
    fn insert(&mut self, option: NewtypeOption) {
        let _: bool = self.options.insert(option);
    }
    fn set_to_err_string_mode(
        &mut self,
        mode: ToErrStringMode,
        meta: &syn::meta::ParseNestedMeta<'_>,
    ) -> syn::Result<()> {
        if self.to_err_string_mode.replace(mode).is_some() {
            return Err(meta.error("only one to_err_string mode can be selected"));
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
    match gen_newtype_ts(&input) {
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
    match gen_enum_from_str_ts(&input) {
        Ok(v) => v.into(),
        Err(er) => er.into_compile_error().into(),
    }
}
#[allow(clippy::single_call_fn)] // keeps top-level derive flow separate from token generation details
fn gen_newtype_ts(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let attrs = parse_newtype_attrs(&input.attrs)?;
    validate_newtype_attrs(&attrs, input)?;
    let inner_ty = tuple_struct_one_field_ty(input)?;
    validate_newtype_inner_ty_attrs(&attrs, inner_ty)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let display_ts = attrs.contains(NewtypeOption::Display).then(|| {
        quote::quote! {
            impl #impl_generics std::fmt::Display for #ident #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }
    });
    let as_ref_str_ts = attrs.contains(NewtypeOption::AsRefStr).then(|| {
        quote::quote! {
            impl #impl_generics AsRef<str> for #ident #ty_generics #where_clause {
                fn as_ref(&self) -> &str {
                    AsRef::<str>::as_ref(&self.0)
                }
            }
        }
    });
    let as_ref_ts = attrs.contains(NewtypeOption::AsRef).then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub const fn as_ref(&self) -> &#inner_ty {
                    &self.0
                }
            }
        }
    });
    let as_slice_ts = attrs.contains(NewtypeOption::AsSlice).then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn as_slice(&self) -> &<#inner_ty as std::ops::Deref>::Target
                where
                    #inner_ty: std::ops::Deref,
                {
                    std::ops::Deref::deref(&self.0)
                }
            }
        }
    });
    let deref_ts = attrs.contains(NewtypeOption::Deref).then(|| {
        quote::quote! {
            impl #impl_generics std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = <#inner_ty as std::ops::Deref>::Target;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    });
    let from_ts = attrs.contains(NewtypeOption::From).then(|| {
        quote::quote! {
            impl #impl_generics From<#inner_ty> for #ident #ty_generics #where_clause {
                fn from(value: #inner_ty) -> Self {
                    Self(value)
                }
            }
        }
    });
    let getter_ts = attrs.contains(NewtypeOption::Getter).then(|| {
        let trait_ident = quote::format_ident!("Get{ident}");
        let fn_ident = quote::format_ident!("get_{}", ident_to_snake(ident));
        quote::quote! {
            pub trait #trait_ident {
                fn #fn_ident(&self) -> &#inner_ty;
            }
            impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
                fn #fn_ident(&self) -> &#inner_ty {
                    &self.0
                }
            }
            impl #impl_generics #trait_ident for &#ident #ty_generics #where_clause {
                fn #fn_ident(&self) -> &#inner_ty {
                    &self.0
                }
            }
        }
    });
    let into_inner_ts = attrs.contains(NewtypeOption::IntoInner).then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn into_inner(self) -> #inner_ty {
                    self.0
                }
            }
        }
    });
    let into_vec_ts = attrs.contains(NewtypeOption::IntoVec).then(|| {
        quote::quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                #[must_use]
                pub fn into_vec(self) -> #inner_ty {
                    self.0
                }
            }
        }
    });
    let to_err_string_ts = gen_to_err_string_ts(&attrs, ident);
    Ok(quote::quote! {
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
    })
}
#[allow(clippy::single_call_fn)] // keeps enum parsing derive independent from newtype tuple-struct generation
fn gen_enum_from_str_ts(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &input.ident;
    let data_enum = match &input.data {
        syn::Data::Enum(v) => v,
        syn::Data::Struct(_) | syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input,
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
            Ok((&variant.ident, ident_to_snake(&variant.ident)))
        })
        .collect::<syn::Result<Vec<(&syn::Ident, String)>>>()?;
    let allowed_values = variants
        .iter()
        .map(|(_, name)| name.as_str())
        .collect::<Vec<&str>>()
        .join(", ");
    let arms = variants.iter().map(|(variant_ident, name)| {
        quote::quote! {
            v if v.eq_ignore_ascii_case(#name) => Ok(Self::#variant_ident),
        }
    });
    Ok(quote::quote! {
        impl std::str::FromStr for #ident {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#arms)*
                    _ => Err(format!("Unknown value: {s}. Allowed values: {allowed_values}", allowed_values = #allowed_values)),
                }
            }
        }
    })
}
#[allow(clippy::single_call_fn)] // attr parsing is intentionally isolated from code generation
fn parse_newtype_attrs(attrs: &[syn::Attribute]) -> syn::Result<NewtypeAttrs> {
    attrs
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
                    return acc.set_to_err_string_mode(ToErrStringMode::Display, &meta);
                }
                if meta.path.is_ident("to_err_string_as_ref_str") {
                    return acc.set_to_err_string_mode(ToErrStringMode::AsRefStr, &meta);
                }
                if meta.path.is_ident("to_err_string_debug") {
                    return acc.set_to_err_string_mode(ToErrStringMode::Debug, &meta);
                }
                Err(meta.error("unknown newtype option"))
            })?;
            Ok(acc)
        })
}
#[allow(clippy::single_call_fn)] // validation stays named so proc-macro diagnostics are not mixed into generation
fn validate_newtype_attrs(attrs: &NewtypeAttrs, input: &syn::DeriveInput) -> syn::Result<()> {
    if attrs.options.is_empty() && attrs.to_err_string_mode.is_none() {
        return Err(syn::Error::new_spanned(
            input,
            "Newtype requires at least one #[newtype(...)] option",
        ));
    }
    Ok(())
}
#[allow(clippy::single_call_fn)] // string wrapper policy belongs to newtype validation before From impl generation
fn validate_newtype_inner_ty_attrs(attrs: &NewtypeAttrs, inner_ty: &syn::Type) -> syn::Result<()> {
    if attrs.contains(NewtypeOption::From) && type_path_ends_with_ident(inner_ty, "String") {
        return Err(syn::Error::new_spanned(
            inner_ty,
            "#[newtype(from)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead",
        ));
    }
    Ok(())
}
#[allow(clippy::single_call_fn)] // tuple field extraction is separate to keep derive input validation explicit
fn tuple_struct_one_field_ty(input: &syn::DeriveInput) -> syn::Result<&syn::Type> {
    let data_struct = match &input.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "Newtype supports only tuple structs",
            ));
        }
    };
    let unnamed = match &data_struct.fields {
        syn::Fields::Unnamed(v) => &v.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => {
            return Err(syn::Error::new_spanned(
                input,
                "Newtype supports only tuple structs",
            ));
        }
    };
    if unnamed.len() != 1 {
        return Err(syn::Error::new_spanned(
            input,
            "Newtype supports only one-field tuple structs",
        ));
    }
    unnamed
        .first()
        .map(|field| &field.ty)
        .ok_or_else(|| syn::Error::new_spanned(input, "Newtype field not found"))
}
#[allow(clippy::single_call_fn)] // newtype validation only needs terminal path ident matching for concrete String wrappers
fn type_path_ends_with_ident(ty: &syn::Type, ident: &str) -> bool {
    match ty {
        syn::Type::Path(v) if v.qself.is_none() => v
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == ident),
        syn::Type::Path(_) | _ => false,
    }
}
#[allow(clippy::single_call_fn)] // proc-macro generated getter names need local snake_case conversion without adding another dependency
fn ident_to_snake(ident: &syn::Ident) -> String {
    let (out, _) = ident.to_string().chars().fold(
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
    out
}
#[allow(clippy::single_call_fn)] // ToErrString code generation has distinct modes from base newtype impls
fn gen_to_err_string_ts(
    attrs: &NewtypeAttrs,
    ident: &syn::Ident,
) -> Option<proc_macro2::TokenStream> {
    attrs.to_err_string_mode.map(|mode| match mode {
        ToErrStringMode::AsRefStr => {
            quote::quote! {
                impl loc_lib::ToErrString for #ident {
                    fn to_err_string(&self) -> loc_lib::ToErrStringValue {
                        loc_lib::ToErrStringValue::try_from(AsRef::<str>::as_ref(&self.0).to_owned()).unwrap_or_else(loc_lib::ToErrStringValue::from)
                    }
                }
            }
        }
        ToErrStringMode::Debug => {
            quote::quote! {
                impl loc_lib::ToErrString for #ident {
                    fn to_err_string(&self) -> loc_lib::ToErrStringValue {
                        loc_lib::ToErrStringValue::try_from(format!("{:?}", self.0)).unwrap_or_else(loc_lib::ToErrStringValue::from)
                    }
                }
            }
        }
        ToErrStringMode::Display => {
            quote::quote! {
                impl loc_lib::ToErrString for #ident {
                    fn to_err_string(&self) -> loc_lib::ToErrStringValue {
                        loc_lib::ToErrStringValue::try_from(self.0.to_string()).unwrap_or_else(loc_lib::ToErrStringValue::from)
                    }
                }
            }
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
        let result = super::gen_newtype_ts(&input);
        assert!(result.is_err(), "f9b7c2a1");
        if let Err(er) = result {
            assert_eq!(
                er.to_string(),
                "#[newtype(from)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead"
            );
        }
    }
}
