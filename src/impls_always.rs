use super::{BufRead, Cursor, Empty, Read, Repeat, Result, Seek, SeekFrom, Sink, Write};
use crate::ErrorKind;

impl<T> Read for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read = Read::read(&mut self.fill_buf()?, buf)?;
        self.consume(read);
        Ok(read)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Read::read_exact(&mut self.fill_buf()?, buf)?;
        self.consume(buf.len());
        Ok(())
    }
}

impl<T> BufRead for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn fill_buf(&mut self) -> Result<&[u8]> {
        let buffer = self.inner.as_ref();
        let pos = self.pos.min(buffer.len() as u64) as usize;
        Ok(&buffer[pos..])
    }
    fn consume(&mut self, amt: usize) {
        self.pos += amt as u64;
    }
}

impl Write for Cursor<&mut [u8]> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let pos = self.pos.min(self.inner.len() as u64) as usize;
        let ref mut slice = &mut self.inner[pos..];
        let n = Write::write(slice, buf)?;
        self.pos += n as u64;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<T> Seek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let (base_pos, offset) = match pos {
            SeekFrom::Start(n) => {
                self.pos = n;
                return Ok(n);
            }
            SeekFrom::End(n) => (self.inner.as_ref().len() as u64, n),
            SeekFrom::Current(n) => (self.pos, n),
        };

        let new_pos = if offset >= 0 {
            base_pos.checked_add(offset as u64)
        } else {
            base_pos.checked_sub(offset.wrapping_neg() as u64)
        };

        self.pos = new_pos.ok_or_else(|| ErrorKind::InvalidInput)?;
        Ok(self.pos)
    }

    fn stream_position(&mut self) -> Result<u64> {
        Ok(self.pos)
    }
}

impl Read for Empty {
    fn read(&mut self, _: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}

impl BufRead for Empty {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        Ok(&[])
    }

    fn consume(&mut self, _: usize) {}
}

impl Seek for Empty {
    fn seek(&mut self, _: SeekFrom) -> Result<u64> {
        Ok(0)
    }
    fn stream_position(&mut self) -> Result<u64> {
        Ok(0)
    }
}

impl Read for Repeat {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        buf.iter_mut().for_each(|b| *b = self.byte);
        Ok(buf.len())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Read::read(self, buf)?;
        Ok(())
    }
}

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Write for &'_ Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
