# Mini Redis Client and Server

## Introduction

This project is the learning journey into the Rust programming language, focusing on asynchronous programming and network programming. With the interest in understanding the internals of how high-performance networked applications work, I have built a mini Redis client and server based on the [tokio mini-redis repository](https://github.com/tokio-rs/mini-redis/tree/master) and this [tokio tutorial](https://tokio.rs/tokio/tutorial). Through this project, leveraging the Tokio runtime, I've explored the complexities of network programming, and async programming paradigms that power modern IO-heavy applications.

## Project Overview

The Mini Redis Client and Server project is an application designed to mimic the basic functionalities of Redis, a popular in-memory data store, using Rust and the Tokio framework for asynchronous IO operations. This implementation focuses on key features of Redis, such as set and get commands. The `publisher-subscriber` feature is in the works. 

## Features

- **Async/Await Syntax**: Utilizes Rust's async/await syntax for non-blocking network IO operations.
- **Basic Redis Commands**: Supports basic commands like SET and GET, allowing for simple key-value storage functionality.
- **Tokio Runtime**: Built on top of Tokio, a powerful async runtime for Rust, enabling scalable network applications.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your machine. This project uses Rust v1.76.0.

### Installation

1. Clone the repository:
```sh
git clone https://github.com/kabilanmohanraj/mini-redis-rust.git
```

2. Navigate into the project directory:
```sh
cd mini-redis-rust/mini-redis
```

3. Build the project:
```sh
cargo build --release
```

### Running the Server

Run the server using Cargo:
```sh
cargo run --bin server
```

### Running the Client

In a separate terminal, use the client to connect to the server and execute commands:
```sh
cargo run --bin client-cli set mykey myvalue
cargo run --bin client-cli get mykey
```

To script a client (for custom post-processing) and execute commands, use this command
```sh
cargo run --bin client
```

## Learning Outcomes

- **Rust Programming**: Gained hands-on experience with Rust, focusing on its safety features and concurrency model.
- **Asynchronous Programming**: Explored async programming principles and patterns, understanding how to write non-blocking code using async/await, handling I/O as resources.
- **Network Programming**: Delved into the basics of network programming, learning about sockets, TCP/IP, and building networked services with Tokio.

## Future Work

- **Command Expansion**: Adding support for more Redis commands to enhance the functionality of the mini Redis server.
- **Performance Optimization**: Investigating opportunities for performance improvements and optimizations in the client-server communication.
- **Authentication and Security**: Implementing basic security features, such as client authentication and encrypted communication.
