use super::*;

impl<T> Nameables for Option<T>
where
    T: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
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

impl<A, B> Nameables for (A, B)
where
    A: Nameables,
    B: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .collect()
    }
}

impl<A, B, C> Nameables for (A, B, C)
where
    A: Nameables,
    B: Nameables,
    C: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .chain(self.2.nameables())
            .collect()
    }
}

impl<A, B, C, D> Nameables for (A, B, C, D)
where
    A: Nameables,
    B: Nameables,
    C: Nameables,
    D: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .chain(self.2.nameables())
            .chain(self.3.nameables())
            .collect()
    }
}

impl<A, B, C, D, E> Nameables for (A, B, C, D, E)
where
    A: Nameables,
    B: Nameables,
    C: Nameables,
    D: Nameables,
    E: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .chain(self.2.nameables())
            .chain(self.3.nameables())
            .chain(self.4.nameables())
            .collect()
    }
}

impl<A, B, C, D, E, F> Nameables for (A, B, C, D, E, F)
where
    A: Nameables,
    B: Nameables,
    C: Nameables,
    D: Nameables,
    E: Nameables,
    F: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .chain(self.2.nameables())
            .chain(self.3.nameables())
            .chain(self.4.nameables())
            .chain(self.5.nameables())
            .collect()
    }
}

impl<A, B, C, D, E, F, G> Nameables for (A, B, C, D, E, F, G)
where
    A: Nameables,
    B: Nameables,
    C: Nameables,
    D: Nameables,
    E: Nameables,
    F: Nameables,
    G: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .chain(self.2.nameables())
            .chain(self.3.nameables())
            .chain(self.4.nameables())
            .chain(self.5.nameables())
            .chain(self.6.nameables())
            .collect()
    }
}

impl<A, B, C, D, E, F, G, H> Nameables for (A, B, C, D, E, F, G, H)
where
    A: Nameables,
    B: Nameables,
    C: Nameables,
    D: Nameables,
    E: Nameables,
    F: Nameables,
    G: Nameables,
    H: Nameables,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .chain(self.2.nameables())
            .chain(self.3.nameables())
            .chain(self.4.nameables())
            .chain(self.5.nameables())
            .chain(self.6.nameables())
            .chain(self.7.nameables())
            .collect()
    }
}
