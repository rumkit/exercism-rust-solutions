#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter()
        .flat_map(|&value| {
            let mut value = value;
            let mut buf = [0x80, 0x80, 0x80, 0x80, 0x0];
            let mut i = buf.len() - 1;
            loop {
                buf[i] |= (value & 0x7F) as u8;
                value >>= 7;
                if value == 0 {
                    break;
                }
                i -= 1
            };
            buf.iter().skip(i).copied().collect::<Vec<_>>()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut results = vec![];
    let mut buf = vec![];
    for &b in bytes {
        buf.push(b);
        if b & 0x80 == 0 {
            let mut result = 0u32;
            buf.iter()
                .rev()
                .enumerate()
                .for_each(|(i, b)| result |= ((b & 0x7F) as u32) << (i * 7));

            results.push(result);
            buf.clear();
        }
    }

    match buf.len() {
        0 => Ok(results),
        _ => Err(Error::IncompleteNumber)
    }
}