use std::{fs::File, io::Write};
use rand::Rng;
fn main() {
    let mut str = String::new();
    println!("How many files?");
    std::io::stdin().read_line(&mut str).expect("Cannot read line!");
    let f_num: u32 = str.trim().parse().expect("Not a number!");
    str = String::new();
    println!("How many track in each file?");
    std::io::stdin().read_line(&mut str).expect("Cannot read line!");
    let t_num: u32 = str.trim().parse().expect("Not a number!");
    for i in 0..f_num {
        let mut file = File::create(format!("disk.in{}", i)).unwrap();
        let mut rng = rand::thread_rng();
        for _ in 0..t_num {
            writeln!(file, "{}", rng.gen_range(0..1000)).unwrap();
        }
    }
}
