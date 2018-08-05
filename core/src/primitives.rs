use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct Type<T> {
    path: PhantomData<T>
}

impl<T> Type<T> {
    pub fn new() -> Type<T> {
        Type {
            path: PhantomData
        }
    }
}

pub struct Const<P, T: Sized, F: Fn() -> T> {
    crate path: Path<P>,
    crate ty: Type<T>,
    crate init: F
}

impl<P, T: Sized, F: Fn() -> T> Const<P, T, F> {
    pub fn new(path: P, init: F) -> Const<P, T, F> {
        Const {
            path: Path::new(),
            ty: Type::new(),
            init
        }
    }
}

impl<P, T: Sized, F: Fn() -> T> Entry for Const<P, T, F> {
    type Path = Path<P>;
    type Data = F;

    fn get_path(&self) -> Path<P> {
        Path::new()
    }

    fn get_data(self) -> (Path<P>, F) {
        (self.path, self.init)
    }

    fn borrow_data(&self) -> &F {
        &self.init
    }
}