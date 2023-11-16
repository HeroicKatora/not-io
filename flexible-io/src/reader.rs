use std::io::{BufRead, Read, Seek};

pub struct Reader<R: Read> {
    inner: R,
    seek: Option<*mut dyn Seek>,
    buf: Option<*mut dyn BufRead>,
}

impl<R: Read> Reader<R> {
    pub fn set_buf<'lt>(&mut self)
    where
        R: BufRead + 'lt,
    {
        let vtable = (&mut self.inner) as &mut (dyn BufRead + 'lt) as *mut (dyn BufRead + 'lt);
        // Safety: Transmuting pointer-to-pointer, and they only differ by lifetime. Types must not
        // be specialized on lifetime parameters.
        self.buf = Some(unsafe { core::mem::transmute::<_, *mut (dyn BufRead + 'static)>(vtable) });
    }

    pub fn set_seek<'lt>(&mut self)
    where
        R: Seek + 'lt,
    {
        let vtable = (&mut self.inner) as &mut (dyn Seek + 'lt) as *mut (dyn Seek + 'lt);
        // Safety: Transmuting pointer-to-pointer, and they only differ by lifetime. Types must not
        // be specialized on lifetime parameters.
        self.seek = Some(unsafe { core::mem::transmute::<_, *mut (dyn Seek + 'static)>(vtable) });
    }

    pub fn buf(&self) -> Option<&(dyn BufRead + '_)> {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.buf?);
        Some(unsafe { &* local })
    }

    pub fn buf_mut(&mut self) -> Option<&mut (dyn BufRead + '_)> {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.buf?);
        Some(unsafe { &mut* local })
    }

    pub fn seek(&self) -> Option<&(dyn Seek + '_)> {
        let ptr = &self.inner as *const R;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &* local })
    }

    pub fn seek_mut(&mut self) -> Option<&mut (dyn Seek + '_)> {
        let ptr = &mut self.inner as *mut R;
        let local = ptr.with_metadata_of(self.seek?);
        Some(unsafe { &mut* local })
    }
}
