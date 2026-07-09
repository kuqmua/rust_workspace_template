#[proc_macro_derive(TryFromEnv)]
pub fn try_from_env(v: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let dotenv_sc = naming::DotenvSc;
    let dotenv_ucc = naming::DotenvUcc;
    let env_var_name_sc = naming::EnvVarNameSc;
    let std_env_var_er_sc = naming::StdEnvVarErSc;
    let std_env_var_er_ucc = naming::StdEnvVarErUcc;
    let try_from_std_env_var_ok_ucc = naming::TryFromStdEnvVarOkUcc;
    let di: syn::DeriveInput = syn::parse(v).expect("e45f75c2");
    let ident = &di.ident;
    let ident_try_from_env_er_ucc = naming::prm::SelfTryFromEnvErUcc::from_tokens(&ident);
    let data_struct = match di.data {
        syn::Data::Struct(v0) => v0,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("54289ad5"),
    };
    let fields_named = match data_struct.fields {
        syn::Fields::Named(v0) => v0.named,
        syn::Fields::Unnamed(_) | syn::Fields::Unit => panic!("330b2512"),
    };
    let field_ident = |field: &syn::Field, exp_id: &'static str| {
        field.ident.clone().unwrap_or_else(|| panic!("{exp_id}"))
    };
    let er_ts = {
        let vrts_ts = fields_named.iter().map(|el| {
            let el_ident = field_ident(el, "2ecb63c1");
            let el_ident_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&el_ident);
            let el_ty = &el.ty;
            quote::quote! {
                #el_ident_ucc_ts {
                    #el_ident: <#el_ty as config_lib::TryFromStdEnvVarOk>::Error,
                }
            }
        });
        quote::quote! {
            #[derive(Debug, thiserror::Error, optml::Optml)]
            pub enum #ident_try_from_env_er_ucc {
                #dotenv_ucc {
                    #dotenv_sc: dotenv::Error,
                },
                #std_env_var_er_ucc {
                    #std_env_var_er_sc: std::env::VarError,
                    env_var_name: config_lib::EnvVarName,
                },
                #(#vrts_ts),*
            }
        }
    };
    let display_er_ts = {
        let vrts_ts = fields_named.iter().map(|el| {
            let el_ident = field_ident(el, "8b79a379");
            let el_ident_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&el_ident);
            quote::quote! {
                Self::#el_ident_ucc_ts { #el_ident } => write!(f, "{}", #el_ident)
            }
        });
        macros_helpers::gen_impl_display_ts::gen_impl_display_ts(
            &proc_macro2::TokenStream::new(),
            &ident_try_from_env_er_ucc,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {
                match self {
                    Self::#dotenv_ucc {
                        #dotenv_sc
                    } => write!(f, "{}", #dotenv_sc),
                    Self::#std_env_var_er_ucc {
                        #std_env_var_er_sc,
                        env_var_name
                    } => write!(f, "{} {}", #std_env_var_er_sc, env_var_name),
                    #(#vrts_ts),*
                }
            },
        )
    };
    let try_from_env_ts = {
        let fields_init_ts = fields_named.iter().map(|el| {
            let el_ident = field_ident(el, "ebf4e1b2");
            let el_ty = &el.ty;
            let el_ident_quotes_upper_sc_string =
                syn::LitStr::new(&naming_cmn::ToTokensToUpperScStr::case(&el_ident), ident.span());
            let el_ident_ucc_ts = naming_cmn::ToTokensToUccTs::case_or_panic(&el_ident);
            quote::quote! {
                let #el_ident = config_lib::parse_required_env_var(
                    config_lib::EnvVarNameRef::from(#el_ident_quotes_upper_sc_string),
                    |#std_env_var_er_sc, #env_var_name_sc| #ident_try_from_env_er_ucc::#std_env_var_er_ucc {
                        #std_env_var_er_sc,
                        #env_var_name_sc,
                    },
                    |v| <
                        #el_ty as
                        config_lib::#try_from_std_env_var_ok_ucc
                    >::try_from_std_env_var_ok(v),
                    |#el_ident| #ident_try_from_env_er_ucc::#el_ident_ucc_ts {
                        #el_ident,
                    },
                )?;
            }
        });
        let fields_ts = fields_named.iter().map(|el| &el.ident);
        quote::quote! {
            impl #ident {
                pub fn try_from_env() -> Result<Self, #ident_try_from_env_er_ucc> {
                    if let Err(er) = dotenv::dotenv() {
                        return Err(#ident_try_from_env_er_ucc::#dotenv_ucc {
                            #dotenv_sc: er,
                        });
                    }
                    #(#fields_init_ts)*
                    Ok(Self {
                        #(#fields_ts),*
                    })
                }
            }
        }
    };
    let generated = quote::quote! {
        #er_ts
        #display_er_ts
        #try_from_env_ts
    };
    // println!("{generated}");
    generated.into()
}
