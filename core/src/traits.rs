use std::ops::{Index, Deref, DerefMut};

pub trait Symbol: Copy + Clone {
    type Type;
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

pub trait GetVirtual<'this, S>
where
    S: Symbol,
    Self: 'this,
{
    type Output: VirtualRef<'this, S>;

    fn get_ref(&'this self) -> Self::Output;
}

pub trait GetVirtualMut<'this, S>
where
    S: Symbol,
    Self: 'this,
{
    type Output: VirtualRefMut<'this, S>;

    fn get_mut_ref(&'this mut self) -> Self::Output;
}
