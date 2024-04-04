use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumFromChar, attributes(char, init))]
pub fn derive_enum_from_char(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::enum_from_char(input)
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}

mod expand {
    use proc_macro2::TokenStream;
    use syn::{spanned::Spanned, DataEnum, DeriveInput, Error, Result};
    use quote::quote;
    
    pub(crate) fn enum_from_char(input: DeriveInput) -> Result<TokenStream> {
        match input.data {
            syn::Data::Enum(data) => {
                let name = input.ident;
                let match_arms = expand_match_arms(data)?;
                
                Ok(quote!(
                    impl TryFrom<char> for #name {
                        type Error = String;
                        fn try_from(c: char) -> Result<Self, Self::Error> {
                            match c {
                                #match_arms
                                _ => Err(format!("Cannot create enum from character '{}'", c)),
                            }
                        }
                    }
                ))
            },
            _ => Err(Error::new(input.span(), "#[derive(EnumFromChar) expects an enum")),
        }
    }

    fn expand_match_arms(data: DataEnum) -> Result<TokenStream> {
        let mut match_arms = vec![];
        for variant in data.variants {
            // Only parse variants with a "char" attribute
            if let Some(char_attr) = variant.attrs.iter().find(|attr| attr.path().is_ident("char")) {
                let variant_ident = variant.ident.clone();
                let char_expr = match &char_attr.meta {
                    syn::Meta::NameValue(mnv) => &mnv.value,
                    _ => return Err(Error::new(variant.span(), "#[derive(EnumFromChar) expects attribute #[char = ...]"))
                };
                let init_attr = variant.attrs.iter().find(|attr| attr.path().is_ident("init"));
                match variant.fields {
                    syn::Fields::Named(_) => {
                        let init_attr = init_attr
                            .ok_or(Error::new(variant.span(), "#[derive(EnumFromChar) expects variants with named fields to have an attribute #[init { ... }]"))?;
                        let init_expr = match &init_attr.meta {
                            syn::Meta::List(list) => &list.tokens,
                            _ => return Err(Error::new(variant.span(), "#[derive(EnumFromChar) expects attribute #[init { ... }]"))
                        };
                        match_arms.push(quote!(#char_expr => Ok(Self::#variant_ident{#init_expr}),))
                    },
                    syn::Fields::Unnamed(_) => {
                        let init_attr = init_attr
                            .ok_or(Error::new(variant.span(), "#[derive(EnumFromChar) expects variants with unnamed fields to have an attribute #[init(...)]"))?;
                        let init_expr = match &init_attr.meta {
                            syn::Meta::List(list) => &list.tokens,
                            _ => return Err(Error::new(variant.span(), "#[derive(EnumFromChar) expects attribute #[init(...)]"))
                        };
                        match_arms.push(quote!(#char_expr => Ok(Self::#variant_ident(#init_expr)),))
                    },
                    syn::Fields::Unit => {
                        if init_attr.is_some() {
                            return Err(Error::new(variant.span(), "#[derive(EnumFromChar) expects unit variants to have no init attribute"));
                        }
                        match_arms.push(quote!(#char_expr => Ok(Self::#variant_ident),))
                    },
                }
            }
        }
        Ok(quote!(#(#match_arms)*))
    }
}
