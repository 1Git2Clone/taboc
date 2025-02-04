#[macro_export]
macro_rules! er {
    ($($arg:tt)*) => {
        $crate::prelude::anyhow!(format!($($arg)*))
    };
}
