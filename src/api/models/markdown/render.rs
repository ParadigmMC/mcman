use crate::api::{
    models::metadata::AddonMetadata,
    utils::markdown::{HeaderAlignment, MarkdownHeader, MarkdownTable},
};

use super::{MarkdownOptions, MarkdownOutput, MdColumn};

impl MarkdownOptions {
    pub fn render_addons(&self, list: Vec<AddonMetadata>) -> String {
        self.table_addons(list, self.output_type)
            .render(self.output_type)
    }

    pub fn table_addons(&self, list: Vec<AddonMetadata>, output: MarkdownOutput) -> MarkdownTable {
        let mut table = MarkdownTable::new();

        for column in &self.columns {
            table.headers.push(MarkdownHeader(
                self.titles
                    .get(column)
                    .cloned()
                    .unwrap_or_else(|| column.to_string()),
                HeaderAlignment::default(),
            ));
        }

        for meta in &list {
            let mut row = vec![];

            for column in &self.columns {
                match column {
                    MdColumn::Name => row.push(if self.name_includes_link {
                        if let Some(link) = &meta.link {
                            match output {
                                MarkdownOutput::ASCII => format!("[{}]({})", meta.name, link),
                                MarkdownOutput::HTML => {
                                    format!("<a href={}>{}</a>", link, meta.name)
                                },
                            }
                        } else {
                            meta.name.clone()
                        }
                    } else {
                        meta.name.clone()
                    }),
                    MdColumn::Description => row.push(meta.description.clone().unwrap_or_default()),
                    MdColumn::Link => row.push(meta.link.clone().unwrap_or_default()),
                    MdColumn::Version => row.push(meta.version.clone().unwrap_or_default()),
                    MdColumn::Icon => row.push(match output {
                        MarkdownOutput::ASCII => meta.source.markdown_tag(),
                        MarkdownOutput::HTML => meta.source.html(),
                        _ => meta.source.into_str().to_owned(),
                    }),
                }
            }

            table.add_row(row);
        }

        table
    }
}
