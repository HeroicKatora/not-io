use not_io::{AllowStd, BufRead, Read, Write};

fn is_read<R: Read>() {}
fn is_write<W: Write>() {}
fn is_buf_read<R: BufRead>() {}

const XXX: () = {
    let _ = is_read::<&'static [u8]>;
    let _ = is_write::<&'static mut [u8]>;
    let _ = is_read::<AllowStd<&'static [u8]>>;
    let _ = is_write::<AllowStd<&'static mut [u8]>>;
    let _ = is_buf_read::<&'static [u8]>;
};

#[test]
fn evaluate_consts() {
    let _: () = XXX;
}
