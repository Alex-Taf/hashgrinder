use instant::Instant;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::process;

use super::hashes;

lazy_static! {
    static ref SAVE_HASHES_PATH: String = format!("./saved");
    static ref LOCAL_HASH_PATH: String = format!("./saved/hashes.saved");
}

// debug
static mut LOAD_TIME: u128 = 0;

#[derive(Serialize, Deserialize)]
struct LocalHash {
    hash: String,
    plaintext: String,
} 

fn load_local_hive() -> Vec<LocalHash> {
    let json_file_path = Path::new(&*LOCAL_HASH_PATH);

    let data = fs::read_to_string(json_file_path).unwrap();
    let mut local_hive: Vec<LocalHash> = Vec::new();
    if fs::metadata(json_file_path).unwrap().len() != 0 {
        local_hive = serde_json::from_str(&data).unwrap();
    }
    local_hive
}

fn crack(line: &str, hash_len: usize, hash_input: &str, now: std::time::Instant) {
    let formatted_hash: String = hashes::define_hash(line, hash_len);

    if formatted_hash == hash_input {
        unsafe {
            println!(
                "🤍 Cracked! {} -> \"{}\" in {} ms",
                formatted_hash,
                line,
                now.elapsed().as_millis() - LOAD_TIME
            );
        }
        
        // loading the hash file, adding the new hash and saving locally.
        let mut local_hive = load_local_hive();

        let new_hash = LocalHash {
            hash: formatted_hash,
            plaintext: line.to_string(),
        };

        local_hive.push(new_hash);
        let json = serde_json::to_string(&local_hive).unwrap();

        let mut file_write = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(&*LOCAL_HASH_PATH)
            .unwrap();

        // let mut file_write = fs::File::create(&*LOCAL_HASH_PATH);
        // println!("{:x}", file_write.bytes());
        //println!("{:x}", json);

        if let Err(e) = writeln!(file_write, "{}", &json) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // process::exit(0);
    }
}

pub fn decode(wordlist_file: &str, hash_input: &str) {
    // check for saved hashes locally
    let _ = fs::create_dir_all(&*SAVE_HASHES_PATH);

    let f = fs::File::open(&*LOCAL_HASH_PATH);
    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match fs::File::create(&*LOCAL_HASH_PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Unexpected error!"),
        },
    };

    let local_hive = load_local_hive();

    // check if the current hash matches to the hash in the local_hive.
    for hash_object in local_hive {
        if hash_object.hash == hash_input {
            println!(
                "🤍 Saved hash found! {} -> \"{}\"",
                hash_object.hash, hash_object.plaintext
            );
        }
    }

    // sanity check
    let valid_lens = vec![32, 40, 64, 128];
    let hash_len = hash_input.len();
    if !valid_lens.contains(&hash_len) {
        println!("❌ Invalid hash length!");
        //process::exit(1);
    }

    let now = Instant::now();

    let file = fs::read_to_string(wordlist_file).unwrap();
    let newline_split = file.split("\n");
    let dict: Vec<&str> = newline_split.collect();

    // debug
    unsafe {
        LOAD_TIME = now.elapsed().as_millis();
        println!("loaded the wordlist file in {} millisecs.", LOAD_TIME);
    }

    // rayon goes brr
    dict.par_iter().for_each(|lines| {
        crack(lines, hash_len, hash_input, now);
    });

    //Ok(())
}
