pub fn message_to_bits(message: &str) -> Vec<u8> {
    message.bytes().flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1)).collect()
}

pub fn bits_to_message(bits: &[u8]) -> Option<String> {
    bits.chunks(8)
        .map(|byte_bits| {
            let mut byte = 0;
            for (i, &bit) in byte_bits.iter().enumerate() {
                byte |= bit << (7 - i);
            }
            Some(byte)
        })
        .collect::<Option<Vec<u8>>>()
        .and_then(|bytes| String::from_utf8(bytes).ok())
}
