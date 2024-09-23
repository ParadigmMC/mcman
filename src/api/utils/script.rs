#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shell {
    Sh,
    Bat,
}

impl Shell {
    pub fn generate_script(self, lines: &[String]) -> String {
        format!(
            "{}{line_sep}{}{line_sep}",
            self.header(),
            lines.join(self.line_sep()),
            line_sep = self.line_sep()
        )
    }

    pub fn comment(self, comment: &str) -> String {
        match self {
            Self::Sh => format!("# {comment}"),
            Self::Bat => format!(":: {comment}"),
        }
    }

    pub const fn header(self) -> &'static str {
        match self {
            Self::Sh => "#!/bin/sh",
            Self::Bat => "@echo off",
        }
    }

    pub const fn file_ext(self) -> &'static str {
        match self {
            Self::Sh => "sh",
            Self::Bat => "bat",
        }
    }

    pub const fn line_sep(self) -> &'static str {
        match self {
            Self::Sh => "\n",
            Self::Bat => "\r\n",
        }
    }

    pub const fn script_args(self) -> &'static str {
        match self {
            Self::Sh => "\"$@\"",
            Self::Bat => "%*",
        }
    }
}
