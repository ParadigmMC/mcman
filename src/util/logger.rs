#![allow(dead_code)]

#[derive(Debug)]
pub enum Logger {
    Main,

    Task { indent: usize },

    List { indent: usize, len: usize },
}

impl Logger {
    pub fn new() -> Self {
        Self::Main
    }

    pub fn log(&self, text: &str) {
        let space = " ";
        let indent = self.get_indent();

        println!("{space:indent$}{text}",);
    }

    pub fn item(&self, idx: usize, text: &str) {
        match self {
            Self::List { indent, len } => {
                println!(
                    "{:indent$}({:idx_w$}/{len}) {text}",
                    " ",
                    idx + 1,
                    idx_w = len.to_string().len()
                );
            }

            _ => unimplemented!(),
        }
    }

    pub fn list(&self, len: usize) -> Logger {
        Logger::List {
            indent: self.get_indent(),
            len,
        }
    }

    pub fn task(&self, name: &str, indent: usize) -> Logger {
        self.log(name);

        Logger::Task {
            indent: self.get_indent() + indent,
        }
    }

    pub fn get_indent(&self) -> usize {
        match self {
            Self::Main => 0,
            Self::Task { indent } => *indent,
            Self::List { indent, len, .. } => indent + (len.to_string().len() * 2) + 4,
        }
    }
}
