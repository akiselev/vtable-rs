#![feature(async_await, await_macro, associated_type_defaults, coerce_unsized, pin, fn_traits, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident, never_type, specialization, unboxed_closures)]

use std::ops::{Index, Deref, DerefMut};
use std::borrow::Borrow;
use std::mem::PinMut;
use std::boxed::PinBox;
use std::ops::Add;

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

mod path;
pub use crate::path::*;

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
    let vec = Vec::<u8>::with_capacity(capacity);
    unsafe {
        let vec_ptr = vec.as_ptr();
        let ptr = vec_ptr.offset(capacity as isize);
        // println!("test {:?}", ptr as usize);
        let ptr: *mut () = std::mem::transmute(ptr);
        let fold = InitFold {};
        let ptr = list.to_ref().foldr(fold, ptr);
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
            assert_eq!(1, *ptr);
            assert_eq!(2, *(ptr.offset(1)));
            assert_eq!(3, *(ptr.offset(2)));
        };
    }
}

struct Builder<O> {
    data: O
}

trait Init<P, F> {
    type Output;

    fn init(self, func: F) -> Self::Output;
}

trait Parent {
    type Path;
}

impl<H, T> Parent for Path<HCons<H, T>> {
    type Path = T;
}

impl<H, T> Parent for (Path<H>, T) where Path<H>: Parent {
    type Path = <Path<H> as Parent>::Path;
}

impl<O, O1, O2, O3, O4, O5, P, F> Init<P, F> for Builder<O>
where
    O: Add<F, Output=O1>,
    O1: for<'refs> ToRef<'refs, Output=HCons<&'refs F, &'refs O2>>,
    F: for<'init> Fn(&'init O2) -> O3,
    O3: AsChild<P, Output=O4>,
    O1: Add<O4, Output=O5>,
    O4: HList
{
    type Output = O5;

    fn init(self, func: F) -> Self::Output {
        let Builder { data } = self;
        let data = data + func;
        let new = {
            let HCons { head, tail } = data.to_ref();
            head(&tail).as_child()
        };
        data + new
    }
}

trait Replace<P, F> {
    type Output;

    fn replace(self, func: F) -> Self::Output;
}

// impl<O, O1, O2, P, F> Replace<P, F> for Builder<O>
// where
//     O: Map<PathMapper, Output=O1>,
//     F: Fn(O1) -> O2,
//     O2: 
// {

// }

fn testfn() {

}

#[cfg(test)]
mod tests2 {
    use crate::*;
    use frunk_core::hlist::*;

    #[test]
    fn test() {
        let x = hlist![0, 1] + hlist![2, 3];
        println!("{:?}", x);
    }
}