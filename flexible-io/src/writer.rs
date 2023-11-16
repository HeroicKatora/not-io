use std::io::{Write, Seek};

/// A writer, which can dynamically provide IO traits.
///
/// The following traits may be optionally dynamically provided:
///
/// * [`Seek`](std::io::Seek).
///
/// The struct comes with a number of setter methods. The call to these requires proof to the
/// compiler that the bound is met, inserting the vtable from the impl instance. Afterward, the
/// bound is not required by any user. Using the (mutable) getters recombines the vtable with the
/// underlying value.
pub struct Writer<W> {
    inner: W,
    write: *mut dyn Write,
    seek: Option<*mut dyn Seek>,
}

impl<W: Write> Writer<W> {
    pub fn new(mut writer: W) -> Self {
        let write = lifetime_erase_trait_vtable!((&mut writer): '_ as Write);

        Writer {
            inner: writer,
            write,
            seek: None,
        }
    }

    /// Provide access to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Provide mutable access to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    pub fn set_seek(&mut self)
    where
        W: Seek,
    {
        self.seek = Some(lifetime_erase_trait_vtable!((&mut self.inner): '_ as Seek));
    }
}

impl<W> Writer<W> {
    pub fn as_write(&self) -> &(dyn Write + '_) {
        let ptr = &self.inner as *const W;
        let local = ptr.with_metadata_of(self.write);
        unsafe { &* local }
    }

    pub fn as_write_mut(&mut self) -> &mut (dyn Write + '_) {
        let ptr = &mut self.inner as *mut W;
        let local = ptr.with_metadata_of(self.write);
        unsafe { &mut* local }
    }

    pub fn as_seek(&self) -> Option<&(dyn Seek + '_)> {
        let ptr = &self.inner as *const W;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &* local })
    }

    pub fn as_seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = &mut self.inner as *mut W;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &mut* local })
    }
}
