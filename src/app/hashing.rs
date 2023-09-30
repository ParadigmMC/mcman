use sha2::Sha256;
use digest::Digest;

use crate::App;

impl App {
    pub fn hash_sha256(contents: &str) -> String {
        let mut hasher = Sha256::new();
    
        hasher.update(contents);
    
        base16ct::lower::encode_string(&hasher.finalize())
    }
}
