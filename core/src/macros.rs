
#[macro_export]
macro_rules! create_path {
    ($id:ident) => {
        #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
        struct $id;

        impl crate::path::DebugPath for $id {
            fn fmt_path(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($id))
            }

            fn get_name() -> String {
                stringify!($id).to_owned()
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, stringify!($id))
            }
        }

        impl std::ops::Add<$id> for HNil
        {
            type Output = HCons<$id, HNil>;

            fn add(self, other: $id) -> HCons<$id, HNil> {
                HCons { head: other, tail: self }
            }
        }

        impl std::ops::Add<HNil> for $id
        {
            type Output = HCons<$id, HNil>;

            fn add(self, other: HNil) -> HCons<$id, HNil> {
                HCons { head: self, tail: other }
            }
        }

        impl<H, T> std::ops::Add<HCons<H, T>> for $id
        where T: HList
        {
            type Output = <HCons<$id, HNil> as Add<HCons<H, T>>>::Output;

            fn add(self, other: HCons<H, T>) -> <HCons<$id, HNil> as Add<HCons<H, T>>>::Output {
                HCons { head: self, tail: HNil } + other
            }
        }
    };
    ($($id:ident,)*) => {
        $(create_path!($id);)*
    };
    ($($id:ident),*) => {
        $(create_path!($id);)*
    }
}