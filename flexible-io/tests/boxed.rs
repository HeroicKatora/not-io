#[path = "motivation.rs"]
mod motivation;

use flexible_io::{reader::ReaderBox, Reader};

use motivation::{read_with_skip, IoReport};

#[test]
fn boxed_case() {
    let data: &'static [u8] = b"Hello, world!";
    let read = Reader::new(std::io::Cursor::new(data));

    check_boxed(
        read.into_boxed(),
        IoReport {
            num_seek: 0,
            num_read: 3,
        },
    );

    let mut read = Reader::new(std::io::Cursor::new(data));
    read.set_seek();

    check_boxed(
        read.into_boxed(),
        IoReport {
            num_seek: 1,
            num_read: 2,
        },
    );
}

fn check_boxed(mut boxed: ReaderBox<'_>, expect: IoReport) {
    let mut buffer = vec![];
    let report = read_with_skip(boxed.as_mut(), 7, &mut buffer).unwrap();
    assert_eq!(buffer, b"world!");
    assert_eq!(report.num_seek, expect.num_seek);
    assert!(report.num_read >= expect.num_read);
}
