use crate::decode::read_compound_tag;

#[test]
fn test_hello_world_read() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../../test/binary/hello_world.dat").to_vec());
    let hello_world = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(hello_world.name.as_ref().unwrap(), "hello world");
    assert_eq!(hello_world.get_str("name").unwrap(), "Bananrama");
}

#[test]
fn test_servers_read() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../../test/binary/servers.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert!(root_tag.name.as_ref().unwrap().is_empty());
    let servers = root_tag.get_compound_tag_vec("servers").unwrap();
    assert_eq!(servers.len(), 1);

    let server = servers[0];
    let ip = server.get_str("ip").unwrap();
    let name = server.get_str("name").unwrap();
    let hide_address = server.get_bool("hideAddress").unwrap();

    assert_eq!(ip, "localhost:25565");
    assert_eq!(name, "Minecraft Server");
    assert!(hide_address);
}

#[test]
#[allow(clippy::excessive_precision)]
fn test_big_test_read() {
    use std::io::Cursor;

    let mut cursor = Cursor::new(include_bytes!("../../test/binary/bigtest.dat").to_vec());
    let root_tag = read_compound_tag(&mut cursor).unwrap();

    assert_eq!(root_tag.name.as_ref().unwrap(), "Level");
    assert_eq!(root_tag.get_i8("byteTest").unwrap(), i8::max_value());
    assert_eq!(root_tag.get_i16("shortTest").unwrap(), i16::max_value());
    assert_eq!(root_tag.get_i32("intTest").unwrap(), i32::max_value());
    assert_eq!(root_tag.get_i64("longTest").unwrap(), i64::max_value());
    assert_eq!(root_tag.get_f32("floatTest").unwrap(), 0.4982314705848694);
    assert_eq!(root_tag.get_f64("doubleTest").unwrap(), 0.4931287132182315);
    assert_eq!(
        root_tag.get_str("stringTest").unwrap(),
        "HELLO WORLD THIS IS A TEST STRING ÅÄÖ!"
    );

    let byte_array = root_tag.get_i8_vec("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))").unwrap();

    for i in 0..1000 {
        let value = *byte_array.get(i).unwrap();
        let expected = ((i * i * 255 + i * 7) % 100) as i8;

        assert_eq!(value, expected);
    }

    let compound_tag_vec = root_tag
        .get_compound_tag_vec("listTest (compound)")
        .unwrap();

    assert_eq!(compound_tag_vec.len(), 2);

    let list_compound_tag_1 = compound_tag_vec[0];
    let list_compound_tag_2 = compound_tag_vec[1];

    assert_eq!(
        list_compound_tag_1.get_str("name").unwrap(),
        "Compound tag #0"
    );
    assert_eq!(
        list_compound_tag_1.get_i64("created-on").unwrap(),
        1264099775885
    );
    assert_eq!(
        list_compound_tag_2.get_str("name").unwrap(),
        "Compound tag #1"
    );
    assert_eq!(
        list_compound_tag_2.get_i64("created-on").unwrap(),
        1264099775885
    );

    let nested_compound_tag = root_tag.get_compound_tag("nested compound test").unwrap();
    let egg_compound_tag = nested_compound_tag.get_compound_tag("egg").unwrap();
    let ham_compound_tag = nested_compound_tag.get_compound_tag("ham").unwrap();

    assert_eq!(egg_compound_tag.get_str("name").unwrap(), "Eggbert");
    assert_eq!(egg_compound_tag.get_f32("value").unwrap(), 0.5);
    assert_eq!(ham_compound_tag.get_str("name").unwrap(), "Hampus");
    assert_eq!(ham_compound_tag.get_f32("value").unwrap(), 0.75);
}
