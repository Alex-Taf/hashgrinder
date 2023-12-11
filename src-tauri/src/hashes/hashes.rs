use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

pub fn define_hash(line: &str, hash_length: usize) -> String {
    match hash_length {
        // MD5 Define hash by length
        32 => {
            let mut hash_generator = Md5::new();
            hash_generator.update(line.as_bytes());
            format!("{:x}", hash_generator.finalize())
        }

        // SHA-1 Define hash by length
        40 => {
            let mut hash_generator = Sha1::new();
            hash_generator.update(line.as_bytes());
            format!("{}", hash_generator.digest().to_string())
        }

        64 => {
            let mut hash_generator = Sha256::new();
            hash_generator.update(line.as_bytes());
            format!("{:x}", hash_generator.finalize())
        }

        128 => {
            let mut hash_generator = Sha512::new();
            hash_generator.update(line.as_bytes());
            format!("{:x}", hash_generator.finalize())
        }

        _ => "".to_string(),
    }
}
