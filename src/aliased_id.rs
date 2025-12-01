use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use crate::{Aliasable, ShortId, bracketed};

static ALIASES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait AliasedId: ShortId + Debug {
    const SHOW_SHORT_ID: bool = true;
    const BRACKETS: (&'static str, &'static str) = ("⟪", "⟫");

    fn with_alias(self, alias: &str) -> Aliased<Self>
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
        Aliased(self)
    }

    fn default_alias(&self) -> String {
        bracketed(&self.short(), Self::BRACKETS)
    }
}

impl<T> Aliasable for T
where
    T: AliasedId,
{
    fn display(&self) -> String {
        get_alias_string(self)
    }
}

fn get_alias_string<T>(id: &T) -> String
where
    T: AliasedId,
{
    ALIASES
        .lock()
        .unwrap()
        .get(&format!("{id:?}"))
        .cloned()
        .unwrap_or_else(|| id.default_alias())
}

pub struct Aliased<T>(T);

impl<T> From<T> for Aliased<T>
where
    T: Aliasable,
{
    fn from(value: T) -> Self {
        Aliased(value)
    }
}

impl<T> std::fmt::Display for Aliased<T>
where
    T: Aliasable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl<T> std::fmt::Debug for Aliased<T>
where
    T: Aliasable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl<T> std::ops::Deref for Aliased<T>
where
    T: Aliasable,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
    fn test_aliased_id() {
        let id1 = TestId(1234567890).with_alias("foo");
        let id2 = TestId(2345678901).with_alias("bar");
        let id3 = TestId(3456789012).with_alias("baz");
        let idx = TestId(12349876).with_alias("qux");
        let idz = TestId(987654321);

        assert_eq!(id1.to_string(), "⟪ID|1234|foo⟫");
        assert_eq!(id2.to_string(), "⟪ID|2345|bar⟫");
        assert_eq!(id3.to_string(), "⟪ID|3456|baz⟫");

        assert_eq!(idx.to_string(), "⟪ID|1234|qux⟫");
        assert_eq!(idz.aliased().to_string(), "⟪ID|9876⟫");
    }
}
