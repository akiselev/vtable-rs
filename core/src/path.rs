use std::fmt::{Formatter, Debug, Result as DebugResult};
use std::marker::PhantomData;

use frunk::hlist::{HCons, HNil};
use frunk_core::indices::{Here, There};
use frunk_core::traits::*;

#[derive(Copy)]
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

macro_rules! path {
    ($id:ident) => {
        #[derive(Debug)]
        struct $id;

        impl DebugPath for $id {
            fn fmt_path(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", stringify!($id))
            }
        }
    };
    ($($id:ident,)*) => {
        $(path!($id);)*
    };
    ($($id:ident),*) => {
        $(path!($id);)*
    }
}

pub trait DebugPath {
    fn fmt_path(f: &mut Formatter) -> DebugResult;
}

impl DebugPath for HNil {
    fn fmt_path(f: &mut Formatter) -> DebugResult {
        write!(f, "")
    }
}

impl<H: DebugPath, T: DebugPath> DebugPath for HCons<H, T> {
    fn fmt_path(f: &mut Formatter) -> DebugResult {
        <H as DebugPath>::fmt_path(f)?;
        write!(f, "/")?;
        <T as DebugPath>::fmt_path(f)
    }
}



impl<P: DebugPath> Debug for Path<P> {
    fn fmt(&self, f: &mut Formatter) -> DebugResult {
        write!(f, "/")?;
        <P as DebugPath>::fmt_path(f)
    }
}

trait ChildOfInner<P> {}

impl<H> ChildOfInner<HNil> for HCons<H, HNil> {}
impl<P, H, T> ChildOfInner<P> for HCons<H, T> where T: ChildOfInner<P> {}

pub trait ChildOf<P> {}

impl<P, O, P1, O1> ChildOf<P> for Path<O>
where O: IntoReverse<Output=O1>, P: IntoReverse<Output=P1>, O1: ChildOfInner<P1>
{}

pub trait TestTrait {}

impl TestTrait for f32 {}

#[cfg(test)]
mod tests {
    use crate::*;

    path!(P1, P2, P3, P4);

    #[test]
    fn add_child() {
        let list = hlist![
            (Path::<Hlist![P2]>::new(), 2f32),
            (Path::<Hlist![P4]>::new(), 4f32),
        ];

        println!("1: {:?}", list);
        let list = AsChild::<Hlist![P1]>::as_child(list);
        let list = AsChild::<Hlist![P2]>::as_child(list);
        let list = AsChild::<Hlist![P3]>::as_child(list);
        println!("2: {:?}", list);
        // let list2 = FilterBy::<TestTrait>::filter_by(list);
    }
}