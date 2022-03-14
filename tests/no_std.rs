use not_io::{AllowStd, Read, Write};

extern crate alloc;
use alloc::{string::String, vec::Vec};

// Make sure that this includes the no-`alloc` subset of tests.
#[path = "no_alloc.rs"]
mod _alloc;

fn is_read<R: Read>() {}
fn is_write<W: Write>() {}

const XXX: () = {
    let _ = is_read::<&'static [u8]>;
    let _ = is_write::<&'static mut [u8]>;
    let _ = is_write::<Vec<u8>>;
    let _ = is_read::<AllowStd<&'static [u8]>>;
    let _ = is_write::<AllowStd<&'static mut [u8]>>;
    let _ = is_write::<AllowStd<Vec<u8>>>;
};

#[test]
fn evaluate_consts() {
    let _: () = XXX;
}

#[test]
fn read_to_buffer() {
    const SOURCE: &str = "Hello, world";
    let ref mut source = SOURCE.as_bytes();
    let elen = source.len();
    let mut buffer = Vec::new();

    assert!(matches!(Read::read_to_end(source, &mut buffer), Ok(rlen) if rlen == elen));
    assert_eq!(buffer, SOURCE.as_bytes());
}

#[test]
fn read_to_string() {
    const SOURCE: &str = "Hello, world";
    let ref mut source = SOURCE.as_bytes();
    let elen = source.len();
    let mut buffer = String::new();

    assert!(matches!(Read::read_to_string(source, &mut buffer), Ok(rlen) if rlen == elen));
    assert_eq!(buffer, SOURCE);
}

#[test]
fn read_to_fail() {
    const SOURCE: &[u8] = b"Hello, \xfeworld";
    let ref mut source = &SOURCE[..];
    let mut buffer = String::new();

    assert!(matches!(Read::read_to_string(source, &mut buffer), Err(_)));
}
