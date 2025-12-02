use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use crate::*;

static ALIASES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait AliasedId: ShortId + Debug {
    fn with_alias(self, alias: &str) -> Self
    where
        Self: Sized,
    {
        let alias = if self.show_short_id() {
            bracketed(&format!("{}|{}", self.short(), alias), self.brackets())
        } else {
            bracketed(&format!("{}‖{}", self.prefix(), alias), self.brackets())
        };
        let existing = ALIASES
            .lock()
            .unwrap()
            .insert(format!("{self:?}"), alias.clone());
        if let Some(old) = existing {
            if old != alias {
                tracing::warn!(?old, new = ?alias, "alias already exists, replacing");
            }
        }
        self
    }

    fn default_alias(&self) -> String {
        bracketed(&self.short(), self.brackets())
    }

    fn show_short_id(&self) -> bool {
        true
    }

    fn brackets(&self) -> (&'static str, &'static str) {
        ("⟪", "⟫")
    }
}

impl<T> ContainsAliases for T
where
    T: AliasedId + Clone,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        vec![self.to_owned().into()]
    }
}

pub(crate) fn get_alias_string(id: &AnyAlias) -> String {
    ALIASES
        .lock()
        .unwrap()
        .get(&format!("{id:?}"))
        .cloned()
        .unwrap_or_else(|| id.default_alias())
}
