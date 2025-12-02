use crate::{Aliased, AliasedId};

use std::fmt::Debug;

pub trait ContainsAliases: Sized + Debug {
    fn aliased_ids(&self) -> Vec<&dyn AliasedId>;

    fn aliased(self) -> Aliased<Self> {
        self.into()
    }
}

impl<T> ContainsAliases for Vec<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<&dyn AliasedId> {
        self.iter().flat_map(|t| t.aliased_ids()).collect()
    }
}

impl<T> ContainsAliases for std::collections::HashSet<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<&dyn AliasedId> {
        self.iter().flat_map(|t| t.aliased_ids()).collect()
    }
}

impl<T> ContainsAliases for std::collections::BTreeSet<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<&dyn AliasedId> {
        self.iter().flat_map(|t| t.aliased_ids()).collect()
    }
}

impl<K, V> ContainsAliases for std::collections::HashMap<K, V>
where
    K: ContainsAliases,
    V: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<&dyn AliasedId> {
        self.iter()
            .flat_map(|(k, v)| k.aliased_ids().into_iter().chain(v.aliased_ids()))
            .collect()
    }
}

impl<K, V> ContainsAliases for std::collections::BTreeMap<K, V>
where
    K: ContainsAliases,
    V: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<&dyn AliasedId> {
        self.iter()
            .flat_map(|(k, v)| k.aliased_ids().into_iter().chain(v.aliased_ids()))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::{AliasedId, ContainsAliases, ShortId};
    use pretty_assertions::assert_eq;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn test_aliased_id_vec() {
        let v = vec![TestId(11111111), TestId(22222222), TestId(33333333)];
        let a = v.aliased();
        assert_eq!(format!("{a}"), "[⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]");
        assert_eq!(format!("{a:?}"), "[⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]");
        assert_eq!(
            format!("{a:#?}"),
            "
[
    ⟪ID|1111⟫,
    ⟪ID|2222⟫,
    ⟪ID|3333⟫,
]
        "
            .trim()
        );

        let s = std::collections::BTreeSet::from([
            TestId(11111111),
            TestId(22222222),
            TestId(33333333),
        ]);
        let a = s.aliased();
        assert_eq!(format!("{a}"), "{⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫}");
        assert_eq!(format!("{a:?}"), "{⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫}");
        assert_eq!(
            format!("{a:#?}"),
            "
{
    ⟪ID|1111⟫,
    ⟪ID|2222⟫,
    ⟪ID|3333⟫,
}
        "
            .trim()
        );
    }

    #[test]
    fn test_aliased_id_maps() {
        let s = std::collections::BTreeMap::from([
            (TestId(11111111), vec![TestId(22222222), TestId(55555555)]),
            (TestId(22222222), vec![TestId(33333333), TestId(66666666)]),
            (TestId(33333333), vec![TestId(44444444), TestId(77777777)]),
        ]);
        let a = s.aliased();
        assert_eq!(
            format!("{a}"),
            "{⟪ID|1111⟫: [⟪ID|2222⟫, ⟪ID|5555⟫], ⟪ID|2222⟫: [⟪ID|3333⟫, ⟪ID|6666⟫], ⟪ID|3333⟫: [⟪ID|4444⟫, ⟪ID|7777⟫]}"
        );
        assert_eq!(
            format!("{a:?}"),
            "{⟪ID|1111⟫: [⟪ID|2222⟫, ⟪ID|5555⟫], ⟪ID|2222⟫: [⟪ID|3333⟫, ⟪ID|6666⟫], ⟪ID|3333⟫: [⟪ID|4444⟫, ⟪ID|7777⟫]}"
        );
        assert_eq!(
            format!("{a:#?}"),
            "
{
    ⟪ID|1111⟫: [
        ⟪ID|2222⟫,
        ⟪ID|5555⟫,
    ],
    ⟪ID|2222⟫: [
        ⟪ID|3333⟫,
        ⟪ID|6666⟫,
    ],
    ⟪ID|3333⟫: [
        ⟪ID|4444⟫,
        ⟪ID|7777⟫,
    ],
}
            "
            .trim()
        );
    }
}
