#![feature(proc_macro)]
// macros
use syn::{named, syn, keyword, punct, do_parse, Token, call};
// Parser combinators
use syn::{Visibility, Ident, Generics, GenericParam, Type, File};
use syn::punctuated::Punctuated;
use syn::token::{Semi, Comma};
use syn::synom::Synom;
use syn::spanned::Spanned;
use proc_macro::{TokenStream as LegacyTokenStream};
use proc_macro2::TokenStream;
use quote::*;

mod interface;

mod utils;

use self::interface::*;

struct SymbolType {
    name: Ident,
    generics: Generics,
}

impl ToTokens for SymbolType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.generics.to_tokens(tokens);
    }
}

impl Synom for SymbolType {
    named!(parse -> Self, do_parse!(
        name: syn!(Ident) >>
        generics: syn!(Generics) >>
        (SymbolType { name, generics })
    ));
}

struct NewSymbol {
    visibility: Visibility,
    name: Ident,
    eql_token: Token![=],
    data_type: Type,
    as_token: Token![as],
    symbol_type: SymbolType,
    sem_token: Token![;],
}

impl Synom for NewSymbol {
    named!(parse -> Self, do_parse!(
        visibility: syn!(Visibility) >>
        name: syn!(Ident) >>
        eql_token: punct!(=) >>
        data_type: syn!(Type) >>
        as_token: keyword!(as) >>
        symbol_type: syn!(SymbolType) >>
        sem_token: punct!(;) >>
        (NewSymbol { visibility, name, as_token, symbol_type, eql_token, data_type, sem_token })
    ));
}

struct Symbols {
    symbols: Vec<NewSymbol>,
}

fn get_symbol_phantomdata(params: &Punctuated<GenericParam, Comma>) -> Punctuated<TokenStream, Semi> {
    let mut markers = Punctuated::new();

    for param in params.iter() {
        let marker = match param {
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                quote! {
                    #ident: ::std::marker::PhantomData<#ident>
                }
            },
            _ => quote! { }
        };
        markers.push_value(marker);
    }

    markers
}

fn get_symbol_phantomdata_init(params: &Punctuated<GenericParam, Comma>) -> Punctuated<TokenStream, Semi> {
    let mut markers = Punctuated::new();

    for param in params.iter() {
        let marker = match param {
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                quote! {
                    #ident: ::std::marker::PhantomData
                }
            },
            _ => quote! { }
        };
        markers.push_value(quote! { #marker {} });
    }

    markers
}

#[proc_macro]
pub fn symbol(input: LegacyTokenStream) -> LegacyTokenStream {
    // Parse the input tokens into a syntax tree
    let input: NewSymbol = syn::parse(input).unwrap();

    // Build the output, possibly using quasi-quotation
    let symbol_type = input.symbol_type;
    let data_type = input.data_type;
    let markers = get_symbol_phantomdata(&symbol_type.generics.params);
    let symbol_decl = if markers.len() > 0 {
        quote! { 
            {
                #markers
            }
        }
    } else {
        quote! { ; }
    };


    let struct_decl = {
        let symbol_name = &symbol_type.name;
        let (impl_generics, ty_generics, where_clause) = symbol_type.generics.split_for_impl();

        let markers_init = get_symbol_phantomdata_init(&symbol_type.generics.params);
        quote! {
            #[derive(Copy, Clone, Debug)]
            struct #symbol_type #symbol_decl

            impl#impl_generics #symbol_name #ty_generics #where_clause {
                fn new() -> #symbol_name #ty_generics {
                    #symbol_name {
                        #markers_init
                    }
                }
            }
        }
    };

    let symbol_impl = {
        let symbol_name = &symbol_type.name;
        let (impl_generics, ty_generics, where_clause) = symbol_type.generics.split_for_impl();

        quote! {
            impl#impl_generics ::vtable::Symbol for #symbol_name #ty_generics #where_clause {
                type Type = #data_type;
            }
        }
    };

    let expanded = quote! {
        #struct_decl

        #symbol_impl
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}

#[proc_macro]
pub fn vtable(input: LegacyTokenStream) -> LegacyTokenStream {
    // Parse the input tokens into a syntax tree
    let code: File = syn::parse(input).unwrap();

    let expanded = quote! {
        #name #blockx
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
