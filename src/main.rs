use john_the_ripper::{incremental, md5};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::SystemTime,
};

fn main() {
    let digest = "08054846bbc9933fd0395f8be516a9f9";
    println!("Generating passwords...");
    let passwords: Vec<String> = incremental::brute_force();
    let passwords = Arc::new(passwords);
    let passwords_length = passwords.len();
    let chunk = passwords_length / 16;
    let mut v = Vec::<std::thread::JoinHandle<()>>::new();
    let found = Arc::new(AtomicBool::new(false));
    let now = SystemTime::now();
    println!("Cracking {} passwords...", passwords_length);
    for t in 0..16 {
        let pw = passwords.clone();
        let found = found.clone();
        let jh = thread::spawn(move || {
            for i in t * chunk..t * chunk + chunk {
                if found.load(Ordering::Relaxed) {
                    return;
                }
                if md5::compute(&pw[i]).to_string() == digest {
                    println!("Found the password! it's: {}", &pw[i]);
                    found.store(true, Ordering::Relaxed);
                    return;
                }
            }
        });
        v.push(jh);
    }
    for jh in v.into_iter() {
        jh.join().unwrap();
    }
    if let Ok(elapsed) = now.elapsed() {
        println!("Took {} seconds to crack!", elapsed.as_secs());
    };
}
