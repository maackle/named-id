#![cfg(test)]

use super::*;
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
fn test_short_id() {
    tracing_subscriber::fmt::init();

    let id1 = TestId(1234567890);
    let id2 = TestId(2345678901);
    let id3 = TestId(3456789012);
    let idx = TestId(12349876);
    assert_eq!(id1.short(), "ID|1234");
    assert_eq!(id2.short(), "ID|2345");
    assert_eq!(id3.short(), "ID|3456");

    assert_eq!(idx.short(), "ID|1234");
}

#[test]
#[ignore = "can't test this because cargo test gives different TypeIds to the same struct"]
fn test_prefix_collision() {
    struct TestId2(u64);
    impl ShortId for TestId2 {
        fn prefix(&self) -> &'static str {
            "ID"
        }

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

#[test]
fn test_aliased_id() {
    let id1 = TestId(1234567890).with_alias("foo");
    let id2 = TestId(2345678901).with_alias("bar");
    let id3 = TestId(3456789012).with_alias("baz");
    let idx = TestId(12349876).with_alias("qux");
    let idz = TestId(987654321);

    assert_eq!(id1.aliased().to_string(), "⟪ID|1234|foo⟫");
    assert_eq!(id2.aliased().to_string(), "⟪ID|2345|bar⟫");
    assert_eq!(id3.aliased().to_string(), "⟪ID|3456|baz⟫");

    assert_eq!(idx.aliased().to_string(), "⟪ID|1234|qux⟫");
    assert_eq!(idz.aliased().to_string(), "⟪ID|9876⟫");
}

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

    let s =
        std::collections::BTreeSet::from([TestId(11111111), TestId(22222222), TestId(33333333)]);
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
