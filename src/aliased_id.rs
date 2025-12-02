use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{LazyLock, Mutex},
};

use crate::*;

// #[cfg(not(test))]
// static PREFIX_CACHE: LazyLock<Mutex<HashMap<&'static str, std::any::TypeId>>> =
//     LazyLock::new(|| Mutex::new(HashMap::new()));

static SHORT_ID_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static ALIASES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait AliasedId: Debug + Display {
    fn with_alias(self, alias: &str) -> Self
    where
        Self: Sized,
    {
        let alias = if let Some(shortener) = self.shortener() {
            let short = shortener.shorten(self.to_string());
            bracketed(&format!("{}|{}", short, alias), self.brackets())
        } else {
            bracketed(alias, self.brackets())
        };
        set_alias_string(&self, &alias);
        self
    }

    fn with_short(self) -> Self
    where
        Self: Sized,
    {
        let short = bracketed(&self.short(), self.brackets());
        set_alias_string(&self, &short);
        self
    }

    fn shortener(&self) -> Option<Shortener>;

    fn short(&self) -> String {
        self.shortener()
            .map(|s| s.shorten(self.to_string()))
            .unwrap_or_else(|| self.to_string())
    }

    fn brackets(&self) -> (&'static str, &'static str) {
        ("⟪", "⟫")
    }
}

pub struct Shortener {
    pub length: usize,
    pub prefix: &'static str,
}

impl Shortener {
    fn shorten(&self, original: String) -> String {
        // assert_prefix_unique(self);
        let mut s = original.clone();

        s.truncate(self.length);

        let short_id = if s.is_empty() {
            format!("{}‖", self.prefix)
        } else {
            format!("{}|{s}", self.prefix)
        };

        if let Some(existing) = SHORT_ID_CACHE
            .lock()
            .unwrap()
            .insert(short_id.clone(), original.clone())
        {
            if existing != original {
                tracing::warn!(
                    old = ?existing,
                    new = ?original,
                    "short ID collision, two values have the same short ID"
                );
            }
        }
        short_id
    }
}

impl<T> ContainsAliases for T
where
    T: AliasedId + Clone + 'static,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        vec![AnyAlias(Box::new(self.clone()))]
    }
}

pub(crate) fn get_alias_string(id: &AnyAlias) -> String {
    ALIASES
        .lock()
        .unwrap()
        .get(&format!("{id:?}"))
        .cloned()
        .unwrap_or_else(|| id.to_string())
}

pub(crate) fn set_alias_string(id: &dyn Debug, alias: &str) {
    let existing = ALIASES
        .lock()
        .unwrap()
        .insert(format!("{id:?}"), alias.to_string());
    if let Some(old) = existing {
        if old != alias {
            tracing::warn!(?old, new = ?alias, "alias already exists, replacing");
        }
    }
}
