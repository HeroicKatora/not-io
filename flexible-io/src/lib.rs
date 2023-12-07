//! Flexible IO allows you to choose seekable or buffered IO at runtime.
//!
//! The motivation of this is enabling APIs in which use of a reader (or writer) can be optimized
//! in case of it being buffered, but it's not required for correct functioning. Or, an API where
//! the Seek requirement is only determined at runtime, such as when a portion of the functionality
//! does not depend on it, and an error should be returned.
//!
//! Note that the wrapped type can not be unsized (`dyn` trait) itself. This may be fixed at a
//! later point to make the reader suitable for use in embedded. In particular, the double
//! indirection of instantiating with `R = &mut dyn Read` wouldn't make sense as the setters would
//! not be usable, their bounds can never be met. And combining traits into a large dyn-trait is
//! redundant as it trait-impls become part of the static validity requirement again.
#![cfg_attr(feature = "unstable_set_ptr_value", feature(set_ptr_value))]
#[deny(missing_docs)]

macro_rules! lifetime_erase_trait_vtable {
    ((&mut $r:expr): $lt:lifetime as $trait:path) => {{
        // Safety: Transmuting pointer-to-pointer, and they only differ by lifetime. Types must not
        // be specialized on lifetime parameters.
        let vtable = (&mut $r) as &mut (dyn $trait + $lt) as *mut (dyn $trait + $lt);
        unsafe { core::mem::transmute::<_, *mut (dyn $trait + 'static)>(vtable) }
    }};
}

/// Provides wrappers for values of [`Read`](std::io::Read) types.
pub mod reader;
/// Provides wrappers for values of [`Write`](std::io::Write) types.
pub mod writer;

mod stable_with_metadata_of;

pub use reader::Reader;
pub use writer::Writer;
