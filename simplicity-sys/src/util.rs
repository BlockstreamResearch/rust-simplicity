pub(crate) fn into_u8_merkle_root(u32_merkle_root: &[u32; 8]) -> [u8; 32] {
    let mut u8_merkle_root = [0; 32];

    for i in 0..8 {
        u8_merkle_root[i * 4..i * 4 + 4].copy_from_slice(&u32_merkle_root[i].to_be_bytes());
    }

    u8_merkle_root
}
