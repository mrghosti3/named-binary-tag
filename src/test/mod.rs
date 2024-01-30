mod decode;
mod encode;

use crate::decode::read_compound_tag;
pub(self) use crate::{CompoundTag, Tag};

#[test]
fn test_compound_tag_i8() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i8("i8", 1);

    assert_eq!(compound_tag.get_i8("i8").unwrap(), 1i8);
}

#[test]
fn test_compound_tag_bool() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_bool("bool", true);

    assert!(compound_tag.get_bool("bool").unwrap());
}

#[test]
fn test_compound_tag_i16() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i16("i16", 2);

    assert_eq!(compound_tag.get_i16("i16").unwrap(), 2i16);
}

#[test]
fn test_compound_tag_i32() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i32("i32", 3);

    assert_eq!(compound_tag.get_i32("i32").unwrap(), 3i32);
}

#[test]
fn test_compound_tag_i64() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i64("i64", 4);

    assert_eq!(compound_tag.get_i64("i64").unwrap(), 4i64);
}

#[test]
fn test_compound_tag_f32() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_f32("f32", 5.1);

    assert_eq!(compound_tag.get_f32("f32").unwrap(), 5.1f32);
}

#[test]
fn test_compound_tag_f64() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_f64("f64", 6.3322);

    assert_eq!(compound_tag.get_f64("f64").unwrap(), 6.3322f64);
}

#[test]
fn test_compound_tag_str() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_str("str", "hello world");

    assert_eq!(compound_tag.get_str("str").unwrap(), "hello world");
}

#[test]
fn test_compound_tag_nested_compound_tag() {
    let mut compound_tag = CompoundTag::new();
    let mut insert_nested_compound_tag = CompoundTag::named("nested");
    insert_nested_compound_tag.insert_i8("i8", 1);
    insert_nested_compound_tag.insert_str("str", "hello world");

    compound_tag.insert_compound_tag("nested_compound_tag", insert_nested_compound_tag);

    let get_nested_compound_tag = compound_tag
        .get_compound_tag("nested_compound_tag")
        .unwrap();

    assert_eq!(get_nested_compound_tag.get_i8("i8").unwrap(), 1i8);
    assert_eq!(
        get_nested_compound_tag.get_str("str").unwrap(),
        "hello world"
    );
}

#[test]
fn test_compound_tag_i8_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i8_vec("i8_vec", vec![0, 1]);

    let i8_vec = compound_tag.get_i8_vec("i8_vec").unwrap();
    assert_eq!(i8_vec[0], 0);
    assert_eq!(i8_vec[1], 1);
}

#[test]
fn test_compound_tag_i32_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i32_vec("i32_vec", vec![7, 8, 9]);

    let i32_vec = compound_tag.get_i32_vec("i32_vec").unwrap();

    assert_eq!(i32_vec[0], 7i32);
    assert_eq!(i32_vec[1], 8i32);
    assert_eq!(i32_vec[2], 9i32);
}

#[test]
fn test_compound_tag_i64_vec() {
    let mut compound_tag = CompoundTag::new();
    compound_tag.insert_i64_vec("i64_vec", vec![10, 11, 12]);
    let i64_vec = compound_tag.get_i64_vec("i64_vec").unwrap();

    assert_eq!(i64_vec[0], 10i64);
    assert_eq!(i64_vec[1], 11i64);
    assert_eq!(i64_vec[2], 12i64);
}

#[test]
fn test_compound_tag_str_vec() {
    let mut compound_tag = CompoundTag::new();
    let insert_str_vec = vec!["a", "b", "c"];

    compound_tag.insert_str_vec("str_vec", insert_str_vec);

    let get_str_vec = compound_tag.get_str_vec("str_vec").unwrap();
    assert_eq!(get_str_vec[0], "a");
    assert_eq!(get_str_vec[1], "b");
    assert_eq!(get_str_vec[2], "c");
}

#[test]
fn test_compound_tag_nested_compound_tag_vec() {
    let mut compound_tag = CompoundTag::new();
    let mut insert_nested_compound_tag_1 = CompoundTag::new();
    let mut insert_nested_compound_tag_2 = CompoundTag::new();

    insert_nested_compound_tag_1.insert_str("str", "test");
    insert_nested_compound_tag_2.insert_i32("i32", 222333111);

    let insert_nested_compound_tag_vec =
        vec![insert_nested_compound_tag_1, insert_nested_compound_tag_2];

    compound_tag.insert_compound_tag_vec("nested_compound_tag_vec", insert_nested_compound_tag_vec);

    let get_nested_compound_tag_vec = compound_tag
        .get_compound_tag_vec("nested_compound_tag_vec")
        .unwrap();

    let get_nested_compound_tag_1 = get_nested_compound_tag_vec[0];
    let get_nested_compound_tag_2 = get_nested_compound_tag_vec[1];

    assert_eq!(get_nested_compound_tag_1.get_str("str").unwrap(), "test");
    assert_eq!(get_nested_compound_tag_2.get_i32("i32").unwrap(), 222333111);
}

