# Simple Kademlia DHT implementation

This is a simple implementation of the Kademlia DHT protocol.

## Building

`cargo build --release`

## Running

### First node

`./target/release/kademlia -l ip:port`

### Other nodes

`./target/release/kademlia -l ip:port -r remote_ip:remote_port`

## Usage

### Storing a value

`I key value`

### Retrieving a value

`G key`

### Quitting

`Q`