use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

#[cfg(not(test))]
static PREFIX_CACHE: LazyLock<Mutex<HashMap<&'static str, std::any::TypeId>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static SHORT_ID_CACHE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub trait ShortId: 'static {
    fn to_short_string(&self) -> String;

    fn prefix(&self) -> &'static str;

    fn short(&self) -> String {
        assert_prefix_unique(self);
        let original = self.to_short_string();
        let mut s = original.clone();

        s.truncate(self.length());

        let short_id = format!("{}|{s}", self.prefix());

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

    fn length(&self) -> usize {
        4
    }
}

#[allow(unused)]
fn assert_prefix_unique<T: ShortId + ?Sized>(t: &T) {
    #[cfg(test)]
    {
        // the same type gets different TypeIds in test suites.
    }

    #[cfg(not(test))]
    {
        let prefix = t.prefix();
        debug_assert!(
            {
                let tid = std::any::TypeId::of::<T>();
                let r = PREFIX_CACHE.lock().unwrap().insert(prefix, tid);
                r.map(|t| t == tid).unwrap_or(true)
            },
            "\"{prefix}\" has already been registered as a ShortId::PREFIX"
        );
    }
}
