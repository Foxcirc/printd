
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
