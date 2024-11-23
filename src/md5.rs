// Challenge
// https://codingchallenges.substack.com/p/coding-challenge-49-password-cracker

// MD5
// https://en.wikipedia.org/wiki/MD5#Algorithm
// https://docs.rs/md5/latest/md5/

// John source code
// https://github.com/openwall/john

use std::fmt::Display;

#[derive(Default)]
pub struct Digest(pub [u8; 16]);

impl Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|h| format!("{:02x}", h))
                .collect::<String>()
        )
    }
}

// MD5 processes a variable-length message into a fixed-length output of 128 bits (8 * 16 = 128, 4 * 32 = 128)
pub fn compute(message: &str) -> Digest {
    // message preprocessing:
    // // break message down into chunks of 512-bit blocks (sixteen 32-bit words)
    // // // pad message so that it is divisible by 512
    // // // to pad, append a single bit (1) to the end of the message, then append zeros until it is a multiple of 512
    // // // but minus 64
    // // // this space of 64 will be filled by bits representing the length of the original message, module 2^64
    // main algorithm
    // s specifies the per-round shift amounts
    let s = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    // Use binary integer part of the sines of integers (Radians) as constants:
    let mut k: [u32; 64] = [0; 64];
    for i in 0..64 {
        k[i] = (f64::floor(2.0f64.powf(32.0) * f64::sin((i as f64) + 1.0).abs())).round() as u32;
        // println!("{:#08x}", k[i]);
    }

    // init constants A, B, C, D
    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    // Pre-processing: adding a single 1 bit
    // to message
    // pad until it is a multiple of the desired length
    let chunk_size = 512 / 8;
    let mut message_bytes = message.as_bytes().to_owned();
    // NOTE: what should happen to the trailing 0's from the original message...?
    // technically the below implementation is wrong because it does not remove those trailing 0's before appending the 1 bit
    let padding_with_leading_bit = 128;
    message_bytes.push(padding_with_leading_bit);

    // pad with zeros
    let message_length_placeholder = 8; // 64 bits
    while (message_bytes.len() + message_length_placeholder) % chunk_size != 0 {
        message_bytes.push(0);
    }

    let mut original_length_in_bytes = (message.len() % 2_usize.pow(8)).to_le_bytes().to_vec();
    message_bytes.append(&mut original_length_in_bytes);

    // for each 512-bit chunk of padded message do
    // modify the 4 32 bit words
    for chunk in message_bytes.chunks(chunk_size) {
        // there are 512 bits (64 bytes) in this chunk
        // this chunk needs to be divided into 16 32-bit (4 bytes) words

        let m: Vec<&[u8]> = chunk.chunks(4).collect();
        // each m has 4 8-bit words, so 32 bits
        // main loop
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;
        for i in 0..64 {
            let mut f: u32 = 0;
            let mut g: u32 = 0;
            if i <= 15 {
                f = (b & c) | ((!b) & d);
                g = i;
            } else if 16 <= i && i <= 31 {
                f = (d & b) | ((!d) & c);
                g = (5 * i + 1) % 16;
            } else if 32 <= i && i <= 47 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else if 48 <= i && i <= 63 {
                f = c ^ (b | (!d));
                g = (7 * i) % 16;
            }
            let bits = u32::from_le_bytes(m[g as usize].try_into().unwrap());

            f = f.wrapping_add(a.wrapping_add(k[i as usize].wrapping_add(bits)));
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s[i as usize]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let digest_vec: Vec<u8> = a0
        .to_le_bytes()
        .iter()
        .chain(&b0.to_le_bytes())
        .chain(&c0.to_le_bytes())
        .chain(&d0.to_le_bytes())
        .cloned()
        .collect();
    let mut digest: [u8; 16] = [0; 16];
    digest.copy_from_slice(&digest_vec[..]);
    return Digest(digest);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fox() {
        let md5_hash = compute("The quick brown fox jumps over the lazy dog").to_string();
        assert_eq!(md5_hash, "9e107d9d372bb6826bd81d3542a419d6");
    }
}
