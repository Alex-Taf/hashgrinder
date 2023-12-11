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
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use tauri::{AppHandle, Manager};

lazy_static! {
    static ref WINDOWS_NEWLINE_PATTERN: String = format!("\r\n");
    static ref UNIX_NEWLINE_PATTERN: String = format!("\n");
    static ref SAVE_HASHES_PATH: String = format!("./saved");
    static ref LOCAL_HASH_PATH: String = format!("./saved/hashes.save");
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

fn crack(app: tauri::AppHandle, line: &str, hash_dict: Vec<&str>, now: std::time::Instant) {
    // loading the hash file, adding the new hash and saving locally.
    // let mut local_hive = load_local_hive();

    let (sender, receiver): (Sender<LocalHash>, Receiver<LocalHash>) = mpsc::channel();

    thread::scope(move |s| {
        let mut local_hive = load_local_hive();
        let mut json: String = String::from("");

        s.spawn(move || {
            hash_dict.iter().for_each(move |hash_line| {
                let hash_length = hash_line.len();
                let formatted_hash: String = lib_hashes::define_hash(line, hash_length);
    
                if formatted_hash == *hash_line {
                    unsafe {
                        app.emit_all(
                            "hashlist-cracked",
                            Payload {
                                message: format!(
                                    "Haslist Cracked and saved to './saved/hashes.saved' in {} ms",
                                    now.elapsed().as_millis() - LOAD_TIME
                                ),
                            },
                        )
                        .unwrap();
                    }
    
                    let new_hash = LocalHash {
                        hash: formatted_hash,
                        plaintext: line.to_string(),
                    };
                    
                    println!("new_hash: {}", new_hash.plaintext);

                    sender.send(new_hash).unwrap();
                }
            });
        }).join().unwrap();

        for received in receiver {
            local_hive.push(received);
            json = serde_json::to_string(&local_hive).unwrap();

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
    });
}

pub fn decode(app: AppHandle, wordlist_file: &str, hashlist_file: &str) -> std::io::Result<()> {
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

    // let local_hive = load_local_hive();

    // check if the current hash matches to the hash in the local_hive.
    // for hash_object in local_hive {
    //     if hash_object.hash == hash_input {
    //         println!(
    //             "🤍 Saved hash found! {} -> \"{}\"",
    //             hash_object.hash, hash_object.plaintext
    //         );
    //     }
    // }

    // let hash_len = hash_input.len();
    let now = Instant::now();

    let wordlist_file = fs::read_to_string(wordlist_file).unwrap();
    let hashlist_file = fs::read_to_string(hashlist_file).unwrap();
    let mut newline_hash_split;
    let mut newline_word_split;

    // check new line splitter on OS types
    if (env::consts::OS == "windows") {
        newline_hash_split = hashlist_file.split(&*WINDOWS_NEWLINE_PATTERN);
        newline_word_split = wordlist_file.split(&*WINDOWS_NEWLINE_PATTERN);
    } else {
        newline_hash_split = hashlist_file.split(&*UNIX_NEWLINE_PATTERN);
        newline_word_split = wordlist_file.split(&*UNIX_NEWLINE_PATTERN);
    }

    let word_dict: Vec<&str> = newline_word_split.collect();
    let hash_dict: Vec<&str> = newline_hash_split.collect();

    app.emit_all(
        "wordlist-loaded",
        Payload {
            message: format!("loaded the wordlist file in 0 ms."),
        },
    )
    .unwrap();

    unsafe {
        LOAD_TIME = now.elapsed().as_millis();
        app.emit_all(
            "wordlist-loaded",
            Payload {
                message: format!("loaded the wordlist file in {} ms.", LOAD_TIME),
            },
        )
        .unwrap();
    }


    word_dict.par_iter().for_each(|lines| {
        crack(app.clone(), lines, hash_dict.clone(), now);
    });

    Ok(())
}
