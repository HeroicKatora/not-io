//! Flexible IO allows you to choose seekable or buffered IO at runtime.
//!
//! The motivation of this is enabling APIs in which use of a reader (or writer) can be optimized
//! in case of it being buffered, but it's not required for correct functioning. Or, an API where
//! the Seek requirement is only determined at runtime, such as when a portion of the functionality
//! does not depend on it, and an error should be returned.
#![feature(set_ptr_value)]

mod reader;
mod writer;

pub use reader::Reader;
pub use writer::Writer;
