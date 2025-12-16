use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{LazyLock, Mutex, atomic::AtomicUsize},
};

use crate::*;

// #[cfg(not(test))]
// static PREFIX_CACHE: LazyLock<Mutex<HashMap<&'static str, std::any::TypeId>>> =
//     LazyLock::new(|| Mutex::new(HashMap::new()));

static SHORT_ID_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static NAMES: LazyLock<Mutex<HashMap<String, Name>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

static SERIAL: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name {
    prefix: Option<&'static str>,
    kind: NameKind,
    brackets: (&'static str, &'static str),
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inner = match &self.kind {
            NameKind::Serial(serial) => format!("#{:03}", serial),
            NameKind::Short(short) => format!("{}", short),
            NameKind::Name(name) => format!("{}", name),
            NameKind::NameShort { name, short } => format!("{}|{}", name, short),
        };

        if let Some(prefix) = self.prefix {
            inner = format!("{}|{}", prefix, inner);
        }
        write!(f, "{}", bracketed(&inner, self.brackets))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NameKind {
    Serial(usize),
    Short(String),
    Name(String),
    NameShort { name: String, short: String },
}

pub trait Nameable: Debug + Display {
    fn with_name(self, name: &str) -> Self
    where
        Self: Sized,
    {
        let name = Name {
            prefix: self.shortener().map(|s| s.prefix),
            kind: NameKind::Name(name.to_string()),
            brackets: self.brackets(),
        };
        set_name(&self, name);
        self
    }

    fn with_name_and_short(self, name: &str) -> Self
    where
        Self: Sized,
    {
        let name = if let Some(shortener) = self.shortener() {
            let short = shortener.shorten(self.to_string());
            Name {
                prefix: Some(shortener.prefix),
                kind: NameKind::NameShort {
                    name: name.to_string(),
                    short,
                },
                brackets: self.brackets(),
            }
        } else {
            Name {
                prefix: None,
                kind: NameKind::Name(name.to_string()),
                brackets: self.brackets(),
            }
        };
        set_name(&self, name);
        self
    }

    fn with_short(self) -> Self
    where
        Self: Sized,
    {
        set_name(
            &self,
            Name {
                prefix: self.shortener().map(|s| s.prefix),
                kind: NameKind::Short(self.short()),
                brackets: self.brackets(),
            },
        );
        self
    }

    fn with_serial(self) -> Self
    where
        Self: Sized,
    {
        let serial = SERIAL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        set_name(
            &self,
            Name {
                prefix: self.shortener().map(|s| s.prefix),
                kind: NameKind::Serial(serial),
                brackets: self.brackets(),
            },
        );
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
        let mut short_id = original.clone();

        short_id.truncate(self.length);

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
        .map(|name| name.to_string())
        .unwrap_or_else(|| id.to_string())
}

pub(crate) fn set_name(id: &dyn Debug, name: Name) {
    use NameKind::*;
    let repr = format!("{id:?}");
    let mut lock = NAMES.lock().unwrap();

    // Only replace "upward" in specificity
    let existing = lock.get(&format!("{id:?}"));
    let replace = existing
        .map(|existing| match (&existing.kind, &name.kind) {
            (Serial(_), Serial(_)) => false,
            (Short(_), Serial(_) | Short(_)) => false,
            (Name(_), Serial(_) | Short(_) | Name(_)) => false,
            (NameShort { .. }, _) => false,
            _ => true,
        })
        .unwrap_or(true);

    if let Some(old) = existing.cloned() {
        if old != name {
            if replace {
                lock.insert(repr.clone(), name.clone());
                tracing::warn!(%old, new = %name, "replacing existing name");
            } else {
                tracing::debug!(%old, new = %name, "name already exists, skipping");
            }
        }
    } else {
        lock.insert(repr.clone(), name.clone());
        tracing::debug!(%repr, %name, "set new name");
    }
}
