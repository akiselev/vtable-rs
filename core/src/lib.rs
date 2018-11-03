// #![feature(async_await, await_macro, associated_type_defaults, unsize, coerce_unsized, pin, fn_traits, arbitrary_self_types, futures_api, proc_macro, proc_macro_span, proc_macro_raw_ident, never_type, specialization, unboxed_closures)]
#![feature(crate_visibility_modifier, nll, unsize, coerce_unsized, fn_traits, pin, arbitrary_self_types, never_type, specialization, unboxed_closures)]

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

mod symbol;
pub use crate::primitives::*;

#[cfg(test)]
mod tests2 {
    use crate::*;
    #[macro_use]
    use crate::macros;
    use frunk_core::hlist::*;

    create_path!(P1, P2, P3, P4, P5);

    #[test]
    fn test() {
        let builder = Builder::new(|builder| {
            let builder = builder.push::<P1, _>(0f32)
                .push::<P2, _>(1f32);
            let other = Builder::new(|builder| {
                builder.push::<P3, _>(7)
            });
            let builder = builder + other;
            builder.with::<P5, _, _>(|builder| {
                builder.push::<P4, _>(23u64)
            }).add(P4, 20f64)
        });

        println!("{}", serde_json::to_string_pretty(&builder).unwrap());
    }
}