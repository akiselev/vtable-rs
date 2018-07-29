#![feature(async_await, await_macro, pin, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident)]

use std::ops::{Index, Deref, DerefMut};
use std::borrow::Borrow;
use std::mem::PinMut;
use vtable_derive::symbol;

use frunk::hlist::{HCons, HNil};
use frunk_core::indices::{Here, There};

mod traits;
pub use crate::traits::*;

#[derive(Copy, Clone, Debug)]
struct PButton<S: Symbol> {
    symbol: S,
}

impl<SYM: Symbol> Symbol for PButton<SYM> {
    type Type = SYM::Type;
}

symbol!(pub width = f32 as Width;);
symbol!(pub height = f32 as Height;);
symbol!(pub top = f32 as Top;);
symbol!(pub left = f32 as Left;);
symbol!(pub text = f32 as Text;);

trait IButton {
    /** Dynamic Dispatch super trait for returning a button object */

    fn render(&self);
}

trait VButton {
    /**  */
    type Width: VPath + Symbol<Type=f32>;
    type Height: VPath + Symbol<Type=f32>;
    type Top: VPath + Symbol<Type=f32>;
    type Left: VPath + Symbol<Type=f32>;
    type Text: VPath + Symbol<Type=String>;
}

struct Prop<'this, T: 'this> {
    ptr: PinMut<'this, T>
}

struct PropPtr<'ptr, 'this: 'ptr, P: VPath, S: Symbol> where S::Type: 'this {
    path: P,
    symbol: S,
    ptr: &'ptr PinMut<'this, S::Type>
}

struct Button<'this> {
    width: Prop<'this, f32>,
    height: Prop<'this, f32>,
    top: Prop<'this, f32>,
    left: Prop<'this, f32>,
    text: Prop<'this, String>
}

struct ObjectBuilder<SYM: Symbol, DATA> {
    symbol: std::marker::PhantomData<SYM>,
    data: DATA, 
}

impl<SYM: Symbol, DATA> ObjectBuilder<SYM, DATA> {
    fn add_child<CHILD: Symbol, CDATA>(self, data: CDATA) where DATA: Append<CHILD, CDATA> {

    }
}

trait Append<SYM: Symbol, DATA> {
    type Output;

    fn append(self, data: DATA) -> Self::Output;
}

impl<SYM: Symbol, DATA> Append<SYM, DATA> for HNil {
    type Output = DATA;

    fn append(self, data: DATA) -> DATA {
        data
    }
}

// impl Object {
//     fn new<PATH: VPath, SUPER>(this: Obj<PATH, SUPER>) -> impl HList {

//     }
// }


// impl<PATH: VPath, SYM: Symbol, HEAD, TAIL> Append<SYM, DATA> for HCons<HEAD, TAIL>

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
