extern crate args;
extern crate getopts;
extern crate kademlia_dht;
extern crate sha3;

use getopts::Occur;
use std::process::exit;

use args::Args;

use kademlia_dht::{Key, Node, NodeData};
use sha3::{Digest, Sha3_256};
use std::thread;
use std::time::Duration;

fn main() {
    let mut args = Args::new("Kademlia DHT", "Kademlia DHT implementation");
    // listen ip port
    args.option("l", "listen", "listen on ip:port", "IP:PORT", Occur::Req, None);
    // optional remote ip port
    args.option("r", "remote", "remote node ip:port", "IP:PORT", Occur::Optional, None);
    if let Err(e) = args.parse(std::env::args()) {
        eprintln!("{}", e);
        exit(1);
    }
    let listen : String = args.value_of("listen").unwrap();
    let remote : Option<String> = args.value_of("remote").ok();

    // split ip:port
    let listen : Vec<&str> = listen.split(":").collect();

    if listen.len() != 2 {
        eprintln!("Invalid listen address");
        exit(1);
    }

    let mut node = match remote.clone() {
        None => Node::new(listen[0], listen[1], None),
        Some(remote_str) => {
            let remote_addr_hash_vec = sha3::Sha3_256::digest(remote_str.as_bytes()).as_slice().to_vec();
            Node::new(listen[0], listen[1], Some(NodeData {
                addr: remote_str,
                id: Key::new(<[u8; 32]>::try_from(remote_addr_hash_vec).unwrap()),
            }))
        }
    };

    let key = get_key("Hello");
    let value = "World";

    if remote.is_none() {
        node.insert(key, value);
    }

    thread::sleep(Duration::from_millis(5000));

    let result = node.get(&key);
    println!("Result: {:?}", result);
    thread::sleep(Duration::from_millis(5000));

    node.kill();
}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Sized + Default + AsMut<[T]>,
        T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

fn get_key(key: &str) -> Key {
    Key(clone_into_array(Sha3_256::digest(key.as_bytes()).as_slice()))
}