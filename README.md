# Rust Chat Server with Tokio

This is a simple chat server written in Rust
 using the Tokio library. It allows multiple u
sers to connect to the server and chat with 
each other in real-time.

Requirements
 Rust 1.54.0 or laterCargo (Rust's package manager)
Installation Clone this repository using git clone https://github.com/your_username/rust-chat-server.git.Navigate to the cloned repository using cd rust-chat-server.Build the project using cargo build.Run the server using cargo run.UsageStart the server by running cargo run.Connect to the server using a client such as Telnet or Netcat. The server listens on port 8080 by default.Chat with other users who are connected to the server.Configuration

The server's configuration can be changed 
by editing the config.toml file. The follo
wing options are available:

host: the IP address or hostname that the server listens on. Default is "0.0.0.0".port: the port number that the server listens on. Default is 8080.max_connections: the maximum number of concurrent connections that the server allows. Default is 100.Contributing

Contributions are welcome! Please
 see the contributing guidelines for more information.

License

This project is licensed under the MIT License.

