pub mod buffer;
pub mod color;
pub mod macros;
pub mod writer;

use core::fmt;
use writer::WRITER;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
