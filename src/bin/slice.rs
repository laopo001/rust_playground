fn main() {
    let v: Vec<u16> = vec![1111, 1, 9999];
    unsafe {
        let s = std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<u16>(),
        )
        .to_vec();

        dbg!(&s);
        let s2 = &std::slice::from_raw_parts(
            s.as_ptr() as *const u16,
            s.len() / std::mem::size_of::<u16>(),
        );
        dbg!(&s2);
    }
}
