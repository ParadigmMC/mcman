pub enum ReloadType {
    Reload,
    Restart,
    ReloadPlugin(String),
    RunCommand(String),
}

pub struct HotReloadConfig {
    pub files: Vec<Entry>,
}

pub struct Entry {
    pub path: String,
    pub action: ReloadType,
}
