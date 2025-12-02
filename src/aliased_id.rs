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
    T: AliasedId,
{
    fn aliased_ids(&self) -> Vec<&dyn AliasedId> {
        vec![self]
    }
}

pub(crate) fn get_alias_string(id: &dyn AliasedId) -> String {
    ALIASES
        .lock()
        .unwrap()
        .get(&format!("{id:?}"))
        .cloned()
        .unwrap_or_else(|| id.default_alias())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestId(u64);

    impl ShortId for TestId {
        fn prefix(&self) -> &'static str {
            "ID"
        }

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

        assert_eq!(id1.aliased().to_string(), "⟪ID|1234|foo⟫");
        assert_eq!(id2.aliased().to_string(), "⟪ID|2345|bar⟫");
        assert_eq!(id3.aliased().to_string(), "⟪ID|3456|baz⟫");

        assert_eq!(idx.aliased().to_string(), "⟪ID|1234|qux⟫");
        assert_eq!(idz.aliased().to_string(), "⟪ID|9876⟫");
    }
}
