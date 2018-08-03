use std::ops::{Index, Deref, DerefMut};

use std::rc::Rc;
use std::cell::Cell;

use frunk::hlist::{HCons, HNil};
use frunk_core::indices::{Here, There};
use frunk_core::traits::*;
use crate::*;

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

pub trait FoldL<Folder, Acc> {
    type Output;
    
    fn foldl(self, folder: Folder, acc: Acc) -> Self::Output;
}

impl<F, Acc> FoldL<F, Acc> for HNil {
    type Output = Acc;

    fn foldl(self, _: F, acc: Acc) -> Self::Output {
        acc
    }
}

impl<F, Acc, H, R, Tail> FoldL<F, Acc> for HCons<H, Tail>
where
    Tail: FoldL<F, R>,
    F: Clone + FnOnce(Acc, H) -> R
{
    type Output = <Tail as FoldL<F, R>>::Output;

    fn foldl(self, folder: F, acc: Acc) -> Self::Output {
        let HCons { head, tail } = self;
        let res = (folder.clone())(acc, head);
        tail.foldl(folder, res)
    }
}

pub trait FoldR<Folder, Init> {
    type Output;

    fn foldr(self, folder: Folder, i: Init) -> Self::Output;
}

impl<F, Init> FoldR<F, Init> for HNil {
    type Output = Init;

    fn foldr(self, _: F, i: Init) -> Self::Output {
        i
    }
}

impl<F, FolderHeadR, H, Tail, Init> FoldR<F, Init>
    for HCons<H, Tail>
where
    Tail: FoldR<F, Init>,
    F: Clone + FnOnce(H, <Tail as FoldR<F, Init>>::Output) -> FolderHeadR,
{
    type Output = FolderHeadR;

    default fn foldr(self, folder: F, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder.clone(), init);
        (folder)(self.head, folded_tail)
    }
}

pub trait Map<Mapper: Clone> {
    type Output;

    fn map(self, mapper: Mapper) -> Self::Output;
}

impl<F: Clone> Map<F> for HNil {
    type Output = HNil;

    fn map(self, _: F) -> Self::Output {
        HNil
    }
}

impl<F, R, H, T, T1, T2> Map<F> for HCons<H, T>
where
    F: Clone + Fn(H) -> R,
    T: Map<F, Output=T1>,
    T1: Add<R, Output=T2>
{
    type Output = T2;

    fn map(self, f: F) -> Self::Output {
        let HCons { head, tail } = self;
        tail.map(f.clone()) + f(head)
    }
}

pub trait AsChild<P> {
    type Output;

    fn as_child(self) -> Self::Output;
}

impl<P> AsChild<P> for HNil {
    type Output = HNil;

    fn as_child(self) -> Self::Output {
        HNil
    }
}

impl<P, P2, C, H, T, T1, T2> AsChild<C> for HCons<(Path<P>, H), T>
where
    T: AsChild<C, Output=T1>,
    C: Add<P, Output=P2>,
    T1: Add<Hlist![(Path<P2>, H)], Output=T2>
{
    type Output = T2;

    fn as_child(self) -> Self::Output {
        self.tail.as_child() + hlist![(Path::new(), self.head.1)]
    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    path!(P1, P2, P3, P4);

    #[test]
    fn add_child() {
        let list = hlist![
            (Path::<Hlist![P1]>::new(), 1),
            (Path::<Hlist![P2]>::new(), 2),
            (Path::<Hlist![P3]>::new(), 3),
            (Path::<Hlist![P4]>::new(), 4),
        ];

        println!("1: {:?}", list);
        let list = AsChild::<Hlist![P1]>::as_child(list);
        let list = AsChild::<Hlist![P2]>::as_child(list);
        let list = AsChild::<Hlist![P3]>::as_child(list);
        println!("2: {:?}", list);        
    }
}