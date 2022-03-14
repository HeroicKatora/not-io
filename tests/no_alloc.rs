use not_io::{AllowStd, Read, Write};

fn is_read<R: Read>() {}
fn is_write<W: Write>() {}

const XXX: () = {
    let _ = is_read::<&'static [u8]>;
    let _ = is_write::<&'static mut [u8]>;
    let _ = is_read::<AllowStd<&'static [u8]>>;
    let _ = is_write::<AllowStd<&'static mut [u8]>>;
};

#[test]
fn evaluate_consts() {
    let _:() = XXX;
}
