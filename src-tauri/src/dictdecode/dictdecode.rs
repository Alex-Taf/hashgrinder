use crate::lib_hashes;
use instant::Instant;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use tauri::{AppHandle, Manager};

lazy_static! {
    static ref WINDOWS_NEWLINE_PATTERN: String = format!("\r\n");
    static ref UNIX_NEWLINE_PATTERN: String = format!("\n");
    static ref SAVE_HASHES_PATH: String = format!("./saved");
    static ref LOCAL_HASH_PATH: String = format!("./saved/hashes.saved");
}

// debug
static mut LOAD_TIME: u128 = 0;

// log event payload
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

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

fn crack(
    app: tauri::AppHandle,
    line: &str,
    hash_len: usize,
    hash_input: &str,
    now: std::time::Instant,
) {
    let formatted_hash: String = lib_hashes::define_hash(line, hash_len);

    if formatted_hash == hash_input {
        unsafe {
            app.emit_all(
                "hash-cracked",
                Payload {
                    message: format!(
                        "Cracked! {} -> \"{}\" in {} ms",
                        formatted_hash,
                        line,
                        now.elapsed().as_millis() - LOAD_TIME
                    ),
                },
            )
            .unwrap();
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

        if let Err(e) = writeln!(file_write, "{}", &json) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub fn decode(app: AppHandle, wordlist_file: &str, hash_input: &str) -> std::io::Result<()> {
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

    let hash_len = hash_input.len();
    let now = Instant::now();

    let file = fs::read_to_string(wordlist_file).unwrap();
    let mut newline_split;

    // check new line splitter on OS types
    if (env::consts::OS == "windows") {
        newline_split = file.split(&*WINDOWS_NEWLINE_PATTERN);
    } else {
        newline_split = file.split(&*UNIX_NEWLINE_PATTERN);
    }

    let dict: Vec<&str> = newline_split.collect();

    // debug
    unsafe {
        LOAD_TIME = now.elapsed().as_millis();
        app.emit_all(
            "wordlist-loaded",
            Payload {
                message: format!("loaded the wordlist file in {} millisecs.", LOAD_TIME),
            },
        )
        .unwrap();
    }

    dict.par_iter().for_each(|lines| {
        crack(app.clone(), lines, hash_len, hash_input, now);
    });

    Ok(())
}
