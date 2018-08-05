// #![feature(async_await, await_macro, associated_type_defaults, unsize, coerce_unsized, pin, fn_traits, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident, never_type, specialization, unboxed_closures)]
#![feature(impl_specialization, rust_2018_preview, nll, unsize, coerce_unsized, fn_traits, pin, arbitrary_self_types, never_type, specialization, unboxed_closures)]

use std::boxed::PinBox;
use std::marker::PhantomData;
use std::ops::Add;

use frunk::*;
use serde::{Serialize};

mod traits;
pub use crate::traits::*;

mod folds;
pub use crate::folds::*;

#[macro_use]
mod macros;

mod path;
pub use crate::path::*;

mod builder;
pub use crate::builder::*;

mod primitives;
pub use crate::primitives::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[macro_use]
    use crate::macros;

    create_path!(P1, P2, P3, P4);

    #[test]
    fn testfn() {
        let list = hlist![
            Const::new(P1, || 1),
            Const::new(P2, || 2),
            Const::new(P3, || 3),
        ];
        unsafe {
            let pinbox = instantiate(&list);
            let ptr: *const u32 = std::mem::transmute(PinBox::into_raw(pinbox));
            assert_eq!(1, *ptr);
            assert_eq!(2, *(ptr.offset(1)));
            assert_eq!(3, *(ptr.offset(2)));
        };
    }
}

#[cfg(test)]
mod tests2 {
    // use crate::*;
    // #[macro_use]
    // use crate::macros;
    // use frunk_core::hlist::*;
    use funky::hlist::*;
    use funky::hlist;


    #[derive(Debug)]
    struct P1;
    #[derive(Debug)]
    struct P2;
    #[derive(Debug)]
    struct P3;
    #[derive(Debug)]
    struct P4;
    #[derive(Debug)]
    struct P5;

    #[test]
    fn testfunky() {
        let h1 = hlist![P1, P2, P3];
        let h2 = hlist![P4, P5];
        let i1 = h1.index_of::<P1, _>();
        println!("1: {:?}", i1);
        let h3 = hlist![h1, h2];
        println!("h3: {:?}", h3);
    }

    // create_path!(P1, P2, P3, P4, P5);

    // #[test]
    // fn test() {
    //     // println!("PATH 1: {:?}", hlist![P1] + (hlist![P2] + hlist![P3]));
    //     // println!("     1: {:?}", hlist![P1] + (hlist![P2] + hlist![P3]));
    //     let builder = Builder::new(|builder| {
    //         let builder = builder.push::<P1, _>(0f32)
    //             .push::<P2, _>(1f32);
    //         // println!("/.....{:?}", builder.data);
    //         let other = Builder::new(|builder| {
    //             builder.push::<P3, _>(7)
    //         });
    //         // println!("/2.....{:?}", other.data);
    //         let builder = builder + other;
    //         builder.with::<P5, _, _>(|builder| {
    //             builder.push::<P4, _>(23u64)
    //         }).add(P4, 20f64)
    //     });

    //     // println!("{}", serde_json::to_string_pretty(&builder).unwrap());
    //     // println!("{:?}", builder.data);
    // }
}