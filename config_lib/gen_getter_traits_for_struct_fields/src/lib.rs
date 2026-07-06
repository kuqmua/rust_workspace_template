#[proc_macro_derive(GenGetterTraitsForStructFields)]
pub fn gen_getter_traits_for_struct_fields(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let di: syn::DeriveInput = syn::parse(input).expect("49780295");
    let ident = &di.ident;
    let datastruct = match di.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("15cd72a2"),
    };
    let generated_traits_impls_ts = datastruct.fields.into_iter().map(|field| {
        let (fi, ucc_fi) = {
            let fi = field.ident.as_ref().expect("e5c23c45");
            (fi, naming::ToTokensToUccStr::case(&fi))
        };
        let ft = field.ty;
        let path_trait_ident = format!("app_state::Get{ucc_fi}")
            .parse::<proc_macro2::TokenStream>()
            .expect("8fb2cb27");
        let fn_name_ident = format!("get_{fi}")
            .parse::<proc_macro2::TokenStream>()
            .expect("a349efd0");
        quote::quote! {
            impl #path_trait_ident for #ident {
                fn #fn_name_ident (&self) -> &#ft {
                    &self.#fi
                }
            }
            impl #path_trait_ident for &#ident {
                fn #fn_name_ident (&self) -> &#ft {
                    &self.#fi
                }
            }
        }
    });
    let generated = quote::quote! {#(#generated_traits_impls_ts)*};
    generated.into()
}
#[proc_macro_derive(GenGetterTrait)]
pub fn gen_getter_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic_loc::panic_loc();
    let di: syn::DeriveInput = syn::parse(input).expect("195b48f5");
    let ident = &di.ident;
    let data_struct = match di.data {
        syn::Data::Struct(v) => v,
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("cd6bbc4e"),
    };
    let fields_unnamed = match data_struct.fields {
        syn::Fields::Unnamed(v) => v.unnamed,
        syn::Fields::Named(_) | syn::Fields::Unit => panic!("577cb86a"),
    };
    assert!(fields_unnamed.len() == 1, "1e82dc7e");
    let first_field_unnamed = fields_unnamed.iter().next().expect("7c2531fd");
    let first_field_unnamed_type = &first_field_unnamed.ty;
    let get_ident_ucc = naming::prm::GetSelfUcc::from_tokens(&ident);
    let get_ident_sc = naming::prm::GetSelfSc::from_tokens(&ident);
    let generated = quote::quote! {
        pub trait #get_ident_ucc {
            fn #get_ident_sc(&self) -> &#first_field_unnamed_type;
        }
    };
    // println!("{generated}");
    generated.into()
}
