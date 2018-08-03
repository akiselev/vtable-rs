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

#[derive(Clone, Debug)]
pub struct InitFold;

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

#[derive(Clone)]
pub struct PathMapper;

impl<'a, P, T, O, Acc> FnOnce<(&'a (Path<P>, T), Acc)> for PathMapper
where Acc: Add<Path<P>, Output=O>
{
    type Output = O;
    
    extern "rust-call" fn call_once(self, args: (&'a (Path<P>, T), Acc)) -> O {
        args.1 + Path::new()
    }
}

impl<'a, Acc> FnOnce<(&'a HNil, Acc)> for PathMapper
{
    type Output = Acc;
    
    extern "rust-call" fn call_once(self, args: (&'a HNil, Acc)) -> Acc {
        args.1
    }
}