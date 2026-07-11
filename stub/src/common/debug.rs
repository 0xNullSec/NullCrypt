#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug_plain {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("  {}", format!($($arg)*));
        }
    };
}
