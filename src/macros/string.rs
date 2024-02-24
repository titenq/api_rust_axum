#[macro_export]
macro_rules! string {
    ($value:expr) => {
        String::from($value)
    };
}
