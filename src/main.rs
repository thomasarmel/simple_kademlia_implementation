extern crate args;
extern crate getopts;
extern crate kademlia_dht;
extern crate sha3;

use getopts::Occur;
use std::process::exit;

use args::Args;

use kademlia_dht::{Key, Node, NodeData};
use sha3::{Digest, Sha3_256};
use std::io;

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

    let mut node = match remote {
        None => Node::new(listen[0], listen[1], None),
        Some(remote_str) => {
            let remote_addr_hash_vec = sha3::Sha3_256::digest(remote_str.as_bytes()).as_slice().to_vec();
            Node::new(listen[0], listen[1], Some(NodeData {
                addr: remote_str,
                id: Key::new(<[u8; 32]>::try_from(remote_addr_hash_vec).unwrap()),
            }))
        }
    };

    println!("Commands");
    println!("\"I key value\" => store key value");
    println!("\"G key\" => get key");
    println!("\"Q\" => Quit");
    let stdin = io::stdin();
    let mut command_buffer = String::new();

    loop {
        stdin.read_line(&mut command_buffer).expect("Failed to read line");
        let command : Vec<&str> = command_buffer.trim().split(" ").collect();

        match command[0] {
            "I" => {
                if command.len() != 3 {
                    println!("Invalid command");
                    command_buffer.clear();
                    continue;
                }
                let key_hash_vec = Sha3_256::digest(command[1].as_bytes()).as_slice().to_vec();
                let key = Key::new(<[u8; 32]>::try_from(key_hash_vec).unwrap());
                node.insert(key, command[2]);
            }
            "G" => {
                if command.len() != 2 {
                    println!("Invalid command");
                    command_buffer.clear();
                    continue;
                }
                let key_hash_vec = Sha3_256::digest(command[1].as_bytes()).as_slice().to_vec();
                let key = Key::new(<[u8; 32]>::try_from(key_hash_vec).unwrap());
                match node.get(&key) {
                    Some(value) => println!("{}", value),
                    None => println!("Not found"),
                }
            }
            "Q" => {
                println!("Quitting...");
                break;
            }
            _ => {
                println!("Invalid command");
            }
        }
        command_buffer.clear();
    }
}