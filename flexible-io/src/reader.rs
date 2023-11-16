use std::io::{BufRead, Read, Seek};


/// A reader, which can dynamically provide IO traits.
///
/// The following traits may be optionally dynamically provided:
///
/// * [`Seek`]
/// * [`BufRead`]
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
///
/// ## Usage
///
/// ```
/// # use flexible_io::Reader;
/// let mut buffer: &[u8] = b"Hello, world!";
/// let mut reader = Reader::new(&mut buffer);
/// assert!(reader.as_buf().is_none());
///
/// // But slices are buffered readers, let's tell everyone.
/// reader.set_buf();
/// assert!(reader.as_buf().is_some());
///
/// // Now use the ReadBuf implementation directly
/// let buffered = reader.as_buf_mut().unwrap();
/// buffered.consume(7);
/// assert_eq!(buffered.fill_buf().unwrap(), b"world!");
/// ```
pub struct Reader<R> {
    inner: R,
    read: *mut dyn Read,
    seek: Option<*mut dyn Seek>,
    buf: Option<*mut dyn BufRead>,
}

impl<R: Read> Reader<R> {
    /// Wrap an underlying reader by-value.
    pub fn new(mut reader: R) -> Self {
        let read = lifetime_erase_trait_vtable!((&mut reader): '_ as Read);

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

    /// Set the V-Table for [`BufRead`].
    ///
    /// After this call, the methods [`Self::as_buf`] and [`Self::as_buf_mut`] will return values.
    pub fn set_buf(&mut self)
    where
        R: BufRead,
    {
        self.buf = Some(lifetime_erase_trait_vtable!((&mut self.inner): '_ as BufRead));
    }

    /// Set the V-Table for [`Seek`].
    ///
    /// After this call, the methods [`Self::as_seek`] and [`Self::as_seek_mut`] will return values.
    pub fn set_seek(&mut self)
    where
        R: Seek,
    {
        self.seek = Some(lifetime_erase_trait_vtable!((&mut self.inner): '_ as Seek));
    }
}

impl<R> Reader<R> {
    /// Get the inner value as a dynamic `Read` reference.
    pub fn as_read(&self) -> &(dyn Read + '_) {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.read);
        unsafe { &* local }
    }

    /// Get the inner value as a mutable dynamic `Read` reference.
    pub fn as_read_mut(&mut self) -> &mut (dyn Read + '_) {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.read);
        unsafe { &mut* local }
    }

    /// Get the inner value as a dynamic `BufRead` reference.
    ///
    /// This returns `None` unless a previous call to [`Self::set_buf`] as executed, by any other caller.
    /// The value can be moved after such call arbitrarily.
    pub fn as_buf(&self) -> Option<&(dyn BufRead + '_)> {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.buf?);
        Some(unsafe { &* local })
    }

    /// Get the inner value as a mutable dynamic `BufRead` reference.
    ///
    /// This returns `None` unless a previous call to [`Self::set_buf`] as executed, by any other caller.
    /// The value can be moved after such call arbitrarily.
    pub fn as_buf_mut(&mut self) -> Option<&mut (dyn BufRead + '_)> {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.buf?);
        Some(unsafe { &mut* local })
    }

    /// Get the inner value as a dynamic `Seek` reference.
    ///
    /// This returns `None` unless a previous call to [`Self::set_seek`] as executed, by any other caller.
    /// The value can be moved after such call arbitrarily.
    pub fn as_seek(&self) -> Option<&(dyn Seek + '_)> {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &* local })
    }

    /// Get the inner value as a mutable dynamic `Seek` reference.
    ///
    /// This returns `None` unless a previous call to [`Self::set_seek`] as executed, by any other caller.
    /// The value can be moved after such call arbitrarily.
    pub fn as_seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &mut* local })
    }
}
