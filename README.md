# Overview

`rust-be-websockets` is a boilerplate template for a websocket backend service written in Rust. It provides a solid foundation with essential dependencies and configurations to streamline websocket backend development.

## Features


- **Actix-Web** for building RESTful APIs
- **GitHub Actions CI** with offline SQLx query validation and test runner


## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/patrikduch/rust-be-websockets.git
cd rust-be-websockets
```

### 2. Build and Run the Application

```sh
cargo build
cargo run
```


docker build -t rust-be-template .

docker run --name rust-be-container -p 8080:8080 --env RUST_LOG=debug rust-be-template

docker-compose up -d

docker build -t rust-be-template .

docker tag rust-be-template:latest patrikduch/rust-be-template:latest

docker push patrikduch/rust-be-template:latest

cargo fix --allow-dirty --tests