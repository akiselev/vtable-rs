use std::ops::CoerceUnsized;

use frunk::hlist::{HCons, HNil};
use frunk_core::traits::*;
use crate::*;

pub trait FoldL<Folder, Acc> {
    type Output;
    
    fn foldl(self, folder: Folder, acc: Acc) -> Self::Output;
}

impl<F, Acc: HList> FoldL<F, Acc> for HNil {
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

impl<F, Init> FoldR<F, Init> for HNil where Init: HList {
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
    FolderHeadR: HList
{
    type Output = FolderHeadR;

    default fn foldr(self, folder: F, init: Init) -> Self::Output {
        let folded_tail = self.tail.foldr(folder.clone(), init);
        (folder)(self.head, folded_tail)
    }
}

pub trait Map<Mapper: Clone> {
    type Output: HList;

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
    T1: Add<R, Output=T2> + HList,
    T2: HList
{
    type Output = T2;

    fn map(self, f: F) -> Self::Output {
        let HCons { head, tail } = self;
        tail.map(f.clone()) + f(head)
    }
}

pub trait Append<O> {
    type Output;

    fn append(self, other: O) -> Self::Output;
}

impl<O, T, T1, T2> Append<O> for T
where T: Add<O, Output=T1>, T1: IntoReverse<Output=T2>
{
    type Output = T2;

    fn append(self, other: O) -> Self::Output {
        (self + other).into_reverse()
    }
}

pub trait AsChild<P> {
    type Output: HList;

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
    T1: Append<Hlist![(Path<P2>, H)], Output=T2> + HList,
    T2: HList
{
    type Output = T2;

    fn as_child(self) -> Self::Output {
        self.tail.as_child().append(hlist![(Path::new(), self.head.1)])
    }
}

trait ToHNil { type Output: HList; fn to_hnil(self) -> Self::Output; }
impl<T> ToHNil for T { type Output = HNil; fn to_hnil(self) -> HNil { HNil } }

pub trait FilterBy<F: ?Sized> {
    type Output: HList;

    fn filter_by(self) -> Self::Output;
}

// default impl<F: ?Sized, T> FilterBy<F> for T where T: ToHNil, <T as ToHNil>::Output: HList {
//     type Output = <T as ToHNil>::Output;

//     fn filter_by(self) -> Self::Output {
//         *!
//     }
// }

impl<F: ?Sized, P, T, H, H1, H2> FilterBy<F> for HCons<(Path<P>, T), H>
where
    F: CoerceUnsized<T>,
    H: FilterBy<F, Output=H1>,
    H1: HList,
    H2: HList,
    HCons<(Path<P>, T), HNil>: Append<H1, Output=H2>,
{
    type Output = <HCons<(Path<P>, T), HNil> as Append<H1>>::Output;

    fn filter_by(self) -> <HCons<(Path<P>, T), HNil> as Append<H1>>::Output {
        HCons { head: self.head, tail: HNil }.append(self.tail.filter_by())
    }
}

// impl<'a, F: ?Sized, P, T, H, H1> FilterBy<F> for HCons<&'a (Path<P>, T), H>
// where
//     F: CoerceUnsized<T>,
//     H: FilterBy<F, Output=H1> + ToHNil<Output=HNil>,
//     H1: HList,
//     H2: HList,
//     HCons<&'a (Path<P>, T), HNil>: Append<H1, Output=H2>,
// {
//     type Output = <HCons<&'a (Path<P>, T), HNil> as Append<H1>>::Output;

//     fn filter_by(self) -> <HCons<&'a (Path<P>, T), HNil> as Append<H1>>::Output {
//         hlist![self.head].append(self.tail.filter_by())
//     }
// }

default impl<H, T, F: ?Sized> FilterBy<F> for HCons<T, H>
where
    H: FilterBy<F>,
{
    type Output = <H as FilterBy<F>>::Output;

    fn filter_by(self) -> Self::Output {
        self.filter_by()
    }
}

// trait DefaultResult {
//     type Output;

//     fn get() -> Self::Output;
// }

// impl<T> DefaultResult for T {
//     type Output = HNil;

//     fn get() -> Self::Output {
//         HNil
//     }
// }

// impl<F: ?Sized, T> FilterBy<F> for T {
//     default type Output = HNil;

//     default fn filter_by(self) -> ! {
//         HNil
//     }
// }