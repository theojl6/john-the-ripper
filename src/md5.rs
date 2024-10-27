// Challenge
// https://codingchallenges.substack.com/p/coding-challenge-49-password-cracker

// MD5
// https://en.wikipedia.org/wiki/MD5#Algorithm
// https://docs.rs/md5/latest/md5/

// John source code
// https://github.com/openwall/john

pub struct Digest(pub [u8; 16]);

pub fn compute(data: &[u8]) -> Digest {
    unimplemented!()
}
