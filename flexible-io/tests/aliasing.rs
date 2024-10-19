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

#[test]
fn writer_any() {
    let data: Vec<u8> = b"Hello, world!".to_vec();
    let mut write = Writer::new(std::io::Cursor::new(data));

    {
        let mut inner = write.as_mut();
        assert!(inner.as_any().is_none());
        assert!(inner.as_any_mut().is_none());
    }

    write.set_any();

    {
        let mut inner = write.as_mut();
        assert!(inner.as_any().is_some());
        assert!(inner.as_any_mut().is_some());

        type Inner = std::io::Cursor<Vec<u8>>;
        assert!(inner
            .as_any()
            .and_then(|v| v.downcast_ref::<Inner>())
            .is_some());
    }
}

#[test]
fn reader_any() {
    let data: Vec<u8> = b"Hello, world!".to_vec();
    let mut write = Reader::new(std::io::Cursor::new(data));

    {
        let mut inner = write.as_mut();
        assert!(inner.as_any().is_none());
        assert!(inner.as_any_mut().is_none());
    }

    write.set_any();

    {
        let mut inner = write.as_mut();
        assert!(inner.as_any().is_some());
        assert!(inner.as_any_mut().is_some());

        type Inner = std::io::Cursor<Vec<u8>>;
        assert!(inner
            .as_any()
            .and_then(|v| v.downcast_ref::<Inner>())
            .is_some());
    }
}
