use john_the_ripper::md5;

fn main() {
    let digest = md5::compute("");
    println!("{}", digest);
}
