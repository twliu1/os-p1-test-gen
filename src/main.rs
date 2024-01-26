use std::{fs::File, io::Write};
use rand::Rng;
fn main() {
    for i in 0..10 {
        let mut file = File::create(format!("disk.in{}", i)).unwrap();
        let mut rng = rand::thread_rng();
        for i in 0..10 {
            let rn = rng.gen_range(0..=999);
            writeln!(file, "{}", rn).unwrap();
        }
    }

}
