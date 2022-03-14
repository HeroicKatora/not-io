use super::Result;
use crate::alloc::vec::Vec;

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
