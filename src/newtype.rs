use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Aliased<T>(T);

impl<T> From<T> for Aliased<T>
where
    T: ContainsAliases,
{
    fn from(value: T) -> Self {
        Aliased(value)
    }
}

impl<T> ContainsAliases for Aliased<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        self.0.aliased_ids()
    }
}

impl<T> std::ops::Deref for Aliased<T>
where
    T: ContainsAliases,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::fmt::Display for Aliased<T>
where
    T: ContainsAliases,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> std::fmt::Debug for Aliased<T>
where
    T: ContainsAliases,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = f.alternate();
        let debug = if pretty {
            format!("{:#?}", self.0)
        } else {
            format!("{:?}", self.0)
        };
        let patterns = self
            .0
            .aliased_ids()
            .into_iter()
            .map(|id| {
                let pat = if pretty {
                    pretty_pattern(&format!("{:#?}", id))
                } else {
                    format!("{:?}", id)
                };
                (pat, get_alias_string(&id))
            })
            .collect::<Vec<_>>();

        let mut result = debug;
        for (pattern, replacement) in patterns {
            if pretty {
                result = regex::Regex::new(&pattern)
                    .unwrap()
                    .replace_all(&result, PrettyReplacer(&replacement))
                    .to_string();
            } else {
                result = result.replace(&pattern, &replacement);
            }
        }
        write!(f, "{}", result)
    }
}

fn pretty_pattern(pretty_dbg: &str) -> String {
    pretty_dbg
        .split('\n')
        .map(|line| format!(" *{}", regex::escape(line)))
        .collect::<Vec<_>>()
        .join("\n")
}

struct PrettyReplacer<'a>(&'a str);

impl<'a> regex::Replacer for PrettyReplacer<'a> {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        for cap in caps.iter() {
            let cap = cap.unwrap();
            let spaces = cap
                .as_str()
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            // let spaces = " ".repeat(cap.start() - 1);
            let r = self
                .0
                .split('\n')
                .map(|line| format!("{spaces}{line}"))
                .collect::<Vec<_>>()
                .join("\n");
            dst.push_str(&r);
        }
    }
}
