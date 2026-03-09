use broken_app::leak_buffer;

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);
}

#[test]
fn counts_zero_bytes() {
    let data = [0_u8, 0, 0, 0, 0];
    assert_eq!(leak_buffer(&data), 0);
}

#[test]
fn counts_empty_buffer() {
    let data = [];
    assert_eq!(leak_buffer(&data), 0);
}

#[test]
fn counts_one_byte() {
    let data = [1_u8];
    assert_eq!(leak_buffer(&data), 1);
}
