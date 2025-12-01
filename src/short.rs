use std::{
    any::TypeId,
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

static PREFIX_CACHE: LazyLock<Mutex<HashMap<String, TypeId>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static SHORT_ID_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
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
}
