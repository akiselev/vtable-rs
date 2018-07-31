#![feature(async_await, await_macro, pin, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident, never_type, specialization, unboxed_closures)]

use std::ops::{Index, Deref, DerefMut};
use std::borrow::Borrow;
use std::mem::PinMut;
use std::boxed::PinBox;
use vtable_derive::symbol;

use std::marker::PhantomData;
use frunk::*;
use frunk::prelude::*;
use frunk_core::*;
use frunk_core::hlist::*;
use frunk_core::indices::*;
use failure::{Error, Fail};

mod traits;
pub use crate::traits::*;

struct Class<P, T> {
    path: Path<P>,
    data: T
}

trait At<Name> {
    type AtRes;

    fn at(self) -> Self::AtRes;
}

trait Push<T> {
    type PushRes;

    fn push(self, other: T) -> Self::PushRes;
}

impl<T> Push<T> for HNil {
    type PushRes = HCons<T, HNil>;

    fn push(self, other: T) -> HCons<T, HNil> {
        HCons {
            head: other,
            tail: HNil
        }
    }
}

impl<T, HEAD, TAIL> Push<T> for HCons<HEAD, TAIL> {
    type PushRes = HCons<T, HCons<HEAD, TAIL>>;

    fn push(self, other: T) -> HCons<T, HCons<HEAD, TAIL>> {
        HCons {
            head: other,
            tail: self
        }
    }
}

impl<Name, P, T> At<Name> for Class<P, T> {
    type AtRes = Class<HCons<Name, P>, T>;

    fn at(self) -> Class<HCons<Name, P>, T> {
        Class {
            path: Path::new(),
            data: self.data
        }
    }
}

trait AddClass<Name, F>: At<Name> {
    type Output;

    fn init(self, func: F) -> Self::Output;
}

trait Entry {
    type Path;
    type Data;

    fn get_data(self) -> Self::Data;
    fn borrow_data(&self) -> &Self::Data;
}

impl<N, T> Entry for (N, T) {
    type Path = N;
    type Data = T;

    fn get_data(self) -> Self::Data {
        self.1
    }

    fn borrow_data(&self) -> &Self::Data {
        &self.1
    }
}

impl<N, T> Entry for Class<N, T> {
    type Path = N;
    type Data = T;

    fn get_data(self) -> Self::Data {
        self.data
    }

    fn borrow_data(&self) -> &Self::Data {
        &self.data
    }
}

impl<'this, P, T: 'this> ToRef<'this> for Class<P, T>
where
    <Class<P, T> as Entry>::Data: ToRef<'this>,
    <T as frunk_core::traits::ToRef<'this>>::Output: 'this,
    T: frunk_core::traits::ToRef<'this>,
{
    type Output = <<Self as Entry>::Data as ToRef<'this>>::Output;

    fn to_ref(&'this self) -> <T as ToRef<'this>>::Output {
        self.borrow_data().to_ref()
    }
}

impl<P, T, Name, F, OLIST: HList, X, Y, FINAL> AddClass<Name, F> for Class<P, T> 
where
    Self: At<Name>,
    Self::AtRes: Entry<Data=T>,
    X: Entry,
    F: Fn(Y) -> OLIST,
    T: Push<(PhantomData<HCons<Name, HCons<P, HNil>>>, F)>,
    <X as Entry>::Data: Fn(Y) -> Class<P, OLIST>,
    <Class<P, T> as At<Name>>::AtRes: Push<(Path<Hlist![Name, P]>, F)>,
    <<Class<P, T> as At<Name>>::AtRes as Push<(Path<Hlist![Name, P]>, F)>>::PushRes: for<'this> ToRef<'this, Output=HCons<X, Y>> + Push<OLIST>,
    <<<Class<P, T> as At<Name>>::AtRes as Push<(Path<HCons<Name, HCons<P, HNil>>>, F)>>::PushRes as Push<OLIST>>::PushRes: Entry<Data=FINAL>
{
    type Output = Class<P, FINAL>;

    fn init(self, func: F) -> Self::Output {
        let builder = self.at();
        let builder = builder.push((Path::new(), func));
        let output = {
            let refs = builder.to_ref();
            let func = refs.head.borrow_data();
            func(refs.tail)
        };
        let final_data = builder.push(output);
        Class {
            path: Path::new(),
            data: final_data.get_data()
        }
    }
}

impl<P: HList, T> Class<P, T> {
    fn at<Name>(self) -> Class<HCons<Name, P>, T> {
        Class {
            path: Path::new(),
            data: self.data
        }
    }

    fn push<Name, O: Sized>(self, other: O) -> Class<P, HCons<(Path<Hlist![Name, P]>, O), T>> {
        let head: (Path<Hlist![Name, P]>, O) = (Path::new(), other);
        Class {
            path: Path::new(),
            data: HCons {
                head: head,
                tail: self.data
            }
        }
    }
    
    fn with<Name, F>(self, constructor: F) -> <Self as AddClass<Name, F>>::Output
    where 
        Self: AddClass<Name, F>
    {
        self.init(constructor)
    }

    fn from<F>(self, constructor: F) -> <Self as AddClass<P, F>>::Output
    where 
        Self: AddClass<P, F>
    {
        self.init(constructor)
    }

    pub fn new<F>(constructor: F) -> <Class<HNil, HNil> as AddClass<HNil, F>>::Output where Class<HNil, HNil>: AddClass<HNil, F> {
        Class {
            path: HNil,
            data: HNil,
        }.init(constructor)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Path<T> {
    path: PhantomData<T>
}

impl<P> Path<P> {
    pub fn new() -> Path<P> {
        Path {
            path: PhantomData
        }
    }
}

pub struct VEntry<P, T> {
    path: Path<P>,
    data: PhantomData<T>
}
