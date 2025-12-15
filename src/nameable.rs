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

static NAMES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait Nameable: Debug + Display {
    fn with_name(self, name: &str) -> Self
    where
        Self: Sized,
    {
        let name = if let Some(shortener) = self.shortener() {
            bracketed(&format!("{}|{}", shortener.prefix, name), self.brackets())
        } else {
            bracketed(name, self.brackets())
        };
        set_name_string(&self, &name);
        self
    }

    fn with_name_and_short(self, name: &str) -> Self
    where
        Self: Sized,
    {
        let name = if let Some(shortener) = self.shortener() {
            let short = shortener.shorten(self.to_string());
            bracketed(&format!("{}|{}", short, name), self.brackets())
        } else {
            bracketed(name, self.brackets())
        };
        set_name_string(&self, &name);
        self
    }

    fn with_short(self) -> Self
    where
        Self: Sized,
    {
        let short = bracketed(&self.short(), self.brackets());
        set_name_string(&self, &short);
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

impl<T> Rename for T
where
    T: Nameable + Clone + 'static,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        vec![AnyNameable(Box::new(self.clone()))]
    }
}

pub(crate) fn get_name_string(id: &AnyNameable) -> String {
    NAMES
        .lock()
        .unwrap()
        .get(&format!("{id:?}"))
        .cloned()
        .unwrap_or_else(|| id.to_string())
}

pub(crate) fn set_name_string(id: &dyn Debug, name: &str) {
    let repr = format!("{id:?}");
    let existing = NAMES.lock().unwrap().insert(repr.clone(), name.to_string());
    if let Some(old) = existing {
        if old != name {
            tracing::warn!(?old, new = ?name, "name already exists, replacing");
        }
    } else {
        tracing::debug!(?repr, ?name, "setting name string");
    }
}
