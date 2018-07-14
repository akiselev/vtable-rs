use std::ops::{Index, Deref, DerefMut};

pub trait Symbol: Copy + 'static {
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

pub trait Virtual<'this, 'virt, S>
where
    S: Symbol,
    Self: 'this,
    'this: 'virt
{
    type Output: VirtualRef<'virt, S>;

    fn get_ref(&'this self) -> Self::Output;
}

pub trait VirtualMut<'this, 'virt, S>
where
    S: Symbol,
    Self: 'this,
    'this: 'virt
{
    type Output: VirtualRefMut<'virt, S>;

    fn get_mut_ref(&'this mut self) -> Self::Output;
}
