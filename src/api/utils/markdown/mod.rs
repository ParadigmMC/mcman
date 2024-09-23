use anyhow::{bail, Result};
use serde::Serialize;
use serde_json::Value;

use crate::api::models::markdown::MarkdownOutput;

pub struct MarkdownTableGenerator<T>
where
    T: Serialize,
{
    items: Vec<T>,
}

impl<T> MarkdownTableGenerator<T>
where
    T: Serialize + std::fmt::Debug,
{
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn generate(self) -> Result<MarkdownTable> {
        let mut table = MarkdownTable::new();

        for item in &self.items {
            let value = serde_json::to_value(item)?;
            let Value::Object(map) = value else {
                bail!("Item {item:#?} does not serialize into a HashMap");
            };

            let mut row = vec![];

            for table_header in &table.headers {
                if let Some(value) = map.get(&table_header.0) {
                    row.push(value.to_string());
                } else {
                    row.push(String::new());
                }
            }

            for (key, value) in &map {
                if !table.headers.iter().any(|h| h.0 == *key) {
                    table
                        .headers
                        .push(MarkdownHeader(key.clone(), HeaderAlignment::default()));
                    row.push(value.to_string());
                }
            }

            table.add_row(row);
        }

        Ok(table)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum HeaderAlignment {
    Left,
    #[default]
    Center,
    Right,
}

pub struct MarkdownHeader(pub String, pub HeaderAlignment);

pub struct MarkdownTable {
    pub headers: Vec<MarkdownHeader>,
    pub rows: Vec<Vec<String>>,
}

impl Default for MarkdownTable {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownTable {
    pub fn new() -> Self {
        Self {
            headers: vec![],
            rows: vec![],
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }
}

impl MarkdownTable {
    pub fn render(&self, output: MarkdownOutput) -> String {
        match output {
            MarkdownOutput::ASCII => self.render_ascii(),
            MarkdownOutput::HTML => self.render_html(),
        }
    }

    pub fn render_html(&self) -> String {
        fn wrap(tag: &'static str, content: String) -> String {
            format!("<{tag}>{content}</{tag}>")
        }

        fn join(li: impl Iterator<Item = String>) -> String {
            li.collect::<String>()
        }

        let header = wrap(
            "thead",
            wrap(
                "tr",
                join(self.headers.iter().map(|h| wrap("th", h.0.clone()))),
            ),
        );

        let body = wrap(
            "tbody",
            join(
                self.rows
                    .iter()
                    .map(|row| wrap("tr", join(row.iter().map(|cell| wrap("td", cell.clone()))))),
            ),
        );

        wrap("table", header + &body)
    }

    pub fn render_ascii(&self) -> String {
        let header_lengths = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, MarkdownHeader(h, _))| {
                [h.len()]
                    .into_iter()
                    .chain(self.rows.iter().map(|row| row[i].len()))
                    .max()
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>();

        let pad = " ";
        let padding = 1;

        let mut output_lines = vec![];

        output_lines.push(format!(
            "|{pad:padding$}{}{pad:padding$}|",
            header_lengths
                .iter()
                .enumerate()
                .map(|(i, w)| format!("{:w$}", self.headers[i].0))
                .collect::<Vec<_>>()
                .join(&format!("{pad:padding$}|{pad:padding$}"))
        ));

        output_lines.push(format!(
            "|{pad:padding$}{}{pad:padding$}|",
            header_lengths
                .iter()
                .map(|w| format!("{:-^w$}", ""))
                .collect::<Vec<_>>()
                .join(&format!("{pad:padding$}|{pad:padding$}"))
        ));

        for row in &self.rows {
            output_lines.push(format!(
                "|{pad:padding$}{}{pad:padding$}|",
                header_lengths
                    .iter()
                    .enumerate()
                    .map(|(i, w)| format!("{:w$}", row[i]))
                    .collect::<Vec<_>>()
                    .join(&format!("{pad:padding$}|{pad:padding$}"))
            ));
        }

        output_lines.join("\n")
    }
}
