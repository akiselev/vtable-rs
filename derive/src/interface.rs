#![feature(proc_macro)]
// macros
use syn::{named, syn, keyword, punct, do_parse, Token, call};
// Parser combinators
use syn::{Visibility, Ident, Generics, GenericParam, Type, Field, Block};
use syn::punctuated::Punctuated;
use syn::token::{Semi, Comma};
use syn::synom::Synom;
use syn::spanned::Spanned;
use proc_macro::{TokenStream as LegacyTokenStream};
use proc_macro2::TokenStream;
use quote::*;

pub enum InterfaceStatement {
    Field(Field),

}

pub struct InterfaceDecl {
    pub name: Ident,
    pub block: Block
    // statements: Vec<InterfaceStatement>
}

impl Synom for InterfaceDecl {
    named!(parse -> Self, do_parse!(
        name: syn!(Ident) >>
        block: syn!(Block) >>
        (InterfaceDecl { name, block })
    ));
}

