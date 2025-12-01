//! A utility for making logs more readable by shortening and aliasing long IDs.

use std::{
    any::TypeId,
    collections::HashMap,
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

static PREFIX_CACHE: LazyLock<Mutex<HashMap<String, TypeId>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static SHORT_ID_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static ALIASES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait ShortId: 'static {
    const PREFIX: &'static str;
    const LENGTH: usize = 4;

    fn to_short_string(&self) -> String;

    fn prefix() -> String {
        let prefix = Self::PREFIX.to_string();
        debug_assert!(
            {
                let tid = TypeId::of::<Self>();
                let r = PREFIX_CACHE.lock().unwrap().insert(prefix.clone(), tid);
                if let Some(existing) = r {
                    existing == tid
                } else {
                    true
                }
            },
            "\"{prefix}\" has already been registered as a ShortId::PREFIX"
        );
        prefix
    }

    fn short(&self) -> String {
        let original = ShortId::to_short_string(self);
        let mut s = original.clone();

        s.truncate(Self::LENGTH);

        let short_id = format!("{}|{s}", Self::prefix());
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

pub trait AliasedId: ShortId + Debug {
    const SHOW_SHORT_ID: bool = true;
    const BRACKETS: (&'static str, &'static str) = ("⟪", "⟫");

    fn aliased(self, alias: &str) -> Self
    where
        Self: Sized,
    {
        let alias = if Self::SHOW_SHORT_ID {
            bracketed(&format!("{}|{}", self.short(), alias), Self::BRACKETS)
        } else {
            bracketed(
                &format!("{}‖{}", <Self as ShortId>::prefix(), alias),
                Self::BRACKETS,
            )
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

    fn alias(&self) -> String {
        ALIASES
            .lock()
            .unwrap()
            .get(&format!("{self:?}"))
            .cloned()
            .unwrap_or_else(|| self.default_alias())
    }

    fn default_alias(&self) -> String {
        bracketed(&self.short(), Self::BRACKETS)
    }
}

fn bracketed(s: &str, brackets: (&'static str, &'static str)) -> String {
    format!("{}{s}{}", brackets.0, brackets.1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestId(u64);

    impl ShortId for TestId {
        const PREFIX: &'static str = "ID";

        fn to_short_string(&self) -> String {
            format!("{}", self.0)
        }
    }

    impl AliasedId for TestId {}

    #[test]
    fn test_short_id() {
        tracing_subscriber::fmt::init();

        let id1 = TestId(1234567890);
        let id2 = TestId(2345678901);
        let id3 = TestId(3456789012);
        let idx = TestId(12349876);
        dbg!();
        assert_eq!(id1.short(), "ID|1234");
        dbg!();
        assert_eq!(id2.short(), "ID|2345");
        dbg!();
        assert_eq!(id3.short(), "ID|3456");

        assert_eq!(idx.short(), "ID|1234");
    }

    #[test]
    fn test_prefix_collision() {
        struct TestId2(u64);
        impl ShortId for TestId2 {
            const PREFIX: &'static str = "ID";
            fn to_short_string(&self) -> String {
                format!("{}", self.0)
            }
        }

        std::panic::catch_unwind(|| {
            TestId2(1234567890).short();
        })
        .unwrap();

        std::panic::catch_unwind(|| {
            TestId(1234567890).short();
        })
        .unwrap_err();
    }

    #[test]
    fn test_aliased_id() {
        let id1 = TestId(1234567890).aliased("foo");
        let id2 = TestId(2345678901).aliased("bar");
        let id3 = TestId(3456789012).aliased("baz");
        let idx = TestId(12349876).aliased("qux");
        let idz = TestId(987654321);

        assert_eq!(id1.alias(), "⟪ID|1234|foo⟫");
        assert_eq!(id2.alias(), "⟪ID|2345|bar⟫");
        assert_eq!(id3.alias(), "⟪ID|3456|baz⟫");

        assert_eq!(idx.alias(), "⟪ID|1234|qux⟫");
        assert_eq!(idz.alias(), "⟪ID|9876⟫");
    }
}
