pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {
        return Err(format!($($arg)*).into())
    };
}
