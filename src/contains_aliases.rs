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
