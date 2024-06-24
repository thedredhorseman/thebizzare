### README.md

# The biZZare

The biZZare is a decentralized application (Dapp) inspired by the early 2000s internet, built with Rust and leveraging the libp2p framework for peer-to-peer communication. This project aims to recreate the nostalgic experience of the early internet with modern, secure technology.

## Current Features

- **Peer-to-Peer Communication**: Uses libp2p for secure, decentralized communication.
- **Gossipsub**: Implements the Gossipsub protocol for message propagation.
- **Kademlia DHT**: Uses Kademlia for distributed hash table (DHT) functionality.
- **MDNS Discovery**: Enables peer discovery on the local network via mDNS.
- **HTTP Proxy**: Provides an HTTP proxy for accessing the network from traditional web browsers.

## How to Build and Run

### Prerequisites

- Rust (latest stable version)
- Cargo (latest stable version)

### Building the Project

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/thebizzare.git
   cd thebizzare
   ```

2. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

### Running the Project

1. Generate a configuration file:

   ```sh
   cargo run -- generate-config
   ```

   This will create a `config.toml` file with default settings.

2. Start the node:

   ```sh
   cargo run -- start
   ```

   The node will start, and you will see log output indicating its status.

3. (Optional) Announce content:

   ```sh
   cargo run -- announce <content_id>
   ```

   Replace `<content_id>` with the ID of the content you wish to announce on the network.

## Contributing

Contributions are welcome! Please submit issues and pull requests on the [GitHub repository](https://github.com/thedredhorseman/thebizzare).

## License

This project is licensed under the AGPL3.

---

Feel free to customize this README further to fit the specifics of your project, such as adding more detailed instructions or links to additional resources.