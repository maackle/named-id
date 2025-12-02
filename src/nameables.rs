use crate::Renamed;

use std::fmt::Debug;

pub trait AnyNameableBounds: Debug + 'static {}
impl<T: Debug + 'static> AnyNameableBounds for T {}

pub struct AnyNameable(pub(crate) Box<dyn AnyNameableBounds>);

impl AnyNameable {
    pub fn new<T: AnyNameableBounds>(t: T) -> Self {
        AnyNameable(Box::new(t))
    }
}

impl std::ops::Deref for AnyNameable {
    type Target = dyn Debug;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::fmt::Display for AnyNameable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for AnyNameable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

pub trait Nameables: Sized + Debug {
    fn nameables(&self) -> Vec<AnyNameable>;

    fn renamed(self) -> Renamed<Self> {
        self.into()
    }
}

impl<T> Nameables for Vec<T>
where
    T: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<T> Nameables for std::collections::HashSet<T>
where
    T: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<T> Nameables for std::collections::BTreeSet<T>
where
    T: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<K, V> Nameables for std::collections::HashMap<K, V>
where
    K: Nameables,
    V: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter()
            .flat_map(|(k, v)| k.nameables().into_iter().chain(v.nameables()))
            .collect()
    }
}

impl<K, V> Nameables for std::collections::BTreeMap<K, V>
where
    K: Nameables,
    V: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter()
            .flat_map(|(k, v)| k.nameables().into_iter().chain(v.nameables()))
            .collect()
    }
}
