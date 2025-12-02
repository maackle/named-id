use crate::{Aliased, AliasedId};

use std::fmt::{Debug, Display};

pub trait AnyAliasable: Debug + Display {}
impl<T: Debug + Display> AnyAliasable for T {}

pub struct AnyAlias(pub(crate) Box<dyn AnyAliasable>);

impl std::ops::Deref for AnyAlias {
    type Target = dyn Debug;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::fmt::Display for AnyAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for AnyAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

pub trait ContainsAliases: Sized + Debug {
    fn aliased_ids(&self) -> Vec<AnyAlias>;

    fn aliased(self) -> Aliased<Self> {
        self.into()
    }
}

impl<T> ContainsAliases for Vec<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        self.iter().flat_map(|t| t.aliased_ids()).collect()
    }
}

impl<T> ContainsAliases for std::collections::HashSet<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        self.iter().flat_map(|t| t.aliased_ids()).collect()
    }
}

impl<T> ContainsAliases for std::collections::BTreeSet<T>
where
    T: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        self.iter().flat_map(|t| t.aliased_ids()).collect()
    }
}

impl<K, V> ContainsAliases for std::collections::HashMap<K, V>
where
    K: ContainsAliases,
    V: ContainsAliases,
{
    fn aliased_ids(&self) -> Vec<AnyAlias> {
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
    fn aliased_ids(&self) -> Vec<AnyAlias> {
        self.iter()
            .flat_map(|(k, v)| k.aliased_ids().into_iter().chain(v.aliased_ids()))
            .collect()
    }
}
