use frunk::prelude::*;
use frunk_core::*;
use frunk_core::hlist::*;
use frunk_core::indices::*;
use crate::*;

#[derive(Clone, Debug)]
pub struct GetSizeFold;

impl<P, T, F> FnOnce<(usize, &Const<P, T, F>)> for GetSizeFold
where T: Sized, F: Fn() -> T
{
    type Output = usize;
    
    extern "rust-call" fn call_once(self, args: (usize, &Const<P, T, F>)) -> usize {
        let (size, var) = args;
        size + std::mem::size_of::<T>()
    }
}

impl<P, T, F> FnMut<(usize, &Const<P, T, F>)> for GetSizeFold
where T: Sized, F: Fn() -> T
{
    extern "rust-call" fn call_mut(&mut self, args: (usize, &Const<P, T, F>)) -> usize {
        let (size, var) = args;
        size + std::mem::size_of::<T>()
    }
}

impl<P, T, F> Fn<(usize, &Const<P, T, F>)> for GetSizeFold
where T: Sized, F: Fn() -> T
{
    
    extern "rust-call" fn call(&self, args: (usize, &Const<P, T, F>)) -> usize {
        let (size, var) = args;
        size + std::mem::size_of::<T>()
    }
}

#[derive(Clone, Debug)]
pub struct InitFold;

// impl<P, T, F> FnOnce<(*mut (), &Const<P, T, F>)> for InitFold
// where T: Sized, F: Fn() -> T
// {
//     type Output = *mut ();
    
//     extern "rust-call" fn call_once(self, args: (*mut (), &Const<P, T, F>)) -> *mut () {
//         let (ptr, var) = args;
//         let ptr = unsafe {
//             let dest: *mut T = std::mem::transmute(ptr);
//             println!("addr: {:?}", dest);
//                 *dest = (&var.init)();
//             let next = dest.offset(1);
//             std::mem::transmute(next)
//         };
//         ptr
//     }
// }

// impl<P, T, F> FnMut<(*mut (), &Const<P, T, F>)> for InitFold
// where T: Sized, F: Fn() -> T
// {
//     extern "rust-call" fn call_mut(&mut self, args: (*mut (), &Const<P, T, F>)) -> *mut () {
//         let (ptr, var) = args;
//         let ptr = unsafe {
//             let dest: *mut T = std::mem::transmute(ptr);
//             println!("addr: {:?}", dest);
//             *dest = (&var.init)();
//             let next = dest.offset(1);
//             std::mem::transmute(next)
//         };
//         ptr
//     }
// }

// impl<P, T, F> Fn<(*mut (), &Const<P, T, F>)> for InitFold
// where T: Sized, F: Fn() -> T
// {
    
//     extern "rust-call" fn call(&self, args: (*mut (), &Const<P, T, F>)) -> *mut () {
//         let (ptr, var) = args;
//         let ptr = unsafe {
//             let dest: *mut T = std::mem::transmute(ptr);
//             println!("addr: {:?}", dest);
//             *dest = (&var.init)();
//             let next = dest.offset(1);
//             std::mem::transmute(next)
//         };
//         ptr
//     }
// }

impl<P, T, F> FnOnce<(&Const<P, T, F>, *mut ())> for InitFold
where T: Sized + std::fmt::Debug, F: Fn() -> T
{
    type Output = *mut ();
    
    extern "rust-call" fn call_once(self, args: (&Const<P, T, F>, *mut ())) -> *mut () {
        let (var, ptr) = args;
        let ptr = unsafe {
            let dest: *mut T = std::mem::transmute(ptr);
            println!("pp1 {:?}", dest);
            let dest = dest.offset(-1);
            let init = (&var.init)();
            println!("init: {:?}", init);
            *dest = init;
            println!("pp2 {:?}", dest);
            std::mem::transmute(dest)
        };
        ptr
    }
}   

impl<P, T, F> FnMut<(&Const<P, T, F>, *mut ())> for InitFold
where T: Sized + std::fmt::Debug, F: Fn() -> T
{
    extern "rust-call" fn call_mut(&mut self, args: (&Const<P, T, F>, *mut ())) -> *mut () {
        let (var, ptr) = args;
        let ptr = unsafe {
            let dest: *mut T = std::mem::transmute(ptr);
            println!("pp1 {:?}", dest);
            let dest = dest.offset(-1);
            let init = (&var.init)();
            println!("init: {:?}", init);
            *dest = init;
            println!("pp2 {:?}", dest);
            std::mem::transmute(dest)
        };
        ptr
    }
}

impl<P, T, F> Fn<(&Const<P, T, F>, *mut ())> for InitFold
where T: Sized + std::fmt::Debug, F: Fn() -> T
{
    
    extern "rust-call" fn call(&self, args: (&Const<P, T, F>, *mut ())) -> *mut () {
        let (var, ptr) = args;
        let ptr = unsafe {
            let dest: *mut T = std::mem::transmute(ptr);
            println!("pp1 {:?}", dest);
            let dest = dest.offset(-1);
            let init = (&var.init)();
            println!("init: {:?}", init);
            *dest = init;
            println!("pp2 {:?}", dest);
            std::mem::transmute(dest)
        };
        ptr
    }
}