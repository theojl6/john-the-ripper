use john_the_ripper::md5;

fn main() {
    let digest = md5::compute("The quick brown fox jumps over the lazy dog");
    println!("{}", digest);
}
