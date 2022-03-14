use not_io::{AllowStd, Read, Write};

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
    let _:() = XXX;
}
