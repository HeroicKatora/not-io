use std::io::{Write, Seek};

pub struct Writer<W: Write> {
    inner: W,
    seek: Option<*mut dyn Seek>,
}

impl<W: Write> Writer<W> {
    pub fn set_seek<'lt>(&mut self)
    where
        W: Seek + 'lt,
    {
        let vtable = (&mut self.inner) as &mut (dyn Seek + 'lt) as *mut (dyn Seek + 'lt);
        // Safety: Transmuting pointer-to-pointer, and they only differ by lifetime. Types must not
        // be specialized on lifetime parameters.
        self.seek = Some(unsafe { core::mem::transmute::<_, *mut (dyn Seek + 'static)>(vtable) });
    }

    pub fn seek(&self) -> Option<&(dyn Seek + '_)> {
        let ptr = &self.inner as *const W;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &* local })
    }

    pub fn seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = &mut self.inner as *mut W;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &mut* local })
    }
}
