//! Wrapper around `pointer::with_metadata_of` such that it can be used on stable. We replace the
//! inherent method with a custom (and rather scary) implementation.

pub trait WithMetadataOf<U: ?Sized> {
    type Output;
    fn with_metadata_of_on_stable(self, ptr: *const U) -> Self::Output;
}

impl<T: ?Sized, U: ?Sized> WithMetadataOf<U> for *const T {
    type Output = *const U;

    #[inline(always)]
    fn with_metadata_of_on_stable(self, ptr: *const U) -> Self::Output {
        #[cfg(not(feature = "unstable_set_ptr_value"))]
        {
            inject_in_metadata_of(self, ptr)
        }

        // Use the inherent method as soon as provided! On stable, recursive.
        #[cfg(feature = "unstable_set_ptr_value")]
        {
            self.with_metadata_of(ptr)
        }
    }
}

impl<T: ?Sized, U: ?Sized> WithMetadataOf<U> for *mut T {
    type Output = *mut U;

    #[inline(always)]
    fn with_metadata_of_on_stable(self, ptr: *const U) -> Self::Output {
        #[cfg(not(feature = "unstable_set_ptr_value"))]
        {
            inject_in_metadata_of_mut(self, ptr)
        }

        // Use the inherent method as soon as provided! On stable, recursive.
        #[cfg(feature = "unstable_set_ptr_value")]
        {
            self.with_metadata_of(ptr)
        }
    }
}

#[cfg(not(feature = "unstable_set_ptr_value"))]
fn inject_in_metadata_of<T: ?Sized, U: ?Sized>(addr: *const T, mut ptr: *const U) -> *const U {
    let repr = (&mut ptr) as *mut *const U as *mut *const u8;
    let addr = addr as *const u8;
    // Safety: fat pointers are assumed to contain at least one concrete sized pointer at the start
    // of their layout. Pointers are all layout and representation compatible.
    unsafe { *repr = addr };
    ptr as *const U
}

#[cfg(not(feature = "unstable_set_ptr_value"))]
fn inject_in_metadata_of_mut<T: ?Sized, U: ?Sized>(addr: *mut T, mut ptr: *const U) -> *mut U {
    let repr = (&mut ptr) as *mut *const U as *mut *mut u8;
    let addr = addr as *mut u8;
    // Safety: fat pointers are assumed to contain at least one concrete sized pointer at the start
    // of their layout. Pointers are all layout and representation compatible.
    unsafe { *repr = addr };
    ptr as *mut U
}
