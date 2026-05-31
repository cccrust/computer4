#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
extern crate alloc;

pub mod io;
pub mod fmt;
#[cfg(unix)]
pub mod opt;

pub use io::Read;
pub use io::Write;
pub use io::File;
pub use io::{stdin, stdout, stderr};
pub use io::{print, println, eprintln, exit, args};

#[macro_export]
#[cfg(target_os = "none")]
macro_rules! println {
    () => { $crate::println("") };
    ($($arg:tt)*) => { $crate::println(&$crate::format!($($arg)*)) };
}

#[macro_export]
#[cfg(target_os = "none")]
macro_rules! print {
    ($($arg:tt)*) => { $crate::print(&$crate::format!($($arg)*)) };
}

#[macro_export]
#[cfg(target_os = "none")]
macro_rules! eprintln {
    () => { $crate::eprintln("") };
    ($($arg:tt)*) => { $crate::eprintln(&$crate::format!($($arg)*)) };
}