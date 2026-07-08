#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum LocFieldAttr {
    EoToErrString,
    EoToErrStringSerde,
    EoLoc,
    EoVecToErrString,
    EoVecToErrStringSerde,
    EoVecLoc,
    EoHashMapKStringVToErrString,
    EoHashMapKStringVToErrStringSerde,
    EoHashMapKStringVLoc,
}
impl std::str::FromStr for LocFieldAttr {
    type Err = ();
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if v == "eo_to_err_string" {
            Ok(Self::EoToErrString)
        } else if v == "eo_to_err_string_serde" {
            Ok(Self::EoToErrStringSerde)
        } else if v == "eo_loc" {
            Ok(Self::EoLoc)
        } else if v == "eo_vec_to_err_string" {
            Ok(Self::EoVecToErrString)
        } else if v == "eo_vec_to_err_string_serde" {
            Ok(Self::EoVecToErrStringSerde)
        } else if v == "eo_vec_loc" {
            Ok(Self::EoVecLoc)
        } else if v == "eo_hashmap_k_string_v_to_err_string" {
            Ok(Self::EoHashMapKStringVToErrString)
        } else if v == "eo_hashmap_k_string_v_to_err_string_serde" {
            Ok(Self::EoHashMapKStringVToErrStringSerde)
        } else if v == "eo_hashmap_k_string_v_loc" {
            Ok(Self::EoHashMapKStringVLoc)
        } else {
            Err(())
        }
    }
}
impl TryFrom<&syn::Field> for LocFieldAttr {
    type Error = String;
    fn try_from(syn_field: &syn::Field) -> Result<Self, Self::Error> {
        let mut supported_attrs = syn_field.attrs.iter().filter_map(|el| {
            if el.path().segments.len() != 1 {
                return None;
            }
            let first_segment_ident = &el.path().segments.first()?.ident;
            std::str::FromStr::from_str(&first_segment_ident.to_string()).ok()
        });
        let opt_attr = supported_attrs.next();
        if supported_attrs.next().is_some() {
            return Err("two or more supported attrs!".to_owned());
        }
        opt_attr.map_or_else(|| Err("opt attr is None".to_owned()), Ok)
    }
}
impl crate::attr_ident_str::AttrIdentStr for LocFieldAttr {
    fn attr_ident_str(&self) -> crate::attr_ident_str::AttrIdentName<'_> {
        crate::attr_ident_str::AttrIdentName::from(match *self {
            Self::EoToErrString => "eo_to_err_string",
            Self::EoToErrStringSerde => "eo_to_err_string_serde",
            Self::EoLoc => "eo_loc",
            Self::EoVecToErrString => "eo_vec_to_err_string",
            Self::EoVecToErrStringSerde => "eo_vec_to_err_string_serde",
            Self::EoVecLoc => "eo_vec_loc",
            Self::EoHashMapKStringVToErrString => "eo_hashmap_k_string_v_to_err_string",
            Self::EoHashMapKStringVToErrStringSerde => "eo_hashmap_k_string_v_to_err_string_serde",
            Self::EoHashMapKStringVLoc => "eo_hashmap_k_string_v_loc",
        })
    }
}
impl LocFieldAttr {
    #[must_use]
    pub fn to_attr_view_ts(&self) -> crate::GeneratedRustTs {
        match format!(
            "#[{}]",
            crate::attr_ident_str::AttrIdentStr::attr_ident_str(self).as_ref()
        )
        .parse::<proc_macro2::TokenStream>()
        {
            Ok(v) => crate::GeneratedRustTs::from(v),
            Err(er) => compile_error_ts(CompileErrorMsg(&er.to_string())),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct CompileErrorMsg<'msg_lt>(&'msg_lt str);
#[derive(Debug, Clone, Copy)]
pub struct SynVariantRef<'variant_lt>(&'variant_lt syn::Variant);
impl<'variant_lt> From<&'variant_lt syn::Variant> for SynVariantRef<'variant_lt> {
    fn from(value: &'variant_lt syn::Variant) -> Self {
        Self(value)
    }
}
fn compile_error_ts(msg: CompileErrorMsg<'_>) -> crate::GeneratedRustTs {
    let msg_value = msg.0;
    crate::GeneratedRustTs::from(quote::quote! {compile_error!(#msg_value);})
}
#[must_use]
pub fn gen_serde_version_of_named_syn_vrt(v: SynVariantRef<'_>) -> crate::GeneratedRustTs {
    let variant = v.0;
    let hash_map_ucc = naming::HashMapUcc;
    let loc_sc = naming::LocSc;
    let string_ts = token_patterns::StringTs;
    let with_serde_ucc = naming::WithSerdeUcc;
    let el_ident = &variant.ident;
    let fields = if let syn::Fields::Named(fields) = &variant.fields {
        &fields.named
    } else {
        return compile_error_ts(CompileErrorMsg("79b0f231: expected named variant fields"));
    };
    let fields_with_serde_ts = fields.iter().map(|el| {
        let Some(el_c25b655e_ident) = el.ident.as_ref() else {
            return compile_error_ts(CompileErrorMsg("438aa90e: expected named field ident"));
        };
        let ts = if *el_c25b655e_ident == *loc_sc.to_string() {
            quote::quote! {#loc_sc: loc_lib::loc::Loc}
        } else {
            let get_hashmap_args = || {
                let segments = if let syn::Type::Path(syn_type_path) = &el.ty {
                    &syn_type_path.path.segments
                } else {
                    return None;
                };
                let last_segment = segments.iter().next_back()?;
                assert!(last_segment.ident == hash_map_ucc.to_string(), "5e1bc6b1");
                let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) = &last_segment.arguments
                else {
                    return None;
                };
                assert!(args.len() == 2, "47cde1b8");
                Some((args.iter().next()?, args.iter().nth(1)?))
            };
            let el_type_ts = {
                let el_type = &el.ty;
                quote::quote! {#el_type}
            };
            let loc_field_attr = match LocFieldAttr::try_from(el) {
                Ok(parsed_attr) => parsed_attr,
                Err(er) => return compile_error_ts(CompileErrorMsg(&format!("2db209a8: {er}"))),
            };
            let el_type_with_serde_ts = match loc_field_attr {
                LocFieldAttr::EoToErrString => quote::quote! {#string_ts},
                LocFieldAttr::EoToErrStringSerde | LocFieldAttr::EoVecToErrStringSerde => {
                    el_type_ts
                }
                LocFieldAttr::EoLoc => match format!("{el_type_ts}{with_serde_ucc}")
                    .parse::<proc_macro2::TokenStream>()
                {
                    Ok(parsed_ts) => parsed_ts,
                    Err(er) => {
                        return compile_error_ts(CompileErrorMsg(&format!("201dc0a4: {er}")));
                    }
                },
                LocFieldAttr::EoVecToErrString => {
                    quote::quote! {
                        Vec<#string_ts>
                    }
                }
                LocFieldAttr::EoVecLoc => {
                    let segments = if let syn::Type::Path(v0) = &el.ty {
                        &v0.path.segments
                    } else {
                        return compile_error_ts(CompileErrorMsg("8d93bf20: expected path type"));
                    };
                    assert!(segments.len() == 1, "0c65bbaa");
                    let Some(first_segment) = segments.iter().next() else {
                        return compile_error_ts(CompileErrorMsg(
                            "595050cf: expected first path segment",
                        ));
                    };
                    let el_vec_type_with_serde_ts = if let syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments { args, .. },
                    ) = &first_segment.arguments
                    {
                        assert!(args.len() == 1, "572a9da8");
                        match format!(
                            "{}{}",
                            {
                                let Some(first_arg) = args.iter().next() else {
                                    return compile_error_ts(CompileErrorMsg(
                                        "e9b33787: expected first generic arg",
                                    ));
                                };
                                quote::quote! {#first_arg}
                            },
                            with_serde_ucc,
                        )
                        .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_ts) => parsed_ts,
                            Err(er) => {
                                return compile_error_ts(CompileErrorMsg(&format!(
                                    "22c364b9: {er}"
                                )));
                            }
                        }
                    } else {
                        return compile_error_ts(CompileErrorMsg(
                            "07c6ab44: expected angle bracketed args",
                        ));
                    };
                    quote::quote! {
                        Vec<#el_vec_type_with_serde_ts>
                    }
                }
                LocFieldAttr::EoHashMapKStringVToErrString => {
                    if get_hashmap_args().is_none() {
                        return compile_error_ts(CompileErrorMsg(
                            "c1d03b71: expected HashMap<K, T>",
                        ));
                    }
                    quote::quote! {
                        std::collections::HashMap<#string_ts, #string_ts>
                    }
                }
                LocFieldAttr::EoHashMapKStringVToErrStringSerde => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_ts(CompileErrorMsg(
                            "e9c6a7d2: expected HashMap<K, T>",
                        ));
                    };
                    quote::quote! {
                        std::collections::HashMap<#string_ts, #second_argument>
                    }
                }
                LocFieldAttr::EoHashMapKStringVLoc => {
                    let Some((_, second_argument)) = get_hashmap_args() else {
                        return compile_error_ts(CompileErrorMsg(
                            "c828da34: expected HashMap<K, T>",
                        ));
                    };
                    let el_hashmap_v_type_with_serde_ts =
                        match format!("{}{}", quote::quote! {#second_argument}, with_serde_ucc)
                            .parse::<proc_macro2::TokenStream>()
                        {
                            Ok(parsed_ts) => parsed_ts,
                            Err(er) => {
                                return compile_error_ts(CompileErrorMsg(&format!(
                                    "86307dbc: {er}"
                                )));
                            }
                        };
                    quote::quote! {
                        std::collections::HashMap<#string_ts, #el_hashmap_v_type_with_serde_ts>
                    }
                }
            };
            quote::quote! {#el_c25b655e_ident: #el_type_with_serde_ts}
        };
        crate::GeneratedRustTs::from(quote::quote! {#ts,})
    });
    crate::GeneratedRustTs::from(quote::quote! {
        #el_ident {
            #(#fields_with_serde_ts)*
        }
    })
}
