use super::*;
use crate::encode::write_compound_tag;

#[test]
fn test_hello_world_write() {
    let mut hello_world = CompoundTag::named("hello world");
    hello_world.insert_str("name", "Bananrama");

    let mut vec = Vec::new();
    write_compound_tag(&mut vec, &hello_world).unwrap();

    assert_eq!(
        vec,
        include_bytes!("../../test/binary/hello_world.dat").to_vec()
    );
}

#[test]
fn test_servers_write() {
    let mut server = CompoundTag::new();

    server.insert_str("ip", "localhost:25565");
    server.insert_str("name", "Minecraft Server");
    server.insert_bool("hideAddress", true);

    let servers = vec![server];

    let mut root_tag = CompoundTag::new();
    root_tag.insert_compound_tag_vec("servers", servers);

    let mut vec = Vec::new();
    write_compound_tag(&mut vec, &root_tag).unwrap();

    assert_eq!(vec, include_bytes!("../../test/binary/servers.dat").to_vec());
}
