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

/// Trait for performing a left fold over an HList
///
/// This trait is part of the implementation of the inherent method
/// [`HCons::foldl`]. Please see that method for more information.
///
/// You only need to import this trait when working with generic
/// HLists or Mappers of unknown type. If the type of everything is known,
/// then `list.foldl(f, acc)` should "just work" even without the trait.
///
/// [`HCons::foldl`]: struct.HCons.html#method.foldl
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
    F: Fn(Acc, H) -> R
{
    type Output = <Tail as FoldL<F, R>>::Output;

    fn foldl(self, folder: F, acc: Acc) -> Self::Output {
        let HCons { head, tail } = self;
        let res = folder(acc, head);
        tail.foldl(folder, res)
    }
}

pub trait FoldR<Folder, Init> {
    type Output;

    /// Perform a right fold over an HList.
    ///
    /// Please see the [inherent method] for more information.
    ///
    /// The only difference between that inherent method and this
    /// trait method is the location of the type parameters.
    /// (here, they are on the trait rather than the method)
    ///
    /// [inherent method]: struct.HCons.html#method.foldr
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

impl<'a, F, R, H, Tail, Init> FoldR<&'a F, Init> for HCons<H, Tail>
where
    Tail: FoldR<&'a F, Init>,
    F: Clone + Fn(H, <Tail as FoldR<&'a F, Init>>::Output) -> R,
{

    fn foldr(self, folder: &'a F, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder, init);
        (folder)(self.head, folded_tail)
    }
}


// impl<'a, F, R, H, Tail, Init> FoldR<F, Init> for HCons<H, Tail>
// where
//     Tail: FoldR<F, Init>,
//     F: Clone + Fn(H, <Tail as FoldR<F, Init>>::Output) -> R,
// {
//     type Output = R;

//     fn foldr(self, folder: F, init: Init) -> Self::Output {
//         let folded_tail = self.tail.foldr(folder.clone(), init);
//         (folder)(self.head, folded_tail)
//     }
// }