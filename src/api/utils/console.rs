use std::{collections::HashMap, fmt::Display};

use console::style;

pub fn log_header(header: &str) {
    println!("{}", style(header).cyan().bold());
}

pub fn log_hashmap<T: Display>(items: HashMap<&'static str, T>) {
    let padding = items.keys().map(|k| k.len()).max().unwrap_or_default();
    for (k, v) in items {
        println!(
            "  {:padding$}: {}",
            style(k).bold(),
            style(v),
        );
    }
}
