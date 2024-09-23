use std::{borrow::Cow, collections::HashSet};

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use super::App;

lazy_static! {
    static ref REPLACEMENT_REGEX: Regex = Regex::new(r"\$\{\{(\w+)?\}\}").unwrap();
}

impl App {
    // this is a modified regex::Regex::replace_all
    pub async fn vars_replace_content<'h>(
        &self,
        haystack: &'h str,
    ) -> Result<(Cow<'h, str>, HashSet<String>)> {
        let mut used_vars = HashSet::new();

        let mut it = REPLACEMENT_REGEX
            .captures_iter(haystack)
            .enumerate()
            .peekable();

        if it.peek().is_none() {
            return Ok((Cow::Borrowed(haystack), HashSet::new()));
        }

        let mut new = String::with_capacity(haystack.len());
        let mut last_match = 0;

        for (_i, cap) in it {
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.push_str(&haystack[last_match..m.start()]);

            // ---
            // actual logic here

            let expr = cap.get(1).map(|x| x.as_str()).unwrap_or_default();
            let (replaced, vars) = self.resolve_variable_expression(expr).await?;
            used_vars.extend(vars);
            new.push_str(&replaced);

            // ---

            last_match = m.end();
        }

        new.push_str(&haystack[last_match..]);

        Ok((Cow::Owned(new), HashSet::new()))
    }

    pub async fn resolve_variable_expression(
        &self,
        expr: &str,
    ) -> Result<(String, HashSet<String>)> {
        let mut visited = HashSet::new();
        visited.insert(expr.to_owned());

        // TODO actual expressions like ternary
        // TODO recursive expansions

        let value = self.resolve_variable_value(expr).await;

        Ok((value.unwrap_or_else(|| expr.to_owned()), visited))
    }

    pub async fn resolve_variable_value(&self, key: &str) -> Option<String> {
        if let Some((_, server)) = &*self.server.read().await {
            if let Some(v) = server.variables.get(key).cloned() {
                return Some(v);
            }
        }

        if let Some((_, network)) = &*self.network.read().await {
            if let Some(v) = network.variables.get(&format!("network_{key}")).cloned() {
                return Some(v);
            }
        }

        if let Ok(v) = std::env::var(key) {
            return Some(v);
        }

        None
    }
}
