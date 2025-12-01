use crate::{Aliased, bracketed};

pub trait Aliasable: Sized {
    fn display(&self) -> String;

    fn aliased(self) -> Aliased<Self> {
        self.into()
    }
}

impl<T> Aliasable for Vec<T>
where
    T: Aliasable,
{
    fn display(&self) -> String {
        bracketed(
            &self
                .iter()
                .map(|t| t.display())
                .collect::<Vec<_>>()
                .join(", "),
            ("[", "]"),
        )
    }
}

impl<T> Aliasable for std::collections::HashSet<T>
where
    T: Aliasable,
{
    fn display(&self) -> String {
        bracketed(
            &self
                .iter()
                .map(|t| t.display())
                .collect::<Vec<_>>()
                .join(", "),
            ("{", "}"),
        )
    }
}

impl<T> Aliasable for std::collections::BTreeSet<T>
where
    T: Aliasable,
{
    fn display(&self) -> String {
        bracketed(
            &self
                .iter()
                .map(|t| t.display())
                .collect::<Vec<_>>()
                .join(", "),
            ("{", "}"),
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::{Aliasable, AliasedId, ShortId};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestId(u64);

    impl ShortId for TestId {
        const PREFIX: &'static str = "ID";

        fn to_short_string(&self) -> String {
            format!("{}", self.0)
        }
    }

    impl AliasedId for TestId {}

    #[test]
    fn test_aliased_id() {
        let v = vec![TestId(11111111), TestId(22222222), TestId(33333333)];
        let a = v.aliased();
        assert_eq!(format!("{a}"), "[⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]");
        assert_eq!(format!("{a:?}"), "[⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]");

        let s = std::collections::BTreeSet::from([
            TestId(11111111),
            TestId(22222222),
            TestId(33333333),
        ]);
        let a = s.aliased();
        assert_eq!(format!("{a}"), "{⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫}");
        assert_eq!(format!("{a:?}"), "{⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫}");
    }
}
