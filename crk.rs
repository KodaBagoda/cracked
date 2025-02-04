use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid amount of arguments!");
        println!("Example: cargo run <sha256 hash>");
        exit(1);
    }

    let wanted_hash: &String = &args[1];
    let password_file: &str  = "src/passwordlist.txt";
    let mut_attempts: i32 = 1;

    println!("Attempting to crack the hash: {}!\n", wanted_hash);

    let password_list: File = File::open(path: password_file).unwrap();
    let reader: BufReader<File> = BufReader::new(inner: password_list);


    for line: Result<String, Error> in reader.lines() {
        let line: String = line.unwrap();
        let password: Vac<u8> = line.trim().to_owned().into_bytes();
        let password_hash: String = format!("{:x}", Sha256::digest(&password))

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
        if &password_hash == wanted_hash {
            println!("Password found after {} attempts!", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(code: 0);
        }
        attempts += 1;
    }

    println!("Password not found after {} attempts!", attempts);
}
