#[macro_export]
macro_rules! use_file {
    ($name:ident: $path:expr) => {
        const $name: &[u8] = include_bytes!($path);
    };
}

#[macro_export]
macro_rules! use_files {
    ($($name:ident: $path:expr),*) => {
        $(
            const $name: &[u8] = include_bytes!($path);
        )*
    };
}
