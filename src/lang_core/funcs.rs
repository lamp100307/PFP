#[macro_export]
macro_rules! string {
    // Если передали &str (или любой expr)
    ($s:expr) => {
        String::from($s)
    };
    // Если передали литерал
    ($s:literal) => {
        String::from($s)
    };
    // Если передали несколько аргументов (как format!)
    ($($arg:tt)*) => {
        format!($($arg)*)
    };
}