// #![feature(async_await, await_macro, associated_type_defaults, unsize, coerce_unsized, pin, fn_traits, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident, never_type, specialization, unboxed_closures)]
#![feature(rust_2018_preview, nll, unsize, coerce_unsized, fn_traits, pin, arbitrary_self_types, never_type, specialization, unboxed_closures)]

use std::borrow::Borrow;
use std::boxed::PinBox;
use std::marker::PhantomData;
use std::mem::PinMut;
use std::mem::size_of;
use std::ops::{Index, Deref, DerefMut};
use std::ops::Add;

use frunk::*;
use frunk::prelude::*;
use frunk_core::*;
use frunk_core::hlist::*;
use frunk_core::indices::*;
use failure::{Error, Fail};
use serde::{Serialize, Deserialize};

mod traits;
pub use crate::traits::*;

mod folds;
pub use crate::folds::*;

#[macro_use]
mod macros;

mod path;
pub use crate::path::*;

mod builder;
pub use crate::builder::*;

mod primitives;
pub use crate::primitives::*;

#[derive(Copy, Clone, Debug)]
pub struct Type<T> {
    path: PhantomData<T>
}

impl<T> Type<T> {
    pub fn new() -> Type<T> {
        Type {
            path: PhantomData
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    struct P1();
    struct P2();
    struct P3();
    struct P4();

    #[test]
    fn testfn() {
        let list = hlist![
            Const::new(P1, || 1),
            Const::new(P2, || 2),
            Const::new(P3, || 3),
        ];
        unsafe {
            let pinbox = instantiate(&list);
            let ptr: *const u32 = std::mem::transmute(PinBox::into_raw(pinbox));
            assert_eq!(1, *ptr);
            assert_eq!(2, *(ptr.offset(1)));
            assert_eq!(3, *(ptr.offset(2)));
        };
    }
}

trait Replace<P, F> {
    type Output;

    fn replace(self, func: F) -> Self::Output;
}


trait Binder<T> {

}

struct Binding<P: Sized> {
    data: P
}

impl<T> Builder<T> {
    pub fn push<P, P1, O: Sized>(self, other: O) -> Builder<HCons<(Path<P1>, O), T>>
    where HNil: Add<P, Output=P1>
    {
        let head: (Path<P1>, O) = (Path::new(), other);
        Builder {
            data: HCons {
                head: head,
                tail: self.data
            }
        }
    }
    
    pub fn new<F, O>(self, constructor: F) -> O
    where 
        Self: Init<F, HNil, Output=O>
    {
        self.init(constructor)
    }

    pub fn pretty_print(&self) where T: Serialize {
        println!("{}", serde_json::to_string(&self.data).unwrap());
    }
}



fn testfn() {

}

#[cfg(test)]
mod tests2 {
    use crate::*;
    #[macro_use]
    use crate::macros;
    use frunk_core::hlist::*;

    create_path!(P1);

    #[test]
    fn test() {
        let builder = Builder { data: HNil };
        let builder = builder.new(|builder| {
            // cls.push::<P1>()
            Builder {
                data: HCons { head: (Path::<P1>::new(), 0f32), tail: HNil }
            }
        });
    }
}