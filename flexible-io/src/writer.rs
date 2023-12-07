use std::io::{Seek, Write};
use crate::stable_with_metadata_of::WithMetadataOf;

/// A writer, which can dynamically provide IO traits.
///
/// The following traits may be optionally dynamically provided:
///
/// * [`Seek`]
///
/// The struct comes with a number of setter methods. The call to these requires proof to the
/// compiler that the bound is met, inserting the vtable from the impl instance. Afterward, the
/// bound is not required by any user. Using the (mutable) getters recombines the vtable with the
/// underlying value.
///
/// ## Usage
///
/// ```
/// # use flexible_io::Writer;
/// use std::io::SeekFrom;
///
/// let mut buffer: Vec<u8> = vec![];
/// let cursor = std::io::Cursor::new(&mut buffer);
/// let mut writer = Writer::new(cursor);
/// assert!(writer.as_seek().is_none());
///
/// writer
///     .as_write_mut()
///     .write_all(b"Hello, brain!")
///     .unwrap();
///
/// // But cursors are seekable, let's tell everyone.
/// writer.set_seek();
/// assert!(writer.as_seek().is_some());
///
/// // Now use the Seek implementation to undo our mistake
/// let seek = writer.as_seek_mut().unwrap();
/// seek.seek(SeekFrom::Start(7));
///
/// writer
///     .as_write_mut()
///     .write_all(b"world!")
///     .unwrap();
///
/// let contents: &Vec<u8> = writer.get_ref().get_ref();
/// assert_eq!(contents, b"Hello, world!");
/// ```
pub struct Writer<W> {
    inner: W,
    write: *mut dyn Write,
    seek: Option<*mut dyn Seek>,
}

/// A mutable reference to a [`Writer`].
///
/// This type acts similar to a *very* fat mutable reference. It can be obtained by constructing a
/// concrete reader type and calling [`Writer::as_mut`].
///
/// Note: Any mutable reference to a `Reader` implements `Into<ReaderMut>` for its lifetime. Use
/// this instead of coercion which would be available if this was a builtin kind of reference.
pub struct WriterMut<'lt> {
    inner: &'lt mut dyn Write,
    seek: Option<*mut dyn Seek>,
}

impl<W: Write> Writer<W> {
    /// Wrap an underlying writer by-value.
    pub fn new(mut writer: W) -> Self {
        let write = lifetime_erase_trait_vtable!((&mut writer): '_ as Write);

        Writer {
            inner: writer,
            write,
            seek: None,
        }
    }
}

impl<W> Writer<W> {
    /// Provide access to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Provide mutable access to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Get a view equivalent to very-fat mutable reference.
    ///
    /// This erases the concrete type `W` which allows consumers that intend to avoid polymorphic
    /// code that monomorphizes. The mutable reference has all accessors of a mutable reference
    /// except it doesn't offer access with the underlying reader's type itself.
    pub fn as_mut(&mut self) -> WriterMut<'_> {
        // Copy out all the vtable portions, we need a mutable reference to `self` for the
        // conversion into a dynamically typed `&mut dyn Read`.
        let Writer {
            inner: _,
            write: _,
            seek,
        } = *self;

        WriterMut {
            inner: self.as_write_mut(),
            seek,
        }
    }

    /// Set the V-Table of [`Seek`].
    ///
    /// After this call, the methods [`Self::as_seek`] and [`Self::as_seek_mut`] will return values.
    pub fn set_seek(&mut self)
    where
        W: Seek,
    {
        self.seek = Some(lifetime_erase_trait_vtable!((&mut self.inner): '_ as Seek));
    }
}

impl<W> Writer<W> {
    /// Get the inner value as a dynamic `Write` reference.
    pub fn as_write(&self) -> &(dyn Write + '_) {
        let ptr = &self.inner as *const W;
        let local = WithMetadataOf::with_metadata_of_on_stable(ptr, self.write);
        unsafe { &*local }
    }

    /// Get the inner value as a mutable dynamic `Write` reference.
    pub fn as_write_mut(&mut self) -> &mut (dyn Write + '_) {
        let ptr = &mut self.inner as *mut W;
        let local = WithMetadataOf::with_metadata_of_on_stable(ptr, self.write);
        unsafe { &mut *local }
    }

    /// Get the inner value as a dynamic `Seek` reference.
    ///
    /// This returns `None` unless a previous call to [`Self::set_seek`] as executed, by any other caller.
    /// The value can be moved after such call arbitrarily.
    pub fn as_seek(&self) -> Option<&(dyn Seek + '_)> {
        let ptr = &self.inner as *const W;
        let local = WithMetadataOf::with_metadata_of_on_stable(ptr, self.seek?);
        Some(unsafe { &*local })
    }

    /// Get the inner value as a mutable dynamic `Seek` reference.
    ///
    /// This returns `None` unless a previous call to [`Self::set_seek`] as executed, by any other caller.
    /// The value can be moved after such call arbitrarily.
    pub fn as_seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = &mut self.inner as *mut W;
        let local = WithMetadataOf::with_metadata_of_on_stable(ptr, self.seek?);
        Some(unsafe { &mut *local })
    }
}

impl WriterMut<'_> {
    pub fn as_write_mut(&mut self) -> &mut (dyn Write + '_) {
        &mut *self.inner
    }

    pub fn as_seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = self.inner as *mut dyn Write;
        let local = WithMetadataOf::with_metadata_of_on_stable(ptr, self.seek?);
        Some(unsafe { &mut *local })
    }
}

impl<'lt, R> From<&'lt mut Writer<R>> for WriterMut<'lt> {
    fn from(value: &'lt mut Writer<R>) -> Self {
        value.as_mut()
    }
}
