use super::*;

// macro_rules! empty_nameables {
//     ($ty:ty) => {
//         impl Nameables for $ty {
//             fn nameables(&self) -> Vec<AnyNameable> {
//                 vec![]
//             }
//         }
//     };
// }

// empty_nameables!(());
// empty_nameables!(u8);
// empty_nameables!(u16);
// empty_nameables!(u32);
// empty_nameables!(u64);
// empty_nameables!(u128);
// empty_nameables!(usize);
// empty_nameables!(i8);
// empty_nameables!(i16);
// empty_nameables!(i32);
// empty_nameables!(i64);
// empty_nameables!(i128);
// empty_nameables!(isize);
// empty_nameables!(f32);
// empty_nameables!(f64);
// empty_nameables!(bool);
// empty_nameables!(char);
// empty_nameables!(String);
// empty_nameables!(std::path::PathBuf);
// // empty_nameables!(Uri);
// // empty_nameables!(Url);
// // empty_nameables!(Uuid);
// // empty_nameables!(DateTime<Utc>);
// // empty_nameables!(DateTime<Local>);
// // empty_nameables!(DateTime<FixedOffset>);
// // empty_nameables!(DateTime<TimeZone>);
// // empty_nameables!(DateTime<Tz>);
// // empty_nameables!(DateTime<Tz>);

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
