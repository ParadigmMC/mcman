use indexmap::IndexMap;
use std::borrow::Cow;

#[derive(Default)]
pub struct MarkdownTable {
    pub headers: Vec<Cow<'static, str>>,
    pub rows: Vec<Vec<String>>,
}

impl MarkdownTable {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_headers(headers: Vec<Cow<'static, str>>) -> Self {
        Self {
            headers,
            rows: vec![],
        }
    }

    pub fn from_map(map: IndexMap<Cow<'static, str>, String>) -> Self {
        let mut table = Self::new();

        table.add_from_map(map);

        table
    }

    pub fn add_from_map(&mut self, map: IndexMap<Cow<'static, str>, String>) -> &mut Self {
        let mut row = self
            .headers
            .iter()
            .map(|header| map.get(header).cloned().unwrap_or_default())
            .collect::<Vec<_>>();

        let hack = self.headers.clone();

        for (k, v) in map.into_iter().filter(|(k, _)| !hack.contains(k)) {
            self.headers.push(k);
            row.push(v);
        }

        self.rows.push(row);

        self
    }

    pub fn render(&self) -> String {
        let mut col_lengths = vec![];

        for idx in 0..self.headers.len() {
            let mut li = vec![self.headers[idx].len()];

            li.extend(self.rows.iter().map(|row| row[idx].len()));

            col_lengths.push(li.into_iter().max().expect("col lengths iter max none"));
        }

        let mut lines = vec![];

        lines.push({
            let cols = col_lengths
                .iter()
                .enumerate()
                .map(|(idx, width)| format!("{:width$}", self.headers[idx]))
                .collect::<Vec<_>>();

            format!("| {} |", cols.join(" | "))
        });

        lines.push({
            let cols = col_lengths
                .iter()
                .map(|length| format!("{:-^width$}", "", width = length))
                .collect::<Vec<_>>();

            format!("| {} |", cols.join(" | "))
        });

        lines.extend(self.rows.iter().map(|row| {
            let cols = (0..row.len())
                .map(|idx| format!("{:width$}", row[idx], width = col_lengths[idx]))
                .collect::<Vec<_>>();

            format!("| {} |", cols.join(" | "))
        }));

        lines.join("\n")
    }

    pub fn render_ascii_lines(&self, headers: bool) -> Vec<String> {
        let col_lengths = (0..self.headers.len())
            .map(|idx| {
                let mut li = vec![self.headers[idx].len()];

                li.extend(
                    self.rows
                        .iter()
                        .map(|row| row.get(idx).unwrap_or(&String::new()).len()),
                );

                li.into_iter().max().expect("col lengths iter max none")
            })
            .collect::<Vec<_>>();

        let mut lines = vec![];

        if headers {
            lines.push({
                let cols = col_lengths
                    .iter()
                    .enumerate()
                    .map(|(idx, width)| format!("{:width$}", self.headers[idx]))
                    .collect::<Vec<_>>();

                cols.join(" ")
            });

            lines.push({
                let cols = col_lengths
                    .iter()
                    .map(|length| format!("{:-^width$}", "", width = length))
                    .collect::<Vec<_>>();

                cols.join(" ")
            });
        }

        lines.extend(self.rows.iter().map(|row| {
            let cols = col_lengths
                .iter()
                .enumerate()
                .take(row.len())
                .map(|(idx, width)| format!("{:width$}", row.get(idx).unwrap_or(&String::new())))
                .collect::<Vec<_>>();

            cols.join(" ")
        }));

        lines
    }

    pub fn render_ascii(&self) -> String {
        self.render_ascii_lines(true).join("\n")
    }
}
