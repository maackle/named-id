#![cfg(test)]

use std::fmt::Display;

use super::*;
use pretty_assertions::assert_eq;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
struct Num(u64);

impl Num {
    pub fn sh(n: u64) -> Self {
        Self(n).with_short()
    }
}

impl Nameable for Num {
    fn shortener(&self) -> Option<Shortener> {
        Some(Shortener {
            length: 4,
            prefix: "ID",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hex([u8; 32]);

impl Hex {
    pub fn sh(n: u8) -> Self {
        Self([n; 32]).with_short()
    }
}

impl Nameable for Hex {
    fn shortener(&self) -> Option<Shortener> {
        Some(Shortener {
            length: 4,
            prefix: "X",
        })
    }
}

impl Display for Hex {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&hex::encode(self.0))
    }
}

#[test]
fn test_short_id() {
    tracing_subscriber::fmt::init();

    let id1 = Num(1234567890);
    let id2 = Num(2345678901);
    let id3 = Num(3456789012);
    let idx = Num(12349876);
    assert_eq!(id1.short(), "ID|1234");
    assert_eq!(id2.short(), "ID|2345");
    assert_eq!(id3.short(), "ID|3456");

    assert_eq!(idx.short(), "ID|1234");
}

#[test]
fn test_named_id() {
    let id1 = Num(1234567890).with_name("foo");
    let id2 = Num(2345678901).with_name("bar");
    let id3 = Num(3456789012).with_name("baz");
    let idx = Num(12349876).with_name("qux");
    let idz = Num(987654321);

    assert_eq!(id1.renamed().to_string(), "⟪ID|1234|foo⟫");
    assert_eq!(id2.renamed().to_string(), "⟪ID|2345|bar⟫");
    assert_eq!(id3.renamed().to_string(), "⟪ID|3456|baz⟫");

    assert_eq!(idx.renamed().to_string(), "⟪ID|1234|qux⟫");
    assert_eq!(idz.renamed().to_string(), "Num(987654321)");
}

#[test]
fn test_named_id_vec() {
    let v = vec![
        Num(11111111).with_short(),
        Num(22222222).with_short(),
        Num(33333333).with_short(),
    ];
    let a = v.renamed();
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

    let s = std::collections::BTreeSet::from([Num(11111111), Num(22222222), Num(33333333)]);
    let a = s.renamed();
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
fn test_named_id_maps() {
    let s = std::collections::BTreeMap::from([
        (
            Num(11111111).with_short(),
            vec![Num(22222222).with_short(), Num(55555555).with_short()],
        ),
        (
            Num(22222222).with_short(),
            vec![Num(33333333).with_short(), Num(66666666).with_short()],
        ),
        (
            Num(33333333).with_short(),
            vec![Num(44444444).with_short(), Num(77777777).with_short()],
        ),
    ]);
    let a = s.renamed();
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

#[test]
fn test_deep_nesting() {
    use named_id_derive::Nameables;

    #[derive(Debug, Clone, Nameables)]
    enum A {
        Nums(Vec<Num>),
        Hex(Hex),
        #[nameables(skip)]
        Skip(Num),
    }

    #[derive(Debug, Clone, Nameables)]
    struct B {
        a: A,
        #[nameables(skip)]
        x: u32,
    }

    #[derive(Debug, Clone, Nameables)]
    struct C {
        aa: Vec<A>,
        bb: Vec<B>,
    }

    let c = C {
        aa: vec![
            A::Nums(vec![
                Num::sh(11111111),
                Num::sh(22222222),
                Num::sh(33333333),
            ]),
            A::Hex(Hex::sh(1)),
            A::Skip(Num(99999999)),
        ],
        bb: vec![
            B {
                a: A::Nums(vec![
                    Num::sh(11111111),
                    Num::sh(22222222),
                    Num::sh(33333333),
                ]),
                x: 1234567890,
            },
            B {
                a: A::Hex(Hex::sh(2)),
                x: 1234567890,
            },
        ],
    };
    let a = c.renamed();
    assert_eq!(
        format!("{a}"),
        "C { aa: [Nums([⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]), Hex(⟪X|0101⟫), Skip(Num(99999999))], bb: [B { a: Nums([⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]), x: 1234567890 }, B { a: Hex(⟪X|0202⟫), x: 1234567890 }] }"
    );
    assert_eq!(
        format!("{a:?}"),
        "C { aa: [Nums([⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]), Hex(⟪X|0101⟫), Skip(Num(99999999))], bb: [B { a: Nums([⟪ID|1111⟫, ⟪ID|2222⟫, ⟪ID|3333⟫]), x: 1234567890 }, B { a: Hex(⟪X|0202⟫), x: 1234567890 }] }"
    );
    assert_eq!(
        format!("{a:#?}"),
        "
C {
    aa: [
        Nums(
            [
                ⟪ID|1111⟫,
                ⟪ID|2222⟫,
                ⟪ID|3333⟫,
            ],
        ),
        Hex(
            ⟪X|0101⟫,
        ),
        Skip(
            Num(
                99999999,
            ),
        ),
    ],
    bb: [
        B {
            a: Nums(
                [
                    ⟪ID|1111⟫,
                    ⟪ID|2222⟫,
                    ⟪ID|3333⟫,
                ],
            ),
            x: 1234567890,
        },
        B {
            a: Hex(
                ⟪X|0202⟫,
            ),
            x: 1234567890,
        },
    ],
}
        "
        .trim()
    );
}
