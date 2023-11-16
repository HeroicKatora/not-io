//! Flexible IO allows you to choose seekable or buffered IO at runtime.
//!
//! The motivation of this is enabling APIs in which use of a reader (or writer) can be optimized
//! in case of it being buffered, but it's not required for correct functioning. Or, an API where
//! the Seek requirement is only determined at runtime, such as when a portion of the functionality
//! does not depend on it, and an error should be returned.
#![feature(set_ptr_value)]

macro_rules! lifetime_erase_trait_vtable {
    ((&mut $r:expr): $lt:lifetime as $trait:path) => {
        {
            // Safety: Transmuting pointer-to-pointer, and they only differ by lifetime. Types must not
            // be specialized on lifetime parameters.
            let vtable = (&mut $r) as &mut (dyn $trait + $lt) as *mut (dyn $trait + $lt);
            unsafe { core::mem::transmute::<_, *mut (dyn $trait + 'static)>(vtable) }
        }
    }
}

mod reader;
mod writer;

pub use reader::Reader;
pub use writer::Writer;
