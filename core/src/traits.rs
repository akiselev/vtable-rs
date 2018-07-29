use std::ops::{Index, Deref, DerefMut};

use std::rc::Rc;
use std::cell::Cell;

use frunk::hlist::{HCons, HNil};
use frunk_core::indices::{Here, There};

pub trait Symbol: Copy + Clone {
    type Type;

    fn as_path(&self) -> HCons<Self, HNil> {
        HCons {
            head: self.clone(),
            tail: HNil
        }
    }
}

pub trait VPath: Symbol {
    type Parent;
}

pub trait Parent {
    type Path: VPath;
    type Symbol: Symbol;
}

impl<SYM: Symbol, PATH: VPath> VPath for HCons<SYM, PATH> {
    type Parent = PATH;
}

impl<SYM: Symbol, PATH: Copy + Clone> Symbol for HCons<SYM, PATH> {
    type Type = SYM::Type;
}

impl<SYM: Symbol> VPath for HCons<SYM, HNil> {
    type Parent = HNil;
}

impl<SYM: Symbol, PSYM: Symbol, PATH: VPath> Parent for HCons<SYM, HCons<PSYM, PATH>> {
    type Path = PATH;
    type Symbol = PSYM;
}

pub trait VirtualRef<'virt, S> : Deref<Target=S::Type>
where
    S: Symbol,
{}

pub trait VirtualRefMut<'virt, S> : DerefMut<Target=S::Type>
where
    S: Symbol,
{}

// pub trait Concrete<'this, 'virt, S>
// where
//     S: Symbol, Self: 'this, 'this: 'virt, Self::Output: 'virt
// {
//     type Output: VirtualRef<'virt, S>;

//     fn get_ref(&self) -> Self::Output;
// }

pub trait GetVirtual<'this, 'virt, S>
where
    S: Symbol,
    Self: 'this,
    'this: 'virt
{
    type Output: VirtualRef<'virt, S>;

    fn get_ref(&'this self) -> Self::Output;
}

pub trait GetVirtualMut<'this, 'virt, S>
where
    S: Symbol,
    Self: 'this,
    'this: 'virt
{
    type Output: VirtualRefMut<'virt, S>;

    fn get_mut_ref(&'this mut self) -> Self::Output;
}
