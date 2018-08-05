use std::fmt::{Formatter, Debug, Result as DebugResult};
use std::marker::PhantomData;
use std::ops::Add;

use frunk::hlist::{HCons, HNil, HList};
use frunk_core::indices::{Here, There};
use frunk_core::traits::*;

use serde::Serialize;
use serde::ser::{Serialize as SerializeTrait, Serializer, SerializeStruct};

#[derive(Copy)]
pub struct Path<T> {
    path: PhantomData<HCons<T, HNil>>
}

impl<P: DebugPath> SerializeTrait for Path<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {

        serializer.serialize_str(&format!("/{}", P::get_name()))
    }
}

impl<T> Clone for Path<T> {
    fn clone(&self) -> Path<T> {
        Path { path: PhantomData }
    }
}

impl<P> Path<P> {
    pub fn new() -> Path<P> {
        Path {
            path: PhantomData
        }
    }
}

#[derive(Copy, Clone, Serialize)]
pub struct Route<P: DebugPath, C: DebugPath, T> {
    parent: Path<P>,
    child: Path<C>,
    data: PhantomData<T>
}

impl<P: DebugPath, C: DebugPath, T> Route<P, C, T> {
    pub fn new() -> Route<P, C, T> {
        Route {
            parent: Path::new(),
            child: Path::new(),
            data: PhantomData
        }
    }

    pub fn path<A>(&self) -> Path<A>
    where P: Add<C, Output=A>
    {
        Path::new()
    }
}


impl<P, C, T, A> From<Route<P, C, T>> for Path<A>
where P: Add<C, Output=A>, C: DebugPath, P: DebugPath
{
    fn from(other: Route<P, C, T>) -> Path<A> {
        Path::new()
    }
}

impl<P, C, T> Debug for Route<P, C, T>
where C: DebugPath, P: DebugPath
{
    fn fmt(&self, f: &mut Formatter) -> DebugResult {
        write!(f, "(")?;
        <P as DebugPath>::fmt_path(f)?;
        write!(f, " -> ")?;
        <C as DebugPath>::fmt_path(f)?;
        write!(f, ")")
    }
}

pub trait DebugPath {
    fn fmt_path(f: &mut Formatter) -> DebugResult;
    fn get_name() -> String;
}

impl DebugPath for HNil {
    fn fmt_path(f: &mut Formatter) -> DebugResult {
        write!(f, "-")
    }

    fn get_name() -> String {
        ":".to_owned()
    }
}

impl<H: DebugPath, T: DebugPath> DebugPath for HCons<H, T> {
    fn fmt_path(f: &mut Formatter) -> DebugResult {
        <H as DebugPath>::fmt_path(f)?;
        write!(f, "/")?;
        <T as DebugPath>::fmt_path(f)
    }

    fn get_name() -> String {
        format!("{}{}", H::get_name(), T::get_name())
    }
}

impl<P: DebugPath> Debug for Path<P> {
    fn fmt(&self, f: &mut Formatter) -> DebugResult {
        write!(f, "/")?;
        <P as DebugPath>::fmt_path(f)
    }
}

pub trait ChildOfInner<P> {}

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

    create_path!(P1, P2, P3, P4);

    #[test]
    fn add_child() {
        let list = hlist![
            (Path::<Hlist![P2]>::new(), 2f32),
            (Path::<Hlist![P4]>::new(), 4f32),
        ];

        let list = AsChild::<Hlist![P1]>::as_child(list);
        let list = AsChild::<Hlist![P2]>::as_child(list);
        let list = AsChild::<Hlist![P3]>::as_child(list);
    }
}