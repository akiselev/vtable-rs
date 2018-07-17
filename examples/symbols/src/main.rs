#![feature(rust_2018_preview, proc_macro)]

use vtable_derive::symbol;

symbol! {
    pub width = f32 as Width;
}

symbol! {
    pub height = f32 as Height<T: Copy>;
}


// symbol! {
//     pub let height: Height<T: Clone> = ::std::collections::VecDeque<T>;
// }


fn main() {
    println!("Hello, world!");
}
