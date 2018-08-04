
#[macro_export]
macro_rules! create_path {
    ($id:ident) => {
        #[derive(Debug, Serialize, Deserialize)]
        struct $id;

        impl crate::path::DebugPath for $id {
            fn fmt_path(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", stringify!($id))
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