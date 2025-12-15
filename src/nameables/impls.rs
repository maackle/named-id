use std::marker::PhantomData;

use super::*;

#[macro_export]
macro_rules! empty_nameables {
    ($ty:ty) => {
        impl Rename for $ty {
            fn nameables(&self) -> Vec<AnyNameable> {
                vec![]
            }
        }
    };
}
empty_nameables!(());
empty_nameables!(u8);
empty_nameables!(u16);
empty_nameables!(u32);
empty_nameables!(u64);
empty_nameables!(u128);
empty_nameables!(usize);
empty_nameables!(i8);
empty_nameables!(i16);
empty_nameables!(i32);
empty_nameables!(i64);
empty_nameables!(i128);
empty_nameables!(isize);
empty_nameables!(f32);
empty_nameables!(f64);
empty_nameables!(bool);
empty_nameables!(char);
empty_nameables!(&'static str);

impl<T> Rename for PhantomData<T> {
    fn nameables(&self) -> Vec<AnyNameable> {
        vec![]
    }
}

impl<T> Rename for Option<T>
where
    T: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<T> Rename for Vec<T>
where
    T: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<T> Rename for std::collections::HashSet<T>
where
    T: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<T> Rename for std::collections::BTreeSet<T>
where
    T: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter().flat_map(|t| t.nameables()).collect()
    }
}

impl<K, V> Rename for std::collections::HashMap<K, V>
where
    K: Rename,
    V: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter()
            .flat_map(|(k, v)| k.nameables().into_iter().chain(v.nameables()))
            .collect()
    }
}

impl<K, V> Rename for std::collections::BTreeMap<K, V>
where
    K: Rename,
    V: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.iter()
            .flat_map(|(k, v)| k.nameables().into_iter().chain(v.nameables()))
            .collect()
    }
}

impl<A, B> Rename for (A, B)
where
    A: Rename,
    B: Rename,
{
    fn nameables(&self) -> Vec<AnyNameable> {
        self.0
            .nameables()
            .into_iter()
            .chain(self.1.nameables())
            .collect()
    }
}

impl<A, B, C> Rename for (A, B, C)
where
    A: Rename,
    B: Rename,
    C: Rename,
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

impl<A, B, C, D> Rename for (A, B, C, D)
where
    A: Rename,
    B: Rename,
    C: Rename,
    D: Rename,
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

impl<A, B, C, D, E> Rename for (A, B, C, D, E)
where
    A: Rename,
    B: Rename,
    C: Rename,
    D: Rename,
    E: Rename,
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

impl<A, B, C, D, E, F> Rename for (A, B, C, D, E, F)
where
    A: Rename,
    B: Rename,
    C: Rename,
    D: Rename,
    E: Rename,
    F: Rename,
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

impl<A, B, C, D, E, F, G> Rename for (A, B, C, D, E, F, G)
where
    A: Rename,
    B: Rename,
    C: Rename,
    D: Rename,
    E: Rename,
    F: Rename,
    G: Rename,
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

impl<A, B, C, D, E, F, G, H> Rename for (A, B, C, D, E, F, G, H)
where
    A: Rename,
    B: Rename,
    C: Rename,
    D: Rename,
    E: Rename,
    F: Rename,
    G: Rename,
    H: Rename,
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
