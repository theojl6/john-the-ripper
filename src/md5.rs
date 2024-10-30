// Challenge
// https://codingchallenges.substack.com/p/coding-challenge-49-password-cracker

// MD5
// https://en.wikipedia.org/wiki/MD5#Algorithm
// https://docs.rs/md5/latest/md5/

// John source code
// https://github.com/openwall/john

pub struct Digest(pub [u8; 16]);

pub fn compute(message: &[u8]) -> Digest {
    // message preprocessing
    // break message down into chunks of 512-bit blocks (sixteen 32-bit words)
    // pad message so that it is divisible by 512
    // to pad, append a single bit (1) to the end of the message, then append zeros until it is a multiple of 512
    // but minus 64
    // this space of 64 will be filled by bits representing the length of the original message, module 2^64
    // main algorithm
    //
    unimplemented!()
}
