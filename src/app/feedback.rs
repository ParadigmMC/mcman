use std::borrow::Cow;

use anyhow::Result;
use console::{style, StyledObject};
use dialoguer::{theme::ColorfulTheme, Input, Confirm, Select};

use crate::util::SelectItem;

use super::App;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressPrefix {
    Resolving,
    Checking,
    Downloading,
    Copying,
    Fetching,
    Exporting,
}

impl Into<Cow<'static, str>> for ProgressPrefix {
    fn into(self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            ProgressPrefix::Resolving => "Resolving",
            ProgressPrefix::Checking => "Checking",
            ProgressPrefix::Downloading => "Downloading",
            ProgressPrefix::Copying => "Copying",
            ProgressPrefix::Fetching => "Fetching",
            ProgressPrefix::Exporting => "Exporting",
        })
    }
}

// 12
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prefix {
    Skipped,
    SkippedWarning,
    Copied,
    Downloaded,

    Imported,
    Exported,
    Rendered,

    Warning,
    Error,
    Info,
    Debug,
}

impl Prefix {
    pub fn as_str(self) -> &'static str {
        match self {
            Prefix::Copied => "      Copied",
            Prefix::Skipped => "     Skipped",
            Prefix::SkippedWarning => "   ! Skipped",
            Prefix::Downloaded => "  Downloaded",
            
            Prefix::Imported => "    Imported",
            Prefix::Exported => "    Exported",
            Prefix::Rendered => "    Rendered",

            Prefix::Error => "     ⚠ Error",
            Prefix::Warning => "      ⚠ Warn",
            Prefix::Info => "      🛈 Info",
            Prefix::Debug => "       debug",
        }
    }

    pub fn styled(self) -> StyledObject<&'static str> {
        match self {
            Prefix::Downloaded | Prefix::Imported | Prefix::Exported | Prefix::Rendered => style(self.as_str()).green().bold(),
            Prefix::Copied | Prefix::Skipped => style(self.as_str()).green(),
            Prefix::Error => style(self.as_str()).red().bold(),
            Prefix::Warning | Prefix::SkippedWarning => style(self.as_str()).yellow().bold(),
            Prefix::Info => style(self.as_str()).bold(),
            Prefix::Debug => style(self.as_str()).dim(),
        }
    }
}

impl Into<Cow<'static, str>> for Prefix {
    fn into(self) -> Cow<'static, str> {
        Cow::Borrowed(self.as_str().trim())
    }
}

impl App {
    pub fn println<S: std::fmt::Display>(&self, message: S) {
        self.multi_progress.suspend(|| println!("{message}"));
    }

    pub fn success<S: std::fmt::Display>(&self, message: S) {
        self.println(format!(
            "  {} {message}",
            ColorfulTheme::default().success_prefix
        ));
    }

    pub fn log<S: std::fmt::Display>(&self, message: S) {
        self.println(format!("  {message}"));
    }

    pub fn notify<S: std::fmt::Display>(&self, prefix: Prefix, message: S) {
        self.println(format!("{} {message}", prefix.styled()));
    }

    pub fn warn<S: std::fmt::Display>(&self, message: S) {
        self.notify(Prefix::Warning, message);
    }

    pub fn error<S: std::fmt::Display>(&self, message: S) {
        self.notify(Prefix::Error, message);
    }

    pub fn info<S: std::fmt::Display>(&self, message: S) {
        self.notify(Prefix::Info, message);
    }
    

    pub fn dbg<S: std::fmt::Display>(&self, message: S) {
        if std::env::var("MCMAN_DEBUG") == Ok("true".to_owned()) {
            self.notify(Prefix::Debug, message);
        }
    }

    pub fn print_job(&self, job: &str) {
        if !self.is_ci() {
            self.println(format!(
                "{} {}",
                ColorfulTheme::default().active_item_prefix,
                style(job).cyan().bold()
            ));
        }
    }

    pub fn is_ci(&self) -> bool {
        std::env::var("CI").ok() == Some("true".to_owned())
    }

    pub fn ci(&self, cmd: &str) {
        if self.is_ci() {
            self.println(cmd);
        }
    }

    pub fn prompt_string(&self, prompt: &str) -> Result<String> {
        Ok(self.multi_progress.suspend(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .interact_text()
        })?)
    }

    pub fn prompt_string_default(&self, prompt: &str, default: &str) -> Result<String> {
        Ok(self.multi_progress.suspend(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(default.to_owned())
                .interact_text()
        })?)
    }

    pub fn prompt_string_filled(&self, prompt: &str, default: &str) -> Result<String> {
        Ok(self.multi_progress.suspend(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(default.to_owned())
                .with_initial_text(default.to_owned())
                .interact_text()
        })?)
    }

    pub fn confirm(&self, prompt: &str) -> Result<bool> {
        Ok(self.multi_progress.suspend(|| {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .wait_for_newline(true)
                .interact()
        })?)
    }

    pub fn select<T: Clone>(&self, prompt: &str, items: &[SelectItem<T>]) -> Result<T> {
        let item = &items[self.multi_progress.suspend(|| {
            Select::with_theme(&ColorfulTheme::default())
                .items(items)
                .with_prompt(prompt)
                .default(0)
                .interact()
        })?];

        Ok(item.0.clone())
    }

    pub fn select_with_default<T: Clone>(&self, prompt: &str, items: &[SelectItem<T>], def: usize) -> Result<T> {
        let item = &items[self.multi_progress.suspend(|| {
            Select::with_theme(&ColorfulTheme::default())
                .items(items)
                .with_prompt(prompt)
                .default(def)
                .interact()
        })?];

        Ok(item.0.clone())
    }
}
