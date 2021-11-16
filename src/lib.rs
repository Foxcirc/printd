
/// Print's a debug expression.
/// 
/// ## Examples
/// 
/// ```rust
/// 
/// printd!(1 + 2);
/// // [src/lib.rs:8] 1 + 2 = 3
/// 
/// printd!("Test message.");
/// // [src/lib.rs:12] Test message.
/// 
/// ```
/// 
/// For more inforation see the readme.
#[macro_export]
macro_rules! printd {
    ($message:literal) => {
        println!("[{}:{}] {:?}", file!(), line!(), $message);
    };
    ($message:literal, $($expr:expr),*) => {
        println!("[{}:{}] {}", file!(), line!(), $message);
        $(
            println!("    {} = {:?}", stringify!($expr), $expr);
        )*
    };
    ($($expr:expr),*) => {
        $(
            println!("[{}:{}] {} = {:?}", file!(), line!(), stringify!($expr), $expr);
        )*
    };
}

/// Print's a debug expression to `stderr`.
/// Works just like `printd`.
#[macro_export]
macro_rules! eprintd {
    ($message:literal) => {
        eprintln!("[{}:{}] {:?}", file!(), line!(), $message);
    };
    ($message:literal, $($expr:expr),*) => {
        eprintln!("[{}:{}] {}", file!(), line!(), $message);
        $(
            eprintln!("    {} = {:?}", stringify!($expr), $expr);
        )*
    };
    ($($expr:expr),*) => {
        $(
            eprintln!("[{}:{}] {} = {:?}", file!(), line!(), stringify!($expr), $expr);
        )*
    };
}
