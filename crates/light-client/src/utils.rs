// pub fn bytes_to_bytes32(bytes: &[u8]) -> Bytes32 {
//     FixedVector::from(bytes.to_vec())
// }

// pub fn bytes32_to_node(bytes: &Bytes32) -> Result<B256> {
//     Ok(B256::from_slice(bytes.iter().as_slice()))
// }

pub fn u64_to_hex_string(val: u64) -> String {
    format!("0x{val:x}")
}
