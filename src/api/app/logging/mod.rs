use std::path::PathBuf;

pub enum Message {
    File(FileMessage, PathBuf),
}

pub enum FileMessage {
    Copied,
    Bootstrapped,
    CacheHit,
}
