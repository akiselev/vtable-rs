#![feature(rust_2018_preview, proc_macro)]

use vtable_derive::{symbol, interface};

symbol! {
    pub width = f32 as Width;
}

symbol! {
    pub height = f32 as Height<T: Copy>;
}

interface! IRect {
    size: Size;
    position: Position;

    
}


// symbol! {
//     pub let height: Height<T: Clone> = ::std::collections::VecDeque<T>;
// }


fn main() {
    println!("Hello, world!");
}
