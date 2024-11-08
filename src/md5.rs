// Challenge
// https://codingchallenges.substack.com/p/coding-challenge-49-password-cracker

// MD5
// https://en.wikipedia.org/wiki/MD5#Algorithm
// https://docs.rs/md5/latest/md5/

// John source code
// https://github.com/openwall/john

#[derive(Default)]
pub struct Digest(pub [u8; 16]);

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
    let mut k: [i64; 64] = [0; 64];
    for i in 0..64 {
        k[i] = (f64::floor(2.0f64.powf(32.0) * f64::sin((i as f64) + 1.0).abs())).round() as i64;
    }

    // init constants A, B, C, D
    const A: i64 = 0x67452301; // A
    const B: i64 = 0xefcdab89; // B
    const C: i64 = 0x98badcfe; // C
    const D: i64 = 0x10325476; // D

    // Pre-processing: adding a single 1 bit
    // to message
    // pad until it is a multiple of the desired length
    let desired_multiple = 512 / 8;
    dbg!(desired_multiple);
    dbg!(message.len());
    let mut message_bytes = message.as_bytes().to_owned();
    let padding_with_leading_bit = 128;
    message_bytes.push(padding_with_leading_bit);

    // pad with zeros
    let message_length_placeholder = 8; // 64
                                        // think this works
    while (message_bytes.len() + message_length_placeholder) % desired_multiple != 0 {
        message_bytes.push(0);
    }

    let mut original_length_in_bytes = (message.len() % 2_usize.pow(8)).to_le_bytes().to_vec();
    dbg!(&original_length_in_bytes);
    message_bytes.append(&mut original_length_in_bytes);
    dbg!(&original_length_in_bytes);

    dbg!(message_bytes.len());
    dbg!(message_bytes.len() * 8);
    return Digest::default();
}
