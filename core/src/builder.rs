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

impl<F, H: DebugPath, O> Entry for Builder<F, H, O> {
    type Path = Path<H>;
    type Data = O;

    fn get_path(&self) -> Self::Path {
        self.path.clone()
    }

    fn get_data(self) -> (Self::Path, Self::Data) {
        (self.path, self.data)
    }

    fn borrow_data(&self) -> &Self::Data {
        &self.data
    }
}

pub struct Class<T, FINAL> {
    class: T,
    data: FINAL
}

pub trait Constructor: Clone {
    type Output;

    fn constructor(&self) -> Self::Output;
}

#[derive(Serialize)]
pub struct Builder<FINAL, H = HNil, O=HNil>
where
    H: DebugPath,
{
    crate path: Path<H>,
    crate data: O,
    crate final_type: PhantomData<FINAL>
}

impl<FINAL> Builder<FINAL, HNil, HNil> {
    pub fn new<F: Clone, O>(constructor: F) -> FINAL
    where 
        F: Clone + FnOnce(Builder<FINAL, HNil, HNil>) -> Builder<FINAL, HNil, O>,
        Builder<FINAL, HNil, HNil>: Init<FINAL, F, HNil>,
        FINAL: Clone,
        Builder<FINAL, HNil, <Builder<FINAL, HNil, HNil> as Init<FINAL, F, HNil>>::Output>: Constructor<Output=FINAL>
    {
        Builder {
            path: Path::new(),
            data: Builder { path: Path::new(), data: HNil, final_type: PhantomData }.init(constructor),
            final_type: PhantomData
        }.constructor()
    }
}

impl<FINAL, H, F, P, O1, O2> Init<FINAL, F, P> for Builder<FINAL, H, O1>
where
    H: DebugPath,
    F: Clone + FnOnce(Builder<FINAL, H, HNil>) -> Builder<FINAL, H, O2>,
    O2: AsChild<P>,
    O1: Add<<O2 as AsChild<P>>::Output>,
    // <O1 as Add<<O2 as AsChild<P>>::Output>>::Output: HList
{
    type Output = <O1 as Add<<O2 as AsChild<P>>::Output>>::Output;

    fn init(self, func: F) -> Self::Output {
        let Builder { path, data, final_type } = self;
        let data = data;
        let new_data = func(Builder { path, data: HNil, final_type }).data;
        let child = new_data.as_child();
        data + child
    }
}

impl<FINAL, H, T> Builder<FINAL, H, T>
where
    H: DebugPath,
{
    pub fn push<C, O: Sized>(self, other: O) -> Builder<FINAL, H, HCons<(Path<<H as Add<C>>::Output>, O), T>>
    where H: Add<C>
    {
        let path = Path::new();
        let head: (Path<<H as Add<C>>::Output>, O) = (path, other);
        Builder {
            path: self.path,
            data: HCons {
                head: head,
                tail: self.data
            },
            final_type: self.final_type
        }
    }

    pub fn add<C, O: Sized>(self, path: C, other: O) -> Builder<FINAL, H, HCons<(Path<<H as Add<C>>::Output>, O), T>>
    where H: Add<C>
    {
        let head: (Path<<H as Add<C>>::Output>, O) = (Path::new(), other);
        Builder {
            path: self.path,
            data: HCons {
                head: head,
                tail: self.data
            },
            final_type: self.final_type
        }
    }

    pub fn pretty_print(&self) where T: Serialize {
        println!("{}", serde_json::to_string(&self.data).unwrap());
    }
}

impl<FINAL, H, T> Builder<FINAL, H, T>
where
    H: DebugPath,
{
    pub fn with<P, F, O>(self, constructor: F) -> Builder<FINAL, H, <Self as Init<FINAL, F, P>>::Output>
    where
        F: Clone + FnOnce(Builder<FINAL, H>) -> Builder<FINAL, H, O>,
        H: Add<P>,
        <H as std::ops::Add<P>>::Output: DebugPath,
        Self: Init<FINAL, F, P>,
    {
        Builder {
            path: Path::new(),
            data: self.init(constructor),
            final_type: PhantomData
        }
    }
}

pub trait Init<FINAL, F, P=HNil> {
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

impl<FINAL, O, H, H2, RHS> Add<Builder<FINAL, H2, RHS>> for Builder<FINAL, H, O>
where
    H: DebugPath,
    H2: DebugPath,
    O: Add<RHS>,
    //RHS: HList, 
    // <O as Add<RHS>>::Output: HList
{
    type Output = Builder<FINAL, H, <O as Add<RHS>>::Output>;

    fn add(self, rhs: Builder<FINAL, H2, RHS>) -> Self::Output {
        Builder {
            path: self.path,
            data: self.data + rhs.data,
            final_type: self.final_type            
        }
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