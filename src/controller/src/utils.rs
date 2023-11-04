// Encode bytes as hex string without 0x prefix.
pub fn hex_encode(data: &[u8]) -> String {
    const HEX_CHARS: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut result = String::with_capacity(data.len() * 2);
    for &byte in data {
        result.push(HEX_CHARS[(byte >> 4) as usize]);
        result.push(HEX_CHARS[(byte & 0x0F) as usize]);
    }
    result
}