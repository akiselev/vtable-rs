#![feature(async_await, await_macro, pin, fn_traits, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident, never_type, specialization, unboxed_closures)]

use std::ops::{Index, Deref, DerefMut};
use std::borrow::Borrow;
use std::mem::PinMut;
use std::boxed::PinBox;

use std::marker::PhantomData;
use frunk::*;
use frunk::prelude::*;
use frunk_core::*;
use frunk_core::hlist::*;
use frunk_core::indices::*;
use failure::{Error, Fail};
use std::mem::size_of;

mod traits;
pub use crate::traits::*;

mod folds;
pub use crate::folds::*;

trait At<Name> {
    type AtRes;

    fn at(self) -> Self::AtRes;
}

trait Entry {
    type Path: Clone;
    type Data;

    fn get_path(&self) -> Self::Path;
    fn get_data(self) -> (Self::Path, Self::Data);
    fn borrow_data(&self) -> &Self::Data;
}

pub trait InitSize {
    const SIZE: usize;
}

impl InitSize for HNil {
    const SIZE: usize = 0;
}

impl<P, T, F> InitSize for Const<P, T, F>
where T: Sized, F: Fn() -> T
{
    const SIZE: usize = size_of::<T>();
}

impl<H: InitSize, T: InitSize> InitSize for HCons<H, T> {
    const SIZE: usize = H::SIZE + T::SIZE;
}

#[derive(Copy, Debug)]
pub struct Path<T> {
    path: PhantomData<T>
}

impl <T> Clone for Path<T> {
    fn clone(&self) -> Path<T> {
        Path::new()
    }
}

impl<P> Path<P> {
    pub fn new() -> Path<P> {
        Path {
            path: PhantomData
        }
    }
}

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

pub struct Const<P, T: Sized, F: Fn() -> T> {
    path: Path<P>,
    ty: Type<T>,
    init: F
}

impl<P, T: Sized, F: Fn() -> T> Const<P, T, F> {
    pub fn new(path: P, init: F) -> Const<P, T, F> {
        Const {
            path: Path::new(),
            ty: Type::new(),
            init
        }
    }
}

impl<P, T: Sized, F: Fn() -> T> Entry for Const<P, T, F> {
    type Path = Path<P>;
    type Data = F;

    fn get_path(&self) -> Path<P> {
        Path::new()
    }

    fn get_data(self) -> (Path<P>, F) {
        (self.path, self.init)
    }

    fn borrow_data(&self) -> &F {
        &self.init
    }
}



pub fn instantiate<'a, T>(list: &'a T) -> PinBox<T>
where
    T: ToRef<'a>,
    <T as ToRef<'a>>::Output: FoldL<GetSizeFold, usize, Output=usize> + FoldR<InitFold, *mut (), Output=*mut ()>,
{
    let fold = GetSizeFold;
    
    let capacity = list.to_ref().foldl(fold, 0);
    println!("cap: {}", capacity);
    let vec = Vec::<u8>::with_capacity(capacity);
    unsafe {
        let vec_ptr = vec.as_ptr();
        println!("p1: {:?}", vec_ptr);
        let ptr = vec_ptr.offset(capacity as isize);
        println!("p2: {:?}", ptr);
        // println!("test {:?}", ptr as usize);
        let ptr: *mut () = std::mem::transmute(ptr);
        println!("starting at... {:?}", ptr);
        let fold = InitFold {};
        let ptr = list.to_ref().foldr(fold, ptr);
        println!("ending at... {:?}", ptr);
        let ptr: *mut u8 = std::mem::transmute(ptr);
        PinBox::<T>::from_raw(std::mem::transmute(vec_ptr))
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
            println!("final ptr: {:?}", ptr);
            assert_eq!(1, *ptr);
            assert_eq!(2, *(ptr.offset(1)));
            println!("final ptr: {:?}", ptr.offset(1));
            assert_eq!(3, *(ptr.offset(2)));
            println!("final ptr: {:?}", ptr.offset(2));

        };
    }
}