#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shell {
    Sh,
    Bat,
}

impl Shell {
    pub fn generate_script(&self, lines: Vec<String>) -> String {
        let mut content = String::new();
        content += self.header();
        content += self.line_sep();
        content += &lines.join(self.line_sep());
        content += self.line_sep();
        content
    }

    pub fn comment(&self, comment: &str) -> String {
        match self {
            Shell::Sh => format!("# {comment}"),
            Shell::Bat => format!(":: {comment}"),
        }
    }

    pub fn header(&self) -> &'static str {
        match self {
            Shell::Sh => "#!/bin/sh",
            Shell::Bat => "@echo off",
        }
    }

    pub fn file_ext(&self) -> &'static str {
        match self {
            Shell::Sh => "sh",
            Shell::Bat => "bat",
        }
    }

    pub fn line_sep(&self) -> &'static str {
        match self {
            Shell::Sh => "\n",
            Shell::Bat => "\r\n",
        }
    }

    pub fn script_args(&self) -> &'static str {
        match self {
            Shell::Sh => "\"$@\"",
            Shell::Bat => "%*",
        }
    }
}