#[test]
fn test_servers_fmt() {
    let mut cursor = std::io::Cursor::new(include_bytes!("../../test/binary/servers.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        &format!("{}", root_tag),
        include_str!("../../test/text/servers.snbt")
    );
    assert_eq!(
        &format!("{:?}", root_tag),
        include_str!("../../test/text/servers.txt")
    );
}

#[test]
fn test_hello_world_fmt() {
    let mut cursor =
        std::io::Cursor::new(include_bytes!("../../test/binary/hello_world.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        &format!("{}", root_tag),
        include_str!("../../test/text/hello_world.snbt")
    );
    assert_eq!(
        &format!("{:?}", root_tag),
        include_str!("../../test/text/hello_world.txt")
    );
}

#[test]
fn test_player_fmt() {
    let mut cursor = std::io::Cursor::new(include_bytes!("../../test/binary/player.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        &format!("{}", root_tag),
        include_str!("../../test/text/player.snbt")
    );
    assert_eq!(
        &format!("{:?}", root_tag),
        include_str!("../../test/text/player.txt")
    );
}

#[test]
fn test_level_fmt() {
    use crate::decode::read_compound_tag;
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../../test/binary/level.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(
        &format!("{}", root_tag),
        include_str!("../../test/text/level.snbt")
    );
    assert_eq!(
        &format!("{:?}", root_tag),
        include_str!("../../test/text/level.txt")
    );
}

#[test]
fn test_is_empty() {
    let mut compound_tag = CompoundTag::new();
    assert!(compound_tag.is_empty());

    compound_tag.insert_i32("test", 123);
    assert!(!compound_tag.is_empty());
}

#[test]
fn test_contains_key() {
    let mut compound_tag = CompoundTag::new();
    assert!(!compound_tag.contains_key("test"));

    compound_tag.insert_i32("test", 123);
    assert!(compound_tag.contains_key("test"));
    assert!(!compound_tag.contains_key("test2"));
}

#[test]
fn test_iter() {
    // Test from_iter
    let mut compound: CompoundTag = vec![
        ("test1", Tag::Int(1)),
        ("test2", Tag::Int(2)),
        ("test3", Tag::Int(3)),
    ]
    .into_iter()
    .collect();

    // Test iter
    {
        let mut iter = compound.iter().map(|(name, tag)| {
            (
                name.as_str(),
                match tag {
                    Tag::Int(value) => *value,
                    _ => panic!(),
                },
            )
        });
        assert_eq!(iter.next(), Some(("test1", 1)));
        assert_eq!(iter.next(), Some(("test2", 2)));
        assert_eq!(iter.next(), Some(("test3", 3)));
        assert_eq!(iter.next(), None);
    }

    // Test iter_mut
    for (name, tag) in compound.iter_mut() {
        if name == "test2" {
            match tag {
                Tag::Int(value) => *value = 10,
                _ => panic!(),
            }
        }
    }

    // Test into_iter
    {
        let mut iter = compound.into_iter().map(|(name, tag)| {
            (
                name,
                match tag {
                    Tag::Int(value) => value,
                    _ => panic!(),
                },
            )
        });
        assert_eq!(iter.next(), Some((String::from("test1"), 1)));
        assert_eq!(iter.next(), Some((String::from("test2"), 10)));
        assert_eq!(iter.next(), Some((String::from("test3"), 3)));
        assert_eq!(iter.next(), None);
    }
}

#[test]
#[cfg(feature = "serde")]
fn test_serde() {
    use serde_test::{assert_tokens, Token};

    let root_tag: CompoundTag = vec![
        ("test1", Tag::Int(1)),
        ("test2", Tag::Int(2)),
        ("test3", Tag::Int(3)),
    ]
    .into_iter()
    .collect();

    assert_tokens(&root_tag, &[
        Token::Struct { name: "CompoundTag", len: 2 },
        Token::Str("name"),
        Token::None,

        Token::Str("tags"),
        Token::Map { len: Some(3) },

        Token::String("test1"),
        Token::Struct { name: "Tag", len: 2 },
        Token::Str("type"),
        Token::Str("Int"),

        Token::Str("c"),
        Token::I32(1),
        Token::StructEnd,

        Token::String("test2"),
        Token::Struct { name: "Tag", len: 2 },
        Token::Str("type"),
        Token::Str("Int"),

        Token::Str("c"),
        Token::I32(2),
        Token::StructEnd,

        Token::String("test3"),
        Token::Struct { name: "Tag", len: 2 },
        Token::Str("type"),
        Token::Str("Int"),

        Token::Str("c"),
        Token::I32(3),
        Token::StructEnd,

        Token::MapEnd,
        Token::StructEnd,
    ])
}
