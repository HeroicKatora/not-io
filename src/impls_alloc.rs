use super::Result;
use crate::alloc::{string::String, vec::Vec};

impl super::Read for &'_ [u8] {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = self.len().min(buf.len());
        buf[..len].copy_from_slice(&self[..len]);
        *self = &self[len..];
        Ok(len)
    }
}

impl super::Write for &'_ mut [u8] {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = self.len().min(buf.len());
        let (head, tail) = core::mem::take(self).split_at_mut(len);
        *self = tail;
        head.copy_from_slice(buf);
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl super::Write for Vec<u8> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn read_to_end<R: super::Read + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
    struct Guard<'vec> {
        buf: &'vec mut Vec<u8>,
        len: usize,
    }

    impl Drop for Guard<'_> {
        fn drop(&mut self) {
            self.buf.truncate(self.len)
        }
    }

    let mut guard = Guard {
        len: buf.len(),
        buf,
    };
    let start_len = guard.len;

    loop {
        // Ensure room.
        if guard.buf.len() == guard.len {
            guard.buf.reserve(32);
            guard.buf.resize(guard.buf.capacity(), 0);
            // FIXME: once it's sound, use `initializer`.
        }

        let buf = &mut guard.buf[guard.len..];
        match r.read(buf) {
            Ok(0) => return Ok(guard.len - start_len),
            Ok(n) => {
                assert!(n <= buf.len());
                guard.len += n;
            }
            Err(e) if e.is_interrupted() => {}
            Err(e) => return Err(e),
        }
    }
}

pub(crate) fn read_to_string<R: super::Read + ?Sized>(
    r: &mut R,
    buf: &mut String,
) -> Result<usize> {
    struct Utf8Guard<'vec> {
        buf: &'vec mut Vec<u8>,
        len: usize,
    }

    let mut guard = unsafe {
        Utf8Guard {
            len: buf.len(),
            buf: buf.as_mut_vec(),
        }
    };

    let ret = read_to_end(r, guard.buf);
    if core::str::from_utf8(&guard.buf[guard.len..]).is_err() {
        ret.and_then(|_| Err(super::Error::from(super::ErrorKind::InvalidData)))
    } else {
        guard.len = guard.buf.len();
        ret
    }
}
