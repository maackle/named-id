#![cfg(test)]

use super::*;
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
struct TestId(u64);

impl AliasedId for TestId {
    fn shortener(&self) -> Option<Shortener> {
        Some(Shortener {
            length: 4,
            prefix: "ID",
        })
    }
}

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
    assert_eq!(idz.aliased().to_string(), "TestId(987654321)");
}

#[test]
fn test_aliased_id_vec() {
    let v = vec![
        TestId(11111111).with_short(),
        TestId(22222222).with_short(),
        TestId(33333333).with_short(),
    ];
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
        (
            TestId(11111111).with_short(),
            vec![TestId(22222222).with_short(), TestId(55555555).with_short()],
        ),
        (
            TestId(22222222).with_short(),
            vec![TestId(33333333).with_short(), TestId(66666666).with_short()],
        ),
        (
            TestId(33333333).with_short(),
            vec![TestId(44444444).with_short(), TestId(77777777).with_short()],
        ),
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
