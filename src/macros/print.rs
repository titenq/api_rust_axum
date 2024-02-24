#[macro_export]
macro_rules! print {
    ($value:expr) => {
        println!("{}", $value);
    };
}
