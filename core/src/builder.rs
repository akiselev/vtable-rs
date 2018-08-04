use std::borrow::Borrow;
use std::boxed::PinBox;
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

use crate::*;

pub trait Entry {
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

pub struct Builder<O> {
    crate data: O
}

pub trait Init<F, P=HNil> {
    type Output;

    fn init(self, func: F) -> Self::Output;
}

pub trait Parent {
    type Path;
}

impl<H, T> Parent for Path<HCons<H, T>> {
    type Path = T;
}

impl<H, T> Parent for (Path<H>, T) where Path<H>: Parent {
    type Path = <Path<H> as Parent>::Path;
}

impl<O, O1, O2, O3, O4, F, P> Init<F, P> for Builder<O>
where
    O: HList + Add<HCons<F, HNil>, Output=O1>,
    F: Clone + FnOnce(Builder<HNil>) -> Builder<O2>,
    O2: AsChild<P, Output=O3>,
    O1: Add<O3, Output=O4>,
    O3: HList, O4: HList
{
    type Output = O4;

    fn init(self, func: F) -> Self::Output {
        let Builder { data } = self;
        let data = data + hlist!(func.clone());
        let new_data = func(Builder { data: HNil }).data;
        let child = new_data.as_child();
        data + child
    }
}

// impl<'this, O, O1, O2, O3, O4, O5, P, F> Init<P, F> for Builder<O>
// where
//     F: Clone + FnOnce(Builder<O>) -> Builder<O2>,
//     HCons<F, HNil>: Add<O2, Output=O3>,
//     O3: AsChild<P, Output=O4>,
//     O: Add<O4, Output=O5>,
//     O2: HList, O4: HList, O5: HList
// {
//     type Output = Builder<O5>;

//     fn init(self, func: F) -> Self::Output {
//         let Builder { data } = self;
        
//         let child = {
//             let input = data.to_ref();
//             let output = (func.clone())(Builder { data: input });
//             hlist![func] + output
//         };
//         Builder {
//             data: data + child.as_child()
//         }
//     }
// }

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