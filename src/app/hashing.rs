use std::collections::HashMap;

use sha2::Sha256;
use digest::Digest;

use super::App;

impl App {
    pub fn get_best_hash(hashes: &HashMap<String, String>) -> Option<(String, String)> {
        hashes
            .get_key_value("sha512")
            .or(hashes.get_key_value("sha256"))
            .or(hashes.get_key_value("md5"))
            .or(hashes.get_key_value("sha1"))
            .map(|(k, v)| (k.clone(), v.clone()))
    }

    pub fn hash_sha256(contents: &str) -> String {
        let mut hasher = Sha256::new();
    
        hasher.update(contents);
    
        base16ct::lower::encode_string(&hasher.finalize())
    }
}
