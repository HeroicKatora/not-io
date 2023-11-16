use std::io::{BufRead, Read, Seek};


/// A reader, which can _dynamically_ implement IO traits.
///
/// The struct comes with a number of setter methods. The call to these requires proof to the
/// compiler that the bound is met, inserting the vtable from the impl instance. Afterward, the
/// bound is not required by any user. Using the (mutable) getters recombines the vtable with the
/// underlying value.
///
/// Note that the value can not be unsized (`dyn` trait) itself. This may be fixed at a later point
/// to make the reader suitable for use in embedded. In particular, the double indirection of
/// instantiating with `R = &mut dyn Read` wouldn't make sense as the setters would not be usable,
/// their bounds can never be met. And combining traits into a large dyn-trait is redundant as it
/// trait-impls become part of the static validity requirement again.
pub struct Reader<R> {
    inner: R,
    read: *mut dyn Read,
    seek: Option<*mut dyn Seek>,
    buf: Option<*mut dyn BufRead>,
}

impl<R: Read> Reader<R> {
    pub fn new<'lt>(mut reader: R) -> Self
        where R: 'lt
    {
        let read = lifetime_erase_trait_vtable!((&mut reader): 'lt as Read);

        Reader {
            inner: reader,
            read,
            seek: None,
            buf: None,
        }
    }

    /// Provide access to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Provide mutable access to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Insert the vtable for the `BufRead` trait.
    pub fn set_buf(&mut self)
    where
        R: BufRead,
    {
        self.buf = Some(lifetime_erase_trait_vtable!((&mut self.inner): '_ as BufRead));
    }

    /// Insert the vtable for the `Seek` trait.
    ///
    /// This call itself requires proof to the compiler that the bound is met, inserting the vtable
    /// from the impl instance. The bound is not required by any user of that inserted vtable.
    pub fn set_seek(&mut self)
    where
        R: Seek,
    {
        self.seek = Some(lifetime_erase_trait_vtable!((&mut self.inner): '_ as Seek));
    }
}

impl<R> Reader<R> {
    pub fn as_read(&self) -> &(dyn Read + '_) {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.read);
        unsafe { &* local }
    }

    pub fn as_read_mut(&mut self) -> &mut (dyn Read + '_) {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.read);
        unsafe { &mut* local }
    }

    pub fn as_buf(&self) -> Option<&(dyn BufRead + '_)> {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.buf?);
        Some(unsafe { &* local })
    }

    pub fn as_buf_mut(&mut self) -> Option<&mut (dyn BufRead + '_)> {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.buf?);
        Some(unsafe { &mut* local })
    }

    pub fn as_seek(&self) -> Option<&(dyn Seek + '_)> {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &* local })
    }

    pub fn as_seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &mut* local })
    }
}
