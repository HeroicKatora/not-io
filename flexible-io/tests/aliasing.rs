use flexible_io::{Reader, Writer};

#[test]
fn reader_reuses() {
    let data: &'static [u8] = b"Hello, world!";
    let mut read = Reader::new(std::io::Cursor::new(data));

    {
        let mut inner = read.as_mut();
        // This is also for miri, check some aliasing assumptions.
        let _ = inner.as_seek_mut();
        assert!(inner.as_seek_mut().is_none());
        let _ = inner.as_read_mut().read(&mut []);
    }

    read.set_seek();

    {
        let mut inner = read.as_mut();
        inner.as_seek_mut();
        let _ = inner.as_seek_mut();
        assert!(inner.as_seek_mut().is_some());
        let _ = inner.as_read_mut().read(&mut []);
    }

    read.into_inner();
}

#[test]
fn writer_reuses() {
    let data: &mut [u8] = &mut { *b"Hello, world!" };
    let mut write = Writer::new(std::io::Cursor::new(data));

    {
        let mut inner = write.as_mut();
        // This is also for miri, check some aliasing assumptions.
        let _ = inner.as_seek_mut();
        assert!(inner.as_seek_mut().is_none());
        let _ = inner.as_write_mut().write(&[]);
    }

    write.set_seek();

    {
        let mut inner = write.as_mut();
        inner.as_seek_mut();
        let _ = inner.as_seek_mut();
        assert!(inner.as_seek_mut().is_some());
        let _ = inner.as_write_mut().write(&[]);
    }

    write.into_inner();
}
