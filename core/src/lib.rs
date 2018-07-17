use std::ops::{Index, Deref, DerefMut};
use std::borrow::Borrow;

mod traits;
pub use traits::*;

struct SizeObj {
    width: f32,
    height: f32,
}

struct PositionObj {
    x: f32,
    y: f32
}

struct TestObj {
    size: SizeObj,
    position: PositionObj
}

struct TestObjRef<'virt, S: Symbol> {
    ptr: &'virt S::Type,
}

impl<'virt, S: Symbol> Deref for TestObjRef<'virt, S> {
    type Target = S::Type;

    fn deref(&self) -> &S::Type {
        self.ptr
    }
}

impl<'virt, S: Symbol> VirtualRef<'virt, S> for TestObjRef<'virt, S> {}

impl<'this> GetVirtual<'this, Size> for TestObj
where
    Self: 'this,
{
    type Output = TestObjRef<'this, Size>;

    fn get_ref(&'this self) -> TestObjRef<'this, Size> {
        TestObjRef { ptr: &self.size }
    }
}

impl<'this> GetVirtual<'this, Position> for TestObj
where
    Self: 'this,
{
    type Output = TestObjRef<'this, Position>;

    fn get_ref(&'this self) -> TestObjRef<'this, Position> {
        TestObjRef { ptr: &self.position }
    }
}

macro_rules! impl_virt {
    () => {

    };
}

macro_rules! Ty {
    ($t:ty) => {
        <$t as Symbol>::Type
    };
}

// impl<S: Symbol, T: This<Width>> Index<T> for Width {
//     type Output = usize;

//     fn index(&self, nucleotide: Nucleotide) -> &usize {
//         match nucleotide {
//             Nucleotide::A => &self.a,
//             Nucleotide::C => &self.c,
//             Nucleotide::G => &self.g,
//             Nucleotide::T => &self.t,
//         }
//     }
// }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Size;
impl Symbol for Size { type Type = SizeObj; }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position;
impl Symbol for Position { type Type = PositionObj; }

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Obj;
impl Symbol for Obj { type Type = TestObj; }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
